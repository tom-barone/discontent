# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	cd extension && npm install
	cd database/initialiser && poetry install

dev: guard-NODE_ENV build
	$(MAKE) -j 2 dev-webpack-watch dev-web-extension

build: guard-NODE_ENV clean install
	cd extension && npx webpack

clean:
	rm -rf extension/dist

test: install
	cd database/initialiser && poetry run python -m unittest discover

deploy:
	cd backend && $(MAKE) deploy

# Secondary targets

dev-webpack-watch: guard-NODE_ENV
	cd extension && npx webpack --watch

dev-web-extension:
	# Open up firefox with the extension loaded and two tabs that are helpful for development
	cd extension/dist && npx web-ext run --start-url https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter about:debugging

get-hacker-news-submissions: install
	# Used to give a good starting point for the scores database
	cd database/initialiser && poetry run python hacker_news_scraper.py --start_date=2023-01-13 --end_date=2023-01-29

process-hacker-news-submissions: install
	# Used to give a good starting point for the scores database
	cd database/initialiser && poetry run python process_submissions.py --input_files "output/submissions_*.csv"

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
