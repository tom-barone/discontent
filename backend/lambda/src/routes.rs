use std::collections::HashMap;

use crate::{
    scoring::calculate_link_scores, types::database::LinkDetail,
    validate::validate_get_scores_request,
};
use aws_sdk_dynamodb::{
    model::{AttributeValue::*, KeysAndAttributes},
    Client,
};
//use futures::stream::FuturesUnordered;
//use futures::stream::StreamExt;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use tracing::*;
use validator::Validate;

//#[instrument(level = "trace")]
//pub async fn admin_votes(
//request: Request,
//client: &Client,
//table_name: &String,
//) -> Result<Response<Body>, Error> {
//let admin_votes = request.payload::<Vec<AdminVote>>()?.ok_or("No payload")?;
//for admin_vote in admin_votes.iter() {
//admin_vote.validate()?;
//}

//for admin_vote in admin_votes.iter() {
//let AdminVote { vote, timestamp } = admin_vote;
//let Vote {
//link,
//user_id,
//vote_value,
//} = vote;
//// Extract day string `2023-02-09`
//let day = &timestamp[..10];

//let write_result = client
//.transact_write_items()
//.transact_items(
//TransactWriteItem::builder()
//.put(
////UpdateItem(PK=link, SK=userId | vote)
//Put::builder()
//// No checks on whether the user can submit multiple votes
////.condition_expression(
////"attribute_not_exists(PK) and attribute_not_exists(SK)",
////)
//.item("PK", S(format!("link#{}", link.hostname)))
//.item("SK", S(format!("user#{}", user_id.hyphenated())))
//.item("entityType", S("Vote".to_string()))
//.item("voteValue", N(vote_value.to_string()))
//.item("voteTimestamp", S(timestamp.to_string()))
//.item("UserVotes_PK", S(user_id.hyphenated().to_string()))
//.table_name(table_name)
//.build(),
//)
//.build(),
//)
//.transact_items(
//TransactWriteItem::builder()
//.put(
////UpdateItem(PK=user, SK=user | userNotes, userIsBanned=false
//Put::builder()
//.item("PK", S(format!("user#{}", user_id.hyphenated())))
//.item("SK", S(format!("user#{}", user_id.hyphenated())))
//.item("entityType", S("User".to_string()))
////.item("userIsBanned", Bool(false))
//.table_name(table_name)
//.build(),
//)
//.build(),
//)
//.transact_items(
//TransactWriteItem::builder()
//.update(
////UpdateItem(PK=link, SK=link | countOfVotes++, sumOfVotes+=vote)
//Update::builder()
//.key("PK", S(format!("link#{}", link.hostname)))
//.key("SK", S(format!("link#{}", link.hostname)))
//.update_expression(format!(
//"SET {},{},{}",
//"countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
//"sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
//"entityType = :entityType",
//))
//.expression_attribute_values(":voteValue", N(vote_value.to_string()))
//.expression_attribute_values(":zero", N(0.to_string()))
//.expression_attribute_values(":one", N(1.to_string()))
//.expression_attribute_values(":entityType", S("Link".to_string()))
//.table_name(table_name)
//.build(),
//)
//.build(),
//)
//.transact_items(
//TransactWriteItem::builder()
//.update(
////UpdateItem(PK=day, SK=link | countOfVotes++, sumOfVotes+=vote)
//Update::builder()
//.key("PK", S(format!("day#{}", day)))
//.key("SK", S(format!("link#{}", link.hostname)))
//.update_expression(format!(
//"SET {},{},{},{}",
//"countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
//"sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
//"entityType = :entityType",
//"DailyLinkHistory_PK = :DailyLinkHistory_PK",
//))
//.expression_attribute_values(":voteValue", N(vote_value.to_string()))
//.expression_attribute_values(":zero", N(0.to_string()))
//.expression_attribute_values(":one", N(1.to_string()))
//.expression_attribute_values(
//":entityType",
//S("LinkHistory".to_string()),
//)
//.expression_attribute_values(
//":DailyLinkHistory_PK",
//S(format!("day#{}", day)),
//)
//.table_name(table_name)
//.build(),
//)
//.build(),
//)
//.transact_items(
//TransactWriteItem::builder()
//.update(
////UpdateItem(PK=day, SK=user | countOfVotes++, sumOfVotes+=vote)
//Update::builder()
//.key("PK", S(format!("day#{}", day)))
//.key("SK", S(format!("user#{}", user_id.hyphenated())))
//.update_expression(format!(
//"SET {},{},{},{}",
//"countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
//"sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
//"entityType = :entityType",
//"DailyUserHistory_PK = :DailyUserHistory_PK",
//))
//.expression_attribute_values(":voteValue", N(vote_value.to_string()))
//.expression_attribute_values(":zero", N(0.to_string()))
//.expression_attribute_values(":one", N(1.to_string()))
//.expression_attribute_values(
//":entityType",
//S("UserHistory".to_string()),
//)
//.expression_attribute_values(
//":DailyUserHistory_PK",
//S(format!("day#{}", day)),
//)
//.table_name(table_name)
//.build(),
//)
//.build(),
//)
//.send()
//.await?;

//debug!(
//"Successfully submitted admin vote [result={:?}]",
//write_result
//);
//}

//let resp = Response::builder()
//.status(200)
//.header("content-type", "text/plain")
//.body("Success".into())
//.map_err(Box::new)?;
//return Ok(resp);
//}

#[instrument(level = "trace")]
pub async fn scores(
    request: Request,
    client: &Client,
    table_name: &String,
) -> Result<Response<Body>, Error> {
    // Extract the links from the query parameters and validate them
    let scores_request = validate_get_scores_request(request.query_string_parameters())?;

    // Combine the requests for link details into a single DynamoDB request
    let mut dynamodb_request_builder = KeysAndAttributes::builder();
    for link in scores_request.links {
        dynamodb_request_builder = dynamodb_request_builder.keys(HashMap::from([
            ("PK".to_string(), S(format!("link#{}", link.hostname))),
            ("SK".to_string(), S(format!("link#{}", link.hostname))),
        ]));
    }

    // Send the request to DynamoDB and wait for the results
    let dynamodb_response = client
        .batch_get_item()
        .request_items(table_name, dynamodb_request_builder.build())
        .send()
        .await?;

    // Extract the link details
    let mut link_details: Vec<LinkDetail> = vec![];
    for item in dynamodb_response
        .responses()
        .ok_or("DynamoDB request error")?
        .get(table_name)
        .ok_or("DynamoDB request error")?
        .iter()
    {
        let link_detail = LinkDetail::try_from(item)?;
        link_detail.validate()?;
        link_details.push(link_detail);
    }

    // Calculate the scores
    let link_scores = calculate_link_scores(link_details);
    let link_scores_json = serde_json::to_string(&link_scores)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(link_scores_json.into())
        .map_err(Box::new)?;
    return Ok(resp);
}
