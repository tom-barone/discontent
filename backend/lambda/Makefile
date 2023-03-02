# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

dev: guard-TABLE_NAME guard-LOG_LEVEL guard-USE_LOCAL_DATABASE guard-RANDOMIZE_SCORES
	@echo "Start the lambda locally"
	# Send a ping after 3 seconds to the cargo-lambda server to wake it up
	sleep 3 && curl "http://localhost:9000/lambda-url/request-handler/v1" &
	cargo lambda watch

build:
	cargo build
	cargo lambda build --release --arm64

clean:
	cargo clean

test:
	cargo test -- --nocapture

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
