# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	bundle install

test: guard-CHROME_EXTENSION_ID guard-FIREFOX_EXTENSION_ID install
	@echo "Check that the backend is running"
	@curl localhost:9000 || (echo "No local backend is running!" && exit 1)
	rake test

clean:
	rm -rf ci

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
