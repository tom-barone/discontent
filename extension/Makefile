ifneq (,$(wildcard ../.env))
    include ../.env
    export
endif

WEBPACK_CMD := npx webpack --mode development --config webpack.config.js --env LAMBDA_API_URL=$(LAMBDA_API_URL)
CHROME_VERSION := $(shell jq -r .version src/manifest.chrome.json)
FIREFOX_VERSION := $(shell jq -r .version src/manifest.firefox.json)

# Primary targets

install:
	npm install

dev: install guard-BROWSER
	$(MAKE) --jobs=2 dev-webpack-watch dev-$(BROWSER)-extension

build: guard-LAMBDA_API_URL clean install
	$(WEBPACK_CMD) --env BROWSER=firefox
	$(WEBPACK_CMD) --env BROWSER=chrome
	mkdir -p dist/packed
	cd dist/firefox && npx web-ext build --overwrite-dest --artifacts-dir ../packed --filename discontent-firefox-$(FIREFOX_VERSION).zip
	cd dist/chrome && zip -r ../packed/discontent-chrome-$(CHROME_VERSION).zip *

clean:
	rm -rf dist

test: install
	npx jest

# Secondary targets

dev-webpack-watch: guard-LAMBDA_API_URL guard-BROWSER
	$(WEBPACK_CMD) --watch --env BROWSER=$(BROWSER)

dev-firefox-extension:
	cd dist/firefox && npx web-ext run --start-url https://www.google.com/search?q=github+en.wikipedia+twitter https://www.bing.com/search?q=github+en.wikipedia+twitter https://duckduckgo.com/?q=github+en.wikipedia+twitter https://html.duckduckgo.com/html/?q=github+en.wikipedia+twitter about:debugging

dev-chrome-extension: #build
	# Open up chrome with the extension loaded and two tabs that are helpful for development
	# TODO: Not very system independent :(
	/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --load-extension=./dist/chrome/ https://www.google.com/search?q=github+en.wikipedia+twitter https://www.bing.com/search?q=github+en.wikipedia+twitter https://duckduckgo.com/?q=github+en.wikipedia+twitter https://html.duckduckgo.com/html/?q=github+en.wikipedia+twitter chrome://extensions/ chrome-extension://kglbdhongcfkafgfgofpgaehafnbgnhd/popup/popup.html

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
