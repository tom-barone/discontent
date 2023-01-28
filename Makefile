# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	npm install

dev: guard-NODE_ENV build
	make -j 2 dev-webpack-watch dev-web-extension

build: guard-NODE_ENV clean install
	npx webpack

clean:
	rm -rf dist

# Secondary targets

dev-webpack-watch: guard-NODE_ENV
	npx webpack --watch

dev-web-extension:
	cd dist && npx web-ext run --start-url https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter about:debugging

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
