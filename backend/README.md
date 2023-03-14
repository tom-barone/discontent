# Discontent Backend

The file `Database-NoSQLWorkbench-Model.json` should be opened the [NoSQL Workbench](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/workbench.html) application, and it contains some sample data to help visualise the database.

The file `Database-CloudFormation-Model.json` is an export from NoSQL Workbench and is used as the schema for AWS Cloudformation.

# Seeding the database

The problem we're up against is how to get things started?

## Good sites

Hacker News submissions seemed like a decent starting point for building a set of "good sites". See `hacker_news_scraper.py` for a python script that scrapes the [front page submissions list](https://news.ycombinator.com/lists) from [HackerNews](https://news.ycombinator.com/news) and gets the links & upvotes.

# Database

This directory contains python scripts to manage the Discontent DynamoDB.

1. Run a local instance of DynamoDB for development and testing
1. Seed the DynamoDB

```bash
poetry run python database.py setup
```
will create the database with the latest schema, and initialise it with seed data

```bash
poetry run python database.py reset
```

- drop: delete the database
- setup: create and seed
- reset: drop and setup



## Requirements

- [Poetry](https://python-poetry.org)

## Usage

Setup the environment

```bash
poetry install
```

Run the scripts

```bash
poetry run python hacker_news_scraper.py --start_date=2023-01-10 --end_date=2023-01-29
poetry run python process_submissions.py --input_files "output/submissions_*.csv"
poetry run python send_post_requests.py --input_file "output/processed_submissions.csv"
```

Sorry for spamming your servers Hacker News. `hacker_news_scraper.py` will use a random `user-agent` on each request, but every 30 or so requests you'll be hit with a 403. Rotate your VPN to reset.

I thought briefly about using their [unban API](https://news.ycombinator.com/item?id=4761102) to get around this... but that might be taking things a bit too far.
