
install:
	cd config && poetry install

dev:
	cargo lambda start

format:
	cargo fmt

build:
	cargo lambda build --release --arm64	

deploy: build
	sam deploy

clean:
	cargo clean

test:
	cargo test

## Secondary targets

convert-NoSQL-workbench-config: install
	cd config && poetry run cfn-flip --yaml Database-CloudFormation-Model.json Database-CloudFormation-Model.yaml
