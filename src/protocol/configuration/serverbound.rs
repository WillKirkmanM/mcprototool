use crate::protocol::types::{
    ChatMode, DisplayedSkinParts, Identifier, KnownPack, MainHand, ParticleStatus,
    ResourcePackResult, VarInt,
};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct ClientInformationConfiguration {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: ParticleStatus,
}

#[derive(Debug, PartialEq)]
pub struct CookieResponseConfiguration {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
pub struct ServerboundPluginMessageConfiguration {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct AcknowledgeFinishConfiguration;

#[derive(Debug, PartialEq)]
pub struct ServerboundKeepAliveConfiguration {
    pub keep_alive_id: i64,
}

#[derive(Debug, PartialEq)]
pub struct PongConfiguration {
    pub id: i32,
}

#[derive(Debug, PartialEq)]
pub struct ResourcePackResponseConfiguration {
    pub uuid: Uuid,
    pub result: ResourcePackResult,
}

#[derive(Debug, PartialEq)]
pub struct ServerboundKnownPacks {
    pub known_packs: Vec<KnownPack>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::{
        ChatMode, DisplayedSkinParts, KnownPack, MainHand, ParticleStatus, ResourcePackResult,
    };
    use uuid::Uuid;

    #[test]
    fn test_client_info_config_instantiation() {
        let packet = ClientInformationConfiguration {
            locale: "en_US".into(),
            view_distance: 8,
            chat_mode: ChatMode::Enabled,
            chat_colors: true,
            displayed_skin_parts: DisplayedSkinParts::all(),
            main_hand: MainHand::Right,
            enable_text_filtering: false,
            allow_server_listings: true,
            particle_status: ParticleStatus::All,
        };
        assert_eq!(packet.locale, "en_US");
        assert_eq!(packet.view_distance, 8);
        assert_eq!(packet.chat_mode, ChatMode::Enabled);
        assert!(packet.chat_colors);
        assert_eq!(packet.displayed_skin_parts, DisplayedSkinParts::all());
        assert_eq!(packet.main_hand, MainHand::Right);
        assert!(!packet.enable_text_filtering);
        assert!(packet.allow_server_listings);
        assert_eq!(packet.particle_status, ParticleStatus::All);
    }

    #[test]
    fn test_cookie_response_config_instantiation() {
        let packet_some = CookieResponseConfiguration {
            key: "my:cookie".into(),
            payload: Some(vec![1, 2, 3]),
        };
        assert_eq!(packet_some.key, "my:cookie");
        assert_eq!(packet_some.payload, Some(vec![1, 2, 3]));
        let packet_none = CookieResponseConfiguration {
            key: "empty:cookie".into(),
            payload: None,
        };
        assert_eq!(packet_none.key, "empty:cookie");
        assert_eq!(packet_none.payload, None);
    }

    #[test]
    fn test_serverbound_plugin_message_config_instantiation() {
        let packet = ServerboundPluginMessageConfiguration {
            channel: "brand".into(),
            data: vec![b'v', b'a', b'n', b'i', b'l', b'l', b'a'],
        };
        assert_eq!(packet.channel, "brand");
        assert_eq!(packet.data, b"vanilla");
    }

    #[test]
    fn test_ack_finish_config_instantiation() {
        let _packet = AcknowledgeFinishConfiguration;
        assert!(true);
    }

    #[test]
    fn test_serverbound_keep_alive_config_instantiation() {
        let packet = ServerboundKeepAliveConfiguration {
            keep_alive_id: 1234567890,
        };
        assert_eq!(packet.keep_alive_id, 1234567890);
    }

    #[test]
    fn test_pong_config_instantiation() {
        let packet = PongConfiguration { id: 9876 };
        assert_eq!(packet.id, 9876);
    }

    #[test]
    fn test_resource_pack_response_config_instantiation() {
        let uuid = Uuid::new_v4();
        let packet = ResourcePackResponseConfiguration {
            uuid,
            result: ResourcePackResult::Accepted,
        };
        assert_eq!(packet.uuid, uuid);
        assert_eq!(packet.result, ResourcePackResult::Accepted);
    }

    #[test]
    fn test_serverbound_known_packs_instantiation() {
        let packet = ServerboundKnownPacks {
            known_packs: vec![
                KnownPack {
                    namespace: "minecraft".into(),
                    id: "core".into(),
                    version: "1.21.4".into(),
                },
                KnownPack {
                    namespace: "custom".into(),
                    id: "datapack".into(),
                    version: "1.0".into(),
                },
            ],
        };
        assert_eq!(packet.known_packs.len(), 2);
        assert_eq!(packet.known_packs[1].namespace, "custom");
    }
}
