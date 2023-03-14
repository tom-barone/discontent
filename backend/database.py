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
import os
import json

CONFIG_FILE = './template.production.yaml'
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


def seed_with_previous_votes():
    SEED_ENDPOINT='<enter_an_endpoint>'
    df = pd.read_csv('./seed/previous_votes.csv')
    df['link'] = df['PK'].apply(lambda x: x.replace('link#', ''))
    df['user'] = df['SK'].apply(lambda x: x.replace('user#', ''))
    df['vote_value'] = df['value'].apply(lambda x: 1 if x == '1' else -1)
    for index, row in df.iterrows():
        vote = {
            "link": {
                "hostname": row['link']
            },
            "value": row['vote_value'],
            # Generate a UUID, <3 bel
            "user_id": row['user'],
        }
        response = requests.post(f'{SEED_ENDPOINT}/v1/vote', json=vote)
        response.raise_for_status()


def create_list_of_bad_sites_and_check_if_still_active():
    bad_sites = set()

    def add_bad_site(site):
        site = site.replace(".*", "").replace("*.", "")
        bad_sites.add(f'http://{site}/')
        bad_sites.add(f'https://{site}/')

    for blocklist in [
            'bad_cloners', 'content_farms', 'extra_content_farms',
            'nearly_content_farms'
    ]:
        with open(f'./seed/public_blocklists/danny0838_{blocklist}.txt') as f:
            for line in f:
                stripped_line = line.strip()
                if stripped_line.startswith('#') or stripped_line.startswith(
                        '/') or stripped_line == '':
                    continue
                site = stripped_line.split(' ')[
                    0]  # The first part is the domain
                add_bad_site(site)
                add_bad_site(f'www.{site}')

    for blocklist in [
            'fake_webstores', 'github_splogs', 'stack_overflow_translations'
    ]:
        with open(f'./seed/public_blocklists/{blocklist}.txt') as f:
            for line in f:
                stripped_line = line.strip()
                site = stripped_line.split('/')[
                    2]  # The first part is the domain
                add_bad_site(site)

    with open('./seed/public_blocklists/wdmpa_content_farms.txt') as f:
        for line in f:
            stripped_line = line.strip()
            if not stripped_line.startswith('*://*.'):
                continue
            add_bad_site(stripped_line[6:-2])
            add_bad_site(f'www.{stripped_line[6:-2]}')

    with open('./tmp/bad_sites_to_check_with_lychee.txt', 'w') as f:
        for site in bad_sites:
            f.write(f'{site}\n')

    os.system('lychee ./tmp/bad_sites_to_check_with_lychee.txt --output '
              './tmp/lychee_output.json --format json --max-redirects 1 '
              '--max-retries 0 --max-concurrency 5000 --timeout 10')

    failed_sites = list()
    with open('./tmp/lychee_output.json') as f:
        lychee_output = json.load(f)
        failures = lychee_output['fail_map'][
            './tmp/bad_sites_to_check_with_lychee.txt']
        for site in failures:
            failed_sites.append(site['url'])

    for site in failed_sites:
        try:
            bad_sites.remove(site)
        except KeyError:
            pass

    checked_bad_sites = set()
    for site in bad_sites:
        if site.startswith('https://'):
            checked_bad_sites.add(site[8:-1])
        else:
            checked_bad_sites.add(site[7:-1])

    with open('./tmp/checked_bad_sites.txt', 'w') as f:
        for site in checked_bad_sites:
            f.write(f'{site}\n')


def generate_production_seed_data():
    """
    Process submissions from Hacker News and output them into
    the DynamoDB seed format

    1. Read the CSV and merge them into a single pandas DataFrame
    2. Map all the urls to just the domain part
    3. Combine duplicate submissions and sum their votes (drop the date column)
    4. Scale the votes so we get results between reasonable numbers
    """
    hacker_news_submissions = glob.glob(
        'seed/hacker_news_submissions/submissions_*.csv')
    bad_sites = 'seed/bad_sites_still_active.csv'
    output = 'seed/seed.ion'

    # Step 1
    df_good_sites = pd.concat([
        pd.read_csv(f, names=['date', 'link', 'votes'])
        for f in hacker_news_submissions
    ])
    df_bad_sites = pd.read_csv(bad_sites, names=['link'])

    # Step 2
    df_good_sites['old_link'] = df_good_sites['link']
    df_good_sites['link'] = df_good_sites['link'].map(
        lambda link: urlparse(link).hostname)
    # Manual fix for _.0xffff.me
    df_good_sites['link'] = df_good_sites['link'].replace(
        '_.0xffff.me', 'me.0xffff.me')

    # Because it's interesting to look at
    ranked_list = df_good_sites.groupby(['link', 'old_link'
                                         ])['votes'].sum().to_frame()
    ranked_list = ranked_list.sort_values(by='votes', ascending=False)
    ranked_list.to_csv('seed/ranked_list_of_good_sites.csv',
                       quoting=csv.QUOTE_ALL)

    # Step 3
    df_good_sites = df_good_sites.groupby(['link'])['votes'].sum().to_frame()
    df_good_sites = df_good_sites.sort_values(by='votes', ascending=False)
    df_good_sites = df_good_sites.reset_index()

    # Step 4
    new_min = 25
    new_max = 50
    current_min = df_good_sites['votes'].min()
    current_max = df_good_sites['votes'].max()
    df_good_sites['scaled_votes'] = ((new_max - new_min) *
                                     (df_good_sites['votes'] - current_min) /
                                     (current_max - current_min) +
                                     new_min).astype(int)
    df_good_sites = df_good_sites.drop(columns=['votes'])
    df_bad_sites['scaled_votes'] = -20

    seed_rows = []
    created_at = '2022-07-27T12:30:00Z'
    day = '2022-07-27'
    user_id = 'beda0000-4822-4342-0990-b92d94d9489a'
    df = pd.concat([df_good_sites, df_bad_sites])

    for index, row in df.iterrows():
        sum_votes = row['scaled_votes']
        count_votes = abs(row['scaled_votes'])
        vote_value = 1 if row['scaled_votes'] > 0 else -1
        # Vote (only need one, it's inconsistent but it works)
        seed_rows.append('$ion_1_0 {Item:{' + f'PK:"link#{row["link"]}",' +
                         f'SK:"user#{user_id}",' + f'value:{vote_value}.,' +
                         f'created_at:"{created_at}",' +
                         f'UserVotes_PK:"{user_id}",' + 'entity_type:"Vote"' +
                         '}}')

        # Links
        seed_rows.append('$ion_1_0 {Item:{' + f'PK:"link#{row["link"]}",' +
                         f'SK:"link#{row["link"]}",' +
                         f'count_of_votes:{count_votes}.,' +
                         f'sum_of_votes:{sum_votes}.,' +
                         'entity_type:"LinkDetail"' + '}}')

        # Link histories
        seed_rows.append('$ion_1_0 {Item:{' + f'PK:"day#{day}",' +
                         f'SK:"link#{row["link"]}",' +
                         f'count_of_votes:{count_votes}.,' +
                         f'sum_of_votes:{sum_votes}.,' +
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
                     f'count_of_votes:{abs(df["scaled_votes"]).sum()}.,' +
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
        'generate_production_seed_data': generate_production_seed_data,
        'seed_with_previous_votes': seed_with_previous_votes
    })
