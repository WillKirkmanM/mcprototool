use crate::protocol::types::{Identifier, JsonTextComponent, Property, VarInt};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DisconnectLogin {
    pub reason: JsonTextComponent,
}

#[derive(Debug, Clone)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<LoginProperty>,
    pub strict_error_handling: bool,
}

#[derive(Debug, Clone)]
pub struct LoginProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Clone)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct CookieRequestLogin {
    pub key: Identifier,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::Property;
    use uuid::Uuid;

    #[test]
    fn test_disconnect_login_instantiation() {
        let packet = DisconnectLogin {
            reason: r#"{"text":"Disconnected by server"}"#.to_string(),
        };
        assert!(!packet.reason.is_empty());
    }

    #[test]
    fn test_encryption_request_instantiation() {
        let packet = EncryptionRequest {
            server_id: "".to_string(),
            public_key: vec![1, 2, 3, 4],
            verify_token: vec![5, 6, 7, 8],
            should_authenticate: true,
        };
        assert!(packet.server_id.is_empty());
        assert_eq!(packet.public_key, vec![1, 2, 3, 4]);
        assert_eq!(packet.verify_token, vec![5, 6, 7, 8]);
        assert!(packet.should_authenticate);
    }

    #[test]
    fn test_login_success_instantiation() {
        let player_uuid = Uuid::new_v4();
        let packet = LoginSuccess {
            uuid: player_uuid,
            username: "Player123".to_string(),
            properties: vec![LoginProperty {
                name: "textures".to_string(),
                value: "base64encodedvalue".to_string(),
                signature: Some("base64signature".to_string()),
            }],
            strict_error_handling: false,
        };
        assert_eq!(packet.uuid, player_uuid);
        assert_eq!(packet.username, "Player123");
        assert_eq!(packet.properties.len(), 1);
        assert_eq!(packet.properties[0].name, "textures");
    }

    #[test]
    fn test_set_compression_instantiation() {
        let packet = SetCompression {
            threshold: VarInt(256),
        };
        assert_eq!(packet.threshold, VarInt(256));
    }

    #[test]
    fn test_login_plugin_request_instantiation() {
        let packet = LoginPluginRequest {
            message_id: VarInt(1),
            channel: "my:channel".to_string(),
            data: vec![10, 20, 30],
        };
        assert_eq!(packet.message_id, VarInt(1));
        assert_eq!(packet.channel, "my:channel");
        assert_eq!(packet.data, vec![10, 20, 30]);
    }

    #[test]
    fn test_cookie_request_login_instantiation() {
        let packet = CookieRequestLogin {
            key: "my:cookie_key".to_string(),
        };
        assert_eq!(packet.key, "my:cookie_key");
    }
}
