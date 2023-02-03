use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdminVote {
    pub link: String,
    pub vote: i32,
    pub user_id: String,
    pub timestamp: String,
}

fn admin_vote(request: Request) -> Result<Response<Body>, Error> {
    let body = request.payload::<AdminVote>()?;
    println!("{:?}", body);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Votes! {}".into())
        .map_err(Box::new)?;
    return Ok(resp);
}
