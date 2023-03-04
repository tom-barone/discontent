use crate::validate::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug)]
pub struct Config {
    pub table_name: String,
    pub use_local_database: bool,
    pub randomize_scores: bool,
    pub use_system_time: bool,
}

#[derive(Debug, Validate, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Link {
    #[validate(custom = "is_hostname_valid")]
    pub hostname: String,
}
impl Link {
    pub fn new(hostname: &str) -> Self {
        Link {
            hostname: hostname.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Score {
    Good,
    Bad,
    Controversial,
    NoScore,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LinkScore {
    #[validate]
    link: Link,
    score: Score,
}
impl LinkScore {
    pub fn new(link: Link, score: Score) -> Self {
        LinkScore { link, score }
    }
}

pub mod api {
    use super::{Link, LinkScore};
    use crate::validate::is_vote_value_valid;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Debug, Validate, Deserialize, PartialEq)]
    pub struct VoteRequest {
        #[validate]
        pub link: Link,
        #[validate(custom = "is_vote_value_valid")]
        pub value: i32,
        pub user_id: Uuid,
    }

    #[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
    pub struct ScoresRequest {
        #[validate]
        #[validate(length(min = 1, max = 100))]
        pub links: Vec<Link>,
    }

    #[derive(Debug, Validate, Serialize)]
    pub struct ScoresResponse {
        #[validate]
        #[validate(length(min = 1, max = 100))]
        scores: Vec<LinkScore>,
    }

    #[derive(Debug, Serialize)]
    pub struct Error {
        pub error: String,
        pub description: serde_json::Value,
    }
}

pub mod database {
    use super::Link;
    use crate::validate::*;
    use aws_sdk_dynamodb::model::AttributeValue;
    use lambda_http::Error;
    use serde::Deserialize;
    use std::collections::HashMap;
    use uuid::Uuid;
    use validator::Validate;
    // TODO: Add validation to these database types

    #[derive(Debug, Validate, Deserialize, PartialEq)]
    pub struct Vote {
        #[validate]
        pub link: Link,
        #[validate(custom = "is_vote_value_valid")]
        pub value: i32,
        pub user_id: Uuid,
        #[validate(custom = "is_timestamp_valid")]
        pub created_at: String
    }
    impl TryFrom<&HashMap<String, AttributeValue>> for Vote {
        type Error = Error;
        fn try_from(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
            let primary_key = hash_map
                .get("PK")
                .ok_or("No PK")?
                .as_s()
                .or(Err("PK is not a string"))?;
            let link = Link::new(primary_key.split('#').nth(1).ok_or("No link")?);
            let value = hash_map
                .get("value")
                .ok_or("No value")?
                .as_n()
                .or(Err("value is not a number"))?
                .parse::<i32>()?;
            let sort_key = hash_map
                .get("SK")
                .ok_or("No SK")?
                .as_s()
                .or(Err("SK is not a string"))?;
            let user_id = Uuid::parse_str(sort_key.split('#').nth(1).ok_or("No user_id")?)?;
            let created_at = hash_map
                .get("created_at")
                .ok_or("No created_at")?
                .as_s()
                .or(Err("created_at is not a string"))?
                .to_string();
            let vote = Vote {
                link,
                value,
                user_id,
                created_at,
            };
            vote.validate()?;
            Ok(vote)
        }
    }

    #[derive(Debug)]
    pub struct UserHistory {
        pub day: String,
        pub count_of_votes: u32,
        pub sum_of_votes: i32,
    }
    impl TryFrom<&HashMap<String, AttributeValue>> for UserHistory {
        type Error = Error;
        fn try_from(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
            let primary_key = hash_map
                .get("PK")
                .ok_or("No PK")?
                .as_s()
                .or(Err("PK is not a string"))?;
            let day = primary_key
                .split('#')
                .nth(1)
                .ok_or("No day found in PK")?
                .to_string();
            let count_of_votes = hash_map
                .get("count_of_votes")
                .ok_or("No count_of_votes")?
                .as_n()
                .or(Err("count_of_votes is not a number"))?
                .parse::<u32>()?;
            let sum_of_votes = hash_map
                .get("sum_of_votes")
                .ok_or("No sum_of_votes")?
                .as_n()
                .or(Err("sum_of_votes is not a number"))?
                .parse::<i32>()?;

            Ok(UserHistory {
                day,
                count_of_votes,
                sum_of_votes,
            })
        }
    }

    #[derive(Debug)]
    pub struct User {
        pub is_banned: bool,
    }
    impl TryFrom<&HashMap<String, AttributeValue>> for User {
        type Error = Error;
        fn try_from(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
            let is_banned = hash_map
                .get("is_banned")
                .ok_or("No is_banned")?
                .as_bool()
                .or(Err("is_banned is not a bool"))?
                .clone();

            Ok(User { is_banned })
        }
    }

    #[derive(Debug)]
    pub struct Settings {
        pub voting_is_disabled: bool,
        pub maximum_votes_per_user_per_day: u32,
    }
    impl TryFrom<&HashMap<String, AttributeValue>> for Settings {
        type Error = Error;
        fn try_from(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
            let voting_is_disabled = hash_map
                .get("voting_is_disabled")
                .ok_or("No voting_is_disabled")?
                .as_bool()
                .or(Err("voting_is_disabled is not a bool"))?
                .clone();
            let maximum_votes_per_user_per_day = hash_map
                .get("maximum_votes_per_user_per_day")
                .ok_or("No maximum_votes_per_user_per_day")?
                .as_n()
                .or(Err("maximum_votes_per_user_per_day is not a number"))?
                .parse::<u32>()?;

            Ok(Settings {
                voting_is_disabled,
                maximum_votes_per_user_per_day,
            })
        }
    }

    #[derive(Debug, Validate, PartialEq)]
    pub struct LinkDetail {
        #[validate]
        pub link: super::Link,
        pub count_of_votes: u32,
        pub sum_of_votes: i32,
    }
    impl TryFrom<&HashMap<String, AttributeValue>> for LinkDetail {
        type Error = Error;
        fn try_from(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
            let primary_key = hash_map
                .get("PK")
                .ok_or("No PK")?
                .as_s()
                .or(Err("PK is not a string"))?;
            let link = Link::new(primary_key.split('#').nth(1).ok_or("No link")?);
            let count_of_votes = hash_map
                .get("count_of_votes")
                .ok_or("No count_of_votes")?
                .as_n()
                .or(Err("count_of_votes is not a number"))?
                .parse::<u32>()?;
            let sum_of_votes = hash_map
                .get("sum_of_votes")
                .ok_or("No sum_of_votes")?
                .as_n()
                .or(Err("sum_of_votes is not a number"))?
                .parse::<i32>()?;

            Ok(LinkDetail {
                link,
                count_of_votes,
                sum_of_votes,
            })
        }
    }
}
