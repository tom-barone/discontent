# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

dev: guard-TABLE_NAME guard-LOG_LEVEL guard-USE_LOCAL_DATABASE guard-RANDOMIZE_SCORES
	cargo lambda start

build:
	cargo lambda build --release --arm64

clean:
	cargo clean

test:
	cargo test -- --nocapture

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
