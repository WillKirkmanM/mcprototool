use crate::protocol::types::{Identifier, VarInt};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}

#[derive(Debug, Clone)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct LoginAcknowledged;

#[derive(Debug, PartialEq)]
pub struct CookieResponseLogin {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_login_start_instantiation() {
        let player_uuid = Uuid::new_v4();
        let packet = LoginStart {
            name: "Player456".to_string(),
            player_uuid,
        };
        assert_eq!(packet.name, "Player456");
        assert_eq!(packet.player_uuid, player_uuid);
    }

    #[test]
    fn test_encryption_response_instantiation() {
        let packet = EncryptionResponse {
            shared_secret: vec![11, 12, 13, 14],
            verify_token: vec![15, 16, 17, 18],
        };
        assert_eq!(packet.shared_secret, vec![11, 12, 13, 14]);
        assert_eq!(packet.verify_token, vec![15, 16, 17, 18]);
    }

    #[test]
    fn test_login_plugin_response_instantiation() {
        let packet_with_data = LoginPluginResponse {
            message_id: VarInt(2),
            data: Some(vec![40, 50, 60]),
        };
        assert_eq!(packet_with_data.message_id, VarInt(2));
        assert_eq!(packet_with_data.data, Some(vec![40, 50, 60]));

        let packet_without_data = LoginPluginResponse {
            message_id: VarInt(3),
            data: None,
        };
        assert_eq!(packet_without_data.message_id, VarInt(3));
        assert_eq!(packet_without_data.data, None);
    }

    #[test]
    fn test_login_acknowledged_instantiation() {
        let _packet = LoginAcknowledged;
        assert!(true);
    }

    #[test]
    fn test_cookie_response_login_instantiation() {
        let packet_with_payload = CookieResponseLogin {
            key: "another:cookie".to_string(),
            payload: Some(vec![1, 2, 3]),
        };
        assert_eq!(packet_with_payload.key, "another:cookie");
        assert_eq!(packet_with_payload.payload, Some(vec![1, 2, 3]));

        let packet_without_payload = CookieResponseLogin {
            key: "empty:cookie".to_string(),
            payload: None,
        };
        assert_eq!(packet_without_payload.key, "empty:cookie");
        assert_eq!(packet_without_payload.payload, None);
    }
}
