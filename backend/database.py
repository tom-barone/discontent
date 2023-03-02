import fire
import boto3
import csv
from cfn_tools import load_yaml
import yaml
import requests
# import aiohttp
import glob
import pandas as pd
from urllib.parse import urlparse
from tqdm import tqdm

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


def generate_from_fixtures():
    fixtures = yaml.safe_load(open('fixtures/database.yaml'))
    for link_detail in tqdm(fixtures['links']):
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
                "vote_value": vote_value,
                # Generate a UUID, <3 bel
                "user_id": f"BEDA{i:04}-4822-4342-0990-b92d94d9489a",
            }
            response = requests.post(f'{API_ENDPOINT}/v1/vote', json=vote)
            response.raise_for_status()


# def seed():
# with open('fixtures/seed_votes.csv', 'r') as f:
# reader = csv.DictReader(f)
# admin_votes = []
# for daily_summary in reader:
# total_votes = int(daily_summary['scaled_votes'])
# vote_value = 1 if total_votes > 0 else -1
# for i in range(total_votes):
# admin_votes.append({
# "timestamp": "2023-02-02T09:36:03Z",
# "vote": {
# "link": daily_summary['link'],
# "vote_value": vote_value,
# "user_id": f"BEDA{i:04}-4822-4342-0990-b92d94d9489a",
# }
# })
# print(daily_summary['link'])

# response = requests.post(f'{API_ENDPOINT}/admin/votes',
# json=admin_votes)
# print(response.text)


def generate_seed_data(input_files, output):
    """
    Process submissions from Hacker News and output them into
    the DynamoDB seed format

    1. Read the CSV and merge them into a single pandas DataFrame
    2. Map all the urls to just the domain part
    3. Combine duplicate submissions and sum their votes (drop the date column)
    4. Scale the votes so we get results between reasonable numbers

    Example usage:
        database.py generate_seed_data
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
    df['link'] = df['link'].map(lambda l: urlparse(l).hostname)
    # Manual fix for _.0xffff.me
    df['link'] = df['link'].replace('_.0xffff.me', 'me.0xffff.me')

    # Step 3
    df = df.groupby(['link'])['votes'].sum().to_frame()
    df = df.sort_values(by='votes', ascending=False)
    df = df.reset_index()

    # Step 4
    new_min = 25
    new_max = 50
    current_min = df['votes'].min()
    current_max = df['votes'].max()
    df['scaled_votes'] = ((new_max - new_min) * (df['votes'] - current_min) /
                          (current_max - current_min) + new_min).astype(int)

    seed_rows = []
    user = 'BEDA0000-4822-4342-0990-b92d94d9489a'
    for index, row in df.iterrows():
        for i in range(row['scaled_votes']):
            # Votes
            seed_rows.append({
                'PK': f'link#{row["link"]}',
                'SK': f'user#{user}',
                'entity_type': 'Vote',
                'vote_value': '1',
                'vote_timestamp': '2022-07-27:12:00Z',
                'UserVotes_PK': user
            })

        # Links
        seed_rows.append({
            'PK': f'link#{row["link"]}',
            'SK': f'link#{row["link"]}',
            'entity_type': 'Link',
            'count_of_votes': row['scaled_votes'],
            'sum_of_votes': row['scaled_votes']
        })

        # Link histories
        seed_rows.append({
            'PK': 'day#2023-01-10',
            'SK': f'link#{row["link"]}',
            'entity_type': 'LinkHistory',
            'count_of_votes': row['scaled_votes'],
            'sum_of_votes': row['scaled_votes'],
            'DailyLinkHistory_PK': 'day#2023-01-10',
        })

    # Users
    seed_rows.append({
        'PK': f'user#{user}',
        'SK': f'user#{user}',
        'entity_type': 'User',
        'user_is_banned': True,  # This user can't vote
        'user_notes': 'Initial HackerNews seed'
    })

    # User histories
    seed_rows.append({
        'PK': 'day#2023-01-10',
        'SK': f'user#{user}',
        'entity_type': 'UserHistory',
        'count_of_votes': df['scaled_votes'].sum(),
        'sum_of_votes': df['scaled_votes'].sum(),
        'DailyUserHistory_PK': 'day#2023-01-10'
    })

    # Settings
    seed_rows.append({
        'PK': 'settings',
        'SK': 'settings',
        'entity_type': 'Settings',
        'voting_is_disabled': False
    })

    seed_df = pd.DataFrame.from_dict(seed_rows)
    seed_df.to_csv(output, quoting=csv.QUOTE_ALL, index=False)


def setup():
    drop()
    create()
    generate_from_fixtures()
    # print(table['TableNames'])
    # print(dynamodb.delete_table(TableName=table))

    # # Create the DynamoDB table.
    # table = dynamodb.create_table(TableName='users',
    # KeySchema=[{
    # 'AttributeName': 'username',
    # 'KeyType': 'HASH'
    # }, {
    # 'AttributeName': 'last_name',
    # 'KeyType': 'RANGE'
    # }],
    # AttributeDefinitions=[
    # {
    # 'AttributeName': 'username',
    # 'AttributeType': 'S'
    # },
    # {
    # 'AttributeName': 'last_name',
    # 'AttributeType': 'S'
    # },
    # ],
    # ProvisionedThroughput={
    # 'ReadCapacityUnits': 5,
    # 'WriteCapacityUnits': 5
    # })

    # # Wait until the table exists.
    # table.wait_until_exists()

    # # Print out some data about the table.
    # table = dynamodb.Table('users')

    # table.put_item(
    # Item={
    # 'username': 'janedoe',
    # 'first_name': 'Jane',
    # 'last_name': 'Doe',
    # 'age': 25,
    # 'account_type': 'standard_user',
    # })

    # # Print out some data about the table.
    # # This will cause a request to be made to DynamoDB and its attribute
    # # values will be set based on the response.
    # print(table.creation_date_time)
    # print(list(dynamodb.tables.all()))
    # print(table.item_count)


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
        'generate_from_fixtures': generate_from_fixtures,
    })
