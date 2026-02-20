use axum::http::{HeaderValue, Request};

pub fn inject_request_id<B>(request: &mut Request<B>) {
    if !request.headers().contains_key("x-request-id") {
        request
            .headers_mut()
            .insert("x-request-id", HeaderValue::from_static("bootstrap-request-id"));
    }
}
