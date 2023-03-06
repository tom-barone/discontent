import fire
import boto3
import csv
from cfn_tools import load_yaml
import yaml
import requests
import glob
import pandas as pd
from urllib.parse import urlparse
from tqdm import tqdm
import amazon.ion.simpleion as ion

CONFIG_FILE = './template.yaml'
API_ENDPOINT = 'http://localhost:9000/lambda-url/request-handler'

dynamodb = boto3.client(
    'dynamodb',
    endpoint_url="http://localhost:8000",
)


def create():
    config = _load_config()

    dynamodb.create_table(
        TableName=config['TableName'],
        KeySchema=config['KeySchema'],
        AttributeDefinitions=config['AttributeDefinitions'],
        GlobalSecondaryIndexes=config['GlobalSecondaryIndexes'],
        BillingMode=config['BillingMode'],
    )


def drop():
    for table in dynamodb.list_tables()['TableNames']:
        dynamodb.delete_table(TableName=table)
        dynamodb.get_waiter('table_not_exists').wait(TableName=table)


def load_settings():
    fixture = yaml.safe_load(open('fixtures/database.yaml'))
    config = _load_config()
    dynamodb.put_item(
        TableName=config['TableName'],
        Item={
            'PK': {
                'S': 'settings'
            },
            'SK': {
                'S': 'settings'
            },
            'entity_type': {
                'S': 'Settings'
            },
            'voting_is_disabled': {
                'BOOL': fixture['settings']['voting_is_disabled']
            },
            'maximum_votes_per_user_per_day': {
                'N': str(fixture['settings']['maximum_votes_per_user_per_day'])
            },
        })
    print('Loaded initial database settings')


def load_development_votes():
    fixtures = yaml.safe_load(open('fixtures/database.yaml'))
    for link_detail in tqdm(fixtures['development_links']):
        count_of_votes = link_detail['count_of_votes']
        sum_of_votes = link_detail['sum_of_votes']
        link = link_detail['link']

        # Generate votes to fit the count and sum
        temp_sum = 0
        vote_value = 0
        for i in tqdm(range(count_of_votes)):
            if temp_sum <= sum_of_votes:
                vote_value = 1
                temp_sum += 1
            else:
                vote_value = -1
                temp_sum -= 1
            vote = {
                "link": {
                    "hostname": link
                },
                "value": vote_value,
                # Generate a UUID, <3 bel
                "user_id": f"beda{i:04}-4822-4342-0990-b92d94d9489a",
            }
            response = requests.post(f'{API_ENDPOINT}/v1/vote', json=vote)
            response.raise_for_status()
    print('Loaded development votes')


def generate_production_seed_data(input_files, output):
    """
    Process submissions from Hacker News and output them into
    the DynamoDB seed format

    1. Read the CSV and merge them into a single pandas DataFrame
    2. Map all the urls to just the domain part
    3. Combine duplicate submissions and sum their votes (drop the date column)
    4. Scale the votes so we get results between reasonable numbers

    Example usage:
        database.py generate_production_seed_data
            --input_files="seed/hacker_news_submissions/submissions_*.csv
            --output="seed/seed.csv"
    """
    submission_files = glob.glob(input_files)

    # Step 1
    df = pd.concat([
        pd.read_csv(f, names=['date', 'link', 'votes'])
        for f in submission_files
    ])

    # Step 2
    df['old_link'] = df['link']
    df['link'] = df['link'].map(lambda link: urlparse(link).hostname)
    # Manual fix for _.0xffff.me
    df['link'] = df['link'].replace('_.0xffff.me', 'me.0xffff.me')

    # Step 3
    df = df.groupby(['link', 'old_link'])['votes'].sum().to_frame()
    df = df.sort_values(by='votes', ascending=False)
    df = df.reset_index()

    # Step 4
    new_min = 25
    new_max = 50
    current_min = df['votes'].min()
    current_max = df['votes'].max()
    df['scaled_votes'] = ((new_max - new_min) * (df['votes'] - current_min) /
                          (current_max - current_min) + new_min).astype(int)
    # Because it's interesting to look at
    df.to_csv('seed/ranked_list.csv', quoting=csv.QUOTE_ALL)

    seed_rows = []
    created_at = '2022-07-27T12:30:00Z'
    day = '2022-07-27'
    user_id = 'beda0000-4822-4342-0990-b92d94d9489a'

    for index, row in df.iterrows():
        for i in range(row['scaled_votes']):
            # Votes
            seed_rows.append('$ion_1_0 {Item:{' + f'PK:"link#{row["link"]}",' +
                             f'SK:"user#{user_id}",' + 'value:1.,' +
                             f'created_at:"{created_at}",' +
                             f'UserVotes_PK:"{user_id}",' +
                             'entity_type:"Vote"' + '}}')

        # Links
        seed_rows.append('$ion_1_0 {Item:{' + f'PK:"link#{row["link"]}",' +
                         f'SK:"link#{row["link"]}",' +
                         f'count_of_votes:{row["scaled_votes"]}.,' +
                         f'sum_of_votes:{row["scaled_votes"]}.,' +
                         'entity_type:"LinkDetail"' + '}}')

        # Link histories
        seed_rows.append('$ion_1_0 {Item:{' + f'PK:"day#{day}",' +
                         f'SK:"link#{row["link"]}",' +
                         f'count_of_votes:{row["scaled_votes"]}.,' +
                         f'sum_of_votes:{row["scaled_votes"]}.,' +
                         f'DailyLinkHistory_PK:"day#{day}",'
                         'entity_type:"LinkHistory"' + '}}')

    # Users
    seed_rows.append('$ion_1_0 {Item:{' + f'PK:"user#{user_id}",' +
                     f'SK:"user#{user_id}",' + 'is_banned:true,' +
                     f'created_at:"{created_at}",' + 'entity_type:"User"' +
                     '}}')

    # User histories
    seed_rows.append('$ion_1_0 {Item:{' + f'PK:"day#{day}",' +
                     f'SK:"user#{user_id}",' + 'is_banned:true,' +
                     f'count_of_votes:{df["scaled_votes"].sum()}.,' +
                     f'sum_of_votes:{df["scaled_votes"].sum()}.,' +
                     f'DailyUserHistory_PK:"day#{day}",' +
                     'entity_type:"UserHistory"' + '}}')

    # Settings
    seed_rows.append('$ion_1_0 {Item:{' + 'PK:"settings",' + 'SK:"settings",' +
                     'voting_is_disabled:false,' +
                     'maximum_votes_per_user_per_day:10.,' +
                     'entity_type:"Settings"' + '}}')

    for item in seed_rows:
        # Check that the items are all valid ion objects
        ion.loads(item)

    # Write seed_rows to output file
    with open(output, "w") as outfile:
        outfile.write("\n".join(seed_rows))


def setup():
    drop()
    create()
    load_settings()


def _load_config():
    with open(CONFIG_FILE) as f:
        config = load_yaml(f)
        properties = config['Resources']['Database']['Properties']
        return {
            'TableName': properties['TableName'],
            'KeySchema': properties['KeySchema'],
            'AttributeDefinitions': properties['AttributeDefinitions'],
            'GlobalSecondaryIndexes': properties['GlobalSecondaryIndexes'],
            'BillingMode': properties['BillingMode'],
        }


if __name__ == '__main__':
    fire.Fire({
        'create': create,
        'drop': drop,
        'setup': setup,
        'load_settings': load_settings,
        'load_development_votes': load_development_votes,
        'generate_production_seed_data': generate_production_seed_data
    })
