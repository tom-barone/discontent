# Hacker News Scraper

This simple python script scrapes the [front page submissions list](https://news.ycombinator.com/lists) from [HackerNews](https://news.ycombinator.com/news) and gets the links & upvotes.

Seemed like a good starting point for the Discontent database.

## Requirements

- [Poetry](https://python-poetry.org)

## Usage

Setup the environment

```bash
poetry install
```

Run the script

```bash
poetry run python hacker_news_scraper.py --start_date=2023-01-10 --end_date=2023-01-29
```

Sorry for spamming your servers Hacker News. This will use a random `user-agent` on each request, but every 30 or so requests you'll be hit with a 403. Rotate your VPN to reset.
