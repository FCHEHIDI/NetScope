use domain::RequestContext;

pub fn sample_request_context() -> RequestContext {
    RequestContext {
        trace_id: "trace-bootstrap".to_string(),
        span_id: "span-bootstrap".to_string(),
        request_id: "req-bootstrap".to_string(),
        idempotency_key: "idem-bootstrap".to_string(),
        route: "/v1/partner/send".to_string(),
        internal_ip: "10.0.0.10".to_string(),
        external_ip: "203.0.113.10".to_string(),
    }
}
