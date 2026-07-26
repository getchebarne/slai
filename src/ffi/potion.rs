use pyo3::prelude::*;

use crate::entity::Entity;
use crate::types::PotionName;
use crate::types::PotionRarity;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "PotionName",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPotionName {
    EnergyPotion,
    BlockPotion,
    StrengthPotion,
    DexterityPotion,
    FirePotion,
    ExplosivePotion,
    WeakPotion,
    FearPotion,
    PoisonPotion,
    SwiftPotion,
    AttackPotion,
    SkillPotion,
    PowerPotion,
    FruitJuice,
}

impl From<PotionName> for PyPotionName {
    fn from(name: PotionName) -> Self {
        match name {
            PotionName::EnergyPotion => Self::EnergyPotion,
            PotionName::BlockPotion => Self::BlockPotion,
            PotionName::StrengthPotion => Self::StrengthPotion,
            PotionName::DexterityPotion => Self::DexterityPotion,
            PotionName::FirePotion => Self::FirePotion,
            PotionName::ExplosivePotion => Self::ExplosivePotion,
            PotionName::WeakPotion => Self::WeakPotion,
            PotionName::FearPotion => Self::FearPotion,
            PotionName::PoisonPotion => Self::PoisonPotion,
            PotionName::SwiftPotion => Self::SwiftPotion,
            PotionName::AttackPotion => Self::AttackPotion,
            PotionName::SkillPotion => Self::SkillPotion,
            PotionName::PowerPotion => Self::PowerPotion,
            PotionName::FruitJuice => Self::FruitJuice,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "PotionRarity",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPotionRarity {
    Common,
    Uncommon,
    Rare,
}

impl From<PotionRarity> for PyPotionRarity {
    fn from(rarity: PotionRarity) -> Self {
        match rarity {
            PotionRarity::Common => Self::Common,
            PotionRarity::Uncommon => Self::Uncommon,
            PotionRarity::Rare => Self::Rare,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Potion",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyPotion {
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

pub(crate) fn snapshot_potion(entity: &Entity) -> PyPotion {
    PyPotion {
        name: entity.potion_name.into(),
        rarity: entity.potion_rarity.into(),
        requires_target: entity.requires_target,
        combat_only: entity.potion_combat_only,
        effects: entity.potion_effects.iter().map(snapshot_effect).collect(),
    }
}
