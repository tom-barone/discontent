# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	npm install

dev: guard-NODE_ENV guard-BROWSER
	$(MAKE) build
	$(MAKE) -j 2 dev-webpack-watch dev-$(BROWSER)-extension

build: guard-NODE_ENV guard-BROWSER clean install
	npx webpack --config webpack.$(BROWSER).config.js

clean:
	rm -rf dist

# Secondary targets

dev-webpack-watch: guard-NODE_ENV guard-BROWSER
	npx webpack --watch --config webpack.$(BROWSER).config.js

dev-firefox-extension:
	# Open up firefox with the extension loaded and two tabs that are helpful for development
	cd dist/firefox && npx web-ext run --start-url https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter about:debugging

dev-chrome-extension:
	# Open up chrome with the extension loaded and two tabs that are helpful for development
	# TODO: Not very system independent :(
	/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --load-extension=./dist/chrome https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter chrome://extensions/

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
