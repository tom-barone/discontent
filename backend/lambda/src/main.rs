mod routes;
mod types;

//use aws_sdk_dynamodb::config::{Builder, Config};
//use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{http::Method, run, service_fn, Body, Error, Request, Response};
use routes::admin_vote;
//use std::env;

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
    //println!("config: {:?}", config);

    //let config = Builder::new()
    //.endpoint_url(
    //// 8000 is the default dynamodb port
    //"http://localhost:8000",
    //)
    //.build();

    //let sdk_config = aws_config::load_from_env().await;
    //let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
    //.endpoint_url("http://localhost:8000")
    //.build();
    //let dynamo_db_client = Client::from_conf(dynamo_config);
    //let request = dynamo_db_client
    //.put_item()
    //.table_name("TEST")
    //.item("username", AttributeValue::S("mr person".to_string()))
    //.item("account", AttributeValue::S("place".to_string()));

    //let _resp = request.send().await?;

    run(service_fn(|request: Request| async {
        root_handler(request).await
    }))
    .await
}

async fn root_handler(request: Request) -> Result<Response<Body>, Error> {
    let sdk_config = aws_config::load_from_env().await;
    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
        .endpoint_url("http://localhost:8000")
        .build();
    let dynamo_db_client = Client::from_conf(dynamo_config);

    let response = match (request.uri().path(), request.method()) {
        // TODO: Fix issue with prod/admin/vote
        ("/admin/vote", &Method::POST) => admin_vote(request, dynamo_db_client).await,
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
        Err(e) => Ok(Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(format!("{:?}", e as lambda_http::Error).into())
            .map_err(Box::new)?),
    }
}
