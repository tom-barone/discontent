import pytest
import boto3
import requests
import os
import json

API_ENDPOINT = 'http://localhost:9000/lambda-url/request-handler'
DATABASE_ENDPOINT = 'http://localhost:8000'
SETTINGS_KEY = {'PK': {'S': 'settings'}, 'SK': {'S': 'settings'}}
TABLE_NAME = os.environ['TABLE_NAME']


@pytest.fixture
def dynamodb():
    return boto3.client(
        'dynamodb',
        endpoint_url=DATABASE_ENDPOINT,
    )


def update_maximum_votes_per_user_per_day(value, dynamodb):
    dynamodb.update_item(
        TableName=TABLE_NAME,
        Key=SETTINGS_KEY,
        UpdateExpression='SET maximum_votes_per_user_per_day = :val',
        ExpressionAttributeValues={':val': {
            'N': str(value)
        }},
    )


def set_is_banned(user_id, value, dynamodb):
    dynamodb.update_item(
        TableName=TABLE_NAME,
        Key={
            'PK': {
                'S': f'user#{user_id}'
            },
            'SK': {
                'S': f'user#{user_id}'
            }
        },
        UpdateExpression='SET is_banned = :val',
        ExpressionAttributeValues={':val': {
            'BOOL': value
        }},
    )


def update_voting_is_disabled(value, dynamodb):
    dynamodb.update_item(
        TableName=TABLE_NAME,
        Key=SETTINGS_KEY,
        UpdateExpression='SET voting_is_disabled = :val',
        ExpressionAttributeValues={':val': {
            'BOOL': value
        }},
    )


def get_scores(hostnames):
    links = [{'hostname': hostname} for hostname in hostnames]
    params = {'from': json.dumps({'links': links})}
    response = requests.get(f'{API_ENDPOINT}/v1/scores', params=params)
    assert response.status_code == 200
    return [x['score'] for x in response.json()]


def vote(hostname, value, user_id):
    vote = {"link": {"hostname": hostname}, "value": value, "user_id": user_id}
    response = requests.post(f'{API_ENDPOINT}/v1/vote', json=vote)
    assert response.status_code == 200
    return


def assert_vote_fails(hostname, value, user_id, reason):
    vote = {"link": {"hostname": hostname}, "value": value, "user_id": user_id}
    response = requests.post(f'{API_ENDPOINT}/v1/vote', json=vote)
    assert response.status_code == 500
    assert response.json()['error'] == reason
    return


def test_backend(dynamodb):
    # Make sure we're using a stubbed out time
    assert os.environ['USE_SYSTEM_TIME'] == 'false'

    # Simple test to make sure we can get a score
    assert get_scores(['a.com', 'b.com']) == ['NoScore', 'NoScore']

    # Check that the Good scoring works
    for i in range(19):
        vote('good.com', 1, f"beda{i:04}-0822-4342-0990-b92d94d9489a")
    assert get_scores(['good.com']) == ['NoScore']
    for i in range(2):
        vote('good.com', 1, f"beda{i:04}-1822-4342-0990-b92d94d9489a")
    assert get_scores(['good.com']) == ['Good']
    assert get_scores(['good.com']) == ['Good']

    # Check that the Bad scoring works
    for i in range(9):
        vote('bad.com', -1, f"beda{i:04}-0822-4342-0990-b92d94d9489a")
    assert get_scores(['bad.com']) == ['NoScore']
    for i in range(2):
        vote('bad.com', -1, f"beda{i:04}-1822-4342-0990-b92d94d9489a")
    assert get_scores(['bad.com']) == ['Bad']
    assert get_scores(['bad.com']) == ['Bad']

    # Check that the Controversial scoring works
    for i in range(30):
        vote('controversial.com', -1,
             f"beda{i:04}-0822-4342-0990-b92d94d9489a")
    assert get_scores(['controversial.com']) == ['Bad']
    for i in range(25):
        vote('controversial.com', 1, f"beda{i:04}-1822-4342-0990-b92d94d9489a")
    assert get_scores(['controversial.com']) == ['Controversial']
    assert get_scores(['controversial.com']) == ['Controversial']

    # CHeck that max votes per user per day works
    for i in range(10):
        # 10 votes no worries
        vote(f"shill{i}.com", 1, "beda9999-0822-4342-0990-b92d94d9489a")
    for i in range(10):
        # Changed the votes, still no worries
        vote(f"shill{i}.com", -1, "beda9999-0822-4342-0990-b92d94d9489a")
    for i in range(10, 15):
        # 11th vote for new sites fail
        assert_vote_fails(f"shill{i}.com", 1,
                          "beda9999-0822-4342-0990-b92d94d9489a",
                          "User has voted too many times today")
    update_maximum_votes_per_user_per_day(15, dynamodb)
    for i in range(10, 15):
        # Can now have 15 a day, so no worries
        vote(f"shill{i}.com", 1, "beda9999-0822-4342-0990-b92d94d9489a")
    for i in range(16, 20):
        # Any more though will fail
        assert_vote_fails(f"shill{i}.com", 1,
                          "beda9999-0822-4342-0990-b92d94d9489a",
                          "User has voted too many times today")
    update_maximum_votes_per_user_per_day(10, dynamodb)

    # Check that voting can be disabled across the board
    update_voting_is_disabled(True, dynamodb)
    for i in range(5):
        assert_vote_fails('good.com', 1,
                          f"beda{i:04}-0822-4342-0990-b92d94d9489a",
                          "Voting is disabled")
        assert_vote_fails('bad.com', 1,
                          f"beda{i:04}-0822-4342-0990-b92d94d9489a",
                          "Voting is disabled")
        assert_vote_fails('controversial.com', 1,
                          f"beda{i:04}-0822-4342-0990-b92d94d9489a",
                          "Voting is disabled")
        assert_vote_fails('random-site.com', 1,
                          f"beda{i:04}-0822-4342-0990-b92d94d9489a",
                          "Voting is disabled")
    update_voting_is_disabled(False, dynamodb)

    # Check that banned users can't vote
    user = "beda0000-5822-4342-0990-b92d94d9489a"
    vote('good.com', 1, user)  # all good
    set_is_banned(user, True, dynamodb)
    assert_vote_fails('good.com', 1, user, "User is banned")
    assert_vote_fails('bad.com', 1, user, "User is banned")
    assert_vote_fails('other.com', 1, user, "User is banned")
    set_is_banned(user, False, dynamodb)
    vote('good.com', -1, user)  # all good again
    vote('bad.com', -1, user)  # all good again
    vote('other.com', -1, user)  # all good again

    # TODO: test incorrectly formatted requests
