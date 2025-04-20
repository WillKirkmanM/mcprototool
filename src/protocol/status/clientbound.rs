use crate::protocol::types::{JsonTextComponent, VarInt};

#[derive(Debug, PartialEq)]
pub struct StatusResponse {
    pub json_response: String,
}

#[derive(Debug, PartialEq)]
pub struct PongResponse {
    pub payload: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_response_instantiation() {
        let packet = StatusResponse {
            json_response: r#"{"version":{"name":"1.21.5","protocol":770},"players":{"max":100,"online":5,"sample":[]},"description":{"text":"Hello world"}}"#.to_string(),
        };
        assert!(!packet.json_response.is_empty());
    }

    #[test]
    fn test_pong_response_instantiation() {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let packet = PongResponse { payload: timestamp };
        assert_eq!(packet.payload, timestamp);
    }
}
