# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	bundle install

test: guard-CHROME_EXTENSION_ID install
	$(MAKE) start_lambda
	@echo "Wait 10 seconds for the lambda to start"
	sleep 10
	@echo "Run the tests"
	rake test
	$(MAKE) stop_lambda

# Secondary targets

start_lambda:
	@echo "Build the lambda so it is ready to run during tests"
	cd ../backend/lambda && $(MAKE) build-dev
	@echo "Spawn an instance of the lambda in the background"
	cd ../backend/lambda && $(MAKE) dev &

stop_lambda:
	pkill cargo-lambda

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
