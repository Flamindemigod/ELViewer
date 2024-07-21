use std::{alloc::{alloc_zeroed, Layout}, collections::HashMap, sync::{Arc, Mutex}};

use anyhow::Result;

use crate::{modules::parser::{data_structs::{abilities::{AbilityInfo, EffectInfo}, log::{BeginLog, MapInfo, SegmentType}}, Lexer}, ParserState};

use super::{combat::{Combat, EncounterGroups, EventsKind}, units::{Unit, UnitTrait}, zone::{Trial, Zone}};


#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Log {
    pub start: BeginLog,
    pub maps: HashMap<u32, MapInfo>,
    pub ability_map: HashMap<u32, AbilityInfo>,
    pub effect_map: HashMap<u32, EffectInfo>,
    pub zones: Vec<Zone>,
}

impl Log {
    pub fn init(lexer: &mut Lexer, state: Arc<Mutex<ParserState>>) -> Result<Box<Self>> {
        let mut log = unsafe {
            let layout = Layout::new::<Self>();
            let l = alloc_zeroed(layout) as *mut Self;
            Box::from_raw(l)
        };
        log.zones = vec![];
        log.ability_map = HashMap::new();
        log.effect_map = HashMap::new();
        let seg_len = lexer.get_len();
        let mut count = 0;
        let mut map = 0;
        let mut units: HashMap<u32, Unit> = HashMap::new();
        while let Some(segment) = lexer.next_segment() {
            count += 1;
            *state.lock().unwrap() = ParserState::Processing(count as f64 / seg_len as f64);

            match segment.line {
                SegmentType::MapInfo(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(event) = zone.encounters.last_mut() {
                            match event {
                                EncounterGroups::NonCombat => {
                                    map = x.id;
                                }
                                EncounterGroups::Combat(e) => {
                                    e.events.insert(segment.time as u32, EventsKind::Map(x.id));
                                }
                            };
                        }
                    }
                    log.maps.insert(x.id, x);
                }
                SegmentType::BeginCombat => {
                    if let Some(zone) = log.zones.last_mut() {
                        zone.encounters.push(EncounterGroups::Combat(Combat::new(
                            segment.time as u32,
                            map,
                            units.clone(),
                        )));
                    }
                }
                SegmentType::EndCombat => {
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(event) = zone.encounters.last_mut() {
                            match event {
                                EncounterGroups::NonCombat => {
                                    unreachable!(
                                        "End Combat Must Be called within a combat context"
                                    );
                                }
                                EncounterGroups::Combat(e) => {
                                    e.duration_ms = segment.time as u32 - e.start_time_ms;
                                }
                            };
                        }

                        zone.encounters.push(EncounterGroups::NonCombat);
                    }
                }
                SegmentType::CombatEvent(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(encounter) = zone.encounters.last_mut() {
                            match encounter {
                                EncounterGroups::NonCombat => (), //Ignore Out of Combat Events
                                EncounterGroups::Combat(e) => {
                                    e.events.insert(segment.time as u32, EventsKind::Combat(x));
                                }
                            };
                        }
                    }
                }

                SegmentType::PlayerInfo(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(encounter) = zone.encounters.last_mut() {
                            match encounter {
                                EncounterGroups::NonCombat => unreachable!(
                                    "PlayerInfo Should always exist within Combat contexts"
                                ),
                                EncounterGroups::Combat(e) => e.players.push(*x),
                            }
                        }
                    }
                }
                SegmentType::UnitAdded(x) => {
                    units.insert(x.unit_id, x.into());
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(encounter) = zone.encounters.last_mut() {
                            match encounter {
                                EncounterGroups::NonCombat => {} //Ignore Out of Combat Events
                                EncounterGroups::Combat(e) => {

                                    // e.events.insert(segment.time as u32, EventsKind::Combat(x));
                                }
                            };
                        }
                    }
                }
                SegmentType::UnitChanged(x) => {
                    if let Some(u) = units.get(&x.unit_id) {
                        units.insert(x.unit_id, u.merge(&x.into()));
                    }
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(encounter) = zone.encounters.last_mut() {
                            match encounter {
                                EncounterGroups::NonCombat => {} //Ignore Out of Combat Events
                                EncounterGroups::Combat(e) => {

                                    // e.events.insert(segment.time as u32, EventsKind::Combat(x));
                                }
                            };
                        }
                    }
                }
                SegmentType::UnitRemoved(x) => {
                    units.remove(&x.unit_id);
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(encounter) = zone.encounters.last_mut() {
                            match encounter {
                                EncounterGroups::NonCombat => {} //Ignore Out of Combat Events
                                EncounterGroups::Combat(e) => {

                                    // e.events.insert(segment.time as u32, EventsKind::Combat(x));
                                }
                            };
                        }
                    }
                }

                SegmentType::ZoneInfo(x) => {
                    units.clear();
                    if let Some(zone) = log.zones.last_mut() {
                        if let Some(t) = zone.trial.as_mut() {
                            if t.duration_ms.is_none() {
                                t.duration_ms = Some(
                                    ((log.start.time_since_epoch_s as usize) * 1000
                                        - t.start_time_ms
                                            .as_ref()
                                            .expect("Start Time must be defined")
                                            .parse::<usize>()
                                            .unwrap()
                                        + segment.time) as u32,
                                )
                            }
                        };
                    };

                    log.zones.push(Zone {
                        zone: x,
                        trial: None,
                        encounters: vec![EncounterGroups::NonCombat],
                    });
                }
                SegmentType::TrialInit(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        zone.trial = Trial::from_init(x);
                    }
                }
                //
                SegmentType::BeginTrial(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        zone.trial = Some(Trial::from_begin(x));
                    }
                }

                SegmentType::EndTrial(x) => {
                    if let Some(zone) = log.zones.last_mut() {
                        zone.trial = Some(Trial::merge_end(zone.trial.as_ref().unwrap(), x));
                    }
                }
                SegmentType::EffectInfo(x) => {
                    log.effect_map.insert(x.ability_id, x);
                }

                SegmentType::AbilityInfo(x) => {
                    log.ability_map.insert(x.ability_id, x);
                }
                SegmentType::BeginLog(x) => {
                    log.start = x;
                }
                _ => (),
            };
        }
        Ok(log)
    }
}
