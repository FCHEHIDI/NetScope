#[derive(Debug, Clone, Default)]
pub struct WsHub;

impl WsHub {
    pub fn topic(&self) -> &'static str {
        "network-events"
    }
}
