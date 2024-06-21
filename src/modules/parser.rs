use std::{io::Lines, path::PathBuf, str::Split, sync::Arc};

//<unitState> refers to the following fields for a unit: unitId, health/max, magicka/max, stamina/max, ultimate/max, werewolf/max, shield, map NX, map NY, headingRadians.
#[derive(Debug)]
pub struct UnitState {
    unit_id: usize,
    health: (usize, usize),
    magicka: (usize, usize),
    stamina: (usize, usize),
    ultimate: (u16, u16),
    werewolf: (u16, u16),
    shield: usize,
    position: (f32, f32),
    heading: f32,
}

//<targetUnitState> is replaced with an asterisk if the source and target are the same.
pub type TargetUnitState = UnitState;

//<equipmentInfo> refers to the following fields for a piece of equipment: slot, id, isCP, level, trait, displayQuality, setId, enchantType, isEnchantCP, enchantLevel, enchantQuality.
#[derive(Debug)]
pub struct Equipment {
    head: Option<EquipmentInfo<ArmorTrait>>,
    shoulders: Option<EquipmentInfo<ArmorTrait>>,
    chest: Option<EquipmentInfo<ArmorTrait>>,
    hand: Option<EquipmentInfo<ArmorTrait>>,
    waist: Option<EquipmentInfo<ArmorTrait>>,
    legs: Option<EquipmentInfo<ArmorTrait>>,
    feet: Option<EquipmentInfo<ArmorTrait>>,
    neck: Option<EquipmentInfo<JewelTrait>>,
    ring1: Option<EquipmentInfo<JewelTrait>>,
    ring2: Option<EquipmentInfo<JewelTrait>>,
    main: Option<WeaponHand>,
    backup: Option<WeaponHand>,
}

#[derive(Debug)]
pub enum WeaponHand {
    OneHand(
        Option<EquipmentInfo<WeaponTrait>>,
        Option<EquipmentInfo<WeaponTrait>>,
    ),
    TwoHand(Option<EquipmentInfo<WeaponTrait>>),
}

#[derive(Debug)]
pub enum EquipmentLevel {
    NoCp(u8),
    Cp(u8),
}

#[derive(Debug)]
pub enum ArmorTrait {
    Divines,
    Invigorating,
    Impenetrable,
    Infused,
    Nirnhoned,
    Reinforced,
    Sturdy,
    WellFitted,
    Training,
}

#[derive(Debug)]
pub enum JewelTrait {
    Arcane,
    Health,
    Robust,
    Harmony,
    Infused,
    Bloodthirsty,
    Protective,
    Swift,
    Triune,
}

#[derive(Debug)]
pub enum WeaponTrait {
    Charged,
    Defending,
    Infused,
    Nirnhoned,
    Powered,
    Precise,
    Sharpened,
    Training,
    Decisive,
}

trait r#TraitMarker {}
impl r#TraitMarker for ArmorTrait {}
impl r#TraitMarker for JewelTrait {}
impl r#TraitMarker for WeaponTrait {}

#[derive(Debug)]
pub enum ArmorEnchant {}

#[derive(Debug)]
pub enum Quality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
}

#[derive(Debug)]
pub struct EquipmentInfo<T>
where
    T: TraitMarker,
{
    id: usize,
    level: EquipmentLevel,
    r#trait: T,
    display_quality: Quality,
    set_id: usize,
    enchant_type: usize,
    enchant_level: EquipmentLevel,
    enchant_quality: Quality,
}

#[derive(Debug)]
pub struct BeginLog {
    time_since_epoch_ms: usize,
    log_version: usize,
    realm_name: Arc<str>,
    language: Arc<str>,
    game_version: Arc<str>,
}

#[derive(Debug)]
pub struct Effect {
    ability_id: usize,
    stack_count: u8,
}

#[derive(Debug)]
pub struct PlayerInfo {
    unit_id: usize,
    long_term_effect: Vec<Effect>,
    equipment_info: Equipment,
    primary_ability_id: [usize; 6],
    backup_ability_id: [usize; 6],
}

#[derive(Debug)]
pub enum Targets {
    SelfTarget,
    Target(UnitState),
    None,
}

#[derive(Debug)]
pub struct BeginCast {
    duration_ms: usize,
    channeled: bool,
    cast_track_id: usize,
    ability_id: usize,
    source: UnitState,
    target: Targets,
}

#[derive(Debug)]
pub enum EndReason {
    Completed,
}
#[derive(Debug)]
pub struct EndCast {
    end_reason: EndReason,
    cast_track_id: usize,
    interrupting_ability_id: Option<usize>,
    interrupting_unit_id: Option<usize>,
}
// pub struct COMBAT_EVENT{actionResult, damageType, powerType, hitValue, overflow, castTrackId, abilityId, <sourceUnitState>, <targetUnitState>}
// pub struct HEALTH_REGEN{effectiveRegen, <unitState>}
#[derive(Debug)]
pub enum UnitType {
    Player,
}

#[derive(Debug)]
pub enum Race {
    DarkElf = 4,
    Khajit = 9,
    HighElf = 7,
    WoodElf = 8,
    Nord = 5,
    Redguard = 2,
}

#[derive(Debug)]
pub enum Class {
    Arcanist = 117,
    Templar = 6,
    DragonKnight = 1,
    Sorcerer = 2,
}

#[derive(Debug)]
pub enum PlayerReaction {
    PlayerAlly,
}

#[derive(Debug)]
pub struct UnitAdded {
    unit_id: usize,
    unit_type: UnitType,
    is_local_player: bool,
    player_per_session_id: u8,
    monster_id: usize,
    is_boss: bool,
    class: Class,
    race: Race,
    name: Arc<str>,
    display_name: Arc<str>,
    character_id: usize,
    level: usize,
    champion_points: u16,
    owner_unit_id: usize,
    reaction: PlayerReaction,
    is_grouped_with_local_player: bool,
}
// pub struct UNIT_CHANGED{unitId, classId, raceId, name, displayName, characterId, level, championPoints, ownerUnitId, reaction, isGroupedWithLocalPlayer}
// pub struct UNIT_REMOVED{unitId}
// pub struct EFFECT_CHANGED{changeType, stackCount, castTrackId, abilityId, <sourceUnitState>, <targetUnitState>, playerInitiatedRemoveCastTrackId:optional}

#[derive(Debug)]
pub struct AbilityInfo {
    ability_id: usize,
    name: Arc<str>,
    icon_path: PathBuf,
    interruptible: bool,
    blockable: bool,
}
#[derive(Debug)]
pub enum EffectType {
    Buff,
}

#[derive(Debug)]
pub enum StatusEffectType {
    None,
}
#[derive(Debug)]
pub enum EffectBarDisplayBehaviour {
    Default,
}
#[derive(Debug)]
pub struct EffectInfo {
    ability_id: usize,
    effect_type: EffectType,
    status_effect_type: StatusEffectType,
    effect_bar_display_behaviour: EffectBarDisplayBehaviour,
    grants_synergy_ability_id: Option<usize>,
}
#[derive(Debug)]
pub struct MapInfo {
    id: usize,
    name: Arc<str>,
    texture_path: PathBuf,
}
#[derive(Debug)]
pub enum DungeonDifficulty {
    Normal,
    Veteran,
}

#[derive(Debug)]
pub struct ZoneInfo {
    id: usize,
    name: Arc<str>,
    dungeon_difficulty: DungeonDifficulty,
}

#[derive(Debug)]
pub struct Trialinit {
    id: u8,
    in_progress: bool,
    completed: bool,
    start_time_ms: usize,
    duration_ms: usize,
    success: bool,
    final_score: usize,
}
// pub struct BEGIN_TRIAL{id, startTimeMS}
// pub struct END_TRIAL{id, durationMS, success, finalScore, finalVitalityBonus }

#[derive(Debug)]
pub enum SegmentType {
    BeginLog(BeginLog),
    // EndLog,
    // BeginCombat,
    // EndCombat,
    // PlayerInfo(PlayerInfo),
    BeginCast(BeginCast),
    EndCast(EndCast),
    // CombatEvent,
    // HealthRegen,
    UnitAdded(UnitAdded),
    // UnitChanged,
    // UnitRemoved,
    // EffectChanged,
    AbilityInfo(AbilityInfo),
    EffectInfo(EffectInfo),
    MapInfo(MapInfo),
    ZoneInfo(ZoneInfo),
    TrialInit(Trialinit),
    // BeginTrial,
    // EndTrial,
}
#[derive(Debug)]
pub struct Segment {
    time: usize, // Time Since Logging Began in MS
    line: SegmentType,
}

pub struct Lexer {
    data: Vec<Arc<str>>,
    current_line: usize,
}

impl Lexer {
    pub fn new(data: Lines<&[u8]>) -> Lexer {
        Lexer {
            data: data
                .filter_map(|l| match l {
                    Ok(v) => Some(v.as_str().into()),
                    Err(_) => None,
                })
                .collect::<Vec<_>>(),
            current_line: 0,
        }
    }

    fn parse_bool(b: &str) -> bool {
        match b {
            "T" => true,
            "F" => false,
            _ => unreachable!(),
        }
    }

    fn parse_class(d: &str) -> Class {
        match d {
            "117" => Class::Arcanist,
            "6" => Class::Templar,
            "1" => Class::DragonKnight,
            "2" => Class::Sorcerer,
            x => unimplemented!("{x} class is not implemented"),
        }
    }

    fn parse_race(d: &str) -> Race {
        match d {
            "4" => Race::DarkElf,
            "9" => Race::Khajit,
            "7" => Race::HighElf,
            "8" => Race::WoodElf,
            "5" => Race::Nord,
            "2" => Race::Redguard,
            x => unimplemented!("{x} race is not implemented"),
        }
    }
    fn parse_source_unit(splits: &mut Split<'_, char>) -> UnitState {
        let unit_id = splits.next().unwrap();
        let mut health_ = splits.next().unwrap().split('/');
        let health = (
            health_.next().unwrap().parse().unwrap(),
            health_.next().unwrap().parse().unwrap(),
        );
        let mut magicka_ = splits.next().unwrap().split('/');
        let magicka = (
            magicka_.next().unwrap().parse().unwrap(),
            magicka_.next().unwrap().parse().unwrap(),
        );
        let mut stamina_ = splits.next().unwrap().split('/');
        let stamina = (
            stamina_.next().unwrap().parse().unwrap(),
            stamina_.next().unwrap().parse().unwrap(),
        );

        let mut ultimate_ = splits.next().unwrap().split('/');
        let ultimate = (
            ultimate_.next().unwrap().parse().unwrap(),
            ultimate_.next().unwrap().parse().unwrap(),
        );

        let mut werewolf_ = splits.next().unwrap().split('/');
        let werewolf = (
            werewolf_.next().unwrap().parse().unwrap(),
            werewolf_.next().unwrap().parse().unwrap(),
        );
        let shield = splits.next().unwrap().parse().unwrap();
        let position_ = splits.next().unwrap();
        let position = (
            position_.parse().unwrap(),
            splits.next().unwrap().parse().unwrap(),
        );
        let heading = splits.next().unwrap().parse().unwrap();

        UnitState {
            unit_id: unit_id.parse().unwrap(),
            health,
            magicka,
            stamina,
            ultimate,
            werewolf,
            shield,
            position,
            heading,
        }
    }

    fn parse_target_unit(splits: &mut Split<'_, char>) -> Targets {
        let unit_id = splits.next().unwrap();
        if unit_id == "0" {
            return Targets::None;
        } else if unit_id == "*" {
            return Targets::SelfTarget;
        }
        let mut health_ = splits.next().unwrap().split('/');
        let health = (
            health_.next().unwrap().parse().unwrap(),
            health_.next().unwrap().parse().unwrap(),
        );
        let mut magicka_ = splits.next().unwrap().split('/');
        let magicka = (
            magicka_.next().unwrap().parse().unwrap(),
            magicka_.next().unwrap().parse().unwrap(),
        );
        let mut stamina_ = splits.next().unwrap().split('/');
        let stamina = (
            stamina_.next().unwrap().parse().unwrap(),
            stamina_.next().unwrap().parse().unwrap(),
        );

        let mut ultimate_ = splits.next().unwrap().split('/');
        let ultimate = (
            ultimate_.next().unwrap().parse().unwrap(),
            ultimate_.next().unwrap().parse().unwrap(),
        );

        let mut werewolf_ = splits.next().unwrap().split('/');
        let werewolf = (
            werewolf_.next().unwrap().parse().unwrap(),
            werewolf_.next().unwrap().parse().unwrap(),
        );
        let shield = splits.next().unwrap().parse().unwrap();
        let position_ = splits.next().unwrap();
        let position = (
            position_.parse().unwrap(),
            splits.next().unwrap().parse().unwrap(),
        );
        let heading = splits.next().unwrap().parse().unwrap();

        Targets::Target(UnitState {
            unit_id: unit_id.parse().unwrap(),
            health,
            magicka,
            stamina,
            ultimate,
            werewolf,
            shield,
            position,
            heading,
        })
    }
    pub fn next_segment(&mut self) -> Option<Segment> {
        let data = &self.data[self.current_line];
        if data.is_empty() {
            return None;
        }
        self.current_line += 1;
        let mut split = data.splitn(3, ',');
        let time = split.next().unwrap();
        let token = split.next().unwrap();
        match token {
            "BEGIN_LOG" => {
                let mut remainder = split.next().unwrap().split(',');
                let time_since_epoch_ms = remainder.next().unwrap();
                let log_version = remainder.next().unwrap();
                let realm_name = remainder.next().unwrap();
                let language = remainder.next().unwrap();
                let game_version = remainder.next().unwrap();
                let line = SegmentType::BeginLog(BeginLog {
                    time_since_epoch_ms: time_since_epoch_ms.parse().unwrap(),
                    log_version: log_version.parse().unwrap(),
                    realm_name: realm_name.replace('"', "").into(),
                    language: language.replace('"', "").into(),
                    game_version: game_version.replace('"', "").into(),
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "ZONE_CHANGED" => {
                let mut remainder = split.next().unwrap().split(',');
                let zone_id = remainder.next().unwrap();
                let zone_name = remainder.next().unwrap();
                let mode = match remainder.next().unwrap() {
                    "VETERAN" => DungeonDifficulty::Veteran,
                    "NORMAL" => DungeonDifficulty::Normal,
                    _ => DungeonDifficulty::Normal,
                };
                let line = SegmentType::ZoneInfo(ZoneInfo {
                    id: zone_id.parse().unwrap(),
                    name: zone_name.replace('"', "").into(),
                    dungeon_difficulty: mode,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "UNIT_ADDED" => {
                let mut remainder = split.next().unwrap().split(',');
                let unit_id = remainder.next().unwrap();
                let unit_type = match remainder.next().unwrap() {
                    "PLAYER" => UnitType::Player,
                    _ => unimplemented!(),
                };
                let is_local_player = Self::parse_bool(remainder.next().unwrap());
                let player_per_session_id = remainder.next().unwrap();
                let monster_id = remainder.next().unwrap();
                let is_boss = Self::parse_bool(remainder.next().unwrap());
                let class = Self::parse_class(remainder.next().unwrap());
                let race = Self::parse_race(remainder.next().unwrap());
                let name = remainder.next().unwrap();
                let display_name = remainder.next().unwrap();
                let character_id = remainder.next().unwrap();
                let level = remainder.next().unwrap();
                let champion_points = remainder.next().unwrap();
                let owner_unit_id = remainder.next().unwrap();
                let reaction = match remainder.next().unwrap() {
                    "PLAYER_ALLY" => PlayerReaction::PlayerAlly,
                    _ => unimplemented!(),
                };
                let is_grouped_with_local_player = Self::parse_bool(remainder.next().unwrap());

                let line = SegmentType::UnitAdded(UnitAdded {
                    unit_id: unit_id.parse().unwrap(),
                    unit_type,
                    is_local_player,
                    player_per_session_id: player_per_session_id.parse().unwrap(),
                    monster_id: monster_id.parse().unwrap(),
                    is_boss,
                    class,
                    race,
                    name: name.replace('"', "").into(),
                    display_name: display_name.replace('"', "").into(),
                    character_id: character_id.parse().unwrap(),
                    level: level.parse().unwrap(),
                    champion_points: champion_points.parse().unwrap(),
                    owner_unit_id: owner_unit_id.parse().unwrap(),
                    reaction,
                    is_grouped_with_local_player,
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "TRIAL_INIT" => {
                let mut remainder = split.next().unwrap().split(',');
                let id = remainder.next().unwrap();
                let in_progress = Self::parse_bool(remainder.next().unwrap());
                let completed = Self::parse_bool(remainder.next().unwrap());
                let start_time_ms = remainder.next().unwrap();
                let duration_ms = remainder.next().unwrap();
                let success = Self::parse_bool(remainder.next().unwrap());
                let final_score = remainder.next().unwrap();

                let line = SegmentType::TrialInit(Trialinit {
                    id: id.parse().unwrap(),
                    in_progress,
                    completed,
                    start_time_ms: start_time_ms.parse().unwrap(),
                    duration_ms: duration_ms.parse().unwrap(),
                    success,
                    final_score: final_score.parse().unwrap(),
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "ABILITY_INFO" => {
                let mut remainder = split.next().unwrap().split(',');
                let ability_id = remainder.next().unwrap();
                let name = remainder.next().unwrap();
                let icon_path = remainder.next().unwrap();
                let interruptible = Self::parse_bool(remainder.next().unwrap());
                let blockable = Self::parse_bool(remainder.next().unwrap());
                let line = SegmentType::AbilityInfo(AbilityInfo {
                    ability_id: ability_id.parse().unwrap(),
                    name: name.replace('"', "").into(),
                    icon_path: PathBuf::from(icon_path.replace('"', "")),
                    interruptible,
                    blockable,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "MAP_CHANGED" => {
                let mut remainder = split.next().unwrap().split(',');
                let id = remainder.next().unwrap();
                let name = remainder.next().unwrap();
                let texture_path = remainder.next().unwrap();
                let line = SegmentType::MapInfo(MapInfo {
                    id: id.parse().unwrap(),
                    name: name.replace('"', "").into(),
                    texture_path: PathBuf::from(texture_path.replace('"', "")),
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "BEGIN_CAST" => {
                let mut remainder = split.next().unwrap().split(',');
                let duration_ms = remainder.next().unwrap();
                let channeled = Self::parse_bool(remainder.next().unwrap());
                let cast_track_id = remainder.next().unwrap();
                let ability_id = remainder.next().unwrap();
                let source = Self::parse_source_unit(&mut remainder);
                let target = Self::parse_target_unit(&mut remainder);

                let line = SegmentType::BeginCast(BeginCast {
                    duration_ms: duration_ms.parse().unwrap(),
                    channeled,
                    cast_track_id: cast_track_id.parse().unwrap(),
                    ability_id: ability_id.parse().unwrap(),
                    source,
                    target,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "END_CAST" => {
                let mut remainder = split.next().unwrap().split(',');
                let end_reason = match remainder.next().unwrap() {
                    "COMPLETED" => EndReason::Completed,
                    x => unimplemented!("{x} End Reason is not implemented"),
                };
                let cast_track_id = remainder.next().unwrap().parse().unwrap();
                let interrupting_ability_id = remainder.next().map(|f| f.parse().unwrap());
                let interrupting_unit_id = remainder.next().map(|f| f.parse().unwrap());
                let line = SegmentType::EndCast(EndCast {
                    end_reason,
                    cast_track_id,
                    interrupting_ability_id,
                    interrupting_unit_id,
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "EFFECT_INFO" => {
                let mut remainder = split.next().unwrap().split(',');
                let ability_id = remainder.next().unwrap().parse().unwrap();
                let effect_type = match remainder.next().unwrap() {
                    "BUFF" => EffectType::Buff,
                    x => unimplemented!("{x} Effect Type is not implemented"),
                };
                let status_effect_type = match remainder.next().unwrap() {
                    "NONE" => StatusEffectType::None,
                    x => unimplemented!("{x} Status Effect Type is not implemented"),
                };
                let effect_bar_display_behaviour = match remainder.next().unwrap() {
                    "DEFAULT" => EffectBarDisplayBehaviour::Default,
                    x => unimplemented!("{x} Effect Bar Display behaviour is not implemented"),
                };
                let grants_synergy_ability_id = remainder.next().map(|f| f.parse().unwrap());

                let line = SegmentType::EffectInfo(EffectInfo {
                    ability_id,
                    effect_type,
                    status_effect_type,
                    effect_bar_display_behaviour,
                    grants_synergy_ability_id,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            x => {
                todo!("{x} is not implemented!(): {}", split.next().unwrap());
            }
        }
    }
}
