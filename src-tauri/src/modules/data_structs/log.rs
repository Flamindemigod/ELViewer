use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    abilities::{AbilityInfo, BeginCast, EffectChanged, EffectInfo, EndCast},
    combat::{CombatEvent, HealthRegen, UnitAdded, UnitChanged, UnitRemoved},
    player::PlayerInfo,
};

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct BeginLog {
    pub time_since_epoch_s: u32, 
    pub log_version: u32,
    pub realm_name: String,
    pub language: String,
    pub game_version: String,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct MapInfo {
    pub id: u32,
    pub name: String,
    pub texture_path: PathBuf,
}
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum DungeonDifficulty {
    Normal,
    Veteran,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct ZoneInfo {
    pub id: u32,
    pub name: String,
    pub dungeon_difficulty: DungeonDifficulty,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Trialinit {
    pub id: u8,
    pub in_progress: bool,
    pub completed: bool,
    pub start_time_ms: u32,
    pub duration_ms: u32,
    pub success: bool,
    pub final_score: u32,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct BeginTrial {
    pub id: u8,
    pub start_time_ms: String, //BigInt Stored as String due to IPC limitations
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EndTrial {
    pub id: u8,
    pub duration_ms: u32,
    pub success: bool,
    pub final_score: u32,
    pub final_vitality_bonus: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum SegmentType {
    BeginLog(BeginLog),
    EndLog,
    BeginCombat,
    EndCombat,
    PlayerInfo(Box<PlayerInfo>),
    BeginCast(BeginCast),
    EndCast(EndCast),
    CombatEvent(CombatEvent),
    HealthRegen(HealthRegen),
    UnitAdded(UnitAdded),
    UnitChanged(UnitChanged),
    UnitRemoved(UnitRemoved),
    EffectChanged(EffectChanged),
    AbilityInfo(AbilityInfo),
    EffectInfo(EffectInfo),
    MapInfo(MapInfo),
    ZoneInfo(ZoneInfo),
    TrialInit(Trialinit),
    BeginTrial(BeginTrial),
    EndTrial(EndTrial),
    EndlessDungeonBuffRemove,
    EndlessDungeonBuffAdd,
    EndlessDungeonStageEnd,
}
#[derive(Debug)]
pub struct Segment {
    pub time: usize, // Time Since Logging Began in MS
    pub line: SegmentType,
}
