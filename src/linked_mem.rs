use crate::{util::until_nul, Context};
#[cfg(windows)]
use std::{ffi::OsString, os::windows::ffi::OsStringExt};

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
    /// Returns the name as [`OsString`].
    #[inline]
    #[cfg(windows)]
    pub fn name_string(&self) -> OsString {
        OsString::from_wide(until_nul(&self.name))
    }

    /// Returns the name as [`OsString`].
    #[inline]
    #[cfg(windows)]
    pub fn identity_string(&self) -> OsString {
        OsString::from_wide(until_nul(&self.identity))
    }

    /// Returns the name as [`OsString`].
    #[inline]
    #[cfg(windows)]
    pub fn description_string(&self) -> OsString {
        OsString::from_wide(until_nul(&self.description))
    }

    /// Parses the current identity JSON contents.
    #[cfg(feature = "json")]
    pub fn parse_identity(&self) -> serde_json::Result<crate::Identity> {
        let string = self.identity_string();
        serde_json::from_str(&string.to_string_lossy())
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
