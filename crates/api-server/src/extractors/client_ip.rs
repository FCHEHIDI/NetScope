use axum::http::HeaderMap;

pub fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}
