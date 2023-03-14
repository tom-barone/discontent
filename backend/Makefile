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
	docker pull amazon/dynamodb-local
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
	sam deploy --config-file samconfig.production.toml --template-file template.production.yaml

deploy-dev: build
	sam deploy --config-file samconfig.development.toml --template-file template.development.yaml

clean: stop-local-database
	rm -rf __pycache__ .pytest_cache tmp

redeploy:
	sam delete
	$(MAKE) deploy

seed: install seed/hacker_news_submissions seed/public_blocklists
	poetry run python database.py generate_production_seed_data
	@echo "Upload generated seed data to s3"
	aws s3 cp seed/seed.ion s3://discontent-seed-bucket/seed.ion
	@echo "When deploying a new CloudFormation stack, the database will be loaded with this seed data"

# Secondary targets

seed/public_blocklists:
	mkdir -p seed/public_blocklists
	curl https://raw.githubusercontent.com/arosh/ublacklist-github-translation/master/uBlacklist.txt -o seed/public_blocklists/github_splogs.txt
	curl https://raw.githubusercontent.com/arosh/ublacklist-stackoverflow-translation/master/uBlacklist.txt -o seed/public_blocklists/stack_overflow_translations.txt
	curl https://raw.githubusercontent.com/franga2000/aliexpress-fake-sites/main/domains_uBlacklist.txt -o seed/public_blocklists/fake_webstores.txt
	curl https://raw.githubusercontent.com/wdmpa/content-farm-list/main/uBlacklist.txt -o seed/public_blocklists/wdmpa_content_farms.txt
	curl https://danny0838.github.io/content-farm-terminator/files/blocklist/content-farms.txt  -o seed/public_blocklists/danny0838_content_farms.txt
	curl https://danny0838.github.io/content-farm-terminator/files/blocklist/nearly-content-farms.txt -o seed/public_blocklists/danny0838_nearly_content_farms.txt
	curl https://danny0838.github.io/content-farm-terminator/files/blocklist/extra-content-farms.txt -o seed/public_blocklists/danny0838_extra_content_farms.txt
	curl https://danny0838.github.io/content-farm-terminator/files/blocklist/bad-cloners.txt -o seed/public_blocklists/danny0838_bad_cloners.txt
	aws s3 cp s3://discontent-seed-bucket/bad_sites_still_active.csv seed/bad_sites_still_active.csv 

start-lambda:
	cd lambda && $(MAKE) dev &

stop-lambda:
	cd lambda && $(MAKE) stop

start-local-database:
	@echo "Start the local database"
	docker run --rm --name dynamodb -d -p 8000:8000 amazon/dynamodb-local -jar DynamoDBLocal.jar -port 8000 -inMemory -cors '*'
	@echo "Wait a couple seconds before running the setup scripts"
	sleep 5 && poetry run python database.py setup

stop-local-database:
	docker stop dynamodb || true

# Fetch the hacker news submissions from S3
# This is better than scraping them all over again
seed/hacker_news_submissions:
	aws s3 cp --recursive s3://discontent-seed-bucket/hacker_news_submissions seed/hacker_news_submissions

guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
