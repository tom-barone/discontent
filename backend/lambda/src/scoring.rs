use crate::scoring::Score::*;
use std::collections::HashMap;

use crate::types::{database::LinkDetail, *};

const GOOD_SCORE_BOUND: &i32 = &20;
const BAD_SCORE_BOUND: &i32 = &-10;

pub fn random_link_scores(links: &Vec<Link>) -> Vec<LinkScore> {
    let score_enums = vec![Good, Bad, Controversial, NoScore];
    let mut scores: Vec<LinkScore> = vec![];
    for link in links {
        // Choose random score from the enums
        let random_score = score_enums
            .get(rand::random::<usize>() % score_enums.len())
            .unwrap()
            .to_owned();
        scores.push(LinkScore::new(link.to_owned(), random_score));
    }
    scores
}

pub fn calculate_link_scores(
    links: &Vec<Link>,
    link_details: &HashMap<Link, LinkDetail>,
) -> Vec<LinkScore> {
    let mut scores: Vec<LinkScore> = vec![];
    for link in links {
        match link_details.get(&link) {
            Some(link_detail) => {
                let LinkDetail {
                    sum_of_votes,
                    count_of_votes,
                    ..
                } = link_detail;

                if sum_of_votes >= GOOD_SCORE_BOUND {
                    scores.push(LinkScore::new(link.to_owned(), Score::Good));
                } else if sum_of_votes <= BAD_SCORE_BOUND {
                    scores.push(LinkScore::new(link.to_owned(), Score::Bad));
                } else if count_of_votes > &50
                    && sum_of_votes > BAD_SCORE_BOUND
                    && sum_of_votes < GOOD_SCORE_BOUND
                {
                    scores.push(LinkScore::new(link.to_owned(), Score::Controversial));
                } else {
                    scores.push(LinkScore::new(link.to_owned(), Score::NoScore));
                }
            }
            None => {
                scores.push(LinkScore::new(link.to_owned(), Score::NoScore));
            }
        }
    }
    scores
}
