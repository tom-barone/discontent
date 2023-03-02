use crate::types::api;
use lambda_http::Error;
use lambda_http::{aws_lambda_events::query_map::QueryMap, Body};
use lazy_static::lazy_static;
use regex::Regex;
use validator::{Validate, ValidationError};

pub fn validate_get_scores_request(query_map: QueryMap) -> Result<api::ScoresRequest, Error> {
    let links_query_parameter = query_map
        .first("from")
        .ok_or("Incorrect query parameters. Expected `from`")?;
    let links = serde_json::from_str::<api::ScoresRequest>(links_query_parameter)?;
    links.validate()?;
    Ok(links)
}

pub fn validate_vote_request(body: &Body) -> Result<api::VoteRequest, Error> {
    let vote_request = serde_json::from_slice::<api::VoteRequest>(body)?;
    vote_request.validate()?;
    Ok(vote_request)
}

lazy_static! {
    // For timestamps in the format "2023-02-02T09:36:03Z"
    static ref TIMESTAMP_REGEX: Regex = Regex::new(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$").unwrap();
}

pub fn is_vote_value_valid(vote_value: i32) -> Result<(), ValidationError> {
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
pub fn is_hostname_valid(hostname: &str) -> Result<(), ValidationError> {
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
    use crate::types::{api::ScoresRequest, Link};
    use std::collections::HashMap;

    // Make it easy to create new ScoresRequest objects
    impl ScoresRequest {
        pub fn new(links: Vec<&str>) -> Self {
            ScoresRequest {
                links: links
                    .into_iter()
                    .map(|link| Link::new(link))
                    .collect::<Vec<Link>>(),
            }
        }
    }

    #[test]
    fn test_validate_get_scores_request() {
        fn test_helper<T: serde::Serialize>(
            query_key: &str,
            query_value: &T,
        ) -> Result<ScoresRequest, Error> {
            validate_get_scores_request(QueryMap::from(HashMap::from([(
                query_key.to_string(),
                serde_json::to_string(query_value.into()).unwrap(),
            )])))
        }

        // Happy path
        let key = "from";
        let value = ScoresRequest::new(vec!["www.google.com", "abc.com", "domain.me"]);
        let result = test_helper(key, &value);
        assert!(result.is_ok());

        // Not enough links
        let key = "from";
        let value = ScoresRequest::new(vec![]);
        let result = test_helper(key, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Validation error: length"));

        // Too many links
        let key = "from";
        let value = ScoresRequest::new(vec!["www.google.com"; 101]);
        let result = test_helper(key, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Validation error: length"));

        // Incorrect query key
        let key = "fromzzz";
        let value = ScoresRequest::new(vec!["www.google.com", "abc.com", "domain.me"]);
        let result = test_helper(key, &value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Incorrect query parameters. Expected `from`"
        );

        // Incorrect value type
        let key = "from";
        let value = "www.google.com"; // should be an array
        let result = test_helper(key, &value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid type"));

        // Invalid hostname
        let key = "from";
        let value = ScoresRequest::new(vec!["www.google.com", "abc;;;com"]);
        let result = test_helper(key, &value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Hostname is invalid"));
    }

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
}
