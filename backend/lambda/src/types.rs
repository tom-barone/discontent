use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;

use validator::{Validate, ValidationError};

lazy_static! {
    // For timestamps in the format "2023-02-02T09:36:03Z"
    static ref TIMESTAMP_REGEX: Regex = Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$").unwrap();
}

#[derive(Debug, Validate, Deserialize, PartialEq)]
pub struct Vote {
    #[validate(custom = "is_hostname_valid")]
    pub link: String,
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

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub voting_is_disabled: bool,
}

fn is_timestamp_valid(timestamp: &String) -> Result<(), ValidationError> {
    if (!TIMESTAMP_REGEX.is_match(&timestamp))
        || (DateTime::parse_from_rfc3339(&timestamp).is_err())
    {
        return Err(ValidationError::new(
            "Timestamp should be in the RFC3339 format 2023-02-02T09:36:03Z",
        ));
    }
    Ok(())
}

fn is_vote_value_valid(vote_value: i32) -> Result<(), ValidationError> {
    if (vote_value != -1) && (vote_value != 1) {
        return Err(ValidationError::new("Vote should be -1 or 1"));
    }
    Ok(())
}

// Copyright 2018-2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT
// https://docs.rs/hostname-validator
/// Validate a hostname according to [IETF RFC 1123](https://tools.ietf.org/html/rfc1123).
///
/// A hostname is valid if the following condition are true:
///
/// - It does not start or end with `-` or `.`.
/// - It does not contain any characters outside of the alphanumeric range, except for `-` and `.`.
/// - It is not empty.
/// - It is 253 or fewer characters.
/// - Its labels (characters separated by `.`) are not empty.
/// - Its labels are 63 or fewer characters.
/// - Its lables do not start or end with '-' or '.'.
fn is_hostname_valid(hostname: &str) -> Result<(), ValidationError> {
    fn is_valid_char(byte: u8) -> bool {
        (b'a'..=b'z').contains(&byte)
            || (b'A'..=b'Z').contains(&byte)
            || (b'0'..=b'9').contains(&byte)
            || byte == b'-'
            || byte == b'.'
    }

    if hostname.bytes().any(|byte| !is_valid_char(byte))
        || hostname.split('.').any(|label| {
            label.is_empty() || label.len() > 63 || label.starts_with('-') || label.ends_with('-')
        })
        || hostname.is_empty()
        || hostname.len() > 253
    {
        Err(ValidationError::new("Hostname is invalid"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vote_value_valid() {
        assert_eq!(is_vote_value_valid(1), Ok(()));
        assert_eq!(is_vote_value_valid(-1), Ok(()));
        for invalid_vote in [2, 0, 223, -5, -9999] {
            assert_eq!(
                is_vote_value_valid(invalid_vote),
                Err(ValidationError::new("Vote should be -1 or 1"))
            );
        }
    }

    #[test]
    fn test_is_timestamp_valid() {
        // Valid timestamps
        for timestamp in &["VaLiD-HoStNaMe"] {
            assert_eq!(is_hostname_valid(timestamp), Ok(()));
        }

        // Invalid timestamps
        for invalid_timestamp in &[
            "2023-02-0209:36:03Z",
            "a2023-02-02T09:36:03Z",
            "2023-02-02T09:3603Z",
            "2023-02-02T09:36:03UTC",
            "2023-99-02T09:36:03Z",
            "2023-09-02T99:36:03Z",
            "2023:09:02T09:36:03Z",
            "2020-12-31 21:07:14-05:00",
        ] {
            assert_eq!(
                is_timestamp_valid(&invalid_timestamp.to_string()),
                Err(ValidationError::new(
                    "Timestamp should be in the RFC3339 format 2023-02-02T09:36:03Z"
                ))
            );
        }
    }

    #[test]
    fn test_is_hostname_valid() {
        // Valid hostnames
        for hostname in &[
            "VaLiD-HoStNaMe",
            "50-name",
            "235235",
            "example.com",
            "VaLid.HoStNaMe",
            "www.place.au",
            "123.456",
        ] {
            assert_eq!(is_hostname_valid(hostname), Ok(()));
        }

        // Invalid hostnames
        for hostname in &[
            "-invalid-name",
            "also-invalid-",
            "asdf@fasd",
            "@asdfl",
            "asd f@",
            ".invalid",
            "invalid.name.",
            "foo.label-is-way-to-longgggggggggggggggggggggggggggggggggggggggggggg.org",
            "invalid.-starting.char",
            "invalid.ending-.char",
            "empty..label",
        ] {
            assert_eq!(
                is_hostname_valid(hostname),
                Err(ValidationError::new("Hostname is invalid"))
            );
        }
    }

    #[test]
    fn test_deserialize_vote() {
        // Bad json types
        for incorrect_json_type in &[
            r#"{"link": 1234,      "vote_value": 1,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
            r#"{"link": "abc.com", "vote_value": "1", "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
            r#"{"link": "abc.com", "vote_value": 1,   "user_id": "s.d..."}"#,
        ] {
            assert!(serde_json::from_str::<Vote>(incorrect_json_type)
                .unwrap_err()
                .is_data());
        }

        // Invalid vote values
        serde_json::from_str::<Vote>(
            r#"{"link": "ab!%^om", "vote_value": 1,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#)
            .unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
                assert_eq!(field, &"link");
                assert_eq!(error[0].code.to_string(), "Hostname is invalid")
            });
        serde_json::from_str::<Vote>(
            r#"{"link": "abc.com", "vote_value": 0,   "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#)
            .unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
                assert_eq!(field, &"vote_value");
                assert_eq!(error[0].code.to_string(), "Vote should be -1 or 1")
            });

        // Correct json values
        let correct_votes = vec![
            Vote {
                link: "abc.com".to_string(),
                vote_value: -1,
                user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
            },
            Vote {
                link: "www.domain.com".to_string(),
                vote_value: 1,
                user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
            },
        ];
        for (i, correct_json_vote) in [
            r#"{"link": "abc.com",        "vote_value": -1,  "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}"#,
            r#"{"link": "www.domain.com", "vote_value": 1,   "user_id": "a4a709001c864dd78d380dc05bfb1e0a"}"#,
        ].iter().enumerate() {
            let parsed_vote = serde_json::from_str::<Vote>(correct_json_vote).unwrap();
            assert!(parsed_vote.validate().is_ok());
            assert_eq!(parsed_vote, correct_votes[i])
        }
    }

    #[test]
    fn test_deserialize_admin_vote() {
        // Bad json types
        for incorrect_json_type in &[r#"{"timestamp": 2023,
            "vote" : {"link": "abc.com", "vote_value": 1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#]
        {
            assert!(serde_json::from_str::<AdminVote>(incorrect_json_type)
                .unwrap_err()
                .is_data());
        }

        // Invalid admin vote values
        for invalid_timestamp in [
            "2023-02-0209:36:03Z",
            "a2023-02-02T09:36:03Z",
            "2023-09-02T99:36:03Z",
        ] {
            serde_json::from_str::<AdminVote>(
                format!("{}{}{}", r#"{"timestamp": ""#, invalid_timestamp, r#"", 
                "vote" : {"link": "abc.com", "vote_value": 1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#).as_str())
            .unwrap()
            .validate()
            .unwrap_err()
            .field_errors()
            .iter()
            .for_each(|(field, error)| {
                assert_eq!(field, &"timestamp");
                assert_eq!(error[0].code.to_string(), "Timestamp should be in the RFC3339 format 2023-02-02T09:36:03Z")
            });
        }
        serde_json::from_str::<AdminVote>(
                r#"{"timestamp": "2023-02-02T09:36:03Z", 
                "vote" : {"link": "abc.com", "vote_value": 0, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#

            )
            .unwrap().validate().unwrap_err().field_errors().iter().for_each(|(field, error)| {
                assert_eq!(field, &"vote_value");
                assert_eq!(error[0].code.to_string(), "Vote should be -1 or 1")
            });

        // Correct json values
        let correct_admin_votes = vec![
            AdminVote {
                timestamp: "2023-02-02T09:36:03Z".to_string(),
                vote: Vote {
                    link: "abc.com".to_string(),
                    vote_value: -1,
                    user_id: uuid::uuid!("a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"),
                },
            },
            AdminVote {
                timestamp: "2019-02-01T23:59:59Z".to_string(),
                vote: Vote {
                    link: "www.t.au".to_string(),
                    vote_value: 1,
                    user_id: uuid::uuid!("58382932-1c86-4dd7-8d38-0dc05bfb1e0a"),
                },
            },
        ];
        for (i, correct_json_admin_votes) in [
            r#"{"timestamp": "2023-02-02T09:36:03Z", 
            "vote" : {"link": "abc.com", "vote_value": -1, "user_id": "a4a70900-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#,
            r#"{"timestamp": "2019-02-01T23:59:59Z", 
            "vote" : {"link": "www.t.au", "vote_value": 1, "user_id": "58382932-1c86-4dd7-8d38-0dc05bfb1e0a"}}"#]
            .iter().enumerate() {
                let parsed_vote = serde_json::from_str::<AdminVote>(correct_json_admin_votes).unwrap();
                assert!(parsed_vote.validate().is_ok());
                assert_eq!(parsed_vote, correct_admin_votes[i])
        }
    }
}
