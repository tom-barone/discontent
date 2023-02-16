install:
	cd lambda && $(MAKE) install
	cd database && $(MAKE) install

build: install
	cd lambda && $(MAKE) build

deploy: build
	sam deploy
	# Build the lambda function

redeploy:
	# Regenerating seed data
	sam delete
	$(MAKE) deploy
