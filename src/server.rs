use crate::protocol::{
    encoding::{
        write_packet_frame, read_packet_frame, DecodeError,
        write_string, write_varint, write_i64,
        read_string_sync, read_uuid_sync, read_varint_sync, 
        read_bool_sync,
    },
    handshaking::serverbound::Handshake,
    login::{
        clientbound::{LoginSuccess, LoginProperty},
        serverbound::LoginStart,
    },
    status::{
        clientbound::{StatusResponse, PongResponse},
        serverbound::{StatusRequest, PingRequest},
    },
    types::{NextState, VarInt, JsonTextComponent},
};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use std::error::Error;
use bytes::{Buf, BytesMut};
use uuid::Uuid;

const STATUS_JSON: &str = r#"{
    "version": {
        "name": "mcprototool-rs 1.20.6",
        "protocol": 767
    },
    "players": {
        "max": 20,
        "online": 0,
        "sample": []
    },
    "description": {
        "text": "Minimal Rust Server"
    },
    "enforcesSecureChat": false,
    "previewsChat": false
}"#;


pub async fn run_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address).await?;
    println!("Minimal Minecraft server listening on {}", address);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            } else {
                println!("Connection closed gracefully: {}", addr);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut buf_writer = BufWriter::new(writer);

    let (handshake_id, mut handshake_data) = read_packet_frame(&mut buf_reader).await?;
    if handshake_id.0 != 0x00 {
        return Err(format!("Expected Handshake packet (ID 0x00), got {:#04x}", handshake_id.0).into());
    }
    let handshake = deserialize_handshake(&mut handshake_data).await?;
    println!("Received Handshake: Proto={}, Addr={}, Port={}, Next={:?}",
        handshake.protocol_version.0, handshake.server_address, handshake.server_port, handshake.next_state);

    match handshake.next_state.0 {
        1 => {
            println!("Handling Status request...");

            let (status_req_id, _status_req_data) = read_packet_frame(&mut buf_reader).await?;
            if status_req_id.0 != 0x00 {
                 return Err(format!("Expected Status Request packet (ID 0x00), got {:#04x}", status_req_id.0).into());
            }

            let status_response_packet = StatusResponse {
                json_response: STATUS_JSON.to_string(),
            };
            let status_response_data = serialize_status_response(&status_response_packet).await?;
            write_packet_frame(&mut buf_writer, VarInt::from(0x00), &status_response_data).await?;
            buf_writer.flush().await?;
            println!("Sent Status Response.");

            let (ping_req_id, mut ping_req_data) = read_packet_frame(&mut buf_reader).await?;
             if ping_req_id.0 != 0x01 {
                 return Err(format!("Expected Ping Request packet (ID 0x01), got {:#04x}", ping_req_id.0).into());
            }

            let ping_request = deserialize_ping_request(&mut ping_req_data).await?;
            println!("Received Ping Request: Payload={}", ping_request.payload);

            let pong_response_packet = PongResponse {
                payload: ping_request.payload,
            };

            let pong_response_data = serialize_pong_response(&pong_response_packet).await?;
            write_packet_frame(&mut buf_writer, VarInt::from(0x01), &pong_response_data).await?;
            buf_writer.flush().await?;
            println!("Sent Pong Response.");

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            println!("Waited briefly after Pong.");

        }
        2 => {
            println!("Handling Login request...");

            let (login_start_id, mut login_start_data) = read_packet_frame(&mut buf_reader).await?;
            if login_start_id.0 != 0x00 {
                return Err(format!("Expected Login Start packet (ID 0x00), got {:#04x}", login_start_id.0).into());
            }
            let login_start = deserialize_login_start(&mut login_start_data).await?;
            println!("Received Login Start: Name={}, UUID={}", login_start.name, login_start.player_uuid);

            let login_success_packet = LoginSuccess {
                uuid: login_start.player_uuid,
                username: login_start.name.clone(),
                properties: vec![],
                strict_error_handling: false,
            };
            let login_success_data = serialize_login_success(&login_success_packet).await?;
            write_packet_frame(&mut buf_writer, VarInt::from(0x02), &login_success_data).await?;
            buf_writer.flush().await?;
            println!("Sent Login Success for {}", login_start.name);
        }
        _ => {
            return Err(format!("Client requested unsupported state: {:?}", handshake.next_state).into());
        }
    }

    Ok(())
}

async fn deserialize_handshake(buf: &mut BytesMut) -> Result<Handshake, DecodeError> {
    let protocol_version = read_varint_sync(buf)?;
    let server_address = read_string_sync(buf)?;
    if buf.remaining() < 2 { return Err(DecodeError::NotEnoughBytes); }
    let server_port = buf.get_u16();
    let next_state = read_varint_sync(buf)?;
    Ok(Handshake { protocol_version, server_address, server_port, next_state })
}

async fn deserialize_login_start(buf: &mut BytesMut) -> Result<LoginStart, DecodeError> {
    let name = read_string_sync(buf)?;
    let player_uuid = read_uuid_sync(buf)?;
    Ok(LoginStart { name, player_uuid })
}

async fn serialize_login_success(packet: &LoginSuccess) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut writer = BufWriter::new(&mut buf);
    writer.write_all(&packet.uuid.into_bytes()).await?;
    write_string(&mut writer, &packet.username).await?;
    write_varint(&mut writer, VarInt(packet.properties.len() as i32)).await?;
    for prop in &packet.properties {
        write_string(&mut writer, &prop.name).await?;
        write_string(&mut writer, &prop.value).await?;
        let is_signed = prop.signature.is_some();
        writer.write_u8(if is_signed { 1 } else { 0 }).await?;
        if let Some(sig) = &prop.signature {
            write_string(&mut writer, sig).await?;
        }
    }

    writer.flush().await?;
    Ok(buf)
}

async fn serialize_status_response(packet: &StatusResponse) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut writer = BufWriter::new(&mut buf);
    write_string(&mut writer, &packet.json_response).await?;
    writer.flush().await?;
    Ok(buf)
}

async fn deserialize_ping_request(buf: &mut BytesMut) -> Result<PingRequest, DecodeError> {
    let payload = read_i64_sync(buf)?;
    Ok(PingRequest { payload })
}

async fn serialize_pong_response(packet: &PongResponse) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut writer = BufWriter::new(&mut buf);
    write_i64(&mut writer, packet.payload).await?;
    writer.flush().await?;
    Ok(buf)
}

fn read_i64_sync(buf: &mut BytesMut) -> Result<i64, DecodeError> {
    if buf.remaining() < 8 { return Err(DecodeError::NotEnoughBytes); }
    Ok(buf.get_i64())
}

async fn write_bool<W: AsyncWriteExt + Unpin>(writer: &mut W, value: bool) -> Result<(), std::io::Error> {
    writer.write_u8(if value { 1 } else { 0 }).await
}
async fn write_uuid<W: AsyncWriteExt + Unpin>(writer: &mut W, value: Uuid) -> Result<(), std::io::Error> {
    writer.write_all(&value.into_bytes()).await
}