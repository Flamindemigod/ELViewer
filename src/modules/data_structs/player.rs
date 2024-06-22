use std::collections::VecDeque;

use crate::modules::parser::parse_bool;

use super::abilities::Effect;

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

//<equipmentInfo> refers to the following fields for a piece of equipment: slot, id, isCP, level, trait, displayQuality, setId, enchantType, isEnchantCP, enchantLevel, enchantQuality.
#[derive(Debug, Default)]
pub struct Equipment {
    pub head: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub shoulders: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub chest: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub hand: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub waist: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub legs: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub feet: Option<EquipmentInfo<ArmorTrait, ArmorEnchantType>>,
    pub neck: Option<EquipmentInfo<JewelTrait, JewelEnchantType>>,
    pub ring1: Option<EquipmentInfo<JewelTrait, JewelEnchantType>>,
    pub ring2: Option<EquipmentInfo<JewelTrait, JewelEnchantType>>,
    pub main: Option<WeaponHand>,
    pub main_poison: Option<EquipmentInfo<PoisonTrait, PoisonEnchantType>>,
    pub backup: Option<WeaponHand>,
    pub backup_poison: Option<EquipmentInfo<PoisonTrait, PoisonEnchantType>>,
}

impl<T, V> EquipmentInfo<T, V>
where
    T: TraitMarker + From<String>,
    V: EnchantMarker,
{
    pub fn parse_equipment(tokens: &mut VecDeque<String>) -> Self {
        let id = tokens.pop_front().unwrap().parse().unwrap();
        let level = if parse_bool(&tokens.pop_front().unwrap()) {
            EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
        } else {
            EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
        };

        let r#trait: T = tokens.pop_front().unwrap().into();
        let display_quality = tokens.pop_front().unwrap().into();
        let set_id = tokens.pop_front().unwrap().parse().unwrap();
        let enchant = Enchant::parse_enchant(tokens);

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
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug)]
pub enum WeaponHand {
    OneHand(
        Option<EquipmentInfo<WeaponTrait, WeaponEnchantType>>,
        Option<
            Either<
                EquipmentInfo<WeaponTrait, WeaponEnchantType>,
                EquipmentInfo<ArmorTrait, ArmorEnchantType>,
            >,
        >,
    ),
    TwoHand(EquipmentInfo<WeaponTrait, WeaponEnchantType>),
}

impl WeaponHand {
    fn check_if_armor(tokens: &VecDeque<String>) -> bool {
        tokens.iter().any(|v| v.contains("ARMOR_"))
    }

    pub fn parse_weapon(ident: &str, tokens: &mut VecDeque<String>, current: Option<Self>) -> Self {
        match ident {
            "BACKUP_MAIN" | "MAIN" => Self::TwoHand(EquipmentInfo::parse_equipment(tokens)),
            "OFF_HAND" | "MAIN_HAND" => {
                if let Some(current_hand) = current {
                    match current_hand {
                        Self::OneHand(mut a, mut b) => {
                            if a.is_none() {
                                a = Some(EquipmentInfo::parse_equipment(tokens));
                            } else if b.is_none() {
                                // Checks if has shield
                                if Self::check_if_armor(tokens) {
                                    b = Some(Either::Right(EquipmentInfo::parse_equipment(tokens)))
                                } else {
                                    b = Some(Either::Left(EquipmentInfo::parse_equipment(tokens)));
                                }
                            }
                            WeaponHand::OneHand(a, b)
                        }
                        WeaponHand::TwoHand(_) => unreachable!(),
                    }
                } else {
                    WeaponHand::OneHand(Some(EquipmentInfo::parse_equipment(tokens)), None)
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum EquipmentLevel {
    NoCp(u8),
    Cp(u8),
}

#[derive(Debug)]
pub enum PoisonTrait {
    None,
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
impl r#TraitMarker for PoisonTrait {}

impl From<String> for ArmorTrait {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ARMOR_DIVINES" => Self::Divines,
            "ARMOR_INFUSED" => Self::Infused,
            "ARMOR_REINFORCED" => Self::Reinforced,
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

#[derive(Debug)]
pub enum Quality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
    Mythic,
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
            x => unimplemented!("{x} Quality is not implemented"),
        }
    }
}

#[derive(Debug)]
pub enum ArmorEnchantType {
    Health,
    Magicka,
    Stamina,
    PrismaticDefense,
}
#[derive(Debug)]
pub enum WeaponEnchantType {
    AbsorbMagicka,
    Berserker,
    ReduceArmor,
    FieryWeapon,
    PoisonedWeapon,
    AbsorbHealth,
    AbsorbStamina,
    ChargedWeapon,
}
#[derive(Debug)]
pub enum JewelEnchantType {
    IncreaseSpellDamage,
    MagickaRegen,
    ReduceSpellCost,
    StaminaRegen,
    ReduceFeatCost,
    HealthRegen,
    IncreasePhysicalDamage,
    ReduceBlockAndBash,
}
#[derive(Debug)]
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
            x => unimplemented!("{x} enchant is not implemented"),
        }
    }
}
impl EnchantMarker for WeaponEnchantType {
    fn parse_enchant(token: &str) -> Option<Self> {
        match token {
            "ABSORB_MAGICKA" => Some(Self::AbsorbMagicka),
            "BERSERKER" => Some(Self::Berserker),
            "REDUCE_ARMOR" => Some(Self::ReduceArmor),
            "FIERY_WEAPON" => Some(Self::FieryWeapon),
            "POISONED_WEAPON" => Some(Self::PoisonedWeapon),
            "ABSORB_HEALTH" => Some(Self::AbsorbHealth),
            "ABSORB_STAMINA" => Some(Self::AbsorbStamina),
            "CHARGED_WEAPON" => Some(Self::ChargedWeapon),
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

#[derive(Debug)]
pub struct Enchant<T>
where
    T: EnchantMarker,
{
    r#type: T,
    level: EquipmentLevel,
    quality: Quality,
}

impl<T> Enchant<T>
where
    T: EnchantMarker,
{
    pub fn parse_enchant(tokens: &mut VecDeque<String>) -> Option<Self> {
        if let Some(enchant_type) = T::parse_enchant(&tokens.pop_front().unwrap()) {
            let level = if parse_bool(&tokens.pop_front().unwrap()) {
                EquipmentLevel::Cp(tokens.pop_front().unwrap().parse::<u8>().unwrap() * 10)
            } else {
                EquipmentLevel::NoCp(tokens.pop_front().unwrap().parse().unwrap())
            };

            let quality = tokens.pop_front().unwrap().into();
            Some(Enchant {
                r#type: enchant_type,
                level,
                quality,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct EquipmentInfo<T, V>
where
    T: TraitMarker,
    V: EnchantMarker,
{
    pub id: usize,
    pub level: EquipmentLevel,
    pub r#trait: T,
    pub display_quality: Quality,
    pub set_id: usize,
    pub enchant: Option<Enchant<V>>,
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub unit_id: usize,
    pub long_term_effect: Vec<Effect>,
    pub equipment_info: Equipment,
    pub primary_ability_id: [usize; 6],
    pub backup_ability_id: [usize; 6],
}

#[derive(Debug)]
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

impl Race {
    pub fn parse_race(d: &str) -> Self {
        match d {
            "4" => Self::DarkElf,
            "9" => Self::Khajit,
            "7" => Self::HighElf,
            "8" => Self::WoodElf,
            "5" => Self::Nord,
            "2" => Self::Redguard,
            "0" => Self::None,
            x => unimplemented!("{x} race is not implemented"),
        }
    }
}

#[derive(Debug)]
pub enum Class {
    Arcanist = 117,
    Templar = 6,
    DragonKnight = 1,
    Sorcerer = 2,
    None = 0,
}

impl Class {
    pub fn parse_class(d: &str) -> Self {
        match d {
            "117" => Self::Arcanist,
            "6" => Self::Templar,
            "1" => Self::DragonKnight,
            "2" => Self::Sorcerer,
            "0" => Self::None,
            x => unimplemented!("{x} class is not implemented"),
        }
    }
}
