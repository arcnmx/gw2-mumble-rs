pub use self::{eod::*, ibs::*, ls1::*, soto::*, wintersday::*};

/// Wintersday Strike Missions.
pub mod wintersday {
    /// Secret Lair of the Snowmen (Public).
    pub const SECRET_LAIR_OF_THE_SNOWMEN: u32 = 1306;
}

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
