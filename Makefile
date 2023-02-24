# Check if a .env file exists, and then load it
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Primary targets

install:
	cd extension && $(MAKE) install
	cd backend && $(MAKE) install

dev: install
	$(MAKE) --jobs=2 dev-extension dev-backend

# Secondary targets

dev-extension:
	cd extension && $(MAKE) dev

dev-backend:
	cd backend && $(MAKE) dev

# Guard to fail the make target if the specified env variable doesn't exist
# https://lithic.tech/blog/2020-05/makefile-wildcards
guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
