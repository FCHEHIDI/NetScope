use serde::{Deserialize, Serialize};

use domain::{OutboundResult, RequestContext};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbeEvent {
    pub trace_id: String,
    pub request_id: String,
    pub route: String,
    pub internal_ip: String,
    pub external_ip: String,
    pub status_code: u16,
    pub latency_ms: u64,
    pub retry_count: u32,
    pub timeout_count: u32,
}

pub fn map_probe_event(context: &RequestContext, result: &OutboundResult) -> ProbeEvent {
    ProbeEvent {
        trace_id: context.trace_id.clone(),
        request_id: context.request_id.clone(),
        route: context.route.clone(),
        internal_ip: context.internal_ip.clone(),
        external_ip: context.external_ip.clone(),
        status_code: result.status_code,
        latency_ms: result.latency_ms,
        retry_count: result.retry_count,
        timeout_count: result.timeout_count,
    }
}
