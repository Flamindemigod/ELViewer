use std::{collections::VecDeque, io::Lines, path::PathBuf, sync::Arc};
use super::{abilities::*, combat::*, log::*, player::*};

pub fn parse_bool(b: &str) -> bool {
    match b {
        "T" => true,
        "F" => false,
        x => unreachable!("{x} should be unreachable!"),
    }
}

pub struct Lexer {
    data: VecDeque<Arc<str>>,
}

impl Lexer {
    pub fn new(data: Lines<&[u8]>) -> Lexer {
        Lexer {
            data: data
                .filter_map(|l| match l {
                    Ok(v) => Some(v.as_str().into()),
                    Err(_) => None,
                })
                .collect::<VecDeque<_>>(),
        }
    }

    fn tokenize(data_: &str) -> Vec<String> {
        let mut data = data_.chars();
        let mut tokens = vec![];
        let mut current_token = String::new();
        let mut brace_count = 0;
        while let Some(char) = data.next() {
            match char {
                '[' => {
                    brace_count += 1;
                    for c in data.by_ref() {
                        if c == ']' {
                            brace_count -= 1;
                        }
                        if c == '[' {
                            brace_count += 1;
                        }
                        if brace_count <= 0 {
                            tokens.push(current_token.clone());
                            current_token.clear();
                            brace_count = 0;
                            data.next();
                            break;
                        }
                        current_token.push(c);
                    }
                }
                '\"' => {
                    for c in data.by_ref() {
                        if c == '\"' {
                            break;
                        };
                        current_token.push(c);
                    }
                }
                ',' => {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                x => current_token.push(x),
            }
        }
        if !current_token.is_empty() {
            tokens.push(current_token.clone());
        }
        tokens
    }

    pub fn next_segment(&mut self) -> Option<Segment> {
        if let Some(data) = self.data.pop_front() {
            let mut split = data.splitn(3, ',');
            let time = split.next().unwrap().parse().unwrap();
            let token = split.next().unwrap();
            let remainder = split.next();
            let mut tokens: VecDeque<_> = if let Some(r) = remainder {
                Self::tokenize(r).into()
            } else {
                vec![].into()
            };
            let line = match token {
                "BEGIN_LOG" => {
                    let time_since_epoch_ms = tokens.pop_front().unwrap().parse().unwrap();
                    let log_version = tokens.pop_front().unwrap().parse().unwrap();
                    let realm_name: String = tokens.pop_front().unwrap();
                    let language: String = tokens.pop_front().unwrap();
                    let game_version: String = tokens.pop_front().unwrap();
                    SegmentType::BeginLog(BeginLog {
                        time_since_epoch_ms,
                        log_version,
                        realm_name: realm_name.into(),
                        language: language.into(),
                        game_version: game_version.into(),
                    })
                }
                "END_LOG" => SegmentType::EndLog,
                "ZONE_CHANGED" => {
                    let id = tokens.pop_front().unwrap().parse().unwrap();
                    let name = tokens.pop_front().unwrap().into();
                    let mode = match tokens.pop_front().unwrap().as_str() {
                        "VETERAN" => DungeonDifficulty::Veteran,
                        "NORMAL" => DungeonDifficulty::Normal,
                        _ => DungeonDifficulty::Normal,
                    };
                    SegmentType::ZoneInfo(ZoneInfo {
                        id,
                        name,
                        dungeon_difficulty: mode,
                    })
                }
                "UNIT_ADDED" => {
                    let unit_id = tokens.pop_front().unwrap();
                    let unit_type = match tokens.pop_front().unwrap().as_str() {
                        "PLAYER" => UnitType::Player,
                        "MONSTER" => UnitType::Monster,
                        "OBJECT" => UnitType::Object,
                        x => unimplemented!("{x} Unit is not implemented"),
                    };
                    let is_local_player = parse_bool(&tokens.pop_front().unwrap());
                    let player_per_session_id = tokens.pop_front().unwrap();
                    let monster_id = tokens.pop_front().unwrap();
                    let is_boss = parse_bool(&tokens.pop_front().unwrap());
                    let class = Class::parse_class(&tokens.pop_front().unwrap());
                    let race = Race::parse_race(&tokens.pop_front().unwrap());
                    let name = tokens.pop_front().unwrap();
                    let display_name = tokens.pop_front().unwrap();
                    let character_id = tokens.pop_front().unwrap();
                    let level = tokens.pop_front().unwrap();
                    let champion_points = tokens.pop_front().unwrap();
                    let owner_unit_id = tokens.pop_front().unwrap();
                    let reaction = match tokens.pop_front().unwrap().as_str() {
                        "PLAYER_ALLY" => PlayerReaction::PlayerAlly,
                        "FRIENDLY" => PlayerReaction::Friendly,
                        "COMPANION" => PlayerReaction::Companion,
                        "NPC_ALLY" => PlayerReaction::NpcAlly,
                        "NEUTRAL" => PlayerReaction::Neutral,
                        "HOSTILE" => PlayerReaction::Hostile,
                        x => unimplemented!("{x} Player Reaction not implemented"),
                    };
                    let is_grouped_with_local_player = parse_bool(&tokens.pop_front().unwrap());

                    SegmentType::UnitAdded(UnitAdded {
                        unit_id: unit_id.parse().unwrap(),
                        unit_type,
                        is_local_player,
                        player_per_session_id: player_per_session_id.parse().unwrap(),
                        monster_id: monster_id.parse().unwrap(),
                        is_boss,
                        class,
                        race,
                        name: name.into(),
                        display_name: display_name.into(),
                        character_id: character_id.parse().unwrap(),
                        level: level.parse().unwrap(),
                        champion_points: champion_points.parse().unwrap(),
                        owner_unit_id: owner_unit_id.parse().unwrap(),
                        reaction,
                        is_grouped_with_local_player,
                    })
                }
                "TRIAL_INIT" => {
                    let id = tokens.pop_front().unwrap();
                    let in_progress = parse_bool(&tokens.pop_front().unwrap());
                    let completed = parse_bool(&tokens.pop_front().unwrap());
                    let start_time_ms = tokens.pop_front().unwrap();
                    let duration_ms = tokens.pop_front().unwrap();
                    let success = parse_bool(&tokens.pop_front().unwrap());
                    let final_score = tokens.pop_front().unwrap();

                    SegmentType::TrialInit(Trialinit {
                        id: id.parse().unwrap(),
                        in_progress,
                        completed,
                        start_time_ms: start_time_ms.parse().unwrap(),
                        duration_ms: duration_ms.parse().unwrap(),
                        success,
                        final_score: final_score.parse().unwrap(),
                    })
                }
                "ABILITY_INFO" => {
                    let ability_id = tokens.pop_front().unwrap();
                    let name = tokens.pop_front().unwrap();
                    let icon_path = tokens.pop_front().unwrap();
                    let interruptible = parse_bool(&tokens.pop_front().unwrap());
                    let blockable = parse_bool(&tokens.pop_front().unwrap());
                    SegmentType::AbilityInfo(AbilityInfo {
                        ability_id: ability_id.parse().unwrap(),
                        name: name.into(),
                        icon_path: PathBuf::from(icon_path),
                        interruptible,
                        blockable,
                    })
                }
                "MAP_CHANGED" => {
                    let id = tokens.pop_front().unwrap();
                    let name = tokens.pop_front().unwrap();
                    let texture_path = tokens.pop_front().unwrap();
                    SegmentType::MapInfo(MapInfo {
                        id: id.parse().unwrap(),
                        name: name.into(),
                        texture_path: PathBuf::from(texture_path),
                    })
                }
                "BEGIN_CAST" => {
                    let duration_ms = tokens.pop_front().unwrap();
                    let channeled = parse_bool(&tokens.pop_front().unwrap());
                    let cast_track_id = tokens.pop_front().unwrap();
                    let ability_id = tokens.pop_front().unwrap();
                    let source = UnitState::parse_source_unit(&mut tokens);
                    let target = Targets::parse_target_unit(&mut tokens);

                    SegmentType::BeginCast(BeginCast {
                        duration_ms: duration_ms.parse().unwrap(),
                        channeled,
                        cast_track_id: cast_track_id.parse().unwrap(),
                        ability_id: ability_id.parse().unwrap(),
                        source,
                        target,
                    })
                }
                "END_CAST" => {
                    let end_reason = match tokens.pop_front().unwrap().as_str() {
                        "COMPLETED" => EndReason::Completed,
                        "PLAYER_CANCELLED" => EndReason::PlayerCancelled,
                        "INTERRUPTED" => EndReason::Interrupted,
                        x => unimplemented!("{x} End Reason is not implemented"),
                    };
                    let cast_track_id = tokens.pop_front().unwrap().parse().unwrap();
                    let interrupting_ability_id = tokens.pop_front().map(|f| f.parse().unwrap());
                    let interrupting_unit_id = tokens.pop_front().map(|f| f.parse().unwrap());
                    SegmentType::EndCast(EndCast {
                        end_reason,
                        cast_track_id,
                        interrupting_ability_id,
                        interrupting_unit_id,
                    })
                }
                "EFFECT_INFO" => {
                    let ability_id = tokens.pop_front().unwrap().parse().unwrap();
                    let effect_type = match tokens.pop_front().unwrap().as_str() {
                        "BUFF" => EffectType::Buff,
                        "DEBUFF" => EffectType::Debuff,
                        x => unimplemented!("{x} Effect Type is not implemented"),
                    };
                    let status_effect_type = match tokens.pop_front().unwrap().as_str() {
                        "NONE" => StatusEffectType::None,
                        "MAGIC" => StatusEffectType::Magic,
                        "SNARE" => StatusEffectType::Snare,
                        "ROOT" => StatusEffectType::Root,
                        "BLEED" => StatusEffectType::Bleed,
                        "POISON" => StatusEffectType::Poison,
                        "ENVIRONMENT" => StatusEffectType::Environment,
                        x => unimplemented!("{x} Status Effect Type is not implemented"),
                    };
                    let effect_bar_display_behaviour = match tokens.pop_front().unwrap().as_str() {
                        "DEFAULT" => EffectBarDisplayBehaviour::Default,
                        "NEVER" => EffectBarDisplayBehaviour::Never,
                        x => unimplemented!("{x} Effect Bar Display behaviour is not implemented"),
                    };
                    let grants_synergy_ability_id = tokens.pop_front().map(|f| f.parse().unwrap());

                    SegmentType::EffectInfo(EffectInfo {
                        ability_id,
                        effect_type,
                        status_effect_type,
                        effect_bar_display_behaviour,
                        grants_synergy_ability_id,
                    })
                }
                "EFFECT_CHANGED" => {
                    let change_type = match tokens.pop_front().unwrap().as_str() {
                        "FADED" => EffectChangeType::Faded,
                        "GAINED" => EffectChangeType::Gained,
                        "UPDATED" => EffectChangeType::Updated,
                        x => unimplemented!("{x} Effect Change Type is not implemented"),
                    };
                    let stack_count = tokens.pop_front().unwrap().parse().unwrap();
                    let cast_track_id = tokens.pop_front().unwrap().parse().unwrap();
                    let ability_id = tokens.pop_front().unwrap().parse().unwrap();
                    let source = UnitState::parse_source_unit(&mut tokens);
                    let target = Targets::parse_target_unit(&mut tokens);
                    let player_initiated_remove_cast_track_id =
                        tokens.pop_front().map(|f| f.parse().unwrap());
                    SegmentType::EffectChanged(EffectChanged {
                        change_type,
                        stack_count,
                        cast_track_id,
                        ability_id,
                        source,
                        target,
                        player_initiated_remove_cast_track_id,
                    })
                }
                "COMBAT_EVENT" => {
                    let action_result =
                        ActionResult::parse_action_result(&tokens.pop_front().unwrap());
                    let damage_type = DamageType::parse_damage_type(&tokens.pop_front().unwrap());
                    let power_type = PowerType::parse_power_type(&tokens.pop_front().unwrap());
                    let hit_value = tokens.pop_front().unwrap().parse().unwrap();
                    let overflow = tokens.pop_front().unwrap().parse().unwrap();
                    let cast_track_id = tokens.pop_front().unwrap().parse().unwrap();
                    let ability_id = tokens.pop_front().unwrap().parse().unwrap();
                    let source = UnitState::parse_source_unit(&mut tokens);
                    let target = Targets::parse_target_unit(&mut tokens);

                    SegmentType::CombatEvent(CombatEvent {
                        action_result,
                        damage_type,
                        power_type,
                        hit_value,
                        overflow,
                        cast_track_id,
                        ability_id,
                        source,
                        target,
                    })
                }
                "UNIT_REMOVED" => SegmentType::UnitRemoved(UnitRemoved {
                    unit_id: tokens.pop_front().unwrap().parse().unwrap(),
                }),
                "HEALTH_REGEN" => {
                    let effective_regen = tokens.pop_front().unwrap().parse().unwrap();
                    let source = UnitState::parse_source_unit(&mut tokens);
                    SegmentType::HealthRegen(HealthRegen {
                        effective_regen,
                        source,
                    })
                }

                "UNIT_CHANGED" => {
                    let unit_id = tokens.pop_front().unwrap();
                    let class = Class::parse_class(&tokens.pop_front().unwrap());
                    let race = Race::parse_race(&tokens.pop_front().unwrap());
                    let name = tokens.pop_front().unwrap();
                    let display_name = tokens.pop_front().unwrap();
                    let character_id = tokens.pop_front().unwrap();
                    let level = tokens.pop_front().unwrap();
                    let champion_points = tokens.pop_front().unwrap();
                    let owner_unit_id = tokens.pop_front().unwrap();
                    let reaction = match tokens.pop_front().unwrap().as_str() {
                        "PLAYER_ALLY" => PlayerReaction::PlayerAlly,
                        "FRIENDLY" => PlayerReaction::Friendly,
                        "COMPANION" => PlayerReaction::Companion,
                        "NPC_ALLY" => PlayerReaction::NpcAlly,
                        "NEUTRAL" => PlayerReaction::Neutral,
                        "HOSTILE" => PlayerReaction::Hostile,
                        x => unimplemented!("{x} Player Reaction not implemented"),
                    };
                    let is_grouped_with_local_player = parse_bool(&tokens.pop_front().unwrap());

                    SegmentType::UnitChanged(UnitChanged {
                        unit_id: unit_id.parse().unwrap(),
                        class,
                        race,
                        name: name.into(),
                        display_name: display_name.into(),
                        character_id: character_id.parse().unwrap(),
                        level: level.parse().unwrap(),
                        champion_points: champion_points.parse().unwrap(),
                        owner_unit_id: owner_unit_id.parse().unwrap(),
                        reaction,
                        is_grouped_with_local_player,
                    })
                }
                "BEGIN_TRIAL" => {
                    let id = tokens.pop_front().unwrap().parse().unwrap();
                    let start_time_ms = tokens.pop_front().unwrap().parse().unwrap();

                    SegmentType::BeginTrial(BeginTrial { id, start_time_ms })
                }
                "END_TRIAL" => {
                    let id = tokens.pop_front().unwrap().parse().unwrap();
                    let duration_ms = tokens.pop_front().unwrap().parse().unwrap();
                    let success = parse_bool(&tokens.pop_front().unwrap());
                    let final_score = tokens.pop_front().unwrap().parse().unwrap();
                    let final_vitality_bonus = tokens.pop_front().unwrap().parse().unwrap();
                    SegmentType::EndTrial(EndTrial {
                        id,
                        duration_ms,
                        success,
                        final_score,
                        final_vitality_bonus,
                    })
                }

                "BEGIN_COMBAT" => SegmentType::BeginCombat,

                "END_COMBAT" => SegmentType::EndCombat,
                "PLAYER_INFO" => {
                    let unit_id = tokens.pop_front().unwrap().parse().unwrap();
                    let lteid = Self::tokenize(tokens.pop_front().unwrap().as_str());
                    let ltestack = Self::tokenize(tokens.pop_front().unwrap().as_str());
                    let long_term_effect = lteid
                        .iter()
                        .zip(ltestack)
                        .map(|(a, s)| Effect {
                            ability_id: a.parse().unwrap(),
                            stack_count: s.parse().unwrap(),
                        })
                        .collect();
                    let mut equipment_info = Equipment::default();
                    for equipment_tokens in Self::tokenize(tokens.pop_front().unwrap().as_str()) {
                        let mut equipment_piece_tokens: VecDeque<_> =
                            Self::tokenize(&equipment_tokens).into();
                        match equipment_piece_tokens.pop_front().unwrap().as_str() {
                            "HEAD" => {
                                equipment_info.head = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "CHEST" => {
                                equipment_info.chest = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }

                            "NECK" => {
                                equipment_info.neck = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }

                            "SHOULDERS" => {
                                equipment_info.shoulders = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }

                            "WAIST" => {
                                equipment_info.waist = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "LEGS" => {
                                equipment_info.legs = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "FEET" => {
                                equipment_info.feet = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "COSTUME" => (),
                            "RING1" => {
                                equipment_info.ring1 = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "RING2" => {
                                equipment_info.ring2 = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "HAND" => {
                                equipment_info.hand = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }
                            "POISON" => {
                                equipment_info.main_poison = Some(EquipmentInfo::parse_equipment(
                                    &mut equipment_piece_tokens,
                                ));
                            }

                            x if ["MAIN_HAND", "OFF_HAND"].contains(&x) => {
                                equipment_info.main = Some(WeaponHand::parse_weapon(
                                    x,
                                    &mut equipment_piece_tokens,
                                    equipment_info.main,
                                ));
                            }
                            x if ["BACKUP_MAIN"].contains(&x) => {
                                equipment_info.backup = Some(WeaponHand::parse_weapon(
                                    x,
                                    &mut equipment_piece_tokens,
                                    equipment_info.backup,
                                ));
                            }

                            slot => unimplemented!(
                                "{slot} Equipment Slot not implemented {equipment_piece_tokens:#?}"
                            ),
                        }
                    }

                    let mut primary_ability_id = Self::tokenize(&tokens.pop_front().unwrap())
                        .iter()
                        .map(|v| v.parse().unwrap())
                        .collect::<Vec<usize>>();
                    primary_ability_id.resize(6, 0);
                    let mut backup_ability_id = Self::tokenize(&tokens.pop_front().unwrap())
                        .iter()
                        .map(|v| v.parse().unwrap())
                        .collect::<Vec<usize>>();
                    backup_ability_id.resize(6, 0);
                    SegmentType::PlayerInfo(Box::new(PlayerInfo {
                        unit_id,
                        long_term_effect,
                        equipment_info,
                        primary_ability_id: primary_ability_id.try_into().unwrap(),
                        backup_ability_id: backup_ability_id.try_into().unwrap(),
                    }))
                }
                x => {
                    todo!("{x} is not implemented!(): {:#?}", tokens);
                }
            };
            Some(Segment { time, line })
        } else {
            None
        }
    }
}
