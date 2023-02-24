use crate::scoring::Score::*;
use std::collections::HashMap;

use crate::types::{database::LinkDetail, *};

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
        // TODO: actually write the score logic
        if link_details.contains_key(&link) {
            scores.push(LinkScore::new(link.to_owned(), Score::Good));
        } else {
            scores.push(LinkScore::new(link.to_owned(), Score::NoScore));
        }
    }
    scores
}
