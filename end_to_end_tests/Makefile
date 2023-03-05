ifneq (,$(wildcard ../.env))
    include ../.env
    export
endif

# Primary targets

install:
	bundle install

test: guard-CHROME_EXTENSION_ID guard-FIREFOX_EXTENSION_ID install
	cd ../extension && $(MAKE) build
	cd ../backend && $(MAKE) dev
	@echo "Check that the backend is running"
	@curl localhost:9000 || (echo "No local backend is running!" && exit 1)
	rake test > ci/test.log
	cat ci/test.log
	cd ../backend && $(MAKE) stop

clean:
	rm -rf ci

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
