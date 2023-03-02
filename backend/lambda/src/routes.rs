use std::collections::HashMap;

use crate::{
    scoring::*,
    types::{api::VoteRequest, database::LinkDetail, Config},
    validate::{validate_get_scores_request, validate_vote_request},
};
use aws_sdk_dynamodb::{
    model::{AttributeValue::*, KeysAndAttributes, Put, TransactWriteItem, Update},
    Client,
};
use chrono::{SecondsFormat, Utc};
use lambda_http::{Body, Error, Request, RequestExt};
use tracing::*;
use validator::Validate;

#[instrument(level = "trace")]
pub async fn vote(
    request: Request,
    config: &Config,
    dynamo_db_client: &Client,
) -> Result<Body, Error> {
    let vote_request = validate_vote_request(request.body())?;
    info!("Vote request: {:?}", vote_request);
    let VoteRequest {
        link,
        user_id,
        vote_value,
    } = vote_request;

    // Get timestamp in "2018-01-26T18:30:09.453Z" format
    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    // Extract day string `2023-02-09`
    let day = &timestamp[..10];

    let write_result = dynamo_db_client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    //UpdateItem(PK=link, SK=userId | vote)
                    Put::builder()
                        // Fail if user has already voted
                        .condition_expression(
                            "attribute_not_exists(PK) and attribute_not_exists(SK)",
                        )
                        .item("PK", S(format!("link#{}", link.hostname)))
                        .item("SK", S(format!("user#{}", user_id.hyphenated())))
                        .item("entity_type", S("Vote".to_string()))
                        .item("vote_value", N(vote_value.to_string()))
                        .item("vote_timestamp", S(timestamp.to_string()))
                        .item("UserVotes_PK", S(user_id.hyphenated().to_string()))
                        .table_name(&config.table_name)
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    //UpdateItem(PK=user, SK=user)
                    Put::builder()
                        .item("PK", S(format!("user#{}", user_id.hyphenated())))
                        .item("SK", S(format!("user#{}", user_id.hyphenated())))
                        .item("entity_type", S("User".to_string()))
                        .table_name(&config.table_name)
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    //UpdateItem(PK=link, SK=link | count_of_votes++, sum_of_votes+=vote)
                    Update::builder()
                        .key("PK", S(format!("link#{}", link.hostname)))
                        .key("SK", S(format!("link#{}", link.hostname)))
                        .update_expression(format!(
                            "SET {},{},{}",
                            "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                            "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :vote_value",
                            "entity_type = :entity_type",
                        ))
                        .expression_attribute_values(":vote_value", N(vote_value.to_string()))
                        .expression_attribute_values(":zero", N(0.to_string()))
                        .expression_attribute_values(":one", N(1.to_string()))
                        .expression_attribute_values(":entity_type", S("Link".to_string()))
                        .table_name(&config.table_name)
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    //UpdateItem(PK=day, SK=link | count_of_votes++, sum_of_votes+=vote)
                    Update::builder()
                        .key("PK", S(format!("day#{}", day)))
                        .key("SK", S(format!("link#{}", link.hostname)))
                        .update_expression(format!(
                            "SET {},{},{},{}",
                            "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                            "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :vote_value",
                            "entity_type = :entity_type",
                            "DailyLinkHistory_PK = :DailyLinkHistory_PK",
                        ))
                        .expression_attribute_values(":vote_value", N(vote_value.to_string()))
                        .expression_attribute_values(":zero", N(0.to_string()))
                        .expression_attribute_values(":one", N(1.to_string()))
                        .expression_attribute_values(":entity_type", S("LinkHistory".to_string()))
                        .expression_attribute_values(
                            ":DailyLinkHistory_PK",
                            S(format!("day#{}", day)),
                        )
                        .table_name(&config.table_name)
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    //UpdateItem(PK=day, SK=user | count_of_votes++, sum_of_votes+=vote)
                    Update::builder()
                        .key("PK", S(format!("day#{}", day)))
                        .key("SK", S(format!("user#{}", user_id.hyphenated())))
                        .update_expression(format!(
                            "SET {},{},{},{}",
                            "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                            "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :vote_value",
                            "entity_type = :entity_type",
                            "DailyUserHistory_PK = :DailyUserHistory_PK",
                        ))
                        .expression_attribute_values(":vote_value", N(vote_value.to_string()))
                        .expression_attribute_values(":zero", N(0.to_string()))
                        .expression_attribute_values(":one", N(1.to_string()))
                        .expression_attribute_values(":entity_type", S("UserHistory".to_string()))
                        .expression_attribute_values(
                            ":DailyUserHistory_PK",
                            S(format!("day#{}", day)),
                        )
                        .table_name(&config.table_name)
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;

    debug!(
        "Successfully submitted vote [result={:?}]",
        write_result
    );

    return Ok(Body::Empty);
}

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
