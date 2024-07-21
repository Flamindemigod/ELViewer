use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::modules::parser::data_structs::{abilities::{BeginCast, EffectChanged, EndCast}, combat::{CombatEvent, HealthRegen, UnitAdded, UnitChanged, UnitRemoved}, player::PlayerInfo};

use super::units::{Boss, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum EventsKind {
    Map(u32),
    Effect(EffectChanged),
    Combat(CombatEvent),
    UnitAdded(UnitAdded),
    UnitChanged(UnitChanged),
    UnitRemoved(UnitRemoved),
    Health(HealthRegen),
    BeginCast(BeginCast),
    EndCast(EndCast),
}


#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Combat {
    pub start_time_ms: u32,
    pub duration_ms: u32,
    pub initial_map: u32,
    pub boss: Option<Boss>,
    pub initial_units: HashMap<u32, Unit>,
    pub players: Vec<PlayerInfo>,
    pub events: BTreeMap<u32, EventsKind>,
}

impl Combat {
    pub fn new(start_time_ms: u32, map: u32, initial_units: HashMap<u32, Unit>) -> Self {
        let boss: Option<Boss> = initial_units
            .iter()
            .find_map(|(_, f)| {
                if let Unit::Hostile(x) = f {
                    if x.is_boss {
                        return Some(Boss {
                            name: x.name.clone(),
                            id: x.unit_id,
                        });
                    }
                }
                None
            })
            .take();
        Self {
            start_time_ms,
            initial_map: map,
            duration_ms: 0,
            boss,
            initial_units,
            players: Vec::new(),
            events: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum EncounterGroups {
    Combat(Combat),
    NonCombat,
}
