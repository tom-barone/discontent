# Contributing

This project is powered by [Makefiles](https://www.gnu.org/software/make/) and environment variables.

Most normal tasks can be run with a make recipe and the correct environment variables. For example:

- `BROWSER=firefox make dev`
- `USE_SYSTEM_TIME=false make test`

For ease, it's recommended to add a `.env` file in the repository root with all your values set in there. The make recipes should complain if an environment variable is not set when it should be.

A sample `.env` for local development and testing would be:

```
BROWSER=firefox
LAMBDA_API_URL=http://localhost:9000/lambda-url/request-handler/v1
TABLE_NAME=Discontent
LOG_LEVEL=info
RANDOMIZE_SCORES=false
USE_LOCAL_DATABASE=true
USE_SYSTEM_TIME=false
HEADLESS=true
CHROME_EXTENSION_ID=kglbdhongcfkafgfgofpgaehafnbgnhd
FIREFOX_EXTENSION_ID={3f504997-80b7-467d-9d7b-e2fbb6d55e34}
```

The environment variables are:

| Variable             | Values                                                                                                                                   | Description                                                                                                             |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| BROWSER              | `chrome` or `firefox`                                                                                                                    | When running `make dev`, it will open this browser                                                                      |
| LAMBDA_API_URL       | `http://localhost:9000/lambda-url/request-handler/v1` or the production API `https://<lambda_id>.execute-api.us-east-1.amazonaws.com/v1` | The endpoint for the extension to use when looking for scores or when voting                                            |
| TABLE_NAME           | `Discontent`                                                                                                                             | The name of the database table, should always be `Discontent`                                                           |
| LOG_LEVEL            | `info`, `request_handler=trace`, ...                                                                                                     | Logging levels for the lambda. See [here](https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging) for reference |
| RANDOMIZE_SCORES     | `true` or `false`                                                                                                                        | Whether the lambda should get scores from the database or generate random ones for development                          |
| USE_LOCAL_DATABASE   | `true` or `false`                                                                                                                        | Should the local lambda look at a local database or connect to the live production database                             |
| USE_SYSTEM_TIME      | `true` or `false`                                                                                                                        | Normally true but set to false when testing. Used to produce reproducible tests                                         |
| HEADLESS             | `true` or `false`                                                                                                                        | Whether to run the end to end tests with headless browsers or not                                                       |
| CHROME_EXTENSION_ID  |                                                                                                                                          | Local extension ID, used during end to end tests                                                                        |
| FIREFOX_EXTENSION_ID |                                                                                                                                          | Local extension ID, used during end to end tests                                                                        |
| ACCESS_KEY           |                                                                                                                                          | AWS key used for deploying the backend                                                                                  |
| SECRET_ACCESS_KEY    |                                                                                                                                          | AWS key used for deploying the backend                                                                                  |

## Building the extension

You'll need: [npm](https://docs.npmjs.com/), [make](https://www.gnu.org/software/make/), [web-ext](https://github.com/mozilla/web-ext)

Recommended root `.env` file:

```
LAMBDA_API_URL=https://2zeiy58jgk.execute-api.us-east-1.amazonaws.com/v1
```

Run `cd extension && make build` in the root directory. The builds will be in:

- Firefox zip file: `<root>/extension/dist/firefox/web-ext-artifacts/discontent-<version>.zip`
- Chrome zip file: `<root>/extension/dist/chrome/discontent.zip`
