dev:
	#ENVIRONMENT=development TABLE_NAME=Discontent cargo lambda start --no-reload
	ENVIRONMENT=production TABLE_NAME=Discontent cargo lambda start --no-reload

format:
	cargo fmt

build:
	cargo lambda build --release --arm64

clean:
	cargo clean

test:
	cargo test -- --nocapture
