use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    abilities::{ActionResult, DamageType, PowerType},
    player::{Class, Race, Targets, UnitState},
};

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct CombatEvent {
    pub action_result: ActionResult,
    pub damage_type: DamageType,
    pub power_type: PowerType,
    pub hit_value: u32,
    pub overflow: u32,
    pub cast_track_id: u32,
    pub ability_id: u32,
    pub source: UnitState,
    pub target: Targets,
}
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct HealthRegen {
    pub effective_regen: u32,
    pub source: UnitState,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitAdded {
    pub unit_id: u32,
    pub unit_type: UnitType,
    pub is_local_player: bool,
    pub player_per_session_id: u8,
    pub monster_id: u32,
    pub is_boss: bool,
    pub class: Class,
    pub race: Race,
    pub name: String,
    pub display_name: String,
    pub character_id: String, //BigInt stored as String due to IPC limitations
    pub level: u32,
    pub champion_points: u16,
    pub owner_unit_id: u32,
    pub reaction: PlayerReaction,
    pub is_grouped_with_local_player: bool,
}
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitChanged {
    pub unit_id: u32,
    pub class: Class,
    pub race: Race,
    pub name: String,
    pub display_name: String,
    pub character_id: String, // BigInt stored as String due to IPC limitations
    pub level: u32,
    pub champion_points: u16,
    pub owner_unit_id: u32,
    pub reaction: PlayerReaction,
    pub is_grouped_with_local_player: bool,
}

impl From<UnitChanged> for UnitAdded{
    fn from(value: UnitChanged) -> Self {
        Self{
    unit_id: value.unit_id,
    unit_type: UnitType::Player,
    is_local_player: false,
    player_per_session_id: 0,
    monster_id: 0,
    is_boss: false,
    class: value.class,
    race: value.race,
    name: value.name,
    display_name: value.display_name,
    character_id: value.character_id,
    level: value.level,
    champion_points: value.champion_points,
    owner_unit_id: value.owner_unit_id,
    reaction: value.reaction,
    is_grouped_with_local_player: value.is_grouped_with_local_player,
        }
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitRemoved {
    pub unit_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum UnitType {
    Player,
    Monster,
    Object,
}

impl From<String> for UnitType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PLAYER" => Self::Player,
            "MONSTER" => Self::Monster,
            "OBJECT" => Self::Object,
            x => unimplemented!("{x} Unit is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum PlayerReaction {
    PlayerAlly,
    Friendly,
    Companion,
    NpcAlly,
    Neutral,
    Hostile,
}

impl From<String> for PlayerReaction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PLAYER_ALLY" => Self::PlayerAlly,
            "FRIENDLY" => Self::Friendly,
            "COMPANION" => Self::Companion,
            "NPC_ALLY" => Self::NpcAlly,
            "NEUTRAL" => Self::Neutral,
            "HOSTILE" => Self::Hostile,
            x => unimplemented!("{x} Player Reaction not implemented"),
        }
    }
}
