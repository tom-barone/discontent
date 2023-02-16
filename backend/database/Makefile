# Primary targets

install:
	poetry install

test: install
	poetry run python -m unittest discover

seed: install seed/hacker_news_submissions seed/seed.csv
	# Upload generated seed data to s3
	aws s3 cp seed/seed.csv s3://discontent-seed-bucket/seed.csv

clean:
	rm -rf seed __pycache__ docker

# Secondary targets

seed/hacker_news_submissions:
	# Fetch the hacker news submissions from S3
	# This is better than scraping them all over again
	aws s3 cp --recursive s3://discontent-seed-bucket/hacker_news_submissions seed/hacker_news_submissions

seed/seed.csv:
	# Create seed CSV
	poetry run python database.py generate_seed_data --input_files="seed/hacker_news_submissions/submissions_*.csv" --output="seed/seed.csv"
