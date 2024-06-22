use std::sync::Arc;

use super::{
    abilities::{ActionResult, DamageType, PowerType},
    player::{Class, Race, Targets, UnitState},
};

#[derive(Debug)]
pub struct CombatEvent {
    pub action_result: ActionResult,
    pub damage_type: DamageType,
    pub power_type: PowerType,
    pub hit_value: usize,
    pub overflow: usize,
    pub cast_track_id: usize,
    pub ability_id: usize,
    pub source: UnitState,
    pub target: Targets,
}
#[derive(Debug)]
pub struct HealthRegen {
    pub effective_regen: usize,
    pub source: UnitState,
}

#[derive(Debug)]
pub struct UnitAdded {
    pub unit_id: usize,
    pub unit_type: UnitType,
    pub is_local_player: bool,
    pub player_per_session_id: u8,
    pub monster_id: usize,
    pub is_boss: bool,
    pub class: Class,
    pub race: Race,
    pub name: Arc<str>,
    pub display_name: Arc<str>,
    pub character_id: usize,
    pub level: usize,
    pub champion_points: u16,
    pub owner_unit_id: usize,
    pub reaction: PlayerReaction,
    pub is_grouped_with_local_player: bool,
}
#[derive(Debug)]
pub struct UnitChanged {
    pub unit_id: usize,
    pub class: Class,
    pub race: Race,
    pub name: Arc<str>,
    pub display_name: Arc<str>,
    pub character_id: usize,
    pub level: usize,
    pub champion_points: u16,
    pub owner_unit_id: usize,
    pub reaction: PlayerReaction,
    pub is_grouped_with_local_player: bool,
}
#[derive(Debug)]
pub struct UnitRemoved {
    pub unit_id: usize,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
