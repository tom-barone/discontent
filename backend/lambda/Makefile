# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

dev:
	cargo lambda start --no-reload

build:
	cargo lambda build --release --arm64

clean:
	cargo clean

test:
	cargo test -- --nocapture
