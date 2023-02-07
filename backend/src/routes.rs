use lambda_http::{Body, Error, Request, RequestExt, Response};

use crate::types::AdminVote;

pub fn admin_vote(request: Request) -> Result<Response<Body>, Error> {
    let body = request.payload::<AdminVote>()?;
    println!("{:?}", body);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Votes! {}".into())
        .map_err(Box::new)?;
    return Ok(resp);
}
