import fire
from datetime import datetime, timedelta
import requests
from bs4 import BeautifulSoup
from tqdm import tqdm
import csv
import os
from fake_useragent import UserAgent

HACKER_NEWS_URL = "https://news.ycombinator.com/front"


# Helper function to iterate through 2 dates (including end date)
def date_range(start_date, end_date):
    for difference in range(int((end_date - start_date).days) + 1):
        yield start_date + timedelta(difference)


# Extract the submission links and return them as a list
def get_submission_links(soup):
    # All links are in a table row with class "athing"
    submission_rows = soup.find_all("tr", class_="athing")

    def extract_href(row_tag):
        href = row_tag.find("span", class_="titleline").find("a").get("href")
        # Fix relative urls that point to Hacker News itself
        if href.startswith("item?id="):
            href = f"https://news.ycombinator.com/{href}"
        return href

    return [extract_href(row) for row in submission_rows]


# Extract the number of votes given the surrounding span tag
def get_submission_votes(soup):
    # All span tags that hold the votes have the class "score"
    span_tags = soup.find_all("span", class_="score")

    def extract_votes(span_tag):
        vote_string = span_tag.get_text().strip().split(" ")
        assert len(vote_string) == 2
        assert vote_string[1] == "points"
        return int(vote_string[0])

    return [extract_votes(span) for span in span_tags]


def get_submissions(soup):
    submission_links = get_submission_links(soup)
    submission_votes = get_submission_votes(soup)

    assert len(submission_links) == 30
    assert len(submission_votes) == 30

    return list(zip(submission_links, submission_votes))


def hacker_news_scraper(start_date: str, end_date: str):
    """
    Hacker News Scraper

    Scrapes links and updates from front page submissions on Hacker News.
    Saves csv results to ./output/submissions_<start_date>_<end_date>.csv

    Example usage: hacker_news_scraper.py --start_date=2023-01-10 --end_date=2023-01-20
    """
    # Parse dates and prepare output
    start = datetime.strptime(start_date, '%Y-%m-%d')
    end = datetime.strptime(end_date, '%Y-%m-%d')
    os.makedirs('output', exist_ok=True)
    ua = UserAgent()

    with open(f'output/submissions_{start_date}_{end_date}.csv',
              'w',
              newline='') as f:
        writer = csv.writer(f, dialect='unix')
        progress = tqdm(date_range(start, end))
        for day in progress:
            day_string = day.strftime('%Y-%m-%d')
            progress.set_description(f"Scraping {day_string}")
            # Sorry
            headers = {'User-Agent': ua.random}
            r = requests.get(HACKER_NEWS_URL,
                             params={'day': day_string},
                             headers=headers)
            if r.ok:
                soup = BeautifulSoup(r.text, 'html.parser')
                submissions = get_submissions(soup)
                for submission in submissions:
                    writer.writerow([day_string, submission[0], submission[1]])
            else:
                print(
                    f"\nError: {day_string} failed with {r.status_code} {r.reason}"
                )

    return


if __name__ == '__main__':
    # Use python-fire to give nice CLI argument parsing
    fire.Fire(hacker_news_scraper)
