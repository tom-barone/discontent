import fire
import glob
import pandas as pd
import csv
from urllib.parse import urlparse


def process_submissions(input_files):
    """
    Process submissions from Hacker News

    This script takes the output from the hacker_news_scraper and processes it
    into something that Discontent can use. The steps are:
    1. Read the CSV and merge them into a single pandas DataFrame
    2. Map all the urls to just the domain part
    3. Combine duplicate submissions and sum their votes (drop the date column)
    4. Output the results to a CSV

    Example usage:
    process_submissions.py --input_files="output/submissions_*.csv"

    """
    submission_files = glob.glob(input_files)

    # Step 1
    df = pd.concat([
        pd.read_csv(f, names=['date', 'link', 'votes'])
        for f in submission_files
    ])

    # Step 2
    df['link'] = df['link'].map(lambda l: urlparse(l).hostname)

    # Step 3
    df = df.groupby(['link'])['votes'].sum().to_frame()
    df = df.sort_values(by='votes', ascending=False)

    # Step 4
    df.to_csv('output/processed_submissions.csv', quoting=csv.QUOTE_ALL)
    return


if __name__ == '__main__':
    # Use python-fire to give nice CLI argument parsing
    fire.Fire(process_submissions)
