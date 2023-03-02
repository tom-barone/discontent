# Primary targets

install:
	poetry install

dev: install start-local-database
	cd lambda && $(MAKE) dev

test:
	poetry run python -m unittest discover
	cd lambda && $(MAKE) test

build:
	cd lambda && $(MAKE) build

deploy: build
	sam deploy

clean: stop-local-database
	rm -rf __pycache__

redeploy:
	# Will regenerate seed data
	sam delete
	$(MAKE) deploy

seed: install seed/hacker_news_submissions seed/seed.csv
	@echo "Upload generated seed data to s3"
	aws s3 cp seed/seed.csv s3://discontent-seed-bucket/seed.csv
	@echo "When deploying a new CloudFormation stack, the database will be loaded with this seed data"

# Secondary targets

start-local-database: docker/dynamodb/shared-local-instance.db

stop-local-database:
	docker-compose down
	rm -rf docker

docker/dynamodb/shared-local-instance.db:
	@echo "Start the local database"
	docker-compose up -d
	sleep 1 # Give a second for the local database to start
	@echo "Load the database fixtures"
	poetry run python database.py setup

seed/hacker_news_submissions:
	# Fetch the hacker news submissions from S3
	# This is better than scraping them all over again
	aws s3 cp --recursive s3://discontent-seed-bucket/hacker_news_submissions seed/hacker_news_submissions

seed/seed.csv:
	poetry run python database.py generate_seed_data --input_files="seed/hacker_news_submissions/submissions_*.csv" --output="seed/seed.csv"
