use serde::{Deserialize, Serialize};
use specta::Type;

use crate::modules::parser::data_structs::{
    combat::{PlayerReaction, UnitAdded, UnitType},
    player::{Class, Race},
};

pub trait UnitTrait {
    fn merge(&self, value: &UnitAdded) -> Self;
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Boss {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Unit {
    Player(UnitPlayer),
    Friendly(UnitFriendly),
    Companion(UnitCompanion),
    NpcAlly(UnitNpcAlly),
    Neutral(UnitNeutral),
    Hostile(UnitHostile),
}

impl UnitTrait for Unit {
    fn merge(&self, value: &UnitAdded) -> Self {
        match self {
            Self::Player(x) => Self::Player(x.merge(value)),
            Self::Friendly(x) => Self::Friendly(x.merge(value)),
            Self::Companion(x) => Self::Companion(x.merge(value)),
            Self::NpcAlly(x) => Self::NpcAlly(x.merge(value)),
            Self::Neutral(x) => Self::Neutral(x.merge(value)),
            Self::Hostile(x) => Self::Hostile(x.merge(value)),
        }
    }
}

impl From<UnitAdded> for Unit {
    fn from(value: UnitAdded) -> Self {
        match value.reaction {
            PlayerReaction::PlayerAlly => Self::Player(value.into()),
            PlayerReaction::Friendly => Self::Friendly(value.into()),
            PlayerReaction::Companion => Self::Companion(value.into()),
            PlayerReaction::NpcAlly => Self::NpcAlly(value.into()),
            PlayerReaction::Neutral => Self::Neutral(value.into()),
            PlayerReaction::Hostile => Self::Hostile(value.into()),
        }
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitPlayer {
    pub unit_id: u32,
    pub is_local_player: bool,
    pub player_per_session_id: u8,
    pub class: Class,
    pub race: Race,
    pub name: String,
    pub display_name: String,
    pub character_id: String, //BigInt stored as String due to IPC limitations
    pub level: u32,
    pub champion_points: u16,
    pub is_grouped_with_local_player: bool,
}

impl From<UnitAdded> for UnitPlayer {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            is_local_player: value.is_local_player,
            player_per_session_id: value.player_per_session_id,
            class: value.class,
            race: value.race,
            name: value.name,
            display_name: value.display_name,
            character_id: value.character_id,
            level: value.level,
            champion_points: value.champion_points,
            is_grouped_with_local_player: value.is_grouped_with_local_player,
        }
    }
}

impl UnitTrait for UnitPlayer {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.is_local_player = value.is_local_player;
        out.player_per_session_id = value.player_per_session_id;
        out.class.clone_from(&value.class);
        out.race.clone_from(&value.race);
        out.name.clone_from(&value.name);
        out.display_name.clone_from(&value.display_name);
        out.character_id.clone_from(&value.character_id);
        out.level = value.level;
        out.champion_points = value.champion_points;
        out.is_grouped_with_local_player = value.is_grouped_with_local_player;
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitFriendly {
    pub unit_id: u32,
    pub monster_id: u32,
    pub name: String,
    pub level: u32,
    pub champion_points: u16,
    pub owner_unit_id: u32,
}

impl From<UnitAdded> for UnitFriendly {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            monster_id: value.monster_id,
            name: value.name,
            level: value.level,
            champion_points: value.champion_points,
            owner_unit_id: value.owner_unit_id,
        }
    }
}

impl UnitTrait for UnitFriendly {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.monster_id = value.monster_id;
        out.name.clone_from(&value.name);
        out.level = value.level;
        out.champion_points = value.champion_points;
        out.owner_unit_id = value.owner_unit_id;
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitCompanion {
    pub unit_id: u32,
    pub monster_id: u32,
    pub name: String,
    pub level: u32,
    pub owner_unit_id: u32,
}

impl From<UnitAdded> for UnitCompanion {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            monster_id: value.monster_id,
            name: value.name,
            level: value.level,
            owner_unit_id: value.owner_unit_id,
        }
    }
}

impl UnitTrait for UnitCompanion {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.monster_id = value.monster_id;
        out.name.clone_from(&value.name);
        out.level = value.level;
        out.owner_unit_id = value.owner_unit_id;
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitNpcAlly {
    pub unit_id: u32,
    pub monster_id: u32,
    pub name: String,
    pub level: u32,
    pub champion_points: u16,
    pub owner_unit_id: u32,
}

impl From<UnitAdded> for UnitNpcAlly {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            monster_id: value.monster_id,
            name: value.name,
            level: value.level,
            champion_points: value.champion_points,
            owner_unit_id: value.owner_unit_id,
        }
    }
}

impl UnitTrait for UnitNpcAlly {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.monster_id = value.monster_id;
        out.name.clone_from(&value.name);
        out.level = value.level;
        out.champion_points = value.champion_points;
        out.owner_unit_id = value.owner_unit_id;
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitNeutral {
    pub unit_id: u32,
    pub unit_type: UnitType,
    pub monster_id: u32,
    pub name: String,
    pub level: u32,
    pub champion_points: u16,
}

impl From<UnitAdded> for UnitNeutral {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            unit_type: value.unit_type,
            monster_id: value.monster_id,
            name: value.name,
            level: value.level,
            champion_points: value.champion_points,
        }
    }
}

impl UnitTrait for UnitNeutral {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.monster_id = value.monster_id;
        out.name.clone_from(&value.name);
        out.level = value.level;
        out.champion_points = value.champion_points;
        out
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitHostile {
    pub unit_id: u32,
    pub monster_id: u32,
    pub is_boss: bool,
    pub name: String,
    pub level: u32,
    pub champion_points: u16,
    pub owner_unit_id: u32,
}

impl From<UnitAdded> for UnitHostile {
    fn from(value: UnitAdded) -> Self {
        Self {
            unit_id: value.unit_id,
            monster_id: value.monster_id,
            is_boss: value.is_boss,
            name: value.name,
            level: value.level,
            champion_points: value.champion_points,
            owner_unit_id: value.owner_unit_id,
        }
    }
}

impl UnitTrait for UnitHostile {
    fn merge(&self, value: &UnitAdded) -> Self {
        let mut out = self.clone();
        out.unit_id = value.unit_id;
        out.monster_id = value.monster_id;
        out.name.clone_from(&value.name);
        out.level = value.level;
        out.champion_points = value.champion_points;
        out.owner_unit_id = value.owner_unit_id;
        out
    }
}
