//use axum::http::HeaderMap;

// pub async fn mirror_image_agent(HeaderMap(header): HeaderMap<UserAgent>)->String
// {
//     header.to_owned()
// }

// pub async fn mirror_image_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
//     user_agent.to_string()
// }

use axum::http::HeaderMap;

pub async fn mirror_image_agent(user_agent: HeaderMap) -> String {
    user_agent.to_string()
}
