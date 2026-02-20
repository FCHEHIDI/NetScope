use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IngressMessage {
    pub request_id: String,
    pub route: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NormalizedMessage {
    pub request_id: String,
    pub route_group: String,
    pub payload: String,
}

pub fn normalize(message: IngressMessage) -> NormalizedMessage {
    let route_group = message.route.replace('/', "-").trim_matches('-').to_string();

    NormalizedMessage {
        request_id: message.request_id,
        route_group,
        payload: message.payload,
    }
}
