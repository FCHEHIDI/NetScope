use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Outcome {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboundResult {
    pub status_code: u16,
    pub latency_ms: u64,
    pub bytes_sent: usize,
    pub bytes_received: usize,
    pub retry_count: u32,
    pub timeout_count: u32,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditEvent {
    pub event_version: String,
    pub event_type: String,
    pub timestamp: String,
    pub trace_id: String,
    pub request_id: String,
    pub route: String,
    pub status_code: u16,
    pub latency_ms: u64,
    pub retry_count: u32,
    pub timeout_count: u32,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestMetricPoint {
    /// Groupe de route normalisé (ex : "orders", "payments").
    pub route_group: String,
    /// IP interne du worker émetteur — conservée avant traduction NAT (invariant I5).
    pub internal_ip: String,
    /// IP externe de la destination (API Gateway ou API finale) — après NAT (invariant I5).
    pub external_ip: String,
    /// Classe de statut HTTP : "2xx", "4xx", "5xx".
    pub status_class: String,
    /// Latence mesurée via horloge monotone (`Instant`) en millisecondes (invariant I3).
    pub latency_ms: u64,
    pub retry_count: u32,
    pub timeout_count: u32,
    pub is_error: bool,
}
