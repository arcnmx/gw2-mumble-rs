use bitflags::bitflags;

/// MumbleLink shared memory.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct LinkedMem {
    /// UI version.
    pub ui_version: u32,

    /// UI tick.
    pub ui_tick: u32,

    /// Position of the player in map coordinate system.
    ///
    /// See [API:1/event_details#Coordinate_recalculation](https://wiki.guildwars2.com/wiki/API:1/event_details#Coordinate_recalculation).
    ///
    /// Updated every frame. Should be able to read 50 times a second.
    pub avatar: Position,

    /// Game name.
    pub name: [u16; 256],

    /// Position of the camera.
    ///
    /// Updated every frame. Should be able to read 50 times a second.
    pub camera: Position,

    /// Identity information as JSON.
    ///
    /// Should only change a few times per second.
    pub identity: [u16; 256],

    /// Length of the following context.
    ///
    /// Hardcoded to `48` for Guild Wars 2 despite [`Context`] being larger.
    pub context_len: u32,

    /// See [`Context`].
    ///
    /// Should only change a few times per second.
    pub context: Context,

    /// Game description.
    pub description: [u16; 2048],
}

impl LinkedMem {
    /// Parses the current identity JSON contents.
    #[cfg(feature = "json")]
    pub fn parse_identity(&self) -> serde_json::Result<Identity> {
        use std::{ffi::OsString, os::windows::prelude::*};

        let os_string = OsString::from_wide(&self.identity);
        let string = os_string.to_string_lossy();
        serde_json::from_str(string.trim_end_matches('\0'))
    }
}

/// Position structure.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Position {
    /// Position in space.
    pub position: [f32; 3],

    /// Unit vector pointing out of the eyes, aka the "At"-vector.
    pub front: [f32; 3],

    /// Unit vector pointing out of the top of the head, aka the "Up"-vector.
    pub top: [f32; 3],
}

/// MumbleLink context specific to Guild Wars 2.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Context {
    /// Address of the server.
    ///
    /// Contains `socketaddr_in` or `socketaddr_in6`.
    pub server_address: [u8; 28],

    /// Id of the current map.
    pub map_id: u32,

    /// Type of the current map.
    ///
    /// See the [`map_type`] module for some known types.
    pub map_type: u32,

    /// Current shard id.
    pub shard_id: u32,

    /// Current instance id.
    pub instance: u32,

    /// Current build id.
    pub build_id: u32,

    /// Current UI state bitflags.
    pub ui_state: UiState,

    /// Compass width in pixels.
    pub compass_width: u16,

    /// Compass height in pixels.
    pub compass_height: u16,

    /// Compass rotation in radians.
    pub compass_rotation: f32,

    /// Player position x in continent coordinates.
    ///
    /// Not updated in competitive modes.
    pub player_x: f32,

    /// Player position y in continent coordinates.
    ///
    /// Not updated in competitive modes.
    pub player_y: f32,

    /// Map center x in continent coordinates.
    ///
    /// Not updated in competitive modes.
    pub map_center_x: f32,

    /// Map center y in continent coordinates.
    ///
    /// Not updated in competitive modes.
    pub map_center_y: f32,

    /// Map scale.
    pub map_scale: f32,

    /// Process id.
    pub process_id: u32,

    /// Currently used mount.
    pub mount_index: Mount,
}

bitflags! {
    /// Current UI state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct UiState: u32 {
        const IS_MAP_OPEN = 0b1;

        const IS_COMPASS_TOP_RIGHT = 0b10;

        const DOES_COMPASS_HAVE_ROTATION_ENABLED = 0b100;

        const GAME_HAS_FOCUS = 0b1000;

        const IS_IN_COMPETITIVE_MODE = 0b10000;

        const TEXTBOX_HAS_FOCUS = 0b100000;

        const IS_IN_COMBAT = 0b1000000;
    }
}

/// Mount.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[repr(u8)]
pub enum Mount {
    None = 0,
    Jackal = 1,
    Griffon = 2,
    Springer = 3,
    Skimmer = 4,
    Raptor = 5,
    RollerBeetle = 6,
    Warclaw = 7,
    Skyscale = 8,
    Skiff = 9,
    SiegeTurtle = 10,
}

/// Information about player identity.
///
/// Parsed as JSON from the `identity` field in [`LinkedMem`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct Identity {
    /// Character name.
    pub name: String,

    /// Character profession.
    pub profession: Profession,

    /// Equipped 3rd specialization.
    ///
    /// `0` if no specialization is present.
    /// See [API:2/specializations](https://wiki.guildwars2.com/wiki/API:2/specializations) for details.
    pub spec: u32,

    /// Character race.
    pub race: Race,

    /// Current map id.
    ///
    /// See [API:2/maps](https://wiki.guildwars2.com/wiki/API:2/maps).
    pub map_id: u32,

    /// Shard id from [`Context`].
    ///
    /// Formerly character homeworld per [API:2/worlds](https://wiki.guildwars2.com/wiki/API:2/worlds).
    /// Not usable since the switch to the megaserver system.
    pub world_id: u32,

    /// Team color.
    ///
    /// See [API:2/colors](https://wiki.guildwars2.com/wiki/API:2/colors).
    pub team_color_id: u32,

    /// Whether the character has a commander tag active.
    pub commander: bool,

    /// Vertical field of view.
    pub fov: f32,

    /// Current user UI scaling.
    #[cfg_attr(feature = "serde", serde(rename = "uisz"))]
    pub ui_scale: UIScaling,
}

/// Character profession.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum Profession {
    Unknown = 0,
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}

/// Character race.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum Race {
    Asura = 0,
    Charr = 1,
    Human = 2,
    Norn = 3,
    Sylvari = 4,
}

/// User UI scaling.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::Display,
    strum::AsRefStr,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)
)]
#[repr(u8)]
pub enum UIScaling {
    Small = 0,
    Normal = 1,
    Large = 2,
    Larger = 3,
}

/// Map type constants.
///
/// For example [`map_type::INSTANCE`](self::INSTANCE) holds the map type id for instances.
pub mod map_type {
    pub const AUTO_REDIRECT: u32 = 0;

    pub const CHARACTER_CREATION: u32 = 1;

    pub const PVP: u32 = 2;

    pub const GVG: u32 = 3;

    pub const INSTANCE: u32 = 4;

    pub const PVE: u32 = 5;

    pub const TOURNAMENT: u32 = 6;

    pub const TUTORIAL: u32 = 7;

    pub const USER_TOURNAMENT: u32 = 8;

    pub const WVW_ETERNAL_BATTLEGROUNDS: u32 = 9;

    pub const WVW_BLUE_BORDERLANDS: u32 = 10;

    pub const WVW_GREEN_BORDERLANDS: u32 = 11;

    pub const WVW_RED_BORDERLANDS: u32 = 12;

    /// Fortune's Vale.
    pub const WVW_REWARD: u32 = 13;

    pub const WVW_OBSIDIAN_SANCTUM: u32 = 14;

    pub const WVW_EDGE_OF_THE_MISTS: u32 = 15;

    /// Mini maps like Mistlock Sanctuary, Aerodrome, etc.
    pub const PVE_MINI: u32 = 16;

    /// PvP 15v15.
    pub const BIG_BATTLE: u32 = 17;

    /// Armistice Bastion.
    pub const WVW_LOUNGE: u32 = 18;
}
