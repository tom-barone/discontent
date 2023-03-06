ifneq (,$(wildcard ../.env))
    include ../.env
    export
endif

# Variables
WEBPACK_CMD = npx webpack --mode development --config webpack.config.js --env LAMBDA_API_URL=$(LAMBDA_API_URL)

# Primary targets

install:
	npm install

dev: guard-BROWSER
	$(MAKE) -j 2 dev-webpack-watch dev-$(BROWSER)-extension

# This is used for production builds as well
# https://extensionworkshop.com/documentation/publish/source-code-submission/#use-of-obfuscated-code
build: guard-LAMBDA_API_URL clean install
	$(WEBPACK_CMD) --env BROWSER=firefox
	$(WEBPACK_CMD) --env BROWSER=chrome
	cd dist/firefox && npx web-ext build --overwrite-dest

clean:
	rm -rf dist

test: install
	npx jest

# Secondary targets

dev-webpack-watch: guard-LAMBDA_API_URL guard-BROWSER
	$(WEBPACK_CMD) --watch --env BROWSER=$(BROWSER)

dev-firefox-extension:
	# Open up firefox with the extension loaded and two tabs that are helpful for development
	cd dist/firefox && npx web-ext run --start-url https://www.google.com/search?q=github+en.wikipedia+twitter https://www.bing.com/search?q=github+en.wikipedia+twitter https://duckduckgo.com/?q=github+en.wikipedia+twitter about:debugging

dev-chrome-extension: build
	# Open up chrome with the extension loaded and two tabs that are helpful for development
	# TODO: Not very system independent :(
	/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --load-extension=./dist/chrome https://www.google.com/search?q=github+en.wikipedia+twitter https://www.bing.com/search?q=github+en.wikipedia+twitter https://duckduckgo.com/?q=github+en.wikipedia+twitter chrome://extensions/ chrome-extension://kglbdhongcfkafgfgofpgaehafnbgnhd/popup/popup.html

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
