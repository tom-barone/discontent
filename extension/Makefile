# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	npm install

dev: guard-BROWSER
	$(MAKE) -j 2 dev-webpack-watch dev-$(BROWSER)-extension

build: guard-BROWSER install
	npx webpack --mode production --config webpack.$(BROWSER).config.js

clean:
	rm -rf dist

test: install
	npx jest

# Secondary targets

dev-webpack-watch: guard-BROWSER
	npx webpack --mode development --watch --config webpack.$(BROWSER).config.js

dev-firefox-extension:
	# Open up firefox with the extension loaded and two tabs that are helpful for development
	cd dist/firefox && npx web-ext run --start-url https://www.google.com/search?q=github+en.wikipedia+twitter about:debugging

dev-chrome-extension:
	# Open up chrome with the extension loaded and two tabs that are helpful for development
	# TODO: Not very system independent :(
	/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --load-extension=./dist/chrome https://www.google.com/search?q=github+en.wikipedia+twitter chrome://extensions/ chrome-extension://kglbdhongcfkafgfgofpgaehafnbgnhd/menu/menu.html

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
