//! Known map ids.

/// Raids.
pub mod raid {
    /// Lion's Arch Aerodrome (Raid lobby).
    pub const AERODROME: u32 = 1155;

    /// Special Forces Training Area (Golem).
    pub const TRAINING_AREA: u32 = 1154;

    /// Spirit Vale (Wing 1).
    pub const SPIRIT_VALE: u32 = 1062;

    /// Salvation Pass (Wing 2).
    pub const SALVATION_PASS: u32 = 1149;

    /// Stronghold of the Faithful (Wing 3).
    pub const STRONGHOLD_OF_THE_FAITHFUL: u32 = 1156;

    /// Bastion of the Penitent (Wing 4).
    pub const BASTION_OF_THE_PENITENT: u32 = 1188;

    /// Hall of Chains (Wing 5).
    pub const HALL_OF_CHAINS: u32 = 1264;

    /// Mythwright Gambit (Wing 6).
    pub const MYTHWRIGHT_GAMBIT: u32 = 1303;

    /// The Key of Ahdashim (Wing 7).
    pub const KEY_OF_AHDASHIM: u32 = 1323;
}

/// Fractals of the Mists.
pub mod fractal {
    /// Mistlock Observatory (Fractal lobby).
    pub const MISTLOCK_OBSERVATORY: u32 = 872;

    /// Uncategorized Fractal.
    pub const UNCATEGORIZED: u32 = 947;

    /// Snowblind Fractal.
    pub const SNOWBLIND: u32 = 948;

    /// Swampland Fractal.
    pub const SWAMPLAND: u32 = 949;

    /// Urban Battleground Fractal.
    pub const URBAN_BATTLEGROUND: u32 = 950;

    /// Aquatic Ruins Fractal.
    pub const AQUATIC_RUINS: u32 = 951;

    /// Cliffside Fractal.
    pub const CLIFFSIDE: u32 = 952;

    /// Underground Facility Fractal.
    pub const UNDERGROUND_FACILITY: u32 = 953;

    /// Volcanic Fractal.
    pub const VOLCANIC: u32 = 954;

    /// Molten Furnace Fractal.
    pub const MOLTEN_FURNANCE: u32 = 955;

    /// Aetherblade Fractal.
    pub const AETHERBLADE: u32 = 956;

    /// Thaumanova Reactor Fractal.
    pub const THAUMANOVA_REACTOR: u32 = 957;

    /// Solid Ocean Fractal.
    pub const SOLID_OCEAN: u32 = 958;

    /// Molten Boss Fractal.
    pub const MOLTEN_BOSS: u32 = 959;

    /// Captain Mai Trin Boss Fractal.
    pub const CAPTAIN_MAI_TRIN_BOSS: u32 = 960;

    /// Chaos Fractal.
    pub const CHAOS: u32 = 1164;

    /// Nightmare Fractal.
    pub const NIGHTMARE: u32 = 1177;

    /// Shattered Observatory Fractal.
    pub const SHATTERED_OBSERVATORY: u32 = 1205;

    /// Twiilight Oasis Fractal.
    pub const TWILIGHT_OASIS: u32 = 1267;

    /// Deepstone Fractal.
    pub const DEEPSTONE: u32 = 1290;

    /// Siren's Reef Fractal.
    pub const SIRENS_REEF: u32 = 1309;

    /// Sunqua Peak Fractal.
    pub const SUNQUA_PEAK: u32 = 1384;

    /// Silent Surf Fractal.
    pub const SILENT_SURF: u32 = 1500;

    /// Lonely Tower Fractal.
    pub const LONELY_TOWER: u32 = 1538;
}

/// Strike Missions.
pub mod strike {
    pub use self::{eod::*, ibs::*, ls1::*, soto::*};

    /// Icebrood Saga Strike Missions.
    pub mod ibs {
        /// Strike Mission: Shiverpeaks Pass (Icebrood Construct).
        pub const SHIVERPEAKS_PASS: u32 = 1332;

        /// Strike Mission: Boneskinner.
        pub const BONESKINNER: u32 = 1339;

        /// Strike Mission: Fraenir of Jormag.
        pub const FRAENIR_OF_JORMAG: u32 = 1341;

        /// Strike Mission: Voice of the Fallen and Claw of the Fallen.
        pub const VOICE_AND_CLAW: u32 = 1346;

        /// Strike Mission: Whisper of Jormag.
        pub const WHISPER_OF_JORMAG: u32 = 1359;

        /// Forging Steel (Ancient Forgeman).
        pub const FORGING_STEEL: u32 = 1368;

        /// Strike Mission: Cold War (Minister of Morale).
        pub const COLD_WAR: u32 = 1374;
    }

    /// End of Dragons Strike Missions.
    pub mod eod {
        /// Strike Mission: Aetherblade Hideout (Mai Trin).
        pub const AETHERBLADE_HIDEOUT: u32 = 1432;

        /// Strike Mission: Xunlai Jade Junkyard (Ankka).
        pub const XUNLAI_JADE_JUNKYARD: u32 = 1450;

        /// Strike Mission: Kaineng Overlook (Minister Li).
        pub const KAINENG_OVERLOOK: u32 = 1451;

        /// Strike Mission: Harvest Temple (The Dragonvoid).
        pub const HARVEST_TEMPLE: u32 = 1437;
    }

    /// Living World Season 1 (rework) Strike Missions.
    pub mod ls1 {
        /// Strike Mission: Old Lion's Court.
        pub const OLD_LIONS_COURT: u32 = 1485;
    }

    /// Secrets of the Obscure Strike Missions.
    pub mod soto {
        /// Strike Mission: Cosmic Observatory (Dagda).
        pub const COSMIC_OBSERVATORY: u32 = 1515;

        /// Strike Mission: Temple of Febe (Cerus).
        pub const TEMPLE_OF_FEBE: u32 = 1520;
    }
}

/// Player Hubs.
pub mod hub {
    pub use super::{fractal::MISTLOCK_OBSERVATORY, raid::AERODROME};

    /// Lions's Arch.
    pub const LIONS_ARCH: u32 = 50;

    /// Mistlock Sanctuary.
    pub const MISTLOCK_SANCTUARY: u32 = 1206;

    /// Eye of the North (IBS).
    pub const EYE_OF_THE_NORTH: u32 = 1370;

    /// Arborstone (EOD).
    pub const ARBORSTONE: u32 = 1428;

    /// Wizard's Tower (SOTO).
    pub const WIZARDS_TOWER: u32 = 1509;
}
