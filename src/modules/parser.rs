use std::{collections::VecDeque, io::Lines, path::PathBuf, sync::Arc};

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

#[derive(Debug)]
pub enum ActionResult {
    AbilityOnCooldown,
    Absorbed,
    BadTarget,
    Bladeturn,
    Blocked,
    BlockedDamage,
    Busy,
    CannotUse,
    CantSeeTarget,
    CantSwapHotbarIsOverridden,
    CantSwapWhileChangingGear,
    CasterDead,
    Charmed,
    CriticalDamage,
    CriticalHeal,
    Damage,
    DamageShielded,
    Defended,
    Died,
    DiedCompanionXp,
    DiedXp,
    Disarmed,
    Disoriented,
    Dodged,
    DotTick,
    DotTickCritical,
    Failed,
    FailedRequirements,
    FailedSiegeCreationRequirements,
    Falling,
    FallDamage,
    Feared,
    GraveyardDisallowedInInstance,
    GraveyardTooClose,
    Heal,
    HealAbsorbed,
    HotTick,
    HotTickCritical,
    Immune,
    InsufficientResource,
    Intercepted,
    Interrupt,
    Invalid,
    InvalidFixture,
    InvalidJusticeTarget,
    InvalidTerrain,
    InAir,
    InCombat,
    InEnemyKeep,
    InEnemyOutpost,
    InEnemyResource,
    InEnemyTown,
    InHideyhole,
    KilledByDaedricWeapon,
    KilledBySubzone,
    KillingBlow,
    Knockback,
    Levitated,
    MercenaryLimit,
    Miss,
    MissingEmptySoulGem,
    MissingFilledSoulGem,
    MobileGraveyardLimit,
    Mounted,
    MustBeInOwnKeep,
    NotEnoughInventorySpace,
    NotEnoughInventorySpaceSoulGem,
    NotEnoughSpaceForSiege,
    NoLocationFound,
    NoRamAttackableTargetWithinRange,
    NoWeaponsToSwapTo,
    NpcTooClose,
    Offbalance,
    Pacified,
    Parried,
    PartialResist,
    PowerDrain,
    PowerEnergize,
    PreciseDamage,
    Queued,
    RamAttackableTargetsAllDestroyed,
    RamAttackableTargetsAllOccupied,
    Recalling,
    Reflected,
    Reincarnating,
    Resist,
    Resurrect,
    Rooted,
    SelfPlayingTribute,
    SiegeLimit,
    SiegeNotAllowedInZone,
    SiegeTooClose,
    Silenced,
    Snared,
    SoulGemResurrectionAccepted,
    Sprinting,
    Staggered,
    Stunned,
    Swimming,
    TargetDead,
    TargetNotInView,
    TargetNotPvpFlagged,
    TargetOutOfRange,
    TargetPlayingTribute,
    TargetTooClose,
    UnevenTerrain,
    Weaponswap,
    WreckingDamage,
    WrongWeapon,
}

#[derive(Debug)]
pub enum DamageType {
    Bleed,
    Cold,
    Disease,
    Drown,
    Earth,
    Fire,
    Generic,
    Magic,
    None,
    Oblivion,
    Physical,
    Poison,
    Shock,
}

#[derive(Debug)]
pub enum PowerType {
    Adrenaline = 8,
    Charges = 5,
    Combo = 3,
    Fervor = 2,
    Finesse = 9,
    Health = -2,
    Invalid = -1,
    Magicka = 0,
    Momentum = 7,
    MountStamina = 11,
    Power = 4,
    Stamina = 6,
    Ultimate = 10,
    Werewolf = 1,
}
#[derive(Debug)]
pub struct CombatEvent {
    action_result: ActionResult,
    damage_type: DamageType,
    power_type: PowerType,
    hit_value: usize,
    overflow: usize,
    cast_track_id: usize,
    ability_id: usize,
    source: UnitState,
    target: Targets,
}
#[derive(Debug)]
pub struct HealthRegen {
    effective_regen: usize,
    source: UnitState,
}
#[derive(Debug)]
pub enum UnitType {
    Player,
    Monster,
}

#[derive(Debug)]
pub enum Race {
    DarkElf = 4,
    Khajit = 9,
    HighElf = 7,
    WoodElf = 8,
    Nord = 5,
    Redguard = 2,
    None = 0,
}

#[derive(Debug)]
pub enum Class {
    Arcanist = 117,
    Templar = 6,
    DragonKnight = 1,
    Sorcerer = 2,

    None = 0,
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
#[derive(Debug)]
pub struct UnitChanged {
    unit_id: usize,
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
#[derive(Debug)]
pub struct UnitRemoved {
    unit_id: usize,
}
#[derive(Debug)]
pub enum EffectChangeType {
    Faded,
    Gained,
    Updated,
}
#[derive(Debug)]
pub struct EffectChanged {
    change_type: EffectChangeType,
    stack_count: u8,
    cast_track_id: usize,
    ability_id: usize,
    source: UnitState,
    target: Targets,
    player_initiated_remove_cast_track_id: Option<usize>,
}

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
    Debuff,
}

#[derive(Debug)]
pub enum StatusEffectType {
    None,
}
#[derive(Debug)]
pub enum EffectBarDisplayBehaviour {
    Default,
    Never,
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

#[derive(Debug)]
pub struct BeginTrial {
    id: u8,
    start_time_ms: usize,
}
// pub struct END_TRIAL{id, durationMS, success, finalScore, finalVitalityBonus }

#[derive(Debug)]
pub enum SegmentType {
    BeginLog(BeginLog),
    // EndLog,
    BeginCombat,
    // EndCombat,
    // PlayerInfo(PlayerInfo),
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
            x => unreachable!("{x} should be unreachable!"),
        }
    }

    fn parse_class(d: &str) -> Class {
        match d {
            "117" => Class::Arcanist,
            "6" => Class::Templar,
            "1" => Class::DragonKnight,
            "2" => Class::Sorcerer,
            "0" => Class::None,
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
            "0" => Race::None,
            x => unimplemented!("{x} race is not implemented"),
        }
    }
    fn parse_source_unit(tokens: &mut VecDeque<String>) -> UnitState {
        let unit_id = tokens.pop_front().unwrap();
        let binding = tokens.pop_front().unwrap();
        let mut health_ = binding.split('/');
        let health = (
            health_.next().unwrap().parse().unwrap(),
            health_.next().unwrap().parse().unwrap(),
        );
        let binding = tokens.pop_front().unwrap();
        let mut magicka_ = binding.split('/');
        let magicka = (
            magicka_.next().unwrap().parse().unwrap(),
            magicka_.next().unwrap().parse().unwrap(),
        );
        let binding = tokens.pop_front().unwrap();
        let mut stamina_ = binding.split('/');
        let stamina = (
            stamina_.next().unwrap().parse().unwrap(),
            stamina_.next().unwrap().parse().unwrap(),
        );

        let binding = tokens.pop_front().unwrap();
        let mut ultimate_ = binding.split('/');
        let ultimate = (
            ultimate_.next().unwrap().parse().unwrap(),
            ultimate_.next().unwrap().parse().unwrap(),
        );

        let binding = tokens.pop_front().unwrap();
        let mut werewolf_ = binding.split('/');
        let werewolf = (
            werewolf_.next().unwrap().parse().unwrap(),
            werewolf_.next().unwrap().parse().unwrap(),
        );
        let shield = tokens.pop_front().unwrap().parse().unwrap();
        let position_ = tokens.pop_front().unwrap();
        let position = (
            position_.parse().unwrap(),
            tokens.pop_front().unwrap().parse().unwrap(),
        );
        let heading = tokens.pop_front().unwrap().parse().unwrap();

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

    fn parse_target_unit(tokens: &mut VecDeque<String>) -> Targets {
        let unit_id = tokens.pop_front().unwrap();
        if unit_id == "0" {
            return Targets::None;
        } else if unit_id == "*" {
            return Targets::SelfTarget;
        }
        let binding = tokens.pop_front().unwrap();
        let mut health_ = binding.split('/');
        let health = (
            health_.next().unwrap().parse().unwrap(),
            health_.next().unwrap().parse().unwrap(),
        );
        let binding = tokens.pop_front().unwrap();
        let mut magicka_ = binding.split('/');
        let magicka = (
            magicka_.next().unwrap().parse().unwrap(),
            magicka_.next().unwrap().parse().unwrap(),
        );
        let binding = tokens.pop_front().unwrap();
        let mut stamina_ = binding.split('/');
        let stamina = (
            stamina_.next().unwrap().parse().unwrap(),
            stamina_.next().unwrap().parse().unwrap(),
        );

        let binding = tokens.pop_front().unwrap();
        let mut ultimate_ = binding.split('/');
        let ultimate = (
            ultimate_.next().unwrap().parse().unwrap(),
            ultimate_.next().unwrap().parse().unwrap(),
        );

        let binding = tokens.pop_front().unwrap();
        let mut werewolf_ = binding.split('/');
        let werewolf = (
            werewolf_.next().unwrap().parse().unwrap(),
            werewolf_.next().unwrap().parse().unwrap(),
        );
        let shield = tokens.pop_front().unwrap().parse().unwrap();
        let position_ = tokens.pop_front().unwrap();
        let position = (
            position_.parse().unwrap(),
            tokens.pop_front().unwrap().parse().unwrap(),
        );
        let heading = tokens.pop_front().unwrap().parse().unwrap();

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
    fn parse_action_result(data: &str) -> ActionResult {
        match data {
            "ABILITY_ON_COOLDOWN" => ActionResult::AbilityOnCooldown,
            "ABSORBED" => ActionResult::Absorbed,
            "BAD_TARGET" => ActionResult::BadTarget,
            "BLADETURN" => ActionResult::Bladeturn,
            "BLOCKED" => ActionResult::Blocked,
            "BLOCKED_DAMAGE" => ActionResult::BlockedDamage,
            "BUSY" => ActionResult::Busy,
            "CANNOT_USE" => ActionResult::CannotUse,
            "CANT_SEE_TARGET" => ActionResult::CantSeeTarget,
            "CANT_SWAP_HOTBAR_IS_OVERRIDDEN" => ActionResult::CantSwapHotbarIsOverridden,
            "CANT_SWAP_WHILE_CHANGING_GEAR" => ActionResult::CantSwapWhileChangingGear,
            "CASTER_DEAD" => ActionResult::CasterDead,
            "CHARMED" => ActionResult::Charmed,
            "CRITICAL_DAMAGE" => ActionResult::CriticalDamage,
            "CRITICAL_HEAL" => ActionResult::CriticalHeal,
            "DAMAGE" => ActionResult::Damage,
            "DAMAGE_SHIELDED" => ActionResult::DamageShielded,
            "DEFENDED" => ActionResult::Defended,
            "DIED" => ActionResult::Died,
            "DIED_COMPANION_XP" => ActionResult::DiedCompanionXp,
            "DIED_XP" => ActionResult::DiedXp,
            "DISARMED" => ActionResult::Disarmed,
            "DISORIENTED" => ActionResult::Disoriented,
            "DODGED" => ActionResult::Dodged,
            "DOT_TICK" => ActionResult::DotTick,
            "DOT_TICK_CRITICAL" => ActionResult::DotTickCritical,
            "FAILED" => ActionResult::Failed,
            "FAILED_REQUIREMENTS" => ActionResult::FailedRequirements,
            "FAILED_SIEGE_CREATION_REQUIREMENTS" => ActionResult::FailedSiegeCreationRequirements,
            "FALLING" => ActionResult::Falling,
            "FALL_DAMAGE" => ActionResult::FallDamage,
            "FEARED" => ActionResult::Feared,
            "GRAVEYARD_DISALLOWED_IN_INSTANCE" => ActionResult::GraveyardDisallowedInInstance,
            "GRAVEYARD_TOO_CLOSE" => ActionResult::GraveyardTooClose,
            "HEAL" => ActionResult::Heal,
            "HEAL_ABSORBED" => ActionResult::HealAbsorbed,
            "HOT_TICK" => ActionResult::HotTick,
            "HOT_TICK_CRITICAL" => ActionResult::HotTickCritical,
            "IMMUNE" => ActionResult::Immune,
            "INSUFFICIENT_RESOURCE" => ActionResult::InsufficientResource,
            "INTERCEPTED" => ActionResult::Intercepted,
            "INTERRUPT" => ActionResult::Interrupt,
            "INVALID" => ActionResult::Invalid,
            "INVALID_FIXTURE" => ActionResult::InvalidFixture,
            "INVALID_JUSTICE_TARGET" => ActionResult::InvalidJusticeTarget,
            "INVALID_TERRAIN" => ActionResult::InvalidTerrain,
            "IN_AIR" => ActionResult::InAir,
            "IN_COMBAT" => ActionResult::InCombat,
            "IN_ENEMY_KEEP" => ActionResult::InEnemyKeep,
            "IN_ENEMY_OUTPOST" => ActionResult::InEnemyOutpost,
            "IN_ENEMY_RESOURCE" => ActionResult::InEnemyResource,
            "IN_ENEMY_TOWN" => ActionResult::InEnemyTown,
            "IN_HIDEYHOLE" => ActionResult::InHideyhole,
            "KILLED_BY_DAEDRIC_WEAPON" => ActionResult::KilledByDaedricWeapon,
            "KILLED_BY_SUBZONE" => ActionResult::KilledBySubzone,
            "KILLING_BLOW" => ActionResult::KillingBlow,
            "KNOCKBACK" => ActionResult::Knockback,
            "LEVITATED" => ActionResult::Levitated,
            "MERCENARY_LIMIT" => ActionResult::MercenaryLimit,
            "MISS" => ActionResult::Miss,
            "MISSING_EMPTY_SOUL_GEM" => ActionResult::MissingEmptySoulGem,
            "MISSING_FILLED_SOUL_GEM" => ActionResult::MissingFilledSoulGem,
            "MOBILE_GRAVEYARD_LIMIT" => ActionResult::MobileGraveyardLimit,
            "MOUNTED" => ActionResult::Mounted,
            "MUST_BE_IN_OWN_KEEP" => ActionResult::MustBeInOwnKeep,
            "NOT_ENOUGH_INVENTORY_SPACE" => ActionResult::NotEnoughInventorySpace,
            "NOT_ENOUGH_INVENTORY_SPACE_SOUL_GEM" => ActionResult::NotEnoughInventorySpaceSoulGem,
            "NOT_ENOUGH_SPACE_FOR_SIEGE" => ActionResult::NotEnoughSpaceForSiege,
            "NO_LOCATION_FOUND" => ActionResult::NoLocationFound,
            "NO_RAM_ATTACKABLE_TARGET_WITHIN_RANGE" => {
                ActionResult::NoRamAttackableTargetWithinRange
            }
            "NO_WEAPONS_TO_SWAP_TO" => ActionResult::NoWeaponsToSwapTo,
            "NPC_TOO_CLOSE" => ActionResult::NpcTooClose,
            "OFFBALANCE" => ActionResult::Offbalance,
            "PACIFIED" => ActionResult::Pacified,
            "PARRIED" => ActionResult::Parried,
            "PARTIAL_RESIST" => ActionResult::PartialResist,
            "POWER_DRAIN" => ActionResult::PowerDrain,
            "POWER_ENERGIZE" => ActionResult::PowerEnergize,
            "PRECISE_DAMAGE" => ActionResult::PreciseDamage,
            "QUEUED" => ActionResult::Queued,
            "RAM_ATTACKABLE_TARGETS_ALL_DESTROYED" => {
                ActionResult::RamAttackableTargetsAllDestroyed
            }
            "RAM_ATTACKABLE_TARGETS_ALL_OCCUPIED" => ActionResult::RamAttackableTargetsAllOccupied,
            "RECALLING" => ActionResult::Recalling,
            "REFLECTED" => ActionResult::Reflected,
            "REINCARNATING" => ActionResult::Reincarnating,
            "RESIST" => ActionResult::Resist,
            "RESURRECT" => ActionResult::Resurrect,
            "ROOTED" => ActionResult::Rooted,
            "SELF_PLAYING_TRIBUTE" => ActionResult::SelfPlayingTribute,
            "SIEGE_LIMIT" => ActionResult::SiegeLimit,
            "SIEGE_NOT_ALLOWED_IN_ZONE" => ActionResult::SiegeNotAllowedInZone,
            "SIEGE_TOO_CLOSE" => ActionResult::SiegeTooClose,
            "SILENCED" => ActionResult::Silenced,
            "SNARED" => ActionResult::Snared,
            "SOUL_GEM_RESURRECTION_ACCEPTED" => ActionResult::SoulGemResurrectionAccepted,
            "SPRINTING" => ActionResult::Sprinting,
            "STAGGERED" => ActionResult::Staggered,
            "STUNNED" => ActionResult::Stunned,
            "SWIMMING" => ActionResult::Swimming,
            "TARGET_DEAD" => ActionResult::TargetDead,
            "TARGET_NOT_IN_VIEW" => ActionResult::TargetNotInView,
            "TARGET_NOT_PVP_FLAGGED" => ActionResult::TargetNotPvpFlagged,
            "TARGET_OUT_OF_RANGE" => ActionResult::TargetOutOfRange,
            "TARGET_PLAYING_TRIBUTE" => ActionResult::TargetPlayingTribute,
            "TARGET_TOO_CLOSE" => ActionResult::TargetTooClose,
            "UNEVEN_TERRAIN" => ActionResult::UnevenTerrain,
            "WEAPONSWAP" => ActionResult::Weaponswap,
            "WRECKING_DAMAGE" => ActionResult::WreckingDamage,
            "WRONG_WEAPON" => ActionResult::WrongWeapon,
            x => unreachable!("{x} is invalid Action Result"),
        }
    }
    fn parse_power_type(data: &str) -> PowerType {
        match data {
            "8" => PowerType::Adrenaline,
            "5" => PowerType::Charges,
            "3" => PowerType::Combo,
            "2" => PowerType::Fervor,
            "9" => PowerType::Finesse,
            "-2" => PowerType::Health,
            "-1" => PowerType::Invalid,
            "0" => PowerType::Magicka,
            "7" => PowerType::Momentum,
            "11" => PowerType::MountStamina,
            "4" => PowerType::Power,
            "6" => PowerType::Stamina,
            "10" => PowerType::Ultimate,
            "1" => PowerType::Werewolf,
            _ => unreachable!("Invalid Power Type"),
        }
    }
    fn parse_damage_type(data: &str) -> DamageType {
        match data {
            "BLEED" => DamageType::Bleed,
            "COLD" => DamageType::Cold,
            "DISEASE" => DamageType::Disease,
            "DROWN" => DamageType::Drown,
            "EARTH" => DamageType::Earth,
            "FIRE" => DamageType::Fire,
            "GENERIC" => DamageType::Generic,
            "MAGIC" => DamageType::Magic,
            "NONE" => DamageType::None,
            "OBLIVION" => DamageType::Oblivion,
            "PHYSICAL" => DamageType::Physical,
            "POISON" => DamageType::Poison,
            "SHOCK" => DamageType::Shock,
            _ => unreachable!("Invalid Damage Type"),
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
        let data = &self.data[self.current_line];
        if data.is_empty() {
            return None;
        }
        self.current_line += 1;
        let mut split = data.splitn(3, ',');
        let time = split.next().unwrap();
        let token = split.next().unwrap();
        let remainder = split.next();
        let mut tokens: VecDeque<_> = if let Some(r) = remainder {
            Self::tokenize(r).into()
        } else {
            vec![].into()
        };
        match token {
            "BEGIN_LOG" => {
                let time_since_epoch_ms = tokens.pop_front().unwrap().parse().unwrap();
                let log_version = tokens.pop_front().unwrap().parse().unwrap();
                let realm_name: String = tokens.pop_front().unwrap();
                let language: String = tokens.pop_front().unwrap();
                let game_version: String = tokens.pop_front().unwrap();
                let line = SegmentType::BeginLog(BeginLog {
                    time_since_epoch_ms,
                    log_version,
                    realm_name: realm_name.into(),
                    language: language.into(),
                    game_version: game_version.into(),
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "ZONE_CHANGED" => {
                let id = tokens.pop_front().unwrap().parse().unwrap();
                let name = tokens.pop_front().unwrap().into();
                let mode = match tokens.pop_front().unwrap().as_str() {
                    "VETERAN" => DungeonDifficulty::Veteran,
                    "NORMAL" => DungeonDifficulty::Normal,
                    _ => DungeonDifficulty::Normal,
                };
                let line = SegmentType::ZoneInfo(ZoneInfo {
                    id,
                    name,
                    dungeon_difficulty: mode,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "UNIT_ADDED" => {
                let unit_id = tokens.pop_front().unwrap();
                let unit_type = match tokens.pop_front().unwrap().as_str() {
                    "PLAYER" => UnitType::Player,
                    "MONSTER" => UnitType::Monster,
                    x => unimplemented!("{x} Unit is not implemented"),
                };
                let is_local_player = Self::parse_bool(&tokens.pop_front().unwrap());
                let player_per_session_id = tokens.pop_front().unwrap();
                let monster_id = tokens.pop_front().unwrap();
                let is_boss = Self::parse_bool(&tokens.pop_front().unwrap());
                let class = Self::parse_class(&tokens.pop_front().unwrap());
                let race = Self::parse_race(&tokens.pop_front().unwrap());
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
                let is_grouped_with_local_player = Self::parse_bool(&tokens.pop_front().unwrap());

                let line = SegmentType::UnitAdded(UnitAdded {
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
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "TRIAL_INIT" => {
                let id = tokens.pop_front().unwrap();
                let in_progress = Self::parse_bool(&tokens.pop_front().unwrap());
                let completed = Self::parse_bool(&tokens.pop_front().unwrap());
                let start_time_ms = tokens.pop_front().unwrap();
                let duration_ms = tokens.pop_front().unwrap();
                let success = Self::parse_bool(&tokens.pop_front().unwrap());
                let final_score = tokens.pop_front().unwrap();

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
                let ability_id = tokens.pop_front().unwrap();
                let name = tokens.pop_front().unwrap();
                let icon_path = tokens.pop_front().unwrap();
                let interruptible = Self::parse_bool(&tokens.pop_front().unwrap());
                let blockable = Self::parse_bool(&tokens.pop_front().unwrap());
                let line = SegmentType::AbilityInfo(AbilityInfo {
                    ability_id: ability_id.parse().unwrap(),
                    name: name.into(),
                    icon_path: PathBuf::from(icon_path),
                    interruptible,
                    blockable,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "MAP_CHANGED" => {
                let id = tokens.pop_front().unwrap();
                let name = tokens.pop_front().unwrap();
                let texture_path = tokens.pop_front().unwrap();
                let line = SegmentType::MapInfo(MapInfo {
                    id: id.parse().unwrap(),
                    name: name.into(),
                    texture_path: PathBuf::from(texture_path),
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "BEGIN_CAST" => {
                let duration_ms = tokens.pop_front().unwrap();
                let channeled = Self::parse_bool(&tokens.pop_front().unwrap());
                let cast_track_id = tokens.pop_front().unwrap();
                let ability_id = tokens.pop_front().unwrap();
                let source = Self::parse_source_unit(&mut tokens);
                let target = Self::parse_target_unit(&mut tokens);

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
                let end_reason = match tokens.pop_front().unwrap().as_str() {
                    "COMPLETED" => EndReason::Completed,
                    x => unimplemented!("{x} End Reason is not implemented"),
                };
                let cast_track_id = tokens.pop_front().unwrap().parse().unwrap();
                let interrupting_ability_id = tokens.pop_front().map(|f| f.parse().unwrap());
                let interrupting_unit_id = tokens.pop_front().map(|f| f.parse().unwrap());
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
                let ability_id = tokens.pop_front().unwrap().parse().unwrap();
                let effect_type = match tokens.pop_front().unwrap().as_str() {
                    "BUFF" => EffectType::Buff,
                    "DEBUFF" => EffectType::Debuff,
                    x => unimplemented!("{x} Effect Type is not implemented"),
                };
                let status_effect_type = match tokens.pop_front().unwrap().as_str() {
                    "NONE" => StatusEffectType::None,
                    x => unimplemented!("{x} Status Effect Type is not implemented"),
                };
                let effect_bar_display_behaviour = match tokens.pop_front().unwrap().as_str() {
                    "DEFAULT" => EffectBarDisplayBehaviour::Default,
                    "NEVER" => EffectBarDisplayBehaviour::Never,
                    x => unimplemented!("{x} Effect Bar Display behaviour is not implemented"),
                };
                let grants_synergy_ability_id = tokens.pop_front().map(|f| f.parse().unwrap());

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
                let source = Self::parse_source_unit(&mut tokens);
                let target = Self::parse_target_unit(&mut tokens);
                let player_initiated_remove_cast_track_id =
                    tokens.pop_front().map(|f| f.parse().unwrap());
                let line = SegmentType::EffectChanged(EffectChanged {
                    change_type,
                    stack_count,
                    cast_track_id,
                    ability_id,
                    source,
                    target,
                    player_initiated_remove_cast_track_id,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "COMBAT_EVENT" => {
                let action_result = Self::parse_action_result(&tokens.pop_front().unwrap());
                let damage_type = Self::parse_damage_type(&tokens.pop_front().unwrap());
                let power_type = Self::parse_power_type(&tokens.pop_front().unwrap());
                let hit_value = tokens.pop_front().unwrap().parse().unwrap();
                let overflow = tokens.pop_front().unwrap().parse().unwrap();
                let cast_track_id = tokens.pop_front().unwrap().parse().unwrap();
                let ability_id = tokens.pop_front().unwrap().parse().unwrap();
                let source = Self::parse_source_unit(&mut tokens);
                let target = Self::parse_target_unit(&mut tokens);

                let line = SegmentType::CombatEvent(CombatEvent {
                    action_result,
                    damage_type,
                    power_type,
                    hit_value,
                    overflow,
                    cast_track_id,
                    ability_id,
                    source,
                    target,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "UNIT_REMOVED" => {
                let line = SegmentType::UnitRemoved(UnitRemoved {
                    unit_id: tokens.pop_front().unwrap().parse().unwrap(),
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "HEALTH_REGEN" => {
                let effective_regen = tokens.pop_front().unwrap().parse().unwrap();
                let source = Self::parse_source_unit(&mut tokens);
                let line = SegmentType::HealthRegen(HealthRegen {
                    effective_regen,
                    source,
                });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }

            "UNIT_CHANGED" => {
                let unit_id = tokens.pop_front().unwrap();
                let class = Self::parse_class(&tokens.pop_front().unwrap());
                let race = Self::parse_race(&tokens.pop_front().unwrap());
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
                    x => unimplemented!("{x} Player Reaction not implemented"),
                };
                let is_grouped_with_local_player = Self::parse_bool(&tokens.pop_front().unwrap());

                let line = SegmentType::UnitChanged(UnitChanged {
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
                });

                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "BEGIN_TRIAL" => {
                let id = tokens.pop_front().unwrap().parse().unwrap();
                let start_time_ms = tokens.pop_front().unwrap().parse().unwrap();

                let line = SegmentType::BeginTrial(BeginTrial { id, start_time_ms });
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            "BEGIN_COMBAT" => {
                let line = SegmentType::BeginCombat;
                Some(Segment {
                    time: time.parse().unwrap(),
                    line,
                })
            }
            x => {
                todo!("{x} is not implemented!(): {:#?}", tokens);
            }
        }
    }
}
