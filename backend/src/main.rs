mod types;
mod routes;


//use aws_sdk_dynamodb::Client;
//use std::env;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use routes::admin_vote;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // Disabling time is handy because CloudWatch will add the ingestion time
        .without_time()
        .init();

    // Create the DynamoDB client
    //let config = aws_config::load_from_env().await;
    //let table_name = env::var("DATABASE").expect("ERROR: Env variable DATABASE should be set");
    //let dynamo_db_client = Client::new(&config);

    run(service_fn(|request: Request| async {
        root_handler(request).await
    }))
    .await
}

async fn root_handler(request: Request) -> Result<Response<Body>, Error> {
    match request.uri().path() {
        "/admin/settings" => {
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("Setings!".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
        "/admin/vote" => admin_vote(request),
        _ => {
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/plain")
                .body("Not Found".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    }
}

