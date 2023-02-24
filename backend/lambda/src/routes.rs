use std::collections::HashMap;

use crate::{
    scoring::*,
    types::{database::LinkDetail, Config},
    validate::validate_get_scores_request,
};
use aws_sdk_dynamodb::{
    model::{AttributeValue::*, KeysAndAttributes},
    Client,
};
use lambda_http::{Body, Error, Request, RequestExt};
use tracing::*;
use validator::Validate;

#[instrument(level = "trace")]
pub async fn scores(
    request: Request,
    config: &Config,
    dynamo_db_client: &Client,
) -> Result<Body, Error> {
    // Extract the links from the query parameters and validate them
    let scores_request = validate_get_scores_request(request.query_string_parameters())?;

    if config.randomize_scores {
        let link_scores = random_link_scores(&scores_request.links);
        let link_scores_json = serde_json::to_string(&link_scores)?;
        return Ok(link_scores_json.into());
    }

    // Combine the requests for link details into a single DynamoDB request
    let mut dynamodb_request_builder = KeysAndAttributes::builder();
    for link in &scores_request.links {
        dynamodb_request_builder = dynamodb_request_builder.keys(HashMap::from([
            ("PK".to_string(), S(format!("link#{}", link.hostname))),
            ("SK".to_string(), S(format!("link#{}", link.hostname))),
        ]));
    }

    // Send the request to DynamoDB and wait for the results
    let dynamodb_response = dynamo_db_client
        .batch_get_item()
        .request_items(&config.table_name, dynamodb_request_builder.build())
        .send()
        .await?;

    // Extract the link details
    //let mut link_details: Vec<LinkDetail> = vec![];
    let mut link_details = HashMap::new();
    for item in dynamodb_response
        .responses()
        .ok_or("DynamoDB request error")?
        .get(&config.table_name)
        .ok_or("DynamoDB request error")?
        .iter()
    {
        let link_detail = LinkDetail::try_from(item)?;
        link_detail.validate()?;
        link_details.insert(link_detail.link.clone(), link_detail);
        //link_details.push(link_detail);
    }

    // Calculate the scores
    let link_scores = calculate_link_scores(&scores_request.links, &link_details);
    let link_scores_json = serde_json::to_string(&link_scores)?;

    return Ok(link_scores_json.into());
}
