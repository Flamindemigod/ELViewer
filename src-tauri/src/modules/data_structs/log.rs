use std::{path::PathBuf, sync::Arc};

use super::{
    abilities::{AbilityInfo, BeginCast, EffectChanged, EffectInfo, EndCast},
    combat::{CombatEvent, HealthRegen, UnitAdded, UnitChanged, UnitRemoved},
    player::PlayerInfo,
};

#[derive(Debug)]
pub struct BeginLog {
    pub time_since_epoch_ms: usize,
    pub log_version: usize,
    pub realm_name: Arc<str>,
    pub language: Arc<str>,
    pub game_version: Arc<str>,
}

#[derive(Debug)]
pub struct MapInfo {
    pub id: usize,
    pub name: Arc<str>,
    pub texture_path: PathBuf,
}
#[derive(Debug)]
pub enum DungeonDifficulty {
    Normal,
    Veteran,
}

#[derive(Debug)]
pub struct ZoneInfo {
    pub id: usize,
    pub name: Arc<str>,
    pub dungeon_difficulty: DungeonDifficulty,
}

#[derive(Debug)]
pub struct Trialinit {
    pub id: u8,
    pub in_progress: bool,
    pub completed: bool,
    pub start_time_ms: usize,
    pub duration_ms: usize,
    pub success: bool,
    pub final_score: usize,
}

#[derive(Debug)]
pub struct BeginTrial {
    pub id: u8,
    pub start_time_ms: usize,
}

#[derive(Debug)]
pub struct EndTrial {
    pub id: u8,
    pub duration_ms: usize,
    pub success: bool,
    pub final_score: usize,
    pub final_vitality_bonus: u8,
}

#[derive(Debug)]
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
