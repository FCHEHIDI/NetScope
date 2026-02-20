use axum::http::HeaderMap;

pub fn extract_trace_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-trace-id")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}
