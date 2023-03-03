use aws_sdk_dynamodb::model::{
    AttributeValue::{self, *},
    *,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::{database::Vote, Config, Link};

pub fn get_settings() -> HashMap<String, AttributeValue> {
    HashMap::from([
        ("PK".to_string(), S("settings".to_string())),
        ("SK".to_string(), S("settings".to_string())),
    ])
}

pub fn get_user(user_id: &Uuid) -> HashMap<String, AttributeValue> {
    HashMap::from([
        (
            "PK".to_string(),
            S(format!("user#{}", user_id.hyphenated())),
        ),
        (
            "SK".to_string(),
            S(format!("user#{}", user_id.hyphenated())),
        ),
    ])
}

pub fn get_daily_user_history(day: &String, user_id: &Uuid) -> HashMap<String, AttributeValue> {
    HashMap::from([
        ("PK".to_string(), S(format!("day#{}", day))),
        (
            "SK".to_string(),
            S(format!("user#{}", user_id.hyphenated())),
        ),
    ])
}

pub fn get_vote(vote: &Vote) -> HashMap<String, AttributeValue> {
    HashMap::from([
        ("PK".to_string(), S(format!("link#{}", vote.link.hostname))),
        (
            "SK".to_string(),
            S(format!("user#{}", vote.user_id.hyphenated())),
        ),
    ])
}

pub fn put_new_user(user_id: &Uuid, created_at: &String, config: &Config) -> TransactWriteItem {
    TransactWriteItem::builder()
        .put(
            Put::builder()
                .item("PK", S(format!("user#{}", user_id.hyphenated())))
                .item("SK", S(format!("user#{}", user_id.hyphenated())))
                .item("entity_type", S("User".to_string()))
                .item("created_at", S(created_at.clone()))
                .item("is_banned", Bool(false))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn put_vote(vote: &Vote, config: &Config) -> TransactWriteItem {
    TransactWriteItem::builder()
        .put(
            Put::builder()
                .item("PK", S(format!("link#{}", vote.link.hostname)))
                .item("SK", S(format!("user#{}", vote.user_id.hyphenated())))
                .item("entity_type", S("Vote".to_string()))
                .item("value", N(vote.value.to_string()))
                .item("created_at", S(vote.created_at.clone()))
                .item("UserVotes_PK", S(vote.user_id.hyphenated().to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn update_link_detail(link: &Link, vote_value: i32, config: &Config) -> TransactWriteItem {
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("link#{}", link.hostname)))
                .key("SK", S(format!("link#{}", link.hostname)))
                .update_expression(format!(
                    "SET {},{},{}",
                    "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                    "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :value",
                    "entity_type = :entity_type",
                ))
                .expression_attribute_values(":value", N(vote_value.to_string()))
                .expression_attribute_values(":zero", N(0.to_string()))
                .expression_attribute_values(":one", N(1.to_string()))
                .expression_attribute_values(":entity_type", S("LinkDetail".to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn update_existing_link_detail(
    link: &Link,
    vote_value_change: i32,
    config: &Config,
) -> TransactWriteItem {
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("link#{}", link.hostname)))
                .key("SK", S(format!("link#{}", link.hostname)))
                .update_expression(format!("SET {}", "sum_of_votes = sum_of_votes + :change",))
                .expression_attribute_values(":change", N(vote_value_change.to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn increment_link_history(day: &String, vote: &Vote, config: &Config) -> TransactWriteItem {
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", day)))
                .key("SK", S(format!("link#{}", vote.link.hostname)))
                .update_expression(format!(
                    "SET {},{},{},{}",
                    "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                    "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :value",
                    "entity_type = :entity_type",
                    "DailyLinkHistory_PK = :DailyLinkHistory_PK",
                ))
                .expression_attribute_values(":value", N(vote.value.to_string()))
                .expression_attribute_values(":zero", N(0.to_string()))
                .expression_attribute_values(":one", N(1.to_string()))
                .expression_attribute_values(":entity_type", S("LinkHistory".to_string()))
                .expression_attribute_values(":DailyLinkHistory_PK", S(format!("day#{}", day)))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn increment_user_history(day: &String, vote: &Vote, config: &Config) -> TransactWriteItem {
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", day)))
                .key("SK", S(format!("user#{}", vote.user_id.hyphenated())))
                .update_expression(format!(
                    "SET {},{},{},{}",
                    "count_of_votes = if_not_exists(count_of_votes, :zero) + :one",
                    "sum_of_votes = if_not_exists(sum_of_votes, :zero) + :value",
                    "entity_type = :entity_type",
                    "DailyUserHistory_PK = :DailyUserHistory_PK",
                ))
                .expression_attribute_values(":value", N(vote.value.to_string()))
                .expression_attribute_values(":zero", N(0.to_string()))
                .expression_attribute_values(":one", N(1.to_string()))
                .expression_attribute_values(":entity_type", S("UserHistory".to_string()))
                .expression_attribute_values(":DailyUserHistory_PK", S(format!("day#{}", day)))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn revert_link_history(old_vote: &Vote, link: &Link, config: &Config) -> TransactWriteItem {
    let old_day = &old_vote.created_at[..10].to_string();
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", old_day)))
                .key("SK", S(format!("link#{}", link.hostname)))
                .update_expression(format!(
                    "SET {},{}",
                    "count_of_votes = count_of_votes - :one",
                    "sum_of_votes = sum_of_votes - :value",
                ))
                .expression_attribute_values(":value", N(old_vote.value.to_string()))
                .expression_attribute_values(":one", N(1.to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn revert_user_history(old_vote: &Vote, user_id: &Uuid, config: &Config) -> TransactWriteItem {
    let old_day = &old_vote.created_at[..10].to_string();
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", old_day)))
                .key("SK", S(format!("user#{}", user_id.hyphenated())))
                .update_expression(format!(
                    "SET {},{}",
                    "count_of_votes = count_of_votes - :one",
                    "sum_of_votes = sum_of_votes - :value",
                ))
                .expression_attribute_values(":value", N(old_vote.value.to_string()))
                .expression_attribute_values(":one", N(1.to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn update_link_history(
    day: &String,
    old_vote: &Vote,
    vote: &Vote,
    config: &Config,
) -> TransactWriteItem {
    let vote_value_change = vote.value - old_vote.value;
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", day)))
                .key("SK", S(format!("link#{}", vote.link.hostname)))
                .update_expression(format!("SET {}", "sum_of_votes = sum_of_votes + :change",))
                .expression_attribute_values(":change", N(vote_value_change.to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}

pub fn update_user_history(
    day: &String,
    old_vote: &Vote,
    vote: &Vote,
    config: &Config,
) -> TransactWriteItem {
    let vote_value_change = vote.value - old_vote.value;
    TransactWriteItem::builder()
        .update(
            Update::builder()
                .key("PK", S(format!("day#{}", day)))
                .key("SK", S(format!("user#{}", vote.user_id.hyphenated())))
                .update_expression(format!("SET {}", "sum_of_votes = sum_of_votes + :change",))
                .expression_attribute_values(":change", N(vote_value_change.to_string()))
                .table_name(&config.table_name)
                .build(),
        )
        .build()
}
