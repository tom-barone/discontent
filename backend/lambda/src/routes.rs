use crate::types::{AdminVote, Vote};
use aws_sdk_dynamodb::{
    model::{
        AttributeValue::{Bool, N, S},
        Put, TransactWriteItem, Update,
    },
    Client,
};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use validator::Validate;

pub async fn admin_vote(request: Request, client: Client) -> Result<Response<Body>, Error> {
    let admin_vote = request.payload::<AdminVote>()?.ok_or("No payload")?;
    admin_vote.validate()?;
    let AdminVote { vote, timestamp } = admin_vote;
    let Vote {
        link,
        user_id,
        vote_value,
    } = vote;

    client
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
                        .item("voteTimestamp", S(timestamp))
                        .item("UserVotes_PK", S(user_id.hyphenated().to_string()))
                        .table_name("Discontent")
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
                        .table_name("Discontent")
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    Update::builder()
                        .key("PK", S(format!("link#{}", link)))
                        .key("SK", S(format!("link#{}", link)))
                        .update_expression(format!(
                            "SET {},{}",
                            "countOfVotes = if_not_exists(countOfVotes, :zero) + :one",
                            "sumOfVotes = if_not_exists(sumOfVotes, :zero) + :voteValue",
                        ))
                        .expression_attribute_values(":voteValue", N(vote_value.to_string()))
                        .expression_attribute_values(":zero", N(0.to_string()))
                        .expression_attribute_values(":one", N(1.to_string()))
                        .table_name("Discontent")
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;

    //UpdateItem(PK=link, SK=link | countOfVotes++, sumOfVotes+=vote)
    //UpdateItem(PK=day, SK=link | countOfVotes++, sumOfVotes+=vote)
    //UpdateItem(PK=day, SK=user | countOfVotes++, sumOfVotes+=vote)

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Success".into())
        .map_err(Box::new)?;
    return Ok(resp);
}
