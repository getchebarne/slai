use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Entity;
use crate::types::PotionName;
use crate::types::PotionRarity;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

mirror_enum!(PyPotionName from PotionName, "PotionName", skip_from_py_object, {
    EnergyPotion, BlockPotion, StrengthPotion, DexterityPotion, FirePotion, ExplosivePotion,
    WeakPotion, FearPotion, PoisonPotion, SwiftPotion, AttackPotion, SkillPotion, PowerPotion,
    FruitJuice, AncientPotion, LiquidBronze, EssenceOfSteel, GhostInAJar, CultistPotion,
    CunningPotion, DistilledChaos, BlessingOfTheForge, EntropicBrew, RegenerationPotion,
    SteroidPotion, SpeedPotion, DuplicateNextCardPlayPotion, ColorlessPotion, GamblersBrew,
    LiquidMemories, SneckoOil, FairyPotion, SmokeBomb,
});

mirror_enum!(PyPotionRarity from PotionRarity, "PotionRarity", skip_from_py_object, {
    Common, Uncommon, Rare,
});

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Potion",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
