deploy: build
	cd lambda && $(MAKE) build
	sam deploy
	# Build the lambda function

redeploy:
	# Regenerating seed data
	sam delete
	$(MAKE) deploy
