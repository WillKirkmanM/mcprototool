use crate::protocol::types::{
    CustomReportDetail, Identifier, JsonTextComponent, KnownPack, RegistryEntry, RegistryTagData,
    ServerLink, VarInt,
};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct CookieRequestConfiguration {
    pub key: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct ClientboundPluginMessageConfiguration {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct DisconnectConfiguration {
    pub reason: JsonTextComponent,
}

#[derive(Debug, PartialEq)]
pub struct FinishConfiguration;

#[derive(Debug, PartialEq)]
pub struct ClientboundKeepAliveConfiguration {
    pub keep_alive_id: i64,
}

#[derive(Debug, PartialEq)]
pub struct PingConfiguration {
    pub id: i32,
}

#[derive(Debug, PartialEq)]
pub struct ResetChat;

#[derive(Debug, PartialEq)]
pub struct RegistryData {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, PartialEq)]
pub struct RemoveResourcePackConfiguration {
    pub uuid: Option<Uuid>,
}

#[derive(Debug, PartialEq)]
pub struct AddResourcePackConfiguration {
    pub uuid: Uuid,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub prompt_message: Option<JsonTextComponent>,
}

#[derive(Debug, PartialEq)]
pub struct StoreCookieConfiguration {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct TransferConfiguration {
    pub host: String,
    pub port: VarInt,
}

#[derive(Debug, PartialEq)]
pub struct FeatureFlags {
    pub feature_flags: Vec<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct UpdateTagsConfiguration {
    pub tags: Vec<RegistryTagData>,
}

#[derive(Debug, PartialEq)]
pub struct ClientboundKnownPacks {
    pub known_packs: Vec<KnownPack>,
}

#[derive(Debug, PartialEq)]
pub struct CustomReportDetailsConfiguration {
    pub details: Vec<CustomReportDetail>,
}

#[derive(Debug, PartialEq)]
pub struct ServerLinksConfiguration {
    pub links: Vec<ServerLink>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::{
        BuiltInServerLinkLabel, CustomReportDetail, KnownPack, RegistryEntry, RegistryTagData,
        ServerLink, ServerLinkLabel, Tag,
    };
    use uuid::Uuid;

    #[test]
    fn test_cookie_request_config_instantiation() {
        let packet = CookieRequestConfiguration {
            key: "my:cookie".into(),
        };
        assert_eq!(packet.key, "my:cookie");
    }

    #[test]
    fn test_plugin_message_config_instantiation() {
        let packet = ClientboundPluginMessageConfiguration {
            channel: "my:channel".into(),
            data: vec![1, 2, 3],
        };
        assert_eq!(packet.channel, "my:channel");
        assert_eq!(packet.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_disconnect_config_instantiation() {
        let packet = DisconnectConfiguration {
            reason: r#"{"text":"Config disconnect"}"#.into(),
        };
        assert!(!packet.reason.is_empty());
    }

    #[test]
    fn test_finish_config_instantiation() {
        let _packet = FinishConfiguration;
        assert!(true);
    }

    #[test]
    fn test_keep_alive_config_instantiation() {
        let packet = ClientboundKeepAliveConfiguration {
            keep_alive_id: 1234567890,
        };
        assert_eq!(packet.keep_alive_id, 1234567890);
    }

    #[test]
    fn test_ping_config_instantiation() {
        let packet = PingConfiguration { id: 9876 };
        assert_eq!(packet.id, 9876);
    }

    #[test]
    fn test_reset_chat_instantiation() {
        let _packet = ResetChat;
        assert!(true);
    }

    #[test]
    fn test_registry_data_instantiation() {
        let packet = RegistryData {
            registry_id: "minecraft:dimension_type".into(),
            entries: vec![RegistryEntry {
                entry_id: "minecraft:overworld".into(),
                data: Some(vec![0x0a, 0x00]), /* NBT placeholder */
            }],
        };
        assert_eq!(packet.registry_id, "minecraft:dimension_type");
        assert_eq!(packet.entries.len(), 1);
    }

    #[test]
    fn test_remove_resource_pack_instantiation() {
        let uuid = Uuid::new_v4();
        let packet_some = RemoveResourcePackConfiguration { uuid: Some(uuid) };
        assert_eq!(packet_some.uuid, Some(uuid));
        let packet_none = RemoveResourcePackConfiguration { uuid: None };
        assert_eq!(packet_none.uuid, None);
    }

    #[test]
    fn test_add_resource_pack_instantiation() {
        let uuid = Uuid::new_v4();
        let packet = AddResourcePackConfiguration {
            uuid,
            url: "http://example.com/pack.zip".into(),
            hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".into(),
            forced: true,
            prompt_message: Some(r#"{"text":"Please accept"}"#.into()),
        };
        assert_eq!(packet.uuid, uuid);
        assert_eq!(packet.url, "http://example.com/pack.zip");
        assert_eq!(packet.hash.len(), 40);
        assert!(packet.forced);
        assert!(packet.prompt_message.is_some());
    }

    #[test]
    fn test_store_cookie_config_instantiation() {
        let packet = StoreCookieConfiguration {
            key: "my:persistent_cookie".into(),
            payload: vec![10, 20, 30],
        };
        assert_eq!(packet.key, "my:persistent_cookie");
        assert_eq!(packet.payload, vec![10, 20, 30]);
    }

    #[test]
    fn test_transfer_config_instantiation() {
        let packet = TransferConfiguration {
            host: "anotherserver.com".into(),
            port: VarInt(25566),
        };
        assert_eq!(packet.host, "anotherserver.com");
        assert_eq!(packet.port, VarInt(25566));
    }

    #[test]
    fn test_feature_flags_instantiation() {
        let packet = FeatureFlags {
            feature_flags: vec![
                "minecraft:vanilla".into(),
                "minecraft:experimental_feature".into(),
            ],
        };
        assert_eq!(packet.feature_flags.len(), 2);
        assert_eq!(packet.feature_flags[0], "minecraft:vanilla");
    }

    #[test]
    fn test_update_tags_config_instantiation() {
        let packet = UpdateTagsConfiguration {
            tags: vec![RegistryTagData {
                registry_id: "minecraft:item".into(),
                tags: vec![Tag {
                    tag_name: "minecraft:logs".into(),
                    entries: vec![VarInt(1), VarInt(2), VarInt(3)],
                }],
            }],
        };
        assert_eq!(packet.tags.len(), 1);
        assert_eq!(packet.tags[0].registry_id, "minecraft:item");
        assert_eq!(packet.tags[0].tags.len(), 1);
    }

    #[test]
    fn test_clientbound_known_packs_instantiation() {
        let packet = ClientboundKnownPacks {
            known_packs: vec![KnownPack {
                namespace: "minecraft".into(),
                id: "core".into(),
                version: "1.21.4".into(),
            }],
        };
        assert_eq!(packet.known_packs.len(), 1);
        assert_eq!(packet.known_packs[0].namespace, "minecraft");
    }

    #[test]
    fn test_custom_report_details_instantiation() {
        let packet = CustomReportDetailsConfiguration {
            details: vec![CustomReportDetail {
                title: "Mod Version".into(),
                description: "1.2.3".into(),
            }],
        };
        assert_eq!(packet.details.len(), 1);
        assert_eq!(packet.details[0].title, "Mod Version");
    }

    #[test]
    fn test_server_links_instantiation() {
        let packet = ServerLinksConfiguration {
            links: vec![
                ServerLink {
                    label: ServerLinkLabel::BuiltIn(BuiltInServerLinkLabel::Website),
                    url: "http://example.com".into(),
                },
                ServerLink {
                    label: ServerLinkLabel::Custom(r#"{"text":"Discord"}"#.into()),
                    url: "http://discord.gg/example".into(),
                },
            ],
        };
        assert_eq!(packet.links.len(), 2);
        assert!(matches!(packet.links[0].label, ServerLinkLabel::BuiltIn(_)));
        assert!(matches!(packet.links[1].label, ServerLinkLabel::Custom(_)));
    }
}
