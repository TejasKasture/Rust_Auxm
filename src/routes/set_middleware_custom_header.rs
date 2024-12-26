use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::body::Body;

use super::read_middleware_custom_header::HeaderMessage;



pub async fn set_middleware_custom_header(
   mut request: Request<Body>, // Use `Body` here for the request type
    next: Next,             // No generic argument for `Next`
) -> Result<Response<Body>, StatusCode> { // Specify the generic type `Body` for `Response`
    // Forward the request to the next middleware or handler
    
    // Add a custom header to the response
    // response
    //     .headers_mut()
    //     .insert("X-Custom-Header", "Hello, Axum!".parse().unwrap());
    let header=request.headers();
    let message=header.get("message").ok_or_else(||{StatusCode::BAD_REQUEST})?;

    let message=message.to_str().map_err(|_error|StatusCode::BAD_REQUEST)?.to_owned();
    let extensions=request.extensions_mut();
    extensions.insert(HeaderMessage(message));
    let  response = next.run(request).await;

    Ok(response)
}
