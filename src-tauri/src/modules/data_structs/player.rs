use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::modules::parser::parse_bool;

use super::abilities::Effect;

//<unitState> refers to the following fields for a unit: unitId, health/max, magicka/max, stamina/max, ultimate/max, werewolf/max, shield, map NX, map NY, headingRadians.
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct UnitState {
    unit_id: u32,
    health: (u32, u32),
    magicka: (u32, u32),
    stamina: (u32, u32),
    ultimate: (u16, u16),
    werewolf: (u16, u16),
    shield: u32,
    position: (f32, f32),
    heading: f32,
}

impl UnitState {
    pub fn parse_source_unit(tokens: &mut VecDeque<String>) -> Self {
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
}

pub trait EquipmentParser{
    fn parse_equipment(tokens: &mut VecDeque<String>) -> Self;
}
pub trait EnchantParser{
    fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> where Self: Sized;
}

//<equipmentInfo> refers to the following fields for a piece of equipment: slot, id, isCP, level, trait, displayQuality, setId, enchantType, isEnchantCP, enchantLevel, enchantQuality.
#[derive(Debug, Default)]
#[taurpc::ipc_type]
pub struct Equipment {
    pub head: Option<EquipmentBody>,
    pub shoulders: Option<EquipmentBody>,
    pub chest: Option<EquipmentBody>,
    pub hand: Option<EquipmentBody>,
    pub waist: Option<EquipmentBody>,
    pub legs: Option<EquipmentBody>,
    pub feet: Option<EquipmentBody>,
    pub neck: Option<EquipmentJewel>,
    pub ring1: Option<EquipmentJewel>,
    pub ring2: Option<EquipmentJewel>,
    pub main: Option<WeaponHand>,
    pub main_poison: Option<EquipmentPoison>,
    pub backup: Option<WeaponHand>,
    pub backup_poison: Option<EquipmentPoison>,
}


#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum WeaponHand {
    OneHand(
        Option<EquipmentWeapon>,
        Option<
            Either<
            EquipmentWeapon,
            EquipmentBody
            >,
        >,
    ),
    TwoHand(EquipmentWeapon),
}

impl WeaponHand {
    fn check_if_armor(tokens: &VecDeque<String>) -> bool {
        tokens.iter().any(|v| v.contains("ARMOR_"))
    }

    pub fn parse_weapon(ident: &str, tokens: &mut VecDeque<String>, current: Option<Self>) -> Self {
        match ident {
            "BACKUP_MAIN" | "MAIN" => Self::TwoHand(EquipmentWeapon::parse_equipment(tokens)),
            "OFF_HAND" | "MAIN_HAND" => {
                if let Some(current_hand) = current {
                    match current_hand {
                        Self::OneHand(mut a, mut b) => {
                            if a.is_none() {
                                a = Some(EquipmentWeapon::parse_equipment(tokens));
                            } else if b.is_none() {
                                // Checks if has shield
                                if Self::check_if_armor(tokens) {
                                    b = Some(Either::Right(EquipmentBody::parse_equipment(tokens)))
                                } else {
                                    b = Some(Either::Left(EquipmentWeapon::parse_equipment(tokens)));
                                }
                            }
                            WeaponHand::OneHand(a, b)
                        }
                        WeaponHand::TwoHand(_) => unreachable!(),
                    }
                } else {
                    WeaponHand::OneHand(Some(EquipmentWeapon::parse_equipment(tokens)), None)
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum EquipmentLevel {
    NoCp(u8),
    Cp(u8),
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum PoisonTrait {
    None,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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
    None,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum JewelTrait {
    Arcane,
    Healthy,
    Robust,
    Harmony,
    Infused,
    Bloodthirsty,
    Protective,
    Swift,
    Triune,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
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

impl From<String> for ArmorTrait {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ARMOR_DIVINES" => Self::Divines,
            "ARMOR_INFUSED" => Self::Infused,
            "ARMOR_REINFORCED" => Self::Reinforced,
            "ARMOR_TRAINING" => Self::Training,
            "ARMOR_WELL_FITTED" => Self::WellFitted,
            "ARMOR_STURDY" => Self::Sturdy,
            "ARMOR_IMPENETRABLE" => Self::Impenetrable,
            "ARMOR_NIRNHONED" => Self::Nirnhoned,
            "NONE" => Self::None,
            x => unimplemented!("{x} trait is not implemented"),
        }
    }
}
impl From<String> for JewelTrait {
    fn from(value: String) -> Self {
        match value.as_str() {
            "JEWELRY_ARCANE" => Self::Arcane,
            "JEWELRY_INFUSED" => Self::Infused,
            "JEWELRY_BLOODTHIRSTY" => Self::Bloodthirsty,
            "JEWELRY_ROBUST" => Self::Robust,
            "JEWELRY_SWIFT" => Self::Swift,
            "JEWELRY_HARMONY" => Self::Harmony,
            "JEWELRY_HEALTHY" => Self::Healthy,
            x => unimplemented!("{x} trait is not implemented"),
        }
    }
}
impl From<String> for WeaponTrait {
    fn from(value: String) -> Self {
        match value.as_str() {
            "WEAPON_INFUSED" => Self::Infused,
            "WEAPON_POWERED" => Self::Powered,
            "WEAPON_NIRNHONED" => Self::Nirnhoned,
            "WEAPON_PRECISE" => Self::Precise,
            "WEAPON_CHARGED" => Self::Charged,
            "WEAPON_DECISIVE" => Self::Decisive,
            "WEAPON_SHARPENED" => Self::Sharpened,
            "WEAPON_DEFENDING" => Self::Defending,
            x => unimplemented!("{x} trait is not implemented"),
        }
    }
}
impl From<String> for PoisonTrait {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NONE" => Self::None,
            x => unimplemented!("{x} trait is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum Quality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
    Mythic,
    Arcane, //Unsure What this quality type is
}

impl From<String> for Quality {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NORMAL" => Self::Normal,
            "FINE" => Self::Fine,
            "SUPERIOR" => Self::Superior,
            "EPIC" => Self::Epic,
            "LEGENDARY" => Self::Legendary,
            "ARTIFACT" => Self::Mythic,
            "ARCANE" => Self::Arcane,
            x => unimplemented!("{x} Quality is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum ArmorEnchantType {
    Health,
    Magicka,
    Stamina,
    PrismaticDefense,
    Invalid,
}
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum WeaponEnchantType {
    AbsorbMagicka,
    Berserker,
    Crusher,
    Weakening,
    FieryWeapon,
    PoisonedWeapon,
    AbsorbHealth,
    AbsorbStamina,
    ChargedWeapon,
    BefouledWeapon,
    Invalid,
}
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum JewelEnchantType {
    IncreaseSpellDamage,
    MagickaRegen,
    ReduceSpellCost,
    StaminaRegen,
    ReduceFeatCost,
    HealthRegen,
    IncreasePhysicalDamage,
    ReduceBlockAndBash,
    Invalid,
}
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum PoisonEnchantType {
    Invalid,
}
trait EnchantMarker {
    fn parse_enchant(token: &str) -> Option<Self>
    where
        Self: Sized;
}
impl EnchantMarker for ArmorEnchantType {
    fn parse_enchant(token: &str) -> Option<Self> {
        match token {
            "MAGICKA" => Some(Self::Magicka),
            "STAMINA" => Some(Self::Stamina),
            "HEALTH" => Some(Self::Health),
            "PRISMATIC_DEFENSE" => Some(Self::PrismaticDefense),
            "INVALID" => Some(Self::Invalid),
            x => unimplemented!("{x} enchant is not implemented"),
        }
    }
}
impl EnchantMarker for JewelEnchantType {
    fn parse_enchant(token: &str) -> Option<Self> {
        match token {
            "INCREASE_SPELL_DAMAGE" => Some(Self::IncreaseSpellDamage),
            "MAGICKA_REGEN" => Some(Self::MagickaRegen),
            "REDUCE_SPELL_COST" => Some(Self::ReduceSpellCost),
            "STAMINA_REGEN" => Some(Self::StaminaRegen),
            "REDUCE_FEAT_COST" => Some(Self::ReduceFeatCost),
            "HEALTH_REGEN" => Some(Self::HealthRegen),
            "INCREASE_PHYSICAL_DAMAGE" => Some(Self::IncreasePhysicalDamage),
            "REDUCE_BLOCK_AND_BASH" => Some(Self::ReduceBlockAndBash),
            "INVALID" => Some(Self::Invalid),
            x => unimplemented!("{x} enchant is not implemented"),
        }
    }
}
impl EnchantMarker for WeaponEnchantType {
    fn parse_enchant(token: &str) -> Option<Self> {
        match token {
            "ABSORB_MAGICKA" => Some(Self::AbsorbMagicka),
            "BERSERKER" => Some(Self::Berserker),
            "REDUCE_ARMOR" => Some(Self::Crusher),
            "FIERY_WEAPON" => Some(Self::FieryWeapon),
            "POISONED_WEAPON" => Some(Self::PoisonedWeapon),
            "ABSORB_HEALTH" => Some(Self::AbsorbHealth),
            "ABSORB_STAMINA" => Some(Self::AbsorbStamina),
            "CHARGED_WEAPON" => Some(Self::ChargedWeapon),
            "REDUCE_POWER" => Some(Self::Weakening),
            "BEFOULED_WEAPON" => Some(Self::BefouledWeapon),
            "INVALID" => Some(Self::Invalid),
            x => unimplemented!("{x} enchant is not implemented"),
        }
    }
}
impl EnchantMarker for PoisonEnchantType {
    fn parse_enchant(token: &str) -> Option<Self> {
        match token {
            "INVALID" => Some(Self::Invalid),
            x => unimplemented!("{x} enchant is not implemented"),
        }
    }
}

impl EnchantParser for ArmorEnchant {
    fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> {
        if let Some(enchant_type) = ArmorEnchantType::parse_enchant(&tokens.pop_front().unwrap()) {
            let level = if parse_bool(&tokens.pop_front().unwrap()) {
                EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
            } else {
                EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
            };

            let quality = tokens.pop_front().unwrap().into();
            Some(Self {
                r#type: enchant_type,
                level,
                quality,
            })
        } else {
            None
        }
    }
}

impl EnchantParser for PoisonEnchant {
    fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> {
        if let Some(enchant_type) = PoisonEnchantType::parse_enchant(&tokens.pop_front().unwrap()) {
            let level = if parse_bool(&tokens.pop_front().unwrap()) {
                EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
            } else {
                EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
            };

            let quality = tokens.pop_front().unwrap().into();
            Some(Self {
                r#type: enchant_type,
                level,
                quality,
            })
        } else {
            None
        }
    }
}
impl EnchantParser for WeaponEnchant {
    fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> {
        if let Some(enchant_type) = WeaponEnchantType::parse_enchant(&tokens.pop_front().unwrap()) {
            let level = if parse_bool(&tokens.pop_front().unwrap()) {
                EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
            } else {
                EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
            };

            let quality = tokens.pop_front().unwrap().into();
            Some(Self {
                r#type: enchant_type,
                level,
                quality,
            })
        } else {
            None
        }
    }
}
impl EnchantParser for JewelEnchant {
    fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> {
        if let Some(enchant_type) = JewelEnchantType::parse_enchant(&tokens.pop_front().unwrap()) {
            let level = if parse_bool(&tokens.pop_front().unwrap()) {
                EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
            } else {
                EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
            };

            let quality = tokens.pop_front().unwrap().into();
            Some(Self {
                r#type: enchant_type,
                level,
                quality,
            })
        } else {
            None
        }
    }
}


// #[derive(Debug, Serialize, Deserialize, Clone, Type)]
// pub struct EquipmentInfo<T, V>
// where
//     T: TraitMarker + Type,
//     V: EnchantMarker + Type,
// {
//     pub id: u32,
//     pub level: EquipmentLevel,
//     pub r#trait: T,
//     pub display_quality: Quality,
//     pub set_id: u32,
//     pub enchant: Option<Enchant<V>>,
// }
//
//
//
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct PoisonEnchant
{
    r#type: PoisonEnchantType,
    level: EquipmentLevel,
    quality: Quality,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EquipmentPoison
{
    pub id: u32,
    pub level: EquipmentLevel,
    pub r#trait: PoisonTrait,
    pub display_quality: Quality,
    pub set_id: u32,
    pub enchant: Option<PoisonEnchant>,
}
#[derive(Debug)]
#[taurpc::ipc_type]
pub struct WeaponEnchant
{
    r#type: WeaponEnchantType,
    level: EquipmentLevel,
    quality: Quality,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EquipmentWeapon
{
    pub id: u32,
    pub level: EquipmentLevel,
    pub r#trait: WeaponTrait,
    pub display_quality: Quality,
    pub set_id: u32,
    pub enchant: Option<WeaponEnchant>,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct JewelEnchant
{
    r#type: JewelEnchantType,
    level: EquipmentLevel,
    quality: Quality,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EquipmentJewel
{
    pub id: u32,
    pub level: EquipmentLevel,
    pub r#trait: JewelTrait,
    pub display_quality: Quality,
    pub set_id: u32,
    pub enchant: Option<JewelEnchant>,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct ArmorEnchant
{
    r#type: ArmorEnchantType,
    level: EquipmentLevel,
    quality: Quality,
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct EquipmentBody
{
    pub id: u32,
    pub level: EquipmentLevel,
    pub r#trait: ArmorTrait,
    pub display_quality: Quality,
    pub set_id: u32,
    pub enchant: Option<ArmorEnchant>,
}


impl EquipmentParser for EquipmentWeapon{
    fn parse_equipment(tokens: &mut VecDeque<String>) -> Self {
               let id = tokens.pop_front().unwrap().parse().unwrap();
        let level = if parse_bool(&tokens.pop_front().unwrap()) {
            EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
        } else {
            EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
        };

        let r#trait = tokens.pop_front().unwrap().into();
        let display_quality = tokens.pop_front().unwrap().into();
        let set_id = tokens.pop_front().unwrap().parse().unwrap();
        let enchant = WeaponEnchant::parse_enchant(tokens);

        Self {
            id,
            level,
            r#trait,
            display_quality,
            set_id,
            enchant,
        } 
    }
}


impl EquipmentParser for EquipmentPoison{
    fn parse_equipment(tokens: &mut VecDeque<String>) -> Self {
               let id = tokens.pop_front().unwrap().parse().unwrap();
        let level = if parse_bool(&tokens.pop_front().unwrap()) {
            EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
        } else {
            EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
        };

        let r#trait = tokens.pop_front().unwrap().into();
        let display_quality = tokens.pop_front().unwrap().into();
        let set_id = tokens.pop_front().unwrap().parse().unwrap();
        let enchant = PoisonEnchant::parse_enchant(tokens);

        Self {
            id,
            level,
            r#trait,
            display_quality,
            set_id,
            enchant,
        } 
    }
}


impl EquipmentParser for EquipmentJewel{
    fn parse_equipment(tokens: &mut VecDeque<String>) -> Self {
               let id = tokens.pop_front().unwrap().parse().unwrap();
        let level = if parse_bool(&tokens.pop_front().unwrap()) {
            EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
        } else {
            EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
        };

        let r#trait = tokens.pop_front().unwrap().into();
        let display_quality = tokens.pop_front().unwrap().into();
        let set_id = tokens.pop_front().unwrap().parse().unwrap();
        let enchant = JewelEnchant::parse_enchant(tokens);

        Self {
            id,
            level,
            r#trait,
            display_quality,
            set_id,
            enchant,
        } 
    }
}


impl EquipmentParser for EquipmentBody{
    fn parse_equipment(tokens: &mut VecDeque<String>) -> Self {
               let id = tokens.pop_front().unwrap().parse().unwrap();
        let level = if parse_bool(&tokens.pop_front().unwrap()) {
            EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
        } else {
            EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
        };

        let r#trait = tokens.pop_front().unwrap().into();
        let display_quality = tokens.pop_front().unwrap().into();
        let set_id = tokens.pop_front().unwrap().parse().unwrap();
        let enchant = ArmorEnchant::parse_enchant(tokens);

        Self {
            id,
            level,
            r#trait,
            display_quality,
            set_id,
            enchant,
        } 
    }
}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct PlayerInfo {
    pub unit_id: u32,
    pub long_term_effect: Vec<Effect>,
    pub equipment_info: Equipment,
    pub primary_ability_id: [u32; 6],
    pub backup_ability_id: [u32; 6],
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum Targets {
    SelfTarget,
    Target(UnitState),
    None,
}

impl Targets {
    pub fn parse_target_unit(tokens: &mut VecDeque<String>) -> Targets {
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
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum Race {
    DarkElf = 4,
    Khajit = 9,
    HighElf = 7,
    WoodElf = 8,
    Nord = 5,
    Redguard = 2,
    Breton = 1,
    Argonian = 6,
    Orc = 3,
    Imperial = 10,
    None = 0,
}

impl Race {
    pub fn parse_race(d: &str) -> Self {
        match d {
            "4" => Self::DarkElf,
            "9" => Self::Khajit,
            "7" => Self::HighElf,
            "8" => Self::WoodElf,
            "5" => Self::Nord,
            "2" => Self::Redguard,
            "1" => Self::Breton,
            "6" => Self::Argonian,
            "3" => Self::Orc,
            "10" => Self::Imperial,
            "0" => Self::None,
            x => unimplemented!("{x} race is not implemented"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum Class {
    Arcanist = 117,
    Templar = 6,
    DragonKnight = 1,
    Sorcerer = 2,
    Necromancer = 5,
    Warden = 4,
    NightBlade = 3,
    None = 0,
}

impl Class {
    pub fn parse_class(d: &str) -> Self {
        match d {
            "117" => Self::Arcanist,
            "6" => Self::Templar,
            "1" => Self::DragonKnight,
            "2" => Self::Sorcerer,
            "5" => Self::Necromancer,
            "4" => Self::Warden,
            "3" => Self::NightBlade,
            "0" => Self::None,
            x => unimplemented!("{x} class is not implemented"),
        }
    }
}
