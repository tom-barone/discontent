use crate::types::{AdminVote, Vote};
use aws_sdk_dynamodb::{
    model::{AttributeValue::*, Put, TransactWriteItem, Update},
    Client,
};
//use futures::stream::FuturesUnordered;
//use futures::stream::StreamExt;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use tracing::*;
use validator::Validate;

#[instrument(level = "trace")]
pub async fn admin_votes(
    request: Request,
    client: &Client,
    table_name: &String,
) -> Result<Response<Body>, Error> {
    let admin_votes = request.payload::<Vec<AdminVote>>()?.ok_or("No payload")?;
    for admin_vote in admin_votes.iter() {
        admin_vote.validate()?;
    }

    for admin_vote in admin_votes.iter() {
        let AdminVote { vote, timestamp } = admin_vote;
        let Vote {
            link,
            user_id,
            vote_value,
        } = vote;
        // Extract day string `2023-02-09`
        let day = &timestamp[..10];

        let write_result = client
            .transact_write_items()
            .transact_items(
                TransactWriteItem::builder()
                    .put(
                        //UpdateItem(PK=link, SK=userId | vote)
                        Put::builder()
                            // No checks on whether the user can submit multiple votes
                            //.condition_expression(
                            //"attribute_not_exists(PK) and attribute_not_exists(SK)",
                            //)
                            .item("PK", S(format!("link#{}", link)))
                            .item("SK", S(format!("user#{}", user_id.hyphenated())))
                            .item("entityType", S("Vote".to_string()))
                            .item("voteValue", N(vote_value.to_string()))
                            .item("voteTimestamp", S(timestamp.to_string()))
                            .item("UserVotes_PK", S(user_id.hyphenated().to_string()))
                            .table_name(table_name)
                            .build(),
                    )
                    .build(),
            )
            .transact_items(
                TransactWriteItem::builder()
                    .put(
                        //UpdateItem(PK=user, SK=user | userNotes, userIsBanned=false
                        Put::builder()
                            .item("PK", S(format!("user#{}", user_id.hyphenated())))
                            .item("SK", S(format!("user#{}", user_id.hyphenated())))
                            .item("entityType", S("User".to_string()))
                            //.item("userIsBanned", Bool(false))
                            .table_name(table_name)
                            .build(),
                    )
                    .build(),
            )
            .transact_items(
                TransactWriteItem::builder()
                    .update(
                        //UpdateItem(PK=link, SK=link | countOfVotes++, sumOfVotes+=vote)
                        Update::builder()
                            .key("PK", S(format!("link#{}", link)))
                            .key("SK", S(format!("link#{}", link)))
                            .update_expression(format!(
                                "SET {},{},{}",
                                "countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
                                "sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
                                "entityType = :entityType",
                            ))
                            .expression_attribute_values(":voteValue", N(vote_value.to_string()))
                            .expression_attribute_values(":zero", N(0.to_string()))
                            .expression_attribute_values(":one", N(1.to_string()))
                            .expression_attribute_values(":entityType", S("Link".to_string()))
                            .table_name(table_name)
                            .build(),
                    )
                    .build(),
            )
            .transact_items(
                TransactWriteItem::builder()
                    .update(
                        //UpdateItem(PK=day, SK=link | countOfVotes++, sumOfVotes+=vote)
                        Update::builder()
                            .key("PK", S(format!("day#{}", day)))
                            .key("SK", S(format!("link#{}", link)))
                            .update_expression(format!(
                                "SET {},{},{},{}",
                                "countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
                                "sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
                                "entityType = :entityType",
                                "DailyLinkHistory_PK = :DailyLinkHistory_PK",
                            ))
                            .expression_attribute_values(":voteValue", N(vote_value.to_string()))
                            .expression_attribute_values(":zero", N(0.to_string()))
                            .expression_attribute_values(":one", N(1.to_string()))
                            .expression_attribute_values(
                                ":entityType",
                                S("LinkHistory".to_string()),
                            )
                            .expression_attribute_values(
                                ":DailyLinkHistory_PK",
                                S(format!("day#{}", day)),
                            )
                            .table_name(table_name)
                            .build(),
                    )
                    .build(),
            )
            .transact_items(
                TransactWriteItem::builder()
                    .update(
                        //UpdateItem(PK=day, SK=user | countOfVotes++, sumOfVotes+=vote)
                        Update::builder()
                            .key("PK", S(format!("day#{}", day)))
                            .key("SK", S(format!("user#{}", user_id.hyphenated())))
                            .update_expression(format!(
                                "SET {},{},{},{}",
                                "countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
                                "sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
                                "entityType = :entityType",
                                "DailyUserHistory_PK = :DailyUserHistory_PK",
                            ))
                            .expression_attribute_values(":voteValue", N(vote_value.to_string()))
                            .expression_attribute_values(":zero", N(0.to_string()))
                            .expression_attribute_values(":one", N(1.to_string()))
                            .expression_attribute_values(
                                ":entityType",
                                S("UserHistory".to_string()),
                            )
                            .expression_attribute_values(
                                ":DailyUserHistory_PK",
                                S(format!("day#{}", day)),
                            )
                            .table_name(table_name)
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await?;

        debug!(
            "Successfully submitted admin vote [result={:?}]",
            write_result
        );
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Success".into())
        .map_err(Box::new)?;
    return Ok(resp);
}
