use crate::protocol::types::{NextState, VarInt};

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[derive(Debug, PartialEq)]
pub struct LegacyServerListPing {
    pub payload: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::NextState;

    #[test]
    fn test_handshake_instantiation() {
        let packet = Handshake {
            protocol_version: VarInt(770),
            server_address: "localhost".to_string(),
            server_port: 25565,
            next_state: VarInt(NextState::Login as i32),
        };

        assert_eq!(packet.protocol_version, VarInt(770));
        assert_eq!(packet.server_address, "localhost");
        assert_eq!(packet.server_port, 25565);
        assert_eq!(packet.next_state, VarInt(NextState::Login as i32));
    }

    #[test]
    fn test_legacy_ping_instantiation() {
        let packet = LegacyServerListPing { payload: 0x01 };
        assert_eq!(packet.payload, 0x01);
    }
}
