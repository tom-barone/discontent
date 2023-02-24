use crate::validate::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug)]
pub struct Config {
    pub table_name: String,
    pub use_local_database: bool,
    pub randomize_scores: bool,
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
    use serde::{Deserialize, Serialize};
    use validator::Validate;

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
}

pub mod database {
    use std::collections::HashMap;

    use aws_sdk_dynamodb::model::AttributeValue;
    use lambda_http::Error;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    use super::Link; // We still want to make sure the stuff in the database is valid

    #[derive(Debug, Serialize, Deserialize)]
    pub enum EntityType {
        LinkDetail,
        Vote,
        User,
        LinkHistory,
        UserHistory,
        Settings,
    }

    #[derive(Debug)]
    pub struct Settings {
        pub voting_is_disabled: bool,
    }

    #[derive(Debug, Validate, PartialEq)]
    pub struct LinkDetail {
        #[validate]
        pub link: super::Link,
        pub count_of_votes: u32,
        pub sum_of_votes: u32,
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
                .parse::<u32>()?;

            Ok(LinkDetail {
                link,
                count_of_votes,
                sum_of_votes,
            })
        }
    }
}

// TODO: Rename to VoteRequest or something
#[derive(Debug, Validate, Deserialize, PartialEq)]
pub struct Vote {
    #[validate]
    pub link: Link,
    #[validate(custom = "is_vote_value_valid")]
    pub vote_value: i32,
    pub user_id: Uuid,
}

#[derive(Debug, Validate, Deserialize, PartialEq)]
pub struct AdminVote {
    #[validate]
    pub vote: Vote,
    #[validate(custom = "is_timestamp_valid")]
    pub timestamp: String,
}

//#[cfg(test)]
//mod tests {

//use super::*;

//#[test]
//fn test_deserialize_vote() {
//// Bad json types
//for incorrect_json_type in &[
//r#"{"link": 1234,      "vote_value": 1,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
//r#"{"link": "abc.com", "vote_value": "1", "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
//r#"{"link": "abc.com", "vote_value": 1,   "user_id": "s.d..."}"#,
//] {
//assert!(serde_json::from_str::<Vote>(incorrect_json_type)
//.unwrap_err()
//.is_data());
//}

//// Invalid vote values
//serde_json::from_str::<Vote>(
//r#"{"link": "ab!%^om", "vote_value": 1,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#)
//.unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
//assert_eq!(field, &"link");
//assert_eq!(error[0].code.to_string(), "Hostname is invalid")
//});
//serde_json::from_str::<Vote>(
//r#"{"link": "abc.com", "vote_value": 0,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#)
//.unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
//assert_eq!(field, &"vote_value");
//assert_eq!(error[0].code.to_string(), "Vote should be -1 or 1")
//});

//// Correct json values
//let correct_votes = vec![
//Vote {
//link: Link::new("abc.com"),
//vote_value: -1,
//user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
//},
//Vote {
//link: Link::new("www.domain.com"),
//vote_value: 1,
//user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
//},
//];
//for (i, correct_json_vote) in [
//r#"{"link": "abc.com",        "vote_value": -1,  "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
//r#"{"link": "www.domain.com", "vote_value": 1,   "user_id": "a4a709001c864dd78d380dc05bfb1e0a"}"#,
//].iter().enumerate() {
//let parsed_vote = serde_json::from_str::<Vote>(correct_json_vote).unwrap();
//assert!(parsed_vote.validate().is_ok());
//assert_eq!(parsed_vote, correct_votes[i])
//}
//}

//#[test]
//fn test_deserialize_admin_vote() {
//// Bad json types
//for incorrect_json_type in &[r#"{"timestamp": 2023,
//"vote" : {"link": "abc.com", "vote_value": 1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#]
//{
//assert!(serde_json::from_str::<AdminVote>(incorrect_json_type)
//.unwrap_err()
//.is_data());
//}

//// Invalid admin vote values
//for invalid_timestamp in [
//"2023-02-0209:36:03Z",
//"a2023-02-02T09:36:03Z",
//"2023-09-02T99:36:03Z",
//] {
//serde_json::from_str::<AdminVote>(
//format!("{}{}{}", r#"{"timestamp": ""#, invalid_timestamp, r#"",
//"vote" : {"link": "abc.com", "vote_value": 1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#).as_str())
//.unwrap()
//.validate()
//.unwrap_err()
//.field_errors()
//.iter()
//.for_each(|(field, error)| {
//assert_eq!(field, &"timestamp");
//assert_eq!(error[0].code.to_string(), "Timestamp should be in the RFC3339 format 2023-02-02T09:36:03Z")
//});
//}
//serde_json::from_str::<AdminVote>(
//r#"{"timestamp": "2023-02-02T09:36:03Z",
//"vote" : {"link": "abc.com", "vote_value": 0, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#

//)
//.unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
//assert_eq!(field, &"vote_value");
//assert_eq!(error[0].code.to_string(), "Vote should be -1 or 1")
//});

//// Correct json values
//let correct_admin_votes = vec![
//AdminVote {
//timestamp: "2023-02-02T09:36:03Z".to_string(),
//vote: Vote {
//link: Link::new("abc.com"),
//vote_value: -1,
//user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
//},
//},
//AdminVote {
//timestamp: "2019-02-01T23:59:59Z".to_string(),
//vote: Vote {
//link: Link::new("www.t.au"),
//vote_value: 1,
//user_id: uuid::uuid!("58382932-1c86-4dd7-8d38-0dc05bfb1e0a"),
//},
//},
//];
//for (i, correct_json_admin_votes) in [
//r#"{"timestamp": "2023-02-02T09:36:03Z",
//"vote" : {"link": "abc.com", "vote_value": -1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#,
//r#"{"timestamp": "2019-02-01T23:59:59Z",
//"vote" : {"link": "www.t.au", "vote_value": 1, "user_id": "58382932-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#]
//.iter().enumerate() {
//let parsed_vote = serde_json::from_str::<AdminVote>(correct_json_admin_votes).unwrap();
//assert!(parsed_vote.validate().is_ok());
//assert_eq!(parsed_vote, correct_admin_votes[i])
//}
//}
//}
