use bitflags::bitflags;

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
