use uuid::Uuid;

pub type Identifier = String;
pub type JsonTextComponent = String;

pub type Nbt = Vec<u8>;
pub type Slot = Vec<u8>;
pub type EntityMetadata = Vec<u8>;
pub type ChunkData = Vec<u8>;
pub type LightData = Vec<u8>;
pub type RecipeDisplay = Vec<u8>;
pub type SlotDisplay = Vec<u8>;
pub type IdSet = Vec<u8>;
pub type ChatTypeData = Vec<u8>;
pub type SoundEvent = Vec<u8>;

pub type Angle = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VarInt(pub i32);

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VarLong(pub i64);

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong(value)
    }
}

impl TryFrom<VarInt> for NextState {
    type Error = ();

    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(1) => Ok(NextState::Status),
            VarInt(2) => Ok(NextState::Login),
            VarInt(3) => Ok(NextState::Transfer),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

impl TryFrom<VarInt> for ChatMode {
    type Error = ();
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(ChatMode::Enabled),
            VarInt(1) => Ok(ChatMode::CommandsOnly),
            VarInt(2) => Ok(ChatMode::Hidden),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainHand {
    Left = 0,
    Right = 1,
}

impl TryFrom<VarInt> for MainHand {
    type Error = ();
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(MainHand::Left),
            VarInt(1) => Ok(MainHand::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticleStatus {
    All = 0,
    Decreased = 1,
    Minimal = 2,
}

impl TryFrom<VarInt> for ParticleStatus {
    type Error = ();
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(ParticleStatus::All),
            VarInt(1) => Ok(ParticleStatus::Decreased),
            VarInt(2) => Ok(ParticleStatus::Minimal),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcePackResult {
    SuccessfullyDownloaded = 0,
    Declined = 1,
    FailedToDownload = 2,
    Accepted = 3,
    Downloaded = 4,
    InvalidUrl = 5,
    FailedToReload = 6,
    Discarded = 7,
}

impl TryFrom<VarInt> for ResourcePackResult {
    type Error = ();
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(ResourcePackResult::SuccessfullyDownloaded),
            VarInt(1) => Ok(ResourcePackResult::Declined),
            VarInt(2) => Ok(ResourcePackResult::FailedToDownload),
            VarInt(3) => Ok(ResourcePackResult::Accepted),
            VarInt(4) => Ok(ResourcePackResult::Downloaded),
            VarInt(5) => Ok(ResourcePackResult::InvalidUrl),
            VarInt(6) => Ok(ResourcePackResult::FailedToReload),
            VarInt(7) => Ok(ResourcePackResult::Discarded),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInServerLinkLabel {
    BugReport = 0,
    CommunityGuidelines = 1,
    Support = 2,
    Status = 3,
    Feedback = 4,
    Community = 5,
    Website = 6,
    Forums = 7,
    News = 8,
    Announcements = 9,
}

impl TryFrom<VarInt> for BuiltInServerLinkLabel {
    type Error = ();
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(BuiltInServerLinkLabel::BugReport),
            VarInt(1) => Ok(BuiltInServerLinkLabel::CommunityGuidelines),
            VarInt(2) => Ok(BuiltInServerLinkLabel::Support),
            VarInt(3) => Ok(BuiltInServerLinkLabel::Status),
            VarInt(4) => Ok(BuiltInServerLinkLabel::Feedback),
            VarInt(5) => Ok(BuiltInServerLinkLabel::Community),
            VarInt(6) => Ok(BuiltInServerLinkLabel::Website),
            VarInt(7) => Ok(BuiltInServerLinkLabel::Forums),
            VarInt(8) => Ok(BuiltInServerLinkLabel::News),
            VarInt(9) => Ok(BuiltInServerLinkLabel::Announcements),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerLinkLabel {
    BuiltIn(BuiltInServerLinkLabel),
    Custom(JsonTextComponent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryEntry {
    pub entry_id: Identifier,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub tag_name: Identifier,
    pub entries: Vec<VarInt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistryTagData {
    pub registry_id: Identifier,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomReportDetail {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerLink {
    pub label: ServerLinkLabel,
    pub url: String,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DisplayedSkinParts: u8 {
        const CAPE        = 0x01;
        const JACKET      = 0x02;
        const LEFT_SLEEVE = 0x04;
        const RIGHT_SLEEVE= 0x08;
        const LEFT_PANTS  = 0x10;
        const RIGHT_PANTS = 0x20;
        const HAT         = 0x40;
        // Bit 7 (0x80) is unused
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityAnimation {
    SwingMainArm = 0,
    LeaveBed = 2,
    SwingOffhand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticCategory {
    Mined = 0,
    Crafted = 1,
    Used = 2,
    Broken = 3,
    PickedUp = 4,
    Dropped = 5,
    Killed = 6,
    KilledBy = 7,
    Custom = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossBarAction {
    Add = 0,
    Remove = 1,
    UpdateHealth = 2,
    UpdateTitle = 3,
    UpdateStyle = 4,
    UpdateFlags = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossBarDivision {
    NoDivision = 0,
    SixNotches = 1,
    TenNotches = 2,
    TwelveNotches = 3,
    TwentyNotches = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSuggestionAction {
    Add = 0,
    Remove = 1,
    Set = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    PassThrough = 0,
    FullyFiltered = 1,
    PartiallyFiltered = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEventType {
    NoRespawnBlock = 0,
    BeginRaining = 1,
    EndRaining = 2,
    ChangeGameMode = 3,
    WinGame = 4,
    DemoEvent = 5,
    ArrowHitPlayer = 6,
    RainLevelChange = 7,
    ThunderLevelChange = 8,
    PlayPufferfishSting = 9,
    PlayElderGuardianMobAppearance = 10,
    EnableRespawnScreen = 11,
    LimitedCrafting = 12,
    StartWaitingForLevelChunks = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    MainHand = 0,
    OffHand = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapIconType {
    Player = 0,
    Frame = 1,
    RedMarker = 2,
    BlueMarker = 3,
    TargetX = 4,
    TargetPoint = 5,
    PlayerOffMap = 6,
    PlayerOffLimits = 7,
    Mansion = 8,
    Monument = 9,
    BannerWhite = 10,
    BannerOrange = 11,
    BannerMagenta = 12,
    BannerLightBlue = 13,
    BannerYellow = 14,
    BannerLime = 15,
    BannerPink = 16,
    BannerGray = 17,
    BannerLightGray = 18,
    BannerCyan = 19,
    BannerPurple = 20,
    BannerBlue = 21,
    BannerBrown = 22,
    BannerGreen = 23,
    BannerRed = 24,
    BannerBlack = 25,
    TreasureMarker = 26,
    DesertVillage = 27,
    PlainsVillage = 28,
    SavannaVillage = 29,
    SnowyVillage = 30,
    TaigaVillage = 31,
    JungleTemple = 32,
    SwampHut = 33,
    TrialChambers = 34,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VillagerLevel {
    Novice = 1,
    Apprentice = 2,
    Journeyman = 3,
    Expert = 4,
    Master = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookAtAnchor {
    Feet = 0,
    Eyes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviousGameMode {
    Undefined = -1,
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    MainHand = 0,
    OffHand = 1,
    Boots = 2,
    Leggings = 3,
    Chestplate = 4,
    Helmet = 5,
    Body = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveMode {
    Create = 0,
    Remove = 1,
    Update = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveType {
    Integer = 0,
    Hearts = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Blank,
    Styled(Nbt),
    Fixed(JsonTextComponent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamMethod {
    Create = 0,
    Remove = 1,
    UpdateInfo = 2,
    AddEntities = 3,
    RemoveEntities = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameTagVisibility {
    Always,
    HideForOtherTeams,
    HideForOwnTeam,
    Never,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollisionRule {
    Always,
    PushOtherTeams,
    PushOwnTeam,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamColor {
    Black = 0,
    DarkBlue = 1,
    DarkGreen = 2,
    DarkAqua = 3,
    DarkRed = 4,
    DarkPurple = 5,
    Gold = 6,
    Gray = 7,
    DarkGray = 8,
    Blue = 9,
    Green = 10,
    Aqua = 11,
    Red = 12,
    LightPurple = 13,
    Yellow = 14,
    White = 15,
    Obfuscated = 16,
    Bold = 17,
    Strikethrough = 18,
    Underlined = 19,
    Italic = 20,
    Reset = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCategory {
    Master = 0,
    Music = 1,
    Record = 2,
    Weather = 3,
    Block = 4,
    Hostile = 5,
    Neutral = 6,
    Player = 7,
    Ambient = 8,
    Voice = 9,
}

impl TryFrom<VarInt> for SoundCategory {
    type Error = ();

    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        match value {
            VarInt(0) => Ok(SoundCategory::Master),
            VarInt(1) => Ok(SoundCategory::Music),
            VarInt(2) => Ok(SoundCategory::Record),
            VarInt(3) => Ok(SoundCategory::Weather),
            VarInt(4) => Ok(SoundCategory::Block),
            VarInt(5) => Ok(SoundCategory::Hostile),
            VarInt(6) => Ok(SoundCategory::Neutral),
            VarInt(7) => Ok(SoundCategory::Player),
            VarInt(8) => Ok(SoundCategory::Ambient),
            VarInt(9) => Ok(SoundCategory::Voice),
            _ => Err(()),
        }
    }
}

impl From<SoundCategory> for VarInt {
    fn from(value: SoundCategory) -> Self {
        VarInt(value as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvancementFrameType {
    Task = 0,
    Challenge = 1,
    Goal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeOperation {
    AddAmount = 0,
    AddPercent = 1,
    MultiplyPercent = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statistic {
    pub category_id: VarInt,
    pub statistic_id: VarInt,
    pub value: VarInt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandNode {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuggestionMatch {
    pub match_text: String,
    pub tooltip: Option<JsonTextComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapIcon {
    pub icon_type: VarInt,
    pub x: i8,
    pub z: i8,
    pub direction: u8,
    pub display_name: Option<JsonTextComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapColorPatch {
    pub columns: u8,
    pub rows: u8,
    pub x: u8,
    pub z: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeItem {
    pub item_id: VarInt,
    pub item_count: VarInt,
    pub components: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerchantTrade {
    pub input_item1: TradeItem,
    pub output_item: Slot,
    pub input_item2: Option<TradeItem>,
    pub trade_disabled: bool,
    pub num_trade_uses: i32,
    pub max_trade_uses: i32,
    pub xp: i32,
    pub special_price: i32,
    pub price_multiplier: f32,
    pub demand: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoChatSession {
    pub session_id: Uuid,
    pub public_key_expiry_time: i64,
    pub encoded_public_key: Vec<u8>,
    pub public_key_signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerInfoActionData {
    AddPlayer {
        name: String,
        properties: Vec<PlayerInfoProperty>,
    },
    InitializeChat {
        chat_session: Option<PlayerInfoChatSession>,
    },
    UpdateGameMode {
        game_mode: VarInt,
    },
    UpdateListed {
        listed: bool,
    },
    UpdateLatency {
        ping: VarInt,
    },
    UpdateDisplayName {
        display_name: Option<JsonTextComponent>,
    },
    UpdateListPriority {
        priority: VarInt,
    },
    UpdateHat {
        visible: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoEntry {
    pub uuid: Uuid,
    pub actions: Vec<PlayerInfoActionData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementDisplay {
    pub title: JsonTextComponent,
    pub description: JsonTextComponent,
    pub icon: Slot,
    pub frame_type: VarInt,
    pub flags: i32,
    pub background_texture: Option<Identifier>,
    pub x_coord: f32,
    pub y_coord: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Advancement {
    pub parent_id: Option<Identifier>,
    pub display_data: Option<AdvancementDisplay>,
    pub requirements: Vec<Vec<String>>,
    pub sends_telemetry_data: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementProgressCriterion {
    pub criterion_identifier: Identifier,
    pub date_of_achieving: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementProgress {
    pub criteria: Vec<AdvancementProgressCriterion>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifierData {
    pub id: Identifier,
    pub amount: f64,
    pub operation: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeProperty {
    pub id: VarInt,
    pub value: f64,
    pub modifiers: Vec<AttributeModifierData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EquipmentEntry {
    pub slot: u8,
    pub item: Slot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamEntities {
    pub entities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkSectionBlockUpdate {
    pub block_state_id_and_pos: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeBookEntry {
    pub recipe_id: VarInt,
    pub display: RecipeDisplay,
    pub group_id: VarInt,
    pub category_id: VarInt,
    pub ingredients: Option<Vec<IdSet>>,
    pub flags: u8,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BossBarFlags: u8 {
        const DARKEN_SKY = 0x01;
        const DRAGON_BAR = 0x02;
        const CREATE_FOG = 0x04;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PlayerAbilityFlags: u8 {
        const INVULNERABLE = 0x01;
        const FLYING = 0x02;
        const ALLOW_FLYING = 0x04;
        const CREATIVE_MODE = 0x08;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TeleportFlags: u8 {
        const RELATIVE_X = 0x01;
        const RELATIVE_Y = 0x02;
        const RELATIVE_Z = 0x04;
        const RELATIVE_YAW = 0x08;
        const RELATIVE_PITCH = 0x10;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RecipeBookFlags: u8 {
        const SHOW_NOTIFICATION = 0x01;
        const HIGHLIGHT_NEW = 0x02;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FriendlyFlags: u8 {
        const ALLOW_FRIENDLY_FIRE = 0x01;
        const CAN_SEE_INVISIBLE_TEAMMATES = 0x02;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EntityEffectFlags: u8 {
        const IS_AMBIENT = 0x01;
        const SHOW_PARTICLES = 0x02;
        const SHOW_ICON = 0x04;
        const BLEND = 0x08;
    }

     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct StopSoundFlags: u8 {
        const HAS_SOURCE = 0x01;
        const HAS_SOUND = 0x02;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AdvancementFlags: i32 {
        const HAS_BACKGROUND_TEXTURE = 0x01;
        const SHOW_TOAST = 0x02;
        const HIDDEN = 0x04;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RespawnDataKeptFlags: u8 {
        const KEEP_ATTRIBUTES = 0x01;
        const KEEP_METADATA = 0x02;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdOr<T> {
    Id(VarInt),
    Inline(T),
}

pub type HashedSlot = Slot;

#[derive(Debug, PartialEq, Clone)]
pub struct ArgumentSignature {
    pub argument_name: String,
    pub signature: Vec<u8>,
}
