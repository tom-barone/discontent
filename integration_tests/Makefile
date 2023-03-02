# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	bundle install

test: guard-CHROME_EXTENSION_ID install
	@echo "Check that the lambda is running"
	@curl localhost:9000 || (echo "You may need to run 'make start_lambda'" && exit 1)
	@echo "Build the lambda so it is ready to run during tests"
	rake test

clean:
	rm -rf ci

# Secondary targets

start-lambda:
	@echo "Build the lambda so it is ready to run during tests"
	cd ../backend/lambda && $(MAKE) build-dev
	@echo "Spawn an instance of the lambda in the background"
	cd ../backend/lambda && $(MAKE) dev &
	@echo "Wait 10 seconds for the lambda to start"
	sleep 10

stop-lambda:
	pkill cargo-lambda

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
