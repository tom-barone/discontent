import fire
import boto3
import json

CONFIG_FILE = 'config/Database-CloudFormation-Model.json'

dynamodb = boto3.client('dynamodb', endpoint_url="http://localhost:8000")


def create():
    config = _load_config()
    dynamodb.create_table(
        TableName=config['TableName'],
        KeySchema=config['KeySchema'],
        AttributeDefinitions=config['AttributeDefinitions'],
        GlobalSecondaryIndexes=config['GlobalSecondaryIndexes'],
        BillingMode=config['BillingMode'],
    )
    dynamodb.get_waiter('table_exists').wait(TableName=config['TableName'])


def drop():
    for table in dynamodb.list_tables()['TableNames']:
        dynamodb.delete_table(TableName=table)
        dynamodb.get_waiter('table_not_exists').wait(TableName=table)


# def seed():
# # Load the fixtures into the database
# # Uses the API to create the data


def setup():
    drop()
    create()
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
        config = json.load(f)
        table_name = list(config['Resources'].keys())[0]
        properties = config['Resources'][table_name]['Properties']
        return {
            'TableName': table_name,
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
    })
