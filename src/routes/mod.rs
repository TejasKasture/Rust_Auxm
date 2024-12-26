
mod helloworld;
mod mirror_body_string;
mod mirror_body_json;
mod path_variable;
mod query_params;
//mod mirror_image_agent;
mod mirror_custom_agent;
mod middlelayer_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;

use std::clone;

use axum::{http::Method, middleware, routing::{get,post, Router}, Extension};
use helloworld::hellow_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variable::{path_variable,hard_coded_path};
use query_params::query_params;
//use mirror_image_agent::mirror_image_agent;
use mirror_custom_agent::mirror_custom_agent;
 use tower_http::cors::{Any, CorsLayer};
 use middlelayer_message::middlelayer_message;
 use read_middleware_custom_header::read_middleware_custom_header;
 use set_middleware_custom_header::set_middleware_custom_header;
 use always_errors::always_errors;

 #[derive(Clone)]
 pub struct SharedData
 {
    pub message:String,
 }

pub fn create_routes() -> Router {

let cors = CorsLayer::new()
        .allow_methods([Method::GET,Method::POST])
        .allow_origin(Any);
let shared_data=SharedData{
    message:"HEllo this Tejas Kasture !!!".to_owned(),
};   

    Router::new().route("/read_middleware_custom_header", get(read_middleware_custom_header))
    .route_layer(middleware::from_fn(set_middleware_custom_header))
    .route("/hello", get(hellow_world))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_variable/:id", get(path_variable))
    .route("/path_variable/15", get(hard_coded_path))
    .route("/query_params", get(query_params))
    //.route("/mirror_image_agent", get(mirror_image_agent))
    .route("/mirror_image_agent", get(mirror_custom_agent))
    .route("/middlelayer_message", get(middlelayer_message))
    .layer(cors)
    .layer(Extension(shared_data))
    .route("/always_errors",get(always_errors))
    
}