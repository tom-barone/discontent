use crate::types::{database::LinkDetail, *};

pub fn calculate_link_scores(link_details: Vec<LinkDetail>) -> Vec<LinkScore> {
    let mut scores: Vec<LinkScore> = vec![];
    for link_detail in link_details {
        // TODO: actually write the score logic
        scores.push(LinkScore::new(link_detail.link, Score::Good));
    }
    scores
}
