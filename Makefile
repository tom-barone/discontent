# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

dev: stop
	cd backend && $(MAKE) dev # will run services as background processes
	cd extension && $(MAKE) dev

build:
	cd extension && $(MAKE) build
	cd backend && $(MAKE) build

stop:
	cd backend && $(MAKE) stop # will stop the background processes

test:
	cd extension && $(MAKE) test
	cd backend && $(MAKE) test
	cd end_to_end_tests && $(MAKE) test
	@echo 'Tests succeeded'

# Secondary targets

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
