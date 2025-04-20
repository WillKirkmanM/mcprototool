<p align="center">
  <img src="https://avatars.githubusercontent.com/u/138057124?s=200&v=4" width="150" />
</p>
<h1 align="center">MC Proto Tool</h1>

<p align="center">
  
</p>

<h4 align="center">
  <a href="https://github.com/WillKirkmanM/music/releases">Releases</a>
</h4>

<p align="center">Minecraft Server Protocol with Hands8haking, Configuration, Login, Play, Status & Encoding with Client Connections</p>

## Overview

`mcprototool` is a Rust project demonstrating the implementation of core aspects of the Minecraft Java Edition network protocol. It provides a server capable of handling the Handshaking, Status, and Login protocol states for offline-mode connections.

The primary goal is to showcase how to structure protocol definitions, handle Minecraft's specific data types, manage network connections asynchronously, and process packet sequences according to the protocol state machine.

## Technical Deep Dive

### 1. Minecraft Protocol Fundamentals

The Minecraft protocol is stateful. A connection progresses through distinct states, each with its own set of valid packets that can be sent or received. The primary states involved are:

1.  **Handshaking:** The initial state upon connection. The client sends a single `Handshake` packet specifying the protocol version, server address/port it intended to connect to, and the desired next state (either Status or Login).
2.  **Status:** Used for querying server information (version, MOTD, player count) displayed in the multiplayer server list.
    *   Client -> Server: `Status Request`
    *   Server -> Client: `Status Response` (contains JSON payload)
    *   Client -> Server: `Ping Request` (contains `i64` payload)
    *   Server -> Client: `Pong Response` (echoes `i64` payload)
3.  **Login:** Used for authenticating the player and transitioning to the Play state.
    *   Client -> Server: `Login Start` (contains username, UUID)
    *   Server -> Client (Online Mode): `Encryption Request`
    *   Client -> Server (Online Mode): `Encryption Response`
    *   *Encryption and Compression enabled here*
    *   Server -> Client: `Set Compression` (optional)
    *   Server -> Client: `Login Success` (contains assigned UUID, username, properties)
    *   Server -> Client: `Disconnect (Login)`
    *   Server -> Client: `Login Plugin Request`
    *   Client -> Server: `Login Plugin Response`
4.  **Play:** The main gameplay state where world data, entities, player actions, chat, etc., are exchanged. This state has the largest and most complex set of packets.

### 2. Project Structure

The codebase is organised as follows:

*   **`src/main.rs`:** Entry point for the application. Currently configured to start the server via `server::run_server`. Can be modified to run client-side logic.
*   **`src/server.rs`:** Implements the TCP server logic using `tokio`. It listens for connections, handles the protocol state transitions based on the client's handshake request, and processes Status or Login sequences.
*   **`src/protocol/`:** Contains the core protocol definitions and logic.
    *   **`mod.rs`:** Declares the submodules within `protocol`.
    *   **`types.rs`:** Defines fundamental Minecraft data types (`VarInt`, `VarLong`, `Position`, `JsonTextComponent`, etc.) and potentially helper functions for them.
    *   **`encoding.rs`:** Crucial module for handling serialization and deserialization.
        *   Provides `async` functions (`read_varint`, `write_string`, etc.) for reading/writing protocol types directly from/to asynchronous I/O streams (`AsyncRead`, `AsyncWrite`).
        *   Provides `_sync` functions (`read_varint_sync`, `write_varint_sync`, etc.) for reading/writing protocol types from/to in-memory buffers (`BytesMut`, `Vec<u8>`).
        *   Implements packet framing logic:
            *   `write_packet_frame`: Calculates packet length (ID + Data), writes the length as a VarInt, then writes the Packet ID (as VarInt) and the data.
            *   `read_packet_frame`: Reads the packet length (VarInt), reads that many bytes into a buffer (`BytesMut`), reads the Packet ID (VarInt) from the buffer, and returns the ID and the remaining data buffer.
        *   Defines `DecodeError` for robust error handling during deserialization.
    *   **`handshaking/`**, **`status/`**, **`login/`**, **`play/`:** Submodules organised by protocol state. Each typically contains:
        *   `clientbound.rs`: Struct definitions for packets sent *from* the server *to* the client in that state.
        *   `serverbound.rs`: Struct definitions for packets sent *from* the client *to* the server in that state.

### 3. Networking with Tokio

The server utilizes the `tokio` runtime for asynchronous I/O:

*   **`TcpListener::bind`:** Creates a listener socket bound to the specified address (`127.0.0.1:25565`).
*   **`listener.accept()`:** Asynchronously waits for and accepts incoming TCP connections, yielding a `TcpStream` and the client's address.
*   **`tokio::spawn`:** Each accepted connection is handled in a separate, non-blocking asynchronous task to allow the server to handle multiple clients concurrently.
*   **`stream.into_split()`:** Splits the `TcpStream` into independent readable and writable halves.
*   **`BufReader`/`BufWriter`:** Wraps the raw stream halves to provide buffering, improving performance by reducing the number of underlying system calls.

### 4. Packet Handling and State Management

*   **Framing:** The `encoding::read_packet_frame` and `encoding::write_packet_frame` functions are central to handling Minecraft's length-prefixed packet structure.
*   **Deserialization:** When a packet frame is read, `read_packet_frame` returns the Packet ID and a `BytesMut` buffer containing the packet data. The server then uses the Packet ID to determine which specific packet struct to deserialize the data into, typically using the `_sync` deserialization helpers from `encoding.rs` (e.g., `read_string_sync`, `read_i64_sync`) on the `BytesMut` buffer.
*   **Serialization:** To send a packet, the server constructs the appropriate packet struct, serializes its data fields into a byte vector (using `async` helpers like `write_string`, `write_i64`), and then passes the Packet ID and the serialised data vector to `write_packet_frame` to handle length prefixing and writing to the stream.
*   **State Logic (`server.rs`):** The `handle_connection` function first reads the `Handshake` packet. Based on the `next_state` field in the handshake, it branches into either the Status handling logic or the Login handling logic, ensuring only packets valid for the current state are expected and processed.

### 5. Building and Running

**Prerequisites:**

*   Rust toolchain (including `cargo`): [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

**Running the Server:**

1.  Navigate to the project's root directory in your terminal.
2.  Run the command:
    ```bash
    cargo run
    ```
3.  The server will start listening on `127.0.0.1:25565`. You can test it using a Minecraft client configured for offline mode or tools like the provided `connect.ts` script (using `minecraft-server-util`).

**Running as a Client (Example):**

To run the client-side Status check code (as shown in previous examples):

1.  Modify `src/main.rs` to contain the client connection logic instead of the `server::run_server` call.
2.  Ensure the target server (e.g., the Rust server run in a separate process, or an official Minecraft server) is running.
3.  Run `cargo run`.

### 5. Testing

**Running Tests:**

| Protocol State  | Clientbound (Server -> Client) | Serverbound (Client -> Server) |
| :-------------- | :----------------------------: | :----------------------------: |
| **Handshaking** | N/A                               | ✅                             |
| **Status**      | ✅                             | ✅                             |
| **Login**       | ✅                             | ✅                             |
| **Configuration**| ✅                             | ✅                             |
| **Play**        | ✅                             | ✅                             |

To run all `273` unit tests, execute the following command in the project's root directory:

```bash
$ cargo test


running 237 tests
    test protocol::configuration::clientbound::tests::test_cookie_request_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_custom_report_details_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_ack_finish_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_disconnect_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_finish_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_keep_alive_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_feature_flags_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_remove_resource_pack_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_reset_chat_instantiation ...
     ok
    test protocol::configuration::clientbound::tests::test_store_cookie_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_server_links_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_update_tags_config_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_client_info_config_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_resource_pack_response_config_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_cookie_response_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_add_resource_pack_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_ping_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_registry_data_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_clientbound_known_packs_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_pong_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_plugin_message_config_instantiation ... ok
    test protocol::configuration::clientbound::tests::test_transfer_config_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_serverbound_keep_alive_config_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_serverbound_known_packs_instantiation ... ok
    test protocol::configuration::serverbound::tests::test_serverbound_plugin_message_config_instantiation ... ok
    test protocol::handshaking::serverbound::tests::test_handshake_instantiation ... ok
    test protocol::handshaking::serverbound::tests::test_legacy_ping_instantiation ... ok
    test protocol::login::clientbound::tests::test_cookie_request_login_instantiation ... ok
    test protocol::login::clientbound::tests::test_disconnect_login_instantiation ... ok
    test protocol::login::clientbound::tests::test_encryption_request_instantiation ...
     ok
    test protocol::login::clientbound::tests::test_login_plugin_request_instantiation ... ok
    test protocol::login::clientbound::tests::test_login_success_instantiation ... ok  
    test protocol::login::clientbound::tests::test_set_compression_instantiation ... ok
    test protocol::login::serverbound::tests::test_cookie_response_login_instantiation ... ok
    test protocol::login::serverbound::tests::test_encryption_response_instantiation ... ok
    test protocol::login::serverbound::tests::test_login_acknowledged_instantiation ...
     ok
    test protocol::login::serverbound::tests::test_login_plugin_response_instantiation ... ok
    test protocol::login::serverbound::tests::test_login_start_instantiation ... ok    
    test protocol::play::clientbound::tests::test_acknowledge_block_change ... ok      
    test protocol::play::clientbound::tests::test_add_resource_pack_play ... ok        
    test protocol::play::clientbound::tests::test_award_statistics ... ok
    test protocol::play::clientbound::tests::test_block_action ... ok
    test protocol::play::clientbound::tests::test_block_entity_data ... ok
    test protocol::play::clientbound::tests::test_block_update ... ok
    test protocol::play::clientbound::tests::test_boss_bar ... ok
    test protocol::play::clientbound::tests::test_bundle_delimiter ... ok
    test protocol::play::clientbound::tests::test_change_difficulty ... ok
    test protocol::play::clientbound::tests::test_chat_suggestions ... ok
    test protocol::play::clientbound::tests::test_chunk_batch_finished ... ok
    test protocol::play::clientbound::tests::test_chunk_batch_start ... ok
    test protocol::play::clientbound::tests::test_chunk_biomes ... ok
    test protocol::play::clientbound::tests::test_chunk_data_and_update_light ... ok   
    test protocol::play::clientbound::tests::test_clear_titles ... ok
    test protocol::play::clientbound::tests::test_clientbound_keep_alive_play ... ok   
    test protocol::play::clientbound::tests::test_clientbound_plugin_message_play ... ok
    test protocol::play::clientbound::tests::test_close_container ... ok
    test protocol::play::clientbound::tests::test_combat_death ... ok
    test protocol::play::clientbound::tests::test_command_suggestions_response ... ok  
    test protocol::play::clientbound::tests::test_commands ... ok
    test protocol::play::clientbound::tests::test_cookie_request_play ... ok
    test protocol::play::clientbound::tests::test_custom_report_details_play ... ok    
    test protocol::play::clientbound::tests::test_damage_event ... ok
    test protocol::play::clientbound::tests::test_debug_sample ... ok
    test protocol::play::clientbound::tests::test_delete_message ... ok
    test protocol::play::clientbound::tests::test_disconnect_play ... ok
    test protocol::play::clientbound::tests::test_disguised_chat_message ... ok        
    test protocol::play::clientbound::tests::test_display_objective ... ok
    test protocol::play::clientbound::tests::test_end_combat ... ok
    test protocol::play::clientbound::tests::test_enter_combat ... ok
    test protocol::play::clientbound::tests::test_entity_animation ... ok
    test protocol::play::clientbound::tests::test_entity_effect ... ok
    test protocol::play::clientbound::tests::test_entity_event ... ok
    test protocol::play::clientbound::tests::test_entity_sound_effect ... ok
    test protocol::play::clientbound::tests::test_explosion ... ok
    test protocol::play::clientbound::tests::test_game_event ... ok
    test protocol::play::clientbound::tests::test_initialize_world_border ... ok       
    test protocol::play::clientbound::tests::test_hurt_animation ... ok
    test protocol::play::clientbound::tests::test_link_entities ... ok
    test protocol::play::clientbound::tests::test_login_play ... ok
    test protocol::play::clientbound::tests::test_look_at ... ok
    test protocol::play::clientbound::tests::test_map_data ... ok
    test protocol::play::clientbound::tests::test_merchant_offers ... ok
    test protocol::play::clientbound::tests::test_move_minecart_along_track ... ok     
    test protocol::play::clientbound::tests::test_move_vehicle ... ok
    test protocol::play::clientbound::tests::test_open_book ... ok
    test protocol::play::clientbound::tests::test_open_horse_screen ... ok
    test protocol::play::clientbound::tests::test_open_screen ... ok
    test protocol::play::clientbound::tests::test_open_sign_editor ... ok
    test protocol::play::clientbound::tests::test_particle ... ok
    test protocol::play::clientbound::tests::test_pickup_item ... ok
    test protocol::play::clientbound::tests::test_ping_play ... ok
    test protocol::play::clientbound::tests::test_ping_response_play ... ok
    test protocol::play::clientbound::tests::test_place_ghost_recipe ... ok
    test protocol::play::clientbound::tests::test_player_abilities ... ok
    test protocol::play::clientbound::tests::test_player_chat_message ... ok
    test protocol::play::clientbound::tests::test_player_info_remove ... ok
    test protocol::play::clientbound::tests::test_player_info_update ... ok
    test protocol::play::clientbound::tests::test_player_rotation ... ok
    test protocol::play::clientbound::tests::test_projectile_power ... ok
    test protocol::play::clientbound::tests::test_recipe_book_add ... ok
    test protocol::play::clientbound::tests::test_recipe_book_remove ... ok
    test protocol::play::clientbound::tests::test_recipe_book_settings ... ok
    test protocol::play::clientbound::tests::test_remove_entities ... ok
    test protocol::play::clientbound::tests::test_remove_entity_effect ... ok
    test protocol::play::clientbound::tests::test_remove_resource_pack_play ... ok     
    test protocol::play::clientbound::tests::test_reset_score ... ok
    test protocol::play::clientbound::tests::test_respawn ... ok
    test protocol::play::clientbound::tests::test_select_advancements_tab ... ok       
    test protocol::play::clientbound::tests::test_server_data ... ok
    test protocol::play::clientbound::tests::test_server_links_play ... ok
    test protocol::play::clientbound::tests::test_set_action_bar_text ... ok
    test protocol::play::clientbound::tests::test_set_block_destroy_stage ... ok       
    test protocol::play::clientbound::tests::test_set_border_center ... ok
    test protocol::play::clientbound::tests::test_set_border_lerp_size ... ok
    test protocol::play::clientbound::tests::test_set_border_size ... ok
    test protocol::play::clientbound::tests::test_set_border_warning_delay ... ok      
    test protocol::play::clientbound::tests::test_set_border_warning_distance ... ok   
    test protocol::play::clientbound::tests::test_set_camera ... ok
    test protocol::play::clientbound::tests::test_set_center_chunk ... ok
    test protocol::play::clientbound::tests::test_set_container_content ... ok
    test protocol::play::clientbound::tests::test_set_container_property ... ok        
    test protocol::play::clientbound::tests::test_set_container_slot ... ok
    test protocol::play::clientbound::tests::test_set_cooldown ... ok
    test protocol::play::clientbound::tests::test_set_cursor_item ... ok
    test protocol::play::clientbound::tests::test_set_default_spawn_position ... ok    
    test protocol::play::clientbound::tests::test_set_entity_metadata ... ok
    test protocol::play::clientbound::tests::test_set_entity_velocity ... ok
    test protocol::play::clientbound::tests::test_set_equipment ... ok
    test protocol::play::clientbound::tests::test_set_experience ... ok
    test protocol::play::clientbound::tests::test_set_head_rotation ... ok
    test protocol::play::clientbound::tests::test_set_health ... ok
    test protocol::play::clientbound::tests::test_set_held_item ... ok
    test protocol::play::clientbound::tests::test_set_passengers ... ok
    test protocol::play::clientbound::tests::test_set_player_inventory_slot ... ok     
    test protocol::play::clientbound::tests::test_set_render_distance ... ok
    test protocol::play::clientbound::tests::test_set_simulation_distance ... ok       
    test protocol::play::clientbound::tests::test_set_subtitle_text ... ok
    test protocol::play::clientbound::tests::test_set_tab_list_header_and_footer ... ok
    test protocol::play::clientbound::tests::test_set_ticking_state ... ok
    test protocol::play::clientbound::tests::test_set_title_animation_times ... ok     
    test protocol::play::clientbound::tests::test_set_title_text ... ok
    test protocol::play::clientbound::tests::test_sound_effect ... ok
    test protocol::play::clientbound::tests::test_spawn_entity ... ok
    test protocol::play::clientbound::tests::test_start_configuration ... ok
    test protocol::play::clientbound::tests::test_step_tick ... ok
    test protocol::play::clientbound::tests::test_stop_sound ... ok
    test protocol::play::clientbound::tests::test_store_cookie_play ... ok
    test protocol::play::clientbound::tests::test_synchronize_player_position ... ok   
    test protocol::play::clientbound::tests::test_synchronize_vehicle_position ... ok  
    test protocol::play::clientbound::tests::test_system_chat_message ... ok
    test protocol::play::clientbound::tests::test_tag_query_response ... ok
    test protocol::play::clientbound::tests::test_teleport_entity_play ... ok
    test protocol::play::clientbound::tests::test_test_instance_block_status ... ok    
    test protocol::play::clientbound::tests::test_transfer_play ... ok
    test protocol::play::clientbound::tests::test_unload_chunk ... ok
    test protocol::play::clientbound::tests::test_update_advancements ... ok
    test protocol::play::clientbound::tests::test_update_attributes ... ok
    test protocol::play::clientbound::tests::test_update_entity_position ... ok        
    test protocol::play::clientbound::tests::test_update_entity_position_and_rotation ... ok
    test protocol::play::clientbound::tests::test_update_entity_rotation ... ok        
    test protocol::play::clientbound::tests::test_update_light ... ok
    test protocol::play::clientbound::tests::test_update_objectives ... ok
    test protocol::play::clientbound::tests::test_update_recipes ... ok
    test protocol::play::clientbound::tests::test_update_score ... ok
    test protocol::play::clientbound::tests::test_update_section_blocks ... ok
    test protocol::play::clientbound::tests::test_update_tags_play ... ok
    test protocol::play::clientbound::tests::test_update_teams ... ok
    test protocol::play::clientbound::tests::test_update_time ... ok
    test protocol::play::clientbound::tests::test_world_event ... ok
    test protocol::play::serverbound::tests::test_acknowledge_configuration ... ok     
    test protocol::play::serverbound::tests::test_acknowledge_message ... ok
    test protocol::play::serverbound::tests::test_bundle_item_selected ... ok
    test protocol::play::serverbound::tests::test_change_container_slot_state ... ok   
    test protocol::play::serverbound::tests::test_change_difficulty ... ok
    test protocol::play::serverbound::tests::test_change_recipe_book_settings ... ok   
    test protocol::play::serverbound::tests::test_chat_command ... ok
    test protocol::play::serverbound::tests::test_chat_message ... ok
    test protocol::play::serverbound::tests::test_chunk_batch_received ... ok
    test protocol::play::serverbound::tests::test_click_container ... ok
    test protocol::play::serverbound::tests::test_click_container_button ... ok        
    test protocol::play::serverbound::tests::test_client_information ... ok
    test protocol::play::serverbound::tests::test_client_status ... ok
    test protocol::play::serverbound::tests::test_client_tick_end ... ok
    test protocol::play::serverbound::tests::test_close_container ... ok
    test protocol::play::serverbound::tests::test_command_suggestions_request ... ok   
    test protocol::play::serverbound::tests::test_confirm_teleportation ... ok
    test protocol::play::serverbound::tests::test_cookie_response ... ok
    test protocol::play::serverbound::tests::test_debug_sample_subscription ... ok     
    test protocol::play::serverbound::tests::test_edit_book ... ok
    test protocol::play::serverbound::tests::test_interact ... ok
    test protocol::play::serverbound::tests::test_jigsaw_generate ... ok
    test protocol::play::serverbound::tests::test_lock_difficulty ... ok
    test protocol::play::serverbound::tests::test_move_vehicle ... ok
    test protocol::play::serverbound::tests::test_paddle_boat ... ok
    test protocol::play::serverbound::tests::test_pick_item_from_block ... ok
    test protocol::play::serverbound::tests::test_pick_item_from_entity ... ok
    test protocol::play::serverbound::tests::test_ping_request ... ok
    test protocol::play::serverbound::tests::test_place_recipe ... ok
    test protocol::play::serverbound::tests::test_player_action ... ok
    test protocol::play::serverbound::tests::test_player_command ... ok
    test protocol::play::serverbound::tests::test_player_input ... ok
    test protocol::play::serverbound::tests::test_player_loaded ... ok
    test protocol::play::serverbound::tests::test_player_session ... ok
    test protocol::play::serverbound::tests::test_pong ... ok
    test protocol::play::serverbound::tests::test_program_command_block ... ok
    test protocol::play::serverbound::tests::test_program_command_block_minecart ... ok
    test protocol::play::serverbound::tests::test_program_jigsaw_block ... ok
    test protocol::play::serverbound::tests::test_program_structure_block ... ok       
    test protocol::play::serverbound::tests::test_query_block_entity_tag ... ok        
    test protocol::play::serverbound::tests::test_query_entity_tag ... ok
    test protocol::play::serverbound::tests::test_rename_item ... ok
    test protocol::play::serverbound::tests::test_seen_advancements ... ok
    test protocol::play::serverbound::tests::test_select_trade ... ok
    test protocol::play::serverbound::tests::test_serverbound_keep_alive ... ok        
    test protocol::play::serverbound::tests::test_serverbound_player_abilities ... ok  
    test protocol::play::serverbound::tests::test_serverbound_plugin_message ... ok    
    test protocol::play::serverbound::tests::test_serverbound_resource_pack_response ... ok
    test protocol::play::serverbound::tests::test_set_beacon_effect ... ok
    test protocol::play::serverbound::tests::test_set_creative_mode_slot ... ok
    test protocol::play::serverbound::tests::test_set_held_item ... ok
    test protocol::play::serverbound::tests::test_set_player_movement_flags ... ok     
    test protocol::play::serverbound::tests::test_set_player_position ... ok
    test protocol::play::serverbound::tests::test_set_player_position_and_rotation ... ok
    test protocol::play::serverbound::tests::test_set_player_rotation ... ok
    test protocol::play::serverbound::tests::test_set_seen_recipe ... ok
    test protocol::play::serverbound::tests::test_set_test_block ... ok
    test protocol::play::serverbound::tests::test_signed_chat_command ... ok
    test protocol::play::serverbound::tests::test_swing_arm ... ok
    test protocol::play::serverbound::tests::test_teleport_to_entity ... ok
    test protocol::play::serverbound::tests::test_test_instance_block_action ... ok    
    test protocol::play::serverbound::tests::test_update_sign ... ok
    test protocol::play::serverbound::tests::test_use_item ... ok
    test protocol::play::serverbound::tests::test_use_item_on ... ok
    test protocol::status::clientbound::tests::test_pong_response_instantiation ... ok 
    test protocol::status::clientbound::tests::test_status_response_instantiation ... ok
    test protocol::status::serverbound::tests::test_ping_request_instantiation ... ok  
    test protocol::status::serverbound::tests::test_status_request_instantiation ... ok

test result: ok. 237 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
