use std::collections::HashMap;

use crate::{
    dynamodb::*,
    scoring::*,
    types::{database::*, Config},
    validate::{validate_get_scores_request, validate_vote_request},
};
use aws_sdk_dynamodb::{
    model::{AttributeValue::*, KeysAndAttributes, TransactWriteItem},
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
    let vote = Vote {
        link: vote_request.link.clone(),
        user_id: vote_request.user_id.clone(),
        value: vote_request.value,
        // Get created_at timestamp in "2018-01-26T18:30:09Z" format
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };
    // Extract day string `2023-02-09`
    let day = vote.created_at.clone()[..10].to_string();

    info!("New vote request: {:?}", vote);

    // Get settings and user history
    let settings_and_user_request = dynamo_db_client
        .batch_get_item()
        .request_items(
            &config.table_name,
            KeysAndAttributes::builder()
                .set_keys(Some(vec![
                    get_settings(),
                    get_user(&vote.user_id),
                    get_daily_user_history(&day, &vote.user_id),
                    get_vote(&vote),
                ]))
                .build(),
        )
        .send()
        .await?;
    debug!(
        "Settings and User History response: {:#?}",
        settings_and_user_request
    );

    let mut user_does_not_exist = true;
    let mut user_is_banned = false;
    let mut first_vote_on_link_for_user = true;
    let mut voting_is_disabled = false;
    let mut user_has_voted_too_many_times_today = false;
    let mut maximum_votes_per_user_per_day: u32 = 10;
    let mut old_vote: Option<Vote> = None;
    for item in settings_and_user_request
        .responses()
        .ok_or("DynamoDB request error")?
        .get(&config.table_name)
        .ok_or("DynamoDB request error")?
        .iter()
    {
        let entity_type = item
            .get("entity_type")
            .ok_or("No entity_type")?
            .as_s()
            .or(Err("entity_type is not a string"))?;
        match entity_type.as_str() {
            "Settings" => {
                let settings = Settings::try_from(item)?;
                voting_is_disabled = settings.voting_is_disabled;
                maximum_votes_per_user_per_day = settings.maximum_votes_per_user_per_day;
            }
            "User" => {
                user_does_not_exist = false;
                let user = User::try_from(item)?;
                user_is_banned = user.is_banned;
            }
            "UserHistory" => {
                let daily_user_history = UserHistory::try_from(item)?;
                // Assumes the settings have already been retrieved
                user_has_voted_too_many_times_today =
                    daily_user_history.count_of_votes >= maximum_votes_per_user_per_day;
            }
            "Vote" => {
                first_vote_on_link_for_user = false;
                old_vote = Some(Vote::try_from(item)?);
            }
            _ => {
                return Err("Unknown entity_type".into());
            }
        }
    }

    if user_is_banned {
        return Err("User is banned".into());
    }
    if voting_is_disabled {
        return Err("Voting is disabled".into());
    }
    if user_has_voted_too_many_times_today {
        return Err("User has voted too many times today".into());
    }

    let mut write_requests: Vec<TransactWriteItem> = vec![];
    if user_does_not_exist {
        write_requests.push(put_new_user(&vote.user_id, &vote.created_at, config));
    }
    if first_vote_on_link_for_user {
        write_requests.push(put_vote(&vote, config));
        write_requests.push(update_link_detail(&vote.link, vote.value, config));
        write_requests.push(increment_link_history(&day, &vote, config));
        write_requests.push(increment_user_history(&day, &vote, config));
    } else if let Some(old_vote) = old_vote {
        let old_day = old_vote.created_at[..10].to_string();
        write_requests.push(put_vote(&vote, config));
        write_requests.push(update_existing_link_detail(
            &vote.link,
            -old_vote.value + vote.value, // The change in vote value
            config,
        ));
        // If updates are on the same day
        if old_day == day {
            // Update old day
            write_requests.push(update_link_history(&day, &old_vote, &vote, config));
            write_requests.push(update_user_history(&day, &old_vote, &vote, config));
        } else {
            // Revert old day, increment new day
            write_requests.push(revert_link_history(&old_vote, &vote.link, config));
            write_requests.push(revert_user_history(&old_vote, &vote.user_id, config));
            write_requests.push(increment_link_history(&day, &vote, config));
            write_requests.push(increment_user_history(&day, &vote, config));
        }
    }

    let write_result = dynamo_db_client
        .transact_write_items()
        .set_transact_items(Some(write_requests))
        .send()
        .await?;
        
    debug!("Successfully submitted vote [result={:?}]", write_result);

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
    }

    // Calculate the scores
    let link_scores = calculate_link_scores(&scores_request.links, &link_details);
    let link_scores_json = serde_json::to_string(&link_scores)?;

    return Ok(link_scores_json.into());
}
