ifneq (,$(wildcard ../.env))
    include ../.env
    export
endif

# Primary targets

install:
	poetry install

dev: start
	poetry run python database.py load_development_votes

start: build stop start-local-database start-lambda
	sleep 15 # Wait a bit for everything to get up and running
stop: stop-lambda stop-local-database

build: install
	@echo "Make sure the local dynamodb is pulled and ready to run"
	docker-compose pull
	@echo "Build the lambda function"
	cd lambda && $(MAKE) build

test:
	cd lambda && $(MAKE) test
	$(MAKE) start
	@curl -s localhost:8000 > /dev/null || (echo "No local database found!" && exit 1)
	@curl -s localhost:9000 || (echo "No local lambda found!" && exit 1)
	poetry run pytest # --capture=no to see stdout
	$(MAKE) stop


deploy: build
	sam deploy

clean: stop-local-database
	rm -rf __pycache__

redeploy:
	sam delete
	$(MAKE) deploy

seed: install seed/hacker_news_submissions
	poetry run python database.py generate_production_seed_data --input_files="seed/hacker_news_submissions/submissions_*.csv" --output="seed/seed.ion"
	@echo "Upload generated seed data to s3"
	aws s3 cp seed/seed.ion s3://discontent-seed-bucket/seed.ion
	@echo "When deploying a new CloudFormation stack, the database will be loaded with this seed data"

# Secondary targets

start-lambda:
	cd lambda && $(MAKE) dev &

stop-lambda:
	cd lambda && $(MAKE) stop

start-local-database:
	@echo "Start the local database"
	docker-compose up -d
	@echo "Wait a couple seconds before running the setup scripts"
	sleep 5 && poetry run python database.py setup

stop-local-database:
	docker-compose down
	rm -rf docker

# Fetch the hacker news submissions from S3
# This is better than scraping them all over again
seed/hacker_news_submissions:
	aws s3 cp --recursive s3://discontent-seed-bucket/hacker_news_submissions seed/hacker_news_submissions

guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
