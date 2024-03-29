//! Known map types as constants.
//!
//! For example [`map_type::INSTANCE`](self::INSTANCE) holds the map type id for instances.

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
