
use axum::http::{Request, Response, StatusCode};

pub async fn always_errors()->Result<(),StatusCode>
{
    Err(StatusCode::IM_A_TEAPOT)
}