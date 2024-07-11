use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use specta::Type;

use super::player::{Targets, UnitState};

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Effect {
    pub ability_id: u32,
    pub stack_count: u8,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct BeginCast {
    pub duration_ms: u32,
    pub channeled: bool,
    pub cast_track_id: u32,
    pub ability_id: u32,
    pub source: UnitState,
    pub target: Targets,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum EndReason {
    Completed,
    PlayerCancelled,
    Interrupted,
}

impl From<String> for EndReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "COMPLETED" => Self::Completed,
            "PLAYER_CANCELLED" => Self::PlayerCancelled,
            "INTERRUPTED" => Self::Interrupted,
            x => unimplemented!("{x} End Reason is not implemented"),
        }
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EndCast {
    pub end_reason: EndReason,
    pub cast_track_id: u32,
    pub interrupting_ability_id: Option<u32>,
    pub interrupting_unit_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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

impl From<String> for ActionResult {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ABILITY_ON_COOLDOWN" => Self::AbilityOnCooldown,
            "ABSORBED" => Self::Absorbed,
            "BAD_TARGET" => Self::BadTarget,
            "BLADETURN" => Self::Bladeturn,
            "BLOCKED" => Self::Blocked,
            "BLOCKED_DAMAGE" => Self::BlockedDamage,
            "BUSY" => Self::Busy,
            "CANNOT_USE" => Self::CannotUse,
            "CANT_SEE_TARGET" => Self::CantSeeTarget,
            "CANT_SWAP_HOTBAR_IS_OVERRIDDEN" => Self::CantSwapHotbarIsOverridden,
            "CANT_SWAP_WHILE_CHANGING_GEAR" => Self::CantSwapWhileChangingGear,
            "CASTER_DEAD" => Self::CasterDead,
            "CHARMED" => Self::Charmed,
            "CRITICAL_DAMAGE" => Self::CriticalDamage,
            "CRITICAL_HEAL" => Self::CriticalHeal,
            "DAMAGE" => Self::Damage,
            "DAMAGE_SHIELDED" => Self::DamageShielded,
            "DEFENDED" => Self::Defended,
            "DIED" => Self::Died,
            "DIED_COMPANION_XP" => Self::DiedCompanionXp,
            "DIED_XP" => Self::DiedXp,
            "DISARMED" => Self::Disarmed,
            "DISORIENTED" => Self::Disoriented,
            "DODGED" => Self::Dodged,
            "DOT_TICK" => Self::DotTick,
            "DOT_TICK_CRITICAL" => Self::DotTickCritical,
            "FAILED" => Self::Failed,
            "FAILED_REQUIREMENTS" => Self::FailedRequirements,
            "FAILED_SIEGE_CREATION_REQUIREMENTS" => Self::FailedSiegeCreationRequirements,
            "FALLING" => Self::Falling,
            "FALL_DAMAGE" => Self::FallDamage,
            "FEARED" => Self::Feared,
            "GRAVEYARD_DISALLOWED_IN_INSTANCE" => Self::GraveyardDisallowedInInstance,
            "GRAVEYARD_TOO_CLOSE" => Self::GraveyardTooClose,
            "HEAL" => Self::Heal,
            "HEAL_ABSORBED" => Self::HealAbsorbed,
            "HOT_TICK" => Self::HotTick,
            "HOT_TICK_CRITICAL" => Self::HotTickCritical,
            "IMMUNE" => Self::Immune,
            "INSUFFICIENT_RESOURCE" => Self::InsufficientResource,
            "INTERCEPTED" => Self::Intercepted,
            "INTERRUPT" => Self::Interrupt,
            "INVALID" => Self::Invalid,
            "INVALID_FIXTURE" => Self::InvalidFixture,
            "INVALID_JUSTICE_TARGET" => Self::InvalidJusticeTarget,
            "INVALID_TERRAIN" => Self::InvalidTerrain,
            "IN_AIR" => Self::InAir,
            "IN_COMBAT" => Self::InCombat,
            "IN_ENEMY_KEEP" => Self::InEnemyKeep,
            "IN_ENEMY_OUTPOST" => Self::InEnemyOutpost,
            "IN_ENEMY_RESOURCE" => Self::InEnemyResource,
            "IN_ENEMY_TOWN" => Self::InEnemyTown,
            "IN_HIDEYHOLE" => Self::InHideyhole,
            "KILLED_BY_DAEDRIC_WEAPON" => Self::KilledByDaedricWeapon,
            "KILLED_BY_SUBZONE" => Self::KilledBySubzone,
            "KILLING_BLOW" => Self::KillingBlow,
            "KNOCKBACK" => Self::Knockback,
            "LEVITATED" => Self::Levitated,
            "MERCENARY_LIMIT" => Self::MercenaryLimit,
            "MISS" => Self::Miss,
            "MISSING_EMPTY_SOUL_GEM" => Self::MissingEmptySoulGem,
            "MISSING_FILLED_SOUL_GEM" => Self::MissingFilledSoulGem,
            "MOBILE_GRAVEYARD_LIMIT" => Self::MobileGraveyardLimit,
            "MOUNTED" => Self::Mounted,
            "MUST_BE_IN_OWN_KEEP" => Self::MustBeInOwnKeep,
            "NOT_ENOUGH_INVENTORY_SPACE" => Self::NotEnoughInventorySpace,
            "NOT_ENOUGH_INVENTORY_SPACE_SOUL_GEM" => Self::NotEnoughInventorySpaceSoulGem,
            "NOT_ENOUGH_SPACE_FOR_SIEGE" => Self::NotEnoughSpaceForSiege,
            "NO_LOCATION_FOUND" => Self::NoLocationFound,
            "NO_RAM_ATTACKABLE_TARGET_WITHIN_RANGE" => Self::NoRamAttackableTargetWithinRange,
            "NO_WEAPONS_TO_SWAP_TO" => Self::NoWeaponsToSwapTo,
            "NPC_TOO_CLOSE" => Self::NpcTooClose,
            "OFFBALANCE" => Self::Offbalance,
            "PACIFIED" => Self::Pacified,
            "PARRIED" => Self::Parried,
            "PARTIAL_RESIST" => Self::PartialResist,
            "POWER_DRAIN" => Self::PowerDrain,
            "POWER_ENERGIZE" => Self::PowerEnergize,
            "PRECISE_DAMAGE" => Self::PreciseDamage,
            "QUEUED" => Self::Queued,
            "RAM_ATTACKABLE_TARGETS_ALL_DESTROYED" => Self::RamAttackableTargetsAllDestroyed,
            "RAM_ATTACKABLE_TARGETS_ALL_OCCUPIED" => Self::RamAttackableTargetsAllOccupied,
            "RECALLING" => Self::Recalling,
            "REFLECTED" => Self::Reflected,
            "REINCARNATING" => Self::Reincarnating,
            "RESIST" => Self::Resist,
            "RESURRECT" => Self::Resurrect,
            "ROOTED" => Self::Rooted,
            "SELF_PLAYING_TRIBUTE" => Self::SelfPlayingTribute,
            "SIEGE_LIMIT" => Self::SiegeLimit,
            "SIEGE_NOT_ALLOWED_IN_ZONE" => Self::SiegeNotAllowedInZone,
            "SIEGE_TOO_CLOSE" => Self::SiegeTooClose,
            "SILENCED" => Self::Silenced,
            "SNARED" => Self::Snared,
            "SOUL_GEM_RESURRECTION_ACCEPTED" => Self::SoulGemResurrectionAccepted,
            "SPRINTING" => Self::Sprinting,
            "STAGGERED" => Self::Staggered,
            "STUNNED" => Self::Stunned,
            "SWIMMING" => Self::Swimming,
            "TARGET_DEAD" => Self::TargetDead,
            "TARGET_NOT_IN_VIEW" => Self::TargetNotInView,
            "TARGET_NOT_PVP_FLAGGED" => Self::TargetNotPvpFlagged,
            "TARGET_OUT_OF_RANGE" => Self::TargetOutOfRange,
            "TARGET_PLAYING_TRIBUTE" => Self::TargetPlayingTribute,
            "TARGET_TOO_CLOSE" => Self::TargetTooClose,
            "UNEVEN_TERRAIN" => Self::UnevenTerrain,
            "WEAPONSWAP" => Self::Weaponswap,
            "WRECKING_DAMAGE" => Self::WreckingDamage,
            "WRONG_WEAPON" => Self::WrongWeapon,
            x => unreachable!("{x} is invalid Action Result"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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

impl From<String> for DamageType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BLEED" => Self::Bleed,
            "COLD" => Self::Cold,
            "DISEASE" => Self::Disease,
            "DROWN" => Self::Drown,
            "EARTH" => Self::Earth,
            "FIRE" => Self::Fire,
            "GENERIC" => Self::Generic,
            "MAGIC" => Self::Magic,
            "NONE" => Self::None,
            "OBLIVION" => Self::Oblivion,
            "PHYSICAL" => Self::Physical,
            "POISON" => Self::Poison,
            "SHOCK" => Self::Shock,
            _ => unreachable!("Invalid Damage Type"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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

impl From<String> for PowerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "8" => Self::Adrenaline,
            "5" => Self::Charges,
            "3" => Self::Combo,
            "2" => Self::Fervor,
            "9" => Self::Finesse,
            "-2" => Self::Health,
            "-1" => Self::Invalid,
            "0" => Self::Magicka,
            "7" => Self::Momentum,
            "11" => Self::MountStamina,
            "4" => Self::Power,
            "6" => Self::Stamina,
            "10" => Self::Ultimate,
            "1" => Self::Werewolf,
            _ => unreachable!("Invalid Power Type"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum EffectChangeType {
    Faded,
    Gained,
    Updated,
}

impl From<String> for EffectChangeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "FADED" => Self::Faded,
            "GAINED" => Self::Gained,
            "UPDATED" => Self::Updated,
            x => unimplemented!("{x} Effect Change Type is not implemented"),
        }
    }
}
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EffectChanged {
    pub change_type: EffectChangeType,
    pub stack_count: u8,
    pub cast_track_id: u32,
    pub ability_id: u32,
    pub source: UnitState,
    pub target: Targets,
    pub player_initiated_remove_cast_track_id: Option<u32>,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct AbilityInfo {
    pub ability_id: u32,
    pub name: String,
    pub icon_path: PathBuf,
    pub interruptible: bool,
    pub blockable: bool,
}
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum EffectType {
    Buff,
    Debuff,
}

impl From<String> for EffectType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BUFF" => Self::Buff,
            "DEBUFF" => Self::Debuff,
            x => unimplemented!("{x} Effect Type is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum StatusEffectType {
    None,
    Magic,
    Snare,
    Root,
    Bleed,
    Poison,
    Environment,
    Disease,
}

impl From<String> for StatusEffectType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NONE" => Self::None,
            "MAGIC" => Self::Magic,
            "SNARE" => Self::Snare,
            "ROOT" => Self::Root,
            "BLEED" => Self::Bleed,
            "POISON" => Self::Poison,
            "ENVIRONMENT" => Self::Environment,
            "DISEASE" => Self::Disease,
            x => unimplemented!("{x} Status Effect Type is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum EffectBarDisplayBehaviour {
    Default,
    Never,
    Always,
}
impl From<String> for EffectBarDisplayBehaviour {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DEFAULT" => Self::Default,
            "NEVER" => Self::Never,
            "ALWAYS" => Self::Always,
            x => unimplemented!("{x} Effect Bar Display behaviour is not implemented"),
        }
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EffectInfo {
    pub ability_id: u32,
    pub effect_type: EffectType,
    pub status_effect_type: StatusEffectType,
    pub effect_bar_display_behaviour: EffectBarDisplayBehaviour,
    pub grants_synergy_ability_id: Option<u32>,
}
