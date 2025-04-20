use crate::protocol::types::VarInt;

#[derive(Debug, PartialEq)]
pub struct StatusRequest {}

#[derive(Debug, PartialEq)]
pub struct PingRequest {
    pub payload: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_request_instantiation() {
        let _packet = StatusRequest {};
        assert!(true);
    }

    #[test]
    fn test_ping_request_instantiation() {
        let payload = chrono::Utc::now().timestamp_millis();
        let packet = PingRequest { payload };
        assert_eq!(packet.payload, payload);
    }
}
