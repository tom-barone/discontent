mod routes;
mod scoring;
mod types;
mod validate;

use aws_sdk_dynamodb::Client;
use lambda_http::{
    http::{Method, StatusCode},
    *,
};
use routes::*;
use std::env;
use tracing::*;
use tracing_subscriber::fmt;
use types::Config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (config, dynamo_db_client) = setup().await;
    info!("Loaded config [{:#?}]", config);

    run(service_fn(|request: Request| async {
        root_handler(request, &config, &dynamo_db_client).await
    }))
    .await
}

async fn setup() -> (Config, Client) {
    let table_name = env::var("TABLE_NAME").expect("ERROR: Env variable TABLE_NAME should be set");

    let log_level = env::var("LOG_LEVEL").expect("ERROR: Env variable LOG_LEVEL should be set");
    fmt().with_env_filter(log_level).without_time().init();

    let use_local_database = env::var("USE_LOCAL_DATABASE")
        .expect("ERROR: Env variable USE_LOCAL_DATABASE should be set")
        .parse::<bool>()
        .expect("ERROR: Env variable USE_LOCAL_DATABASE should be a boolean");
    let sdk_config = aws_config::load_from_env().await;
    let mut dynamo_config_builder = aws_sdk_dynamodb::config::Builder::from(&sdk_config);
    if use_local_database {
        dynamo_config_builder = dynamo_config_builder.endpoint_url("http://localhost:8000");
    }
    let dynamo_config = dynamo_config_builder.build();
    let dynamo_db_client = Client::from_conf(dynamo_config);

    let randomize_scores = env::var("RANDOMIZE_SCORES")
        .expect("ERROR: Env variable RANDOMIZE_SCORES should be set")
        .parse::<bool>()
        .expect("ERROR: Env variable RANDOMIZE_SCORES should be a boolean");

    return (
        Config {
            table_name,
            use_local_database,
            randomize_scores,
        },
        dynamo_db_client,
    );
}

#[instrument(level = "trace")]
async fn root_handler(
    request: Request,
    config: &Config,
    dynamo_db_client: &Client,
) -> Result<Response<Body>, Error> {
    let path = request.uri().path();
    let method = request.method();
    let response: Result<Body, Error>;
    if path == "/v1/scores" && method == &Method::GET {
        response = scores(request, &config, &dynamo_db_client).await;
    } else if path == "/v1/vote" && method == &Method::POST {
        response = vote(request, &config, &dynamo_db_client).await;
    } else {
        return not_found();
    }
    match response {
        Ok(body) => success(body),
        // TODO: Handle the HTTP errors better than just chucking them
        // all into a 500 response
        Err(e) => {
            warn!("Could not complete request [error={:#?}]", e);
            server_error(e)
        }
    }
}

fn not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::Empty)
        .unwrap())
}

fn success(body: Body) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, GET")
        .body(body)
        .unwrap())
}

fn server_error(error: Error) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("content-type", "text/plain")
        .body(format!("{:?}", error).into())
        .unwrap())
}
