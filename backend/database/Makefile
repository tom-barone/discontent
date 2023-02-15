
# Primary targets

install:
	poetry install

test: install
	poetry run python -m unittest discover

generate-seed: install
	# Create seed CSV
	poetry run python database.py generate_seed_data --input_files="seed/hacker_news_submissions/submissions_*.csv" --output="seed/seed.csv"
	# Upload to S3
	aws s3 cp seed/seed.csv s3://discontent-seed-bucket/seed.csv

# Secondary targets

# Unused targets

#stop:
	#docker-compose down

#copy-prod-to-local:
	#poetry run dynamodump --noConfirm --mode backup -a zip --region us-east-1 --srcTable TestImpo

#dev: stop
	#docker-compose up --detach
	#sleep 2s
	#$(MAKE) seed

#poetry run dynamodump --noConfirm --mode restore --region local --host localhost --port 8000 --srcTable Discontent --dumpPath seed
