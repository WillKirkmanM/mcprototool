use uuid::Uuid;

use crate::protocol::types::{
    Advancement, AdvancementFrameType, AdvancementProgress, Angle, AttributeProperty,
    BossBarAction, BossBarColor, BossBarDivision, BossBarFlags, ChatSuggestionAction, ChatTypeData,
    ChunkData, ChunkSectionBlockUpdate, CollisionRule, CustomReportDetail, Difficulty,
    EntityAnimation, EntityEffectFlags, EntityMetadata, EquipmentEntry, FilterType, FriendlyFlags,
    GameEventType, GameMode, Hand, IdOr, IdSet, Identifier, JsonTextComponent, LightData,
    LookAtAnchor, MapColorPatch, MapIcon, MerchantTrade, NameTagVisibility, Nbt, NumberFormat,
    ObjectiveMode, ObjectiveType, PlayerInfoEntry, Position, PreviousGameMode, RecipeBookEntry,
    RecipeDisplay, RespawnDataKeptFlags, ServerLink, Slot, SlotDisplay, SoundCategory, SoundEvent,
    Statistic, StopSoundFlags, SuggestionMatch, TeamColor, TeamMethod, TeleportFlags, VarInt,
    VarLong, VillagerLevel,
};
use std::collections::HashMap;

pub struct BundleDelimiter;

pub struct SpawnEntity {
    pub entity_id: VarInt,
    pub entity_uuid: Uuid,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    pub data: VarInt,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

pub struct EntityAnimationPacket {
    pub entity_id: VarInt,
    pub animation: u8,
}

pub struct AwardStatistics {
    pub statistics: Vec<Statistic>,
}

pub struct AcknowledgeBlockChange {
    pub sequence_id: VarInt,
}

pub struct SetBlockDestroyStage {
    pub entity_id: VarInt,
    pub location: Position,
    pub destroy_stage: u8,
}

pub struct BlockEntityData {
    pub location: Position,
    pub block_entity_type: VarInt,
    pub nbt_data: Nbt,
}

pub struct BlockAction {
    pub location: Position,
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: VarInt,
}

pub struct BlockUpdate {
    pub location: Position,
    pub block_id: VarInt,
}

pub enum BossBar {
    Add {
        uuid: Uuid,
        title: JsonTextComponent,
        health: f32,
        color: VarInt,
        division: VarInt,
        flags: u8,
    },
    Remove {
        uuid: Uuid,
    },
    UpdateHealth {
        uuid: Uuid,
        health: f32,
    },
    UpdateTitle {
        uuid: Uuid,
        title: JsonTextComponent,
    },
    UpdateStyle {
        uuid: Uuid,
        color: VarInt,
        division: VarInt,
    },
    UpdateFlags {
        uuid: Uuid,
        flags: u8,
    },
}

pub struct ChangeDifficulty {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}

pub struct ChunkBatchFinished {
    pub batch_size: VarInt,
}

pub struct ChunkBatchStart;

pub struct ChunkBiomes {
    pub chunk_biome_data: Vec<u8>,
}

pub struct ClearTitles {
    pub reset: bool,
}

pub struct CommandSuggestionsResponse {
    pub id: VarInt,
    pub start: VarInt,
    pub length: VarInt,
    pub matches: Vec<SuggestionMatch>,
}

pub struct Commands {
    pub nodes: Vec<u8>,
    pub root_index: VarInt,
}

pub struct CloseContainer {
    pub window_id: VarInt,
}

pub struct SetContainerContent {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot_data: Vec<Slot>,
    pub carried_item: Slot,
}

pub struct SetContainerProperty {
    pub window_id: VarInt,
    pub property: i16,
    pub value: i16,
}

pub struct SetContainerSlot {
    pub window_id: i8,
    pub state_id: VarInt,
    pub slot: i16,
    pub slot_data: Slot,
}

pub struct CookieRequestPlay {
    pub key: Identifier,
}

pub struct SetCooldown {
    pub item_id: VarInt,
    pub cooldown_ticks: VarInt,
}

pub struct ChatSuggestions {
    pub action: VarInt,
    pub entries: Vec<String>,
}

pub struct ClientboundPluginMessagePlay {
    pub channel: Identifier,
    pub data: Vec<u8>,
}

pub struct DamageEvent {
    pub entity_id: VarInt,
    pub source_type_id: VarInt,
    pub source_cause_id: VarInt,
    pub source_direct_id: VarInt,
    pub source_position: Option<(f64, f64, f64)>,
}

pub struct DebugSample {
    pub sample: Vec<i64>,
    pub sample_type: VarInt,
}

pub struct DeleteMessage {
    pub message_id: VarInt,
    pub signature: Option<Vec<u8>>,
}

pub struct DisconnectPlay {
    pub reason: JsonTextComponent,
}

pub struct DisguisedChatMessage {
    pub message: JsonTextComponent,
    pub chat_type: IdOr<ChatTypeData>,
    pub sender_name: JsonTextComponent,
    pub target_name: Option<JsonTextComponent>,
}

pub struct EntityEvent {
    pub entity_id: i32,
    pub entity_status: i8,
}

pub struct TeleportEntityPlay {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

pub struct Explosion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub player_velocity: Option<(f64, f32, f32)>,
    pub explosion_particle_id: VarInt,
    pub explosion_particle_data: Vec<u8>,
    pub explosion_sound: IdOr<SoundEvent>,
}

pub struct UnloadChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

pub struct GameEvent {
    pub event: u8,
    pub value: f32,
}

pub struct OpenHorseScreen {
    pub window_id: VarInt,
    pub inventory_columns_count: VarInt,
    pub entity_id: i32,
}

pub struct HurtAnimation {
    pub entity_id: VarInt,
    pub yaw: f32,
}

pub struct InitializeWorldBorder {
    pub x: f64,
    pub z: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    pub speed: VarLong,
    pub portal_teleport_boundary: VarInt,
    pub warning_blocks: VarInt,
    pub warning_time: VarInt,
}

pub struct ClientboundKeepAlivePlay {
    pub keep_alive_id: i64,
}

pub struct ChunkDataAndUpdateLight {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}

pub struct WorldEvent {
    pub event: i32,
    pub location: Position,
    pub data: i32,
    pub disable_relative_volume: bool,
}

pub struct Particle {
    pub long_distance: bool,
    pub always_visible: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub max_speed: f32,
    pub particle_count: i32,
    pub particle_id: VarInt,
    pub data: Vec<u8>,
}

pub struct UpdateLight {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
    pub light_data: LightData,
}

pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(Identifier, Position)>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

pub struct MapData {
    pub map_id: VarInt,
    pub scale: i8,
    pub locked: bool,
    pub icons: Option<Vec<MapIcon>>,
    pub color_patch: Option<MapColorPatch>,
}

pub struct MerchantOffers {
    pub window_id: VarInt,
    pub trades: Vec<MerchantTrade>,
    pub villager_level: VarInt,
    pub experience: VarInt,
    pub is_regular_villager: bool,
    pub can_restock: bool,
}

pub struct UpdateEntityPosition {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

pub struct UpdateEntityPositionAndRotation {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: bool,
}

pub struct MoveMinecartAlongTrack {
    pub entity_id: VarInt,
    pub steps: Vec<u8>,
    pub weight: f32,
}

pub struct UpdateEntityRotation {
    pub entity_id: VarInt,
    pub yaw: Angle,
    pub pitch: Angle,
    pub on_ground: bool,
}

pub struct MoveVehicle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

pub struct OpenBook {
    pub hand: VarInt,
}

pub struct OpenScreen {
    pub window_id: VarInt,
    pub window_type: VarInt,
    pub window_title: JsonTextComponent,
}

pub struct OpenSignEditor {
    pub location: Position,
    pub is_front_text: bool,
}

pub struct PingPlay {
    pub id: i32,
}

pub struct PingResponsePlay {
    pub payload: i64,
}

pub struct PlaceGhostRecipe {
    pub window_id: VarInt,
    pub recipe_display: RecipeDisplay,
}

pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub field_of_view_modifier: f32,
}

pub struct PlayerChatMessage {
    pub global_index: VarInt,
    pub sender: Uuid,
    pub index: VarInt,
    pub message_signature: Option<Vec<u8>>,
    pub message: String,
    pub timestamp: i64,
    pub salt: i64,
    pub previous_messages: Vec<u8>,
    pub unsigned_content: Option<JsonTextComponent>,
    pub filter_type: VarInt,
    pub filter_type_bits: Option<Vec<u8>>,
    pub chat_type: IdOr<ChatTypeData>,
    pub sender_name: JsonTextComponent,
    pub target_name: Option<JsonTextComponent>,
}

pub struct EndCombat {
    pub duration: VarInt,
}

pub struct EnterCombat;

pub struct CombatDeath {
    pub player_id: VarInt,
    pub message: JsonTextComponent,
}

pub struct PlayerInfoRemove {
    pub uuids: Vec<Uuid>,
}

pub struct PlayerInfoUpdate {
    pub actions: u8,
    pub players: Vec<PlayerInfoEntry>,
}

pub struct LookAt {
    pub feet_or_eyes: VarInt,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub entity_target: Option<(VarInt, VarInt)>,
}

pub struct SynchronizePlayerPosition {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
}

pub struct RecipeBookAdd {
    pub recipes: Vec<RecipeBookEntry>,
    pub replace: bool,
}

pub struct RecipeBookRemove {
    pub recipes: Vec<VarInt>,
}

pub struct RecipeBookSettings {
    pub crafting_open: bool,
    pub crafting_filter: bool,
    pub smelting_open: bool,
    pub smelting_filter: bool,
    pub blast_furnace_open: bool,
    pub blast_furnace_filter: bool,
    pub smoker_open: bool,
    pub smoker_filter: bool,
}

pub struct RemoveEntities {
    pub entity_ids: Vec<VarInt>,
}

pub struct RemoveEntityEffect {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
}

pub struct ResetScore {
    pub entity_name: String,
    pub objective_name: Option<String>,
}

pub struct RemoveResourcePackPlay {
    pub uuid: Option<Uuid>,
}

pub struct AddResourcePackPlay {
    pub uuid: Uuid,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub prompt_message: Option<JsonTextComponent>,
}

pub struct Respawn {
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(Identifier, Position)>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub data_kept: u8,
}

pub struct SetHeadRotation {
    pub entity_id: VarInt,
    pub head_yaw: Angle,
}

pub struct UpdateSectionBlocks {
    pub chunk_section_position: i64,
    pub blocks: Vec<ChunkSectionBlockUpdate>,
}

pub struct SelectAdvancementsTab {
    pub identifier: Option<Identifier>,
}

pub struct ServerData {
    pub motd: JsonTextComponent,
    pub icon: Option<Vec<u8>>,
}

pub struct SetActionBarText {
    pub action_bar_text: JsonTextComponent,
}

pub struct SetBorderCenter {
    pub x: f64,
    pub z: f64,
}

pub struct SetBorderLerpSize {
    pub old_diameter: f64,
    pub new_diameter: f64,
    pub speed: VarLong,
}

pub struct SetBorderSize {
    pub diameter: f64,
}

pub struct SetBorderWarningDelay {
    pub warning_time: VarInt,
}

pub struct SetBorderWarningDistance {
    pub warning_blocks: VarInt,
}

pub struct SetCamera {
    pub camera_id: VarInt,
}

pub struct SetCenterChunk {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
}

pub struct SetRenderDistance {
    pub view_distance: VarInt,
}

pub struct SetCursorItem {
    pub carried_item: Slot,
}

pub struct SetDefaultSpawnPosition {
    pub location: Position,
    pub angle: f32,
}

pub struct DisplayObjective {
    pub position: VarInt,
    pub score_name: String,
}

pub struct SetEntityMetadata {
    pub entity_id: VarInt,
    pub metadata: EntityMetadata,
}

pub struct LinkEntities {
    pub attached_entity_id: i32,
    pub holding_entity_id: i32,
}

pub struct SetEntityVelocity {
    pub entity_id: VarInt,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

pub struct SetEquipment {
    pub entity_id: VarInt,
    pub equipment: Vec<EquipmentEntry>,
}

pub struct SetExperience {
    pub experience_bar: f32,
    pub level: VarInt,
    pub total_experience: VarInt,
}

pub struct SetHealth {
    pub health: f32,
    pub food: VarInt,
    pub food_saturation: f32,
}

pub struct SetHeldItem {
    pub slot: VarInt,
}

pub struct UpdateObjectives {
    pub objective_name: String,
    pub mode: u8,
    pub objective_value: Option<JsonTextComponent>,
    pub objective_type: Option<VarInt>,
    pub number_format: Option<NumberFormat>,
}

pub struct SetPassengers {
    pub entity_id: VarInt,
    pub passengers: Vec<VarInt>,
}

pub struct SetPlayerInventorySlot {
    pub slot: VarInt,
    pub slot_data: Slot,
}

pub enum UpdateTeams {
    Create {
        team_name: String,
        display_name: JsonTextComponent,
        friendly_flags: u8,
        name_tag_visibility: String,
        collision_rule: String,
        team_color: VarInt,
        team_prefix: JsonTextComponent,
        team_suffix: JsonTextComponent,
        entities: Vec<String>,
    },
    Remove {
        team_name: String,
    },
    UpdateInfo {
        team_name: String,
        display_name: JsonTextComponent,
        friendly_flags: u8,
        name_tag_visibility: String,
        collision_rule: String,
        team_color: VarInt,
        team_prefix: JsonTextComponent,
        team_suffix: JsonTextComponent,
    },
    AddEntities {
        team_name: String,
        entities: Vec<String>,
    },
    RemoveEntities {
        team_name: String,
        entities: Vec<String>,
    },
}

pub struct UpdateScore {
    pub entity_name: String,
    pub objective_name: String,
    pub value: VarInt,
    pub display_name: Option<JsonTextComponent>,
    pub number_format: Option<NumberFormat>,
}

pub struct SetSimulationDistance {
    pub simulation_distance: VarInt,
}

pub struct SetSubtitleText {
    pub subtitle_text: JsonTextComponent,
}

pub struct UpdateTime {
    pub world_age: i64,
    pub time_of_day: i64,
    pub time_of_day_increasing: bool,
}

pub struct SetTitleText {
    pub title_text: JsonTextComponent,
}

pub struct SetTitleAnimationTimes {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

pub struct EntitySoundEffect {
    pub sound_event: IdOr<SoundEvent>,
    pub sound_category: VarInt,
    pub entity_id: VarInt,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

pub struct SoundEffect {
    pub sound_event: IdOr<SoundEvent>,
    pub sound_category: VarInt,
    pub effect_position_x: i32,
    pub effect_position_y: i32,
    pub effect_position_z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

pub struct StartConfiguration;

pub struct StopSound {
    pub flags: u8,
    pub source: Option<VarInt>,
    pub sound: Option<Identifier>,
}

pub struct StoreCookiePlay {
    pub key: Identifier,
    pub payload: Vec<u8>,
}

pub struct SystemChatMessage {
    pub content: JsonTextComponent,
    pub overlay: bool,
}

pub struct SetTabListHeaderAndFooter {
    pub header: JsonTextComponent,
    pub footer: JsonTextComponent,
}

pub struct TagQueryResponse {
    pub transaction_id: VarInt,
    pub nbt: Nbt,
}

pub struct PickupItem {
    pub collected_entity_id: VarInt,
    pub collector_entity_id: VarInt,
    pub pickup_item_count: VarInt,
}

pub struct SynchronizeVehiclePosition {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub on_ground: bool,
}

pub struct TestInstanceBlockStatus {
    pub status: JsonTextComponent,
    pub size: Option<(f64, f64, f64)>,
}

pub struct SetTickingState {
    pub tick_rate: f32,
    pub is_frozen: bool,
}

pub struct StepTick {
    pub tick_steps: VarInt,
}

pub struct TransferPlay {
    pub host: String,
    pub port: VarInt,
}

pub struct UpdateAdvancements {
    pub reset_clear: bool,
    pub advancement_mapping: HashMap<Identifier, Advancement>,
    pub identifiers_to_remove: Vec<Identifier>,
    pub progress_mapping: HashMap<Identifier, AdvancementProgress>,
    pub show_advancements: bool,
}

pub struct UpdateAttributes {
    pub entity_id: VarInt,
    pub properties: Vec<AttributeProperty>,
}

pub struct EntityEffect {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub amplifier: VarInt,
    pub duration: VarInt,
    pub flags: u8,
}

pub struct UpdateRecipes {
    pub property_sets: Vec<u8>,
    pub stonecutter_recipes: Vec<u8>,
}

pub struct UpdateTagsPlay {
    pub tags: HashMap<Identifier, Vec<u8>>,
}

pub struct ProjectilePower {
    pub entity_id: VarInt,
    pub power: f64,
}

pub struct CustomReportDetailsPlay {
    pub details: Vec<CustomReportDetail>,
}

pub struct ServerLinksPlay {
    pub links: Vec<ServerLink>,
}

#[cfg(test)]
mod tests {
    use crate::protocol::types::{
        BuiltInServerLinkLabel, EquipmentSlot, MapIconType, PlayerAbilityFlags,
        PlayerInfoActionData, PlayerInfoProperty, RecipeBookFlags, ServerLinkLabel,
        StatisticCategory, TradeItem,
    };

    use super::*;
    use std::collections::HashMap;

    fn dummy_text() -> JsonTextComponent {
        r#"{"text":""}"#.into()
    }
    fn dummy_id() -> Identifier {
        "minecraft:stone".into()
    }
    fn dummy_slot() -> Slot {
        vec![]
    }
    fn dummy_nbt() -> Nbt {
        vec![]
    }
    fn dummy_metadata() -> EntityMetadata {
        vec![]
    }
    fn dummy_chunk_data() -> ChunkData {
        vec![]
    }
    fn dummy_light_data() -> LightData {
        vec![]
    }
    fn dummy_recipe_display() -> RecipeDisplay {
        vec![]
    }
    fn dummy_slot_display() -> SlotDisplay {
        vec![]
    }
    fn dummy_id_set() -> IdSet {
        vec![]
    }
    fn dummy_chat_type_data() -> ChatTypeData {
        vec![]
    }
    fn dummy_sound_event() -> SoundEvent {
        vec![]
    }

    #[test]
    fn test_bundle_delimiter() {
        let _p = BundleDelimiter;
        assert!(true);
    }
    #[test]
    fn test_spawn_entity() {
        let _p = SpawnEntity {
            entity_id: VarInt::from(1),
            entity_uuid: Uuid::new_v4(),
            entity_type: VarInt::from(1),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pitch: 0,
            yaw: 0,
            head_yaw: 0,
            data: VarInt::from(0),
            velocity_x: 0,
            velocity_y: 0,
            velocity_z: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_entity_animation() {
        let _p = EntityAnimationPacket {
            entity_id: VarInt::from(1),
            animation: EntityAnimation::SwingMainArm as u8,
        };
        assert!(true);
    }
    #[test]
    fn test_award_statistics() {
        let _p = AwardStatistics {
            statistics: vec![Statistic {
                category_id: VarInt::from(StatisticCategory::Custom as i32),
                statistic_id: VarInt::from(0),
                value: VarInt::from(1),
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_acknowledge_block_change() {
        let _p = AcknowledgeBlockChange {
            sequence_id: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_set_block_destroy_stage() {
        let _p = SetBlockDestroyStage {
            entity_id: VarInt::from(1),
            location: Position { x: 0, y: 0, z: 0 },
            destroy_stage: 5,
        };
        assert!(true);
    }
    #[test]
    fn test_block_entity_data() {
        let _p = BlockEntityData {
            location: Position { x: 0, y: 0, z: 0 },
            block_entity_type: VarInt::from(1),
            nbt_data: dummy_nbt(),
        };
        assert!(true);
    }
    #[test]
    fn test_block_action() {
        let _p = BlockAction {
            location: Position { x: 0, y: 0, z: 0 },
            action_id: 1,
            action_param: 0,
            block_type: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_block_update() {
        let _p = BlockUpdate {
            location: Position { x: 0, y: 0, z: 0 },
            block_id: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_boss_bar() {
        let uuid = Uuid::new_v4();
        let _p = BossBar::Add {
            uuid,
            title: dummy_text(),
            health: 0.5,
            color: VarInt::from(BossBarColor::Purple as i32),
            division: VarInt::from(BossBarDivision::TenNotches as i32),
            flags: BossBarFlags::DARKEN_SKY.bits(),
        };
        let _p2 = BossBar::Remove { uuid };
        assert!(true);
    }
    #[test]
    fn test_change_difficulty() {
        let _p = ChangeDifficulty {
            difficulty: Difficulty::Hard as u8,
            difficulty_locked: true,
        };
        assert!(true);
    }
    #[test]
    fn test_chunk_batch_finished() {
        let _p = ChunkBatchFinished {
            batch_size: VarInt::from(10),
        };
        assert!(true);
    }
    #[test]
    fn test_chunk_batch_start() {
        let _p = ChunkBatchStart;
        assert!(true);
    }
    #[test]
    fn test_chunk_biomes() {
        let _p = ChunkBiomes {
            chunk_biome_data: vec![],
        };
        assert!(true);
    }
    #[test]
    fn test_clear_titles() {
        let _p = ClearTitles { reset: true };
        assert!(true);
    }
    #[test]
    fn test_command_suggestions_response() {
        let _p = CommandSuggestionsResponse {
            id: VarInt::from(1),
            start: VarInt::from(0),
            length: VarInt::from(5),
            matches: vec![SuggestionMatch {
                match_text: "test".into(),
                tooltip: None,
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_commands() {
        let _p = Commands {
            nodes: vec![],
            root_index: VarInt::from(0),
        };
        assert!(true);
    }
    #[test]
    fn test_close_container() {
        let _p = CloseContainer {
            window_id: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_set_container_content() {
        let _p = SetContainerContent {
            window_id: VarInt::from(1),
            state_id: VarInt::from(1),
            slot_data: vec![dummy_slot()],
            carried_item: dummy_slot(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_container_property() {
        let _p = SetContainerProperty {
            window_id: VarInt::from(1),
            property: 0,
            value: 100,
        };
        assert!(true);
    }
    #[test]
    fn test_set_container_slot() {
        let _p = SetContainerSlot {
            window_id: 1,
            state_id: VarInt::from(1),
            slot: 0,
            slot_data: dummy_slot(),
        };
        assert!(true);
    }
    #[test]
    fn test_cookie_request_play() {
        let _p = CookieRequestPlay { key: dummy_id() };
        assert!(true);
    }
    #[test]
    fn test_set_cooldown() {
        let _p = SetCooldown {
            item_id: VarInt::from(1),
            cooldown_ticks: VarInt::from(20),
        };
        assert!(true);
    }
    #[test]
    fn test_chat_suggestions() {
        let _p = ChatSuggestions {
            action: VarInt::from(ChatSuggestionAction::Add as i32),
            entries: vec!["hello".into()],
        };
        assert!(true);
    }
    #[test]
    fn test_clientbound_plugin_message_play() {
        let _p = ClientboundPluginMessagePlay {
            channel: dummy_id(),
            data: vec![1, 2, 3],
        };
        assert!(true);
    }
    #[test]
    fn test_damage_event() {
        let _p = DamageEvent {
            entity_id: VarInt::from(1),
            source_type_id: VarInt::from(0),
            source_cause_id: VarInt::from(0),
            source_direct_id: VarInt::from(0),
            source_position: Some((1.0, 2.0, 3.0)),
        };
        assert!(true);
    }
    #[test]
    fn test_debug_sample() {
        let _p = DebugSample {
            sample: vec![100, 200],
            sample_type: VarInt::from(0),
        };
        assert!(true);
    }
    #[test]
    fn test_delete_message() {
        let _p = DeleteMessage {
            message_id: VarInt::from(1),
            signature: None,
        };
        assert!(true);
    }
    #[test]
    fn test_disconnect_play() {
        let _p = DisconnectPlay {
            reason: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_disguised_chat_message() {
        let _p = DisguisedChatMessage {
            message: dummy_text(),
            chat_type: IdOr::Id(VarInt::from(0)),
            sender_name: dummy_text(),
            target_name: None,
        };
        assert!(true);
    }
    #[test]
    fn test_entity_event() {
        let _p = EntityEvent {
            entity_id: 1,
            entity_status: 2,
        };
        assert!(true);
    }
    #[test]
    fn test_teleport_entity_play() {
        let _p = TeleportEntityPlay {
            entity_id: VarInt::from(1),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,
        };
        assert!(true);
    }
    #[test]
    fn test_explosion() {
        let _p = Explosion {
            x: 0.0,
            y: 64.0,
            z: 0.0,
            player_velocity: None,
            explosion_particle_id: VarInt::from(0),
            explosion_particle_data: vec![],
            explosion_sound: IdOr::Id(VarInt::from(0)),
        };
        assert!(true);
    }
    #[test]
    fn test_unload_chunk() {
        let _p = UnloadChunk {
            chunk_x: 0,
            chunk_z: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_game_event() {
        let _p = GameEvent {
            event: GameEventType::BeginRaining as u8,
            value: 0.0,
        };
        assert!(true);
    }
    #[test]
    fn test_open_horse_screen() {
        let _p = OpenHorseScreen {
            window_id: VarInt::from(1),
            inventory_columns_count: VarInt::from(2),
            entity_id: 1,
        };
        assert!(true);
    }
    #[test]
    fn test_hurt_animation() {
        let _p = HurtAnimation {
            entity_id: VarInt::from(1),
            yaw: 90.0,
        };
        assert!(true);
    }
    #[test]
    fn test_initialize_world_border() {
        let _p = InitializeWorldBorder {
            x: 0.0,
            z: 0.0,
            old_diameter: 60000.0,
            new_diameter: 60000.0,
            speed: VarLong::from(0),
            portal_teleport_boundary: VarInt::from(29999984),
            warning_blocks: VarInt::from(5),
            warning_time: VarInt::from(15),
        };
        assert!(true);
    }
    #[test]
    fn test_clientbound_keep_alive_play() {
        let _p = ClientboundKeepAlivePlay {
            keep_alive_id: 12345,
        };
        assert!(true);
    }
    #[test]
    fn test_chunk_data_and_update_light() {
        let _p = ChunkDataAndUpdateLight {
            chunk_x: 0,
            chunk_z: 0,
            chunk_data: dummy_chunk_data(),
            light_data: dummy_light_data(),
        };
        assert!(true);
    }
    #[test]
    fn test_world_event() {
        let _p = WorldEvent {
            event: 1000,
            location: Position { x: 0, y: 0, z: 0 },
            data: 0,
            disable_relative_volume: false,
        };
        assert!(true);
    }
    #[test]
    fn test_particle() {
        let _p = Particle {
            long_distance: false,
            always_visible: true,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            offset_x: 0.1,
            offset_y: 0.1,
            offset_z: 0.1,
            max_speed: 0.0,
            particle_count: 10,
            particle_id: VarInt::from(0),
            data: vec![],
        };
        assert!(true);
    }
    #[test]
    fn test_update_light() {
        let _p = UpdateLight {
            chunk_x: VarInt::from(0),
            chunk_z: VarInt::from(0),
            light_data: dummy_light_data(),
        };
        assert!(true);
    }
    #[test]
    fn test_login_play() {
        let _p = LoginPlay {
            entity_id: 1,
            is_hardcore: false,
            dimension_names: vec![dummy_id()],
            max_players: VarInt::from(20),
            view_distance: VarInt::from(10),
            simulation_distance: VarInt::from(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::from(0),
            dimension_name: dummy_id(),
            hashed_seed: 0,
            game_mode: GameMode::Survival as u8,
            previous_game_mode: PreviousGameMode::Undefined as i8,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: VarInt::from(0),
            sea_level: VarInt::from(63),
            enforces_secure_chat: false,
        };
        assert!(true);
    }
    #[test]
    fn test_map_data() {
        let _p = MapData {
            map_id: VarInt::from(0),
            scale: 1,
            locked: false,
            icons: Some(vec![MapIcon {
                icon_type: VarInt::from(MapIconType::Player as i32),
                x: 0,
                z: 0,
                direction: 0,
                display_name: None,
            }]),
            color_patch: Some(MapColorPatch {
                columns: 1,
                rows: 1,
                x: 0,
                z: 0,
                data: vec![0],
            }),
        };
        assert!(true);
    }
    #[test]
    fn test_merchant_offers() {
        let _p = MerchantOffers {
            window_id: VarInt::from(1),
            trades: vec![MerchantTrade {
                input_item1: TradeItem {
                    item_id: VarInt::from(1),
                    item_count: VarInt::from(1),
                    components: vec![],
                },
                output_item: dummy_slot(),
                input_item2: None,
                trade_disabled: false,
                num_trade_uses: 0,
                max_trade_uses: 10,
                xp: 1,
                special_price: 0,
                price_multiplier: 0.05,
                demand: 0,
            }],
            villager_level: VarInt::from(VillagerLevel::Novice as i32),
            experience: VarInt::from(0),
            is_regular_villager: true,
            can_restock: true,
        };
        assert!(true);
    }
    #[test]
    fn test_update_entity_position() {
        let _p = UpdateEntityPosition {
            entity_id: VarInt::from(1),
            delta_x: 0,
            delta_y: 10,
            delta_z: 0,
            on_ground: true,
        };
        assert!(true);
    }
    #[test]
    fn test_update_entity_position_and_rotation() {
        let _p = UpdateEntityPositionAndRotation {
            entity_id: VarInt::from(1),
            delta_x: 0,
            delta_y: 10,
            delta_z: 0,
            yaw: 0,
            pitch: 0,
            on_ground: true,
        };
        assert!(true);
    }
    #[test]
    fn test_move_minecart_along_track() {
        let _p = MoveMinecartAlongTrack {
            entity_id: VarInt::from(1),
            steps: vec![],
            weight: 1.0,
        };
        assert!(true);
    }
    #[test]
    fn test_update_entity_rotation() {
        let _p = UpdateEntityRotation {
            entity_id: VarInt::from(1),
            yaw: 0,
            pitch: 0,
            on_ground: true,
        };
        assert!(true);
    }
    #[test]
    fn test_move_vehicle() {
        let _p = MoveVehicle {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        };
        assert!(true);
    }
    #[test]
    fn test_open_book() {
        let _p = OpenBook {
            hand: VarInt::from(Hand::MainHand as i32),
        };
        assert!(true);
    }
    #[test]
    fn test_open_screen() {
        let _p = OpenScreen {
            window_id: VarInt::from(1),
            window_type: VarInt::from(0),
            window_title: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_open_sign_editor() {
        let _p = OpenSignEditor {
            location: Position { x: 0, y: 0, z: 0 },
            is_front_text: true,
        };
        assert!(true);
    }
    #[test]
    fn test_ping_play() {
        let _p = PingPlay { id: 123 };
        assert!(true);
    }
    #[test]
    fn test_ping_response_play() {
        let _p = PingResponsePlay { payload: 123 };
        assert!(true);
    }
    #[test]
    fn test_place_ghost_recipe() {
        let _p = PlaceGhostRecipe {
            window_id: VarInt::from(1),
            recipe_display: dummy_recipe_display(),
        };
        assert!(true);
    }
    #[test]
    fn test_player_abilities() {
        let _p = PlayerAbilities {
            flags: PlayerAbilityFlags::ALLOW_FLYING.bits(),
            flying_speed: 0.05,
            field_of_view_modifier: 0.1,
        };
        assert!(true);
    }
    #[test]
    fn test_player_chat_message() {
        let _p = PlayerChatMessage {
            global_index: VarInt::from(0),
            sender: Uuid::new_v4(),
            index: VarInt::from(0),
            message_signature: None,
            message: "hello".into(),
            timestamp: 0,
            salt: 0,
            previous_messages: vec![],
            unsigned_content: None,
            filter_type: VarInt::from(FilterType::PassThrough as i32),
            filter_type_bits: None,
            chat_type: IdOr::Id(VarInt::from(0)),
            sender_name: dummy_text(),
            target_name: None,
        };
        assert!(true);
    }
    #[test]
    fn test_end_combat() {
        let _p = EndCombat {
            duration: VarInt::from(100),
        };
        assert!(true);
    }
    #[test]
    fn test_enter_combat() {
        let _p = EnterCombat;
        assert!(true);
    }
    #[test]
    fn test_combat_death() {
        let _p = CombatDeath {
            player_id: VarInt::from(1),
            message: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_player_info_remove() {
        let _p = PlayerInfoRemove {
            uuids: vec![Uuid::new_v4()],
        };
        assert!(true);
    }
    #[test]
    fn test_player_info_update() {
        let _p = PlayerInfoUpdate {
            actions: 0x01 | 0x04,
            players: vec![PlayerInfoEntry {
                uuid: Uuid::new_v4(),
                actions: vec![
                    PlayerInfoActionData::AddPlayer {
                        name: "Player".into(),
                        properties: vec![],
                    },
                    PlayerInfoActionData::UpdateGameMode {
                        game_mode: VarInt::from(GameMode::Survival as i32),
                    },
                ],
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_look_at() {
        let _p = LookAt {
            feet_or_eyes: VarInt::from(LookAtAnchor::Eyes as i32),
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
            entity_target: None,
        };
        assert!(true);
    }
    #[test]
    fn test_synchronize_player_position() {
        let _p = SynchronizePlayerPosition {
            teleport_id: VarInt::from(1),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_player_rotation() {
        let _p = PlayerRotation {
            yaw: 0.0,
            pitch: 0.0,
        };
        assert!(true);
    }
    #[test]
    fn test_recipe_book_add() {
        let _p = RecipeBookAdd {
            recipes: vec![RecipeBookEntry {
                recipe_id: VarInt::from(0),
                display: dummy_recipe_display(),
                group_id: VarInt::from(0),
                category_id: VarInt::from(0),
                ingredients: None,
                flags: RecipeBookFlags::SHOW_NOTIFICATION.bits(),
            }],
            replace: false,
        };
        assert!(true);
    }
    #[test]
    fn test_recipe_book_remove() {
        let _p = RecipeBookRemove {
            recipes: vec![VarInt::from(0)],
        };
        assert!(true);
    }
    #[test]
    fn test_recipe_book_settings() {
        let _p = RecipeBookSettings {
            crafting_open: true,
            crafting_filter: false,
            smelting_open: false,
            smelting_filter: false,
            blast_furnace_open: false,
            blast_furnace_filter: false,
            smoker_open: false,
            smoker_filter: false,
        };
        assert!(true);
    }
    #[test]
    fn test_remove_entities() {
        let _p = RemoveEntities {
            entity_ids: vec![VarInt::from(1), VarInt::from(2)],
        };
        assert!(true);
    }
    #[test]
    fn test_remove_entity_effect() {
        let _p = RemoveEntityEffect {
            entity_id: VarInt::from(1),
            effect_id: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_reset_score() {
        let _p = ResetScore {
            entity_name: "Player".into(),
            objective_name: Some("dummy".into()),
        };
        assert!(true);
    }
    #[test]
    fn test_remove_resource_pack_play() {
        let _p = RemoveResourcePackPlay {
            uuid: Some(Uuid::new_v4()),
        };
        assert!(true);
    }
    #[test]
    fn test_add_resource_pack_play() {
        let _p = AddResourcePackPlay {
            uuid: Uuid::new_v4(),
            url: "".into(),
            hash: "".into(),
            forced: false,
            prompt_message: None,
        };
        assert!(true);
    }
    #[test]
    fn test_respawn() {
        let _p = Respawn {
            dimension_type: VarInt::from(0),
            dimension_name: dummy_id(),
            hashed_seed: 0,
            game_mode: GameMode::Survival as u8,
            previous_game_mode: PreviousGameMode::Undefined as i8,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: VarInt::from(0),
            sea_level: VarInt::from(63),
            data_kept: RespawnDataKeptFlags::KEEP_ATTRIBUTES.bits(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_head_rotation() {
        let _p = SetHeadRotation {
            entity_id: VarInt::from(1),
            head_yaw: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_update_section_blocks() {
        let _p = UpdateSectionBlocks {
            chunk_section_position: 0,
            blocks: vec![ChunkSectionBlockUpdate {
                block_state_id_and_pos: 0,
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_select_advancements_tab() {
        let _p = SelectAdvancementsTab {
            identifier: Some(dummy_id()),
        };
        assert!(true);
    }
    #[test]
    fn test_server_data() {
        let _p = ServerData {
            motd: dummy_text(),
            icon: None,
        };
        assert!(true);
    }
    #[test]
    fn test_set_action_bar_text() {
        let _p = SetActionBarText {
            action_bar_text: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_border_center() {
        let _p = SetBorderCenter { x: 0.0, z: 0.0 };
        assert!(true);
    }
    #[test]
    fn test_set_border_lerp_size() {
        let _p = SetBorderLerpSize {
            old_diameter: 60000.0,
            new_diameter: 50000.0,
            speed: VarLong::from(10000),
        };
        assert!(true);
    }
    #[test]
    fn test_set_border_size() {
        let _p = SetBorderSize { diameter: 60000.0 };
        assert!(true);
    }
    #[test]
    fn test_set_border_warning_delay() {
        let _p = SetBorderWarningDelay {
            warning_time: VarInt::from(15),
        };
        assert!(true);
    }
    #[test]
    fn test_set_border_warning_distance() {
        let _p = SetBorderWarningDistance {
            warning_blocks: VarInt::from(5),
        };
        assert!(true);
    }
    #[test]
    fn test_set_camera() {
        let _p = SetCamera {
            camera_id: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_set_center_chunk() {
        let _p = SetCenterChunk {
            chunk_x: VarInt::from(0),
            chunk_z: VarInt::from(0),
        };
        assert!(true);
    }
    #[test]
    fn test_set_render_distance() {
        let _p = SetRenderDistance {
            view_distance: VarInt::from(10),
        };
        assert!(true);
    }
    #[test]
    fn test_set_cursor_item() {
        let _p = SetCursorItem {
            carried_item: dummy_slot(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_default_spawn_position() {
        let _p = SetDefaultSpawnPosition {
            location: Position { x: 0, y: 64, z: 0 },
            angle: 0.0,
        };
        assert!(true);
    }
    #[test]
    fn test_display_objective() {
        let _p = DisplayObjective {
            position: VarInt::from(1),
            score_name: "sidebar".into(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_entity_metadata() {
        let _p = SetEntityMetadata {
            entity_id: VarInt::from(1),
            metadata: dummy_metadata(),
        };
        assert!(true);
    }
    #[test]
    fn test_link_entities() {
        let _p = LinkEntities {
            attached_entity_id: 1,
            holding_entity_id: 2,
        };
        assert!(true);
    }
    #[test]
    fn test_set_entity_velocity() {
        let _p = SetEntityVelocity {
            entity_id: VarInt::from(1),
            velocity_x: 0,
            velocity_y: 100,
            velocity_z: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_set_equipment() {
        let _p = SetEquipment {
            entity_id: VarInt::from(1),
            equipment: vec![EquipmentEntry {
                slot: EquipmentSlot::MainHand as u8,
                item: dummy_slot(),
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_set_experience() {
        let _p = SetExperience {
            experience_bar: 0.5,
            level: VarInt::from(10),
            total_experience: VarInt::from(100),
        };
        assert!(true);
    }
    #[test]
    fn test_set_health() {
        let _p = SetHealth {
            health: 20.0,
            food: VarInt::from(20),
            food_saturation: 5.0,
        };
        assert!(true);
    }
    #[test]
    fn test_set_held_item() {
        let _p = SetHeldItem {
            slot: VarInt::from(0),
        };
        assert!(true);
    }
    #[test]
    fn test_update_objectives() {
        let _p = UpdateObjectives {
            objective_name: "dummy".into(),
            mode: ObjectiveMode::Create as u8,
            objective_value: Some(dummy_text()),
            objective_type: Some(VarInt::from(ObjectiveType::Integer as i32)),
            number_format: None,
        };
        assert!(true);
    }
    #[test]
    fn test_set_passengers() {
        let _p = SetPassengers {
            entity_id: VarInt::from(1),
            passengers: vec![VarInt::from(2), VarInt::from(3)],
        };
        assert!(true);
    }
    #[test]
    fn test_set_player_inventory_slot() {
        let _p = SetPlayerInventorySlot {
            slot: VarInt::from(9),
            slot_data: dummy_slot(),
        };
        assert!(true);
    }
    #[test]
    fn test_update_teams() {
        let _p = UpdateTeams::Create {
            team_name: "blue".into(),
            display_name: dummy_text(),
            friendly_flags: FriendlyFlags::ALLOW_FRIENDLY_FIRE.bits(),
            name_tag_visibility: "always".into(),
            collision_rule: "always".into(),
            team_color: VarInt::from(TeamColor::Blue as i32),
            team_prefix: dummy_text(),
            team_suffix: dummy_text(),
            entities: vec!["Player".into()],
        };
        assert!(true);
    }
    #[test]
    fn test_update_score() {
        let _p = UpdateScore {
            entity_name: "Player".into(),
            objective_name: "dummy".into(),
            value: VarInt::from(10),
            display_name: None,
            number_format: None,
        };
        assert!(true);
    }
    #[test]
    fn test_set_simulation_distance() {
        let _p = SetSimulationDistance {
            simulation_distance: VarInt::from(10),
        };
        assert!(true);
    }
    #[test]
    fn test_set_subtitle_text() {
        let _p = SetSubtitleText {
            subtitle_text: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_update_time() {
        let _p = UpdateTime {
            world_age: 1000,
            time_of_day: 6000,
            time_of_day_increasing: true,
        };
        assert!(true);
    }
    #[test]
    fn test_set_title_text() {
        let _p = SetTitleText {
            title_text: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_set_title_animation_times() {
        let _p = SetTitleAnimationTimes {
            fade_in: 10,
            stay: 70,
            fade_out: 20,
        };
        assert!(true);
    }
    #[test]
    fn test_entity_sound_effect() {
        let _p = EntitySoundEffect {
            sound_event: IdOr::Id(VarInt::from(0)),
            sound_category: VarInt::from(SoundCategory::Player as i32),
            entity_id: VarInt::from(1),
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_sound_effect() {
        let _p = SoundEffect {
            sound_event: IdOr::Id(VarInt::from(0)),
            sound_category: VarInt::from(SoundCategory::Block as i32),
            effect_position_x: 0,
            effect_position_y: 0,
            effect_position_z: 0,
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        };
        assert!(true);
    }
    #[test]
    fn test_start_configuration() {
        let _p = StartConfiguration;
        assert!(true);
    }
    #[test]
    fn test_stop_sound() {
        let _p = StopSound {
            flags: StopSoundFlags::HAS_SOURCE.bits(),
            source: Some(VarInt::from(SoundCategory::Master as i32)),
            sound: None,
        };
        assert!(true);
    }
    #[test]
    fn test_store_cookie_play() {
        let _p = StoreCookiePlay {
            key: dummy_id(),
            payload: vec![1, 2],
        };
        assert!(true);
    }
    #[test]
    fn test_system_chat_message() {
        let _p = SystemChatMessage {
            content: dummy_text(),
            overlay: false,
        };
        assert!(true);
    }
    #[test]
    fn test_set_tab_list_header_and_footer() {
        let _p = SetTabListHeaderAndFooter {
            header: dummy_text(),
            footer: dummy_text(),
        };
        assert!(true);
    }
    #[test]
    fn test_tag_query_response() {
        let _p = TagQueryResponse {
            transaction_id: VarInt::from(1),
            nbt: dummy_nbt(),
        };
        assert!(true);
    }
    #[test]
    fn test_pickup_item() {
        let _p = PickupItem {
            collected_entity_id: VarInt::from(1),
            collector_entity_id: VarInt::from(2),
            pickup_item_count: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_synchronize_vehicle_position() {
        let _p = SynchronizeVehiclePosition {
            entity_id: VarInt::from(1),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
            on_ground: true,
        };
        assert!(true);
    }
    #[test]
    fn test_test_instance_block_status() {
        let _p = TestInstanceBlockStatus {
            status: dummy_text(),
            size: None,
        };
        assert!(true);
    }
    #[test]
    fn test_set_ticking_state() {
        let _p = SetTickingState {
            tick_rate: 20.0,
            is_frozen: false,
        };
        assert!(true);
    }
    #[test]
    fn test_step_tick() {
        let _p = StepTick {
            tick_steps: VarInt::from(1),
        };
        assert!(true);
    }
    #[test]
    fn test_transfer_play() {
        let _p = TransferPlay {
            host: "server.com".into(),
            port: VarInt::from(25565),
        };
        assert!(true);
    }
    #[test]
    fn test_update_advancements() {
        let _p = UpdateAdvancements {
            reset_clear: false,
            advancement_mapping: HashMap::new(),
            identifiers_to_remove: vec![],
            progress_mapping: HashMap::new(),
            show_advancements: true,
        };
        assert!(true);
    }
    #[test]
    fn test_update_attributes() {
        let _p = UpdateAttributes {
            entity_id: VarInt::from(1),
            properties: vec![AttributeProperty {
                id: VarInt::from(0),
                value: 20.0,
                modifiers: vec![],
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_entity_effect() {
        let _p = EntityEffect {
            entity_id: VarInt::from(1),
            effect_id: VarInt::from(1),
            amplifier: VarInt::from(0),
            duration: VarInt::from(100),
            flags: EntityEffectFlags::SHOW_PARTICLES.bits() | EntityEffectFlags::SHOW_ICON.bits(),
        };
        assert!(true);
    }
    #[test]
    fn test_update_recipes() {
        let _p = UpdateRecipes {
            property_sets: vec![],
            stonecutter_recipes: vec![],
        };
        assert!(true);
    }
    #[test]
    fn test_update_tags_play() {
        let _p = UpdateTagsPlay {
            tags: HashMap::new(),
        };
        assert!(true);
    }
    #[test]
    fn test_projectile_power() {
        let _p = ProjectilePower {
            entity_id: VarInt::from(1),
            power: 1.0,
        };
        assert!(true);
    }
    #[test]
    fn test_custom_report_details_play() {
        let _p = CustomReportDetailsPlay {
            details: vec![CustomReportDetail {
                title: "info".into(),
                description: "data".into(),
            }],
        };
        assert!(true);
    }
    #[test]
    fn test_server_links_play() {
        let _p = ServerLinksPlay {
            links: vec![ServerLink {
                label: ServerLinkLabel::BuiltIn(BuiltInServerLinkLabel::Website),
                url: "url".into(),
            }],
        };
        assert!(true);
    }
}
