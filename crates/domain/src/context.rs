use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestContext {
    pub trace_id: String,
    pub span_id: String,
    pub request_id: String,
    pub idempotency_key: String,
    pub route: String,
    pub internal_ip: String,
    pub external_ip: String,
}
