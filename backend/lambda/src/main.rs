mod routes;
mod types;
mod validate;
mod scoring;

use aws_sdk_dynamodb::Client;
use lambda_http::{http::Method, run, service_fn, Body, Error, Request, Response};
use routes::*;
use std::env;
use tracing::*;
use tracing_subscriber::fmt;

// For logging syntax: https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging
// Set `request_handler=trace` in development to really see what's going on
const PRODUCTION_LOG_LEVEL: &str = "info";
const DEVELOPMENT_LOG_LEVEL: &str = "error,cargo_lambda=info,request_handler=debug"; 
const LOCAL_DYNAMODB_ENDPOINT: &str = "http://localhost:8000";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let environment =
        env::var("ENVIRONMENT").expect("Error: Env variable ENVIRONMENT should be set");
    let table_name = env::var("TABLE_NAME").expect("ERROR: Env variable TABLE_NAME should be set");
    let sdk_config = aws_config::load_from_env().await;
    let mut dynamo_config = aws_sdk_dynamodb::config::Builder::from(&sdk_config);

    // Setup for different environments
    match environment.as_str() {
        "production" => {
            fmt().with_env_filter(PRODUCTION_LOG_LEVEL).without_time().init();
        }
        "development" => {
            fmt().with_env_filter(DEVELOPMENT_LOG_LEVEL).init();
            dynamo_config = dynamo_config.endpoint_url(LOCAL_DYNAMODB_ENDPOINT);
        }
        _ => {
            panic!("Error: ENVIRONMENT should be set to either `production` or `development`")
        }
    }
    let dynamo_db_client = Client::from_conf(dynamo_config.build());
    debug!(
        "Loaded environment from env variable [environment={}]",
        environment
    );
    debug!("Loaded table from env variable [table={}]", table_name);
    info!(
        "Using local dynamodb. [endpoint_url={}]",
        LOCAL_DYNAMODB_ENDPOINT
    );

    run(service_fn(|request: Request| async {
        root_handler(request, &dynamo_db_client, &table_name).await
    }))
    .await
}

#[instrument(level = "trace")]
async fn root_handler(
    request: Request,
    dynamo_db_client: &Client,
    table_name: &String,
) -> Result<Response<Body>, Error> {
    let response = match (request.uri().path(), request.method()) {
        //("/v1/admin/votes", &Method::POST) => {
            //admin_votes(request, dynamo_db_client, table_name).await
        //}
        ("/v1/scores", &Method::GET) => {
            scores(request, dynamo_db_client, table_name).await
        }
        _ => {
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/plain")
                .body("Not Found".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
    match response {
        Ok(..) => response,
        // TODO: Handle the HTTP errors better than just chucking them
        // all into a 500 response
        Err(e) => {
            warn!("Could not complete request [error={:?}]", e);
            Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(format!("{:?}", e).into())
                .map_err(Box::new)?)
        }
    }
}
