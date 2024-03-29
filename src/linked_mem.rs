use crate::Identity;
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
