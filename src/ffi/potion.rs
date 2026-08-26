use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::entity::Entity;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::utils::entity_requires_target;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

mirror_enum!(PyPotionName from PotionName, "PotionName", {
    Energy, Block, Strength, Dexterity, Fire, Explosive,
    Weak, Fear, Poison, Swift, Attack, Skill, Power,
    FruitJuice, Ancient, LiquidBronze, EssenceOfSteel, GhostInAJar, Cultist,
    Cunning, DistilledChaos, BlessingOfTheForge, EntropicBrew, Regeneration,
    Steroid, Speed, Duplication, Colorless, GamblersBrew,
    LiquidMemories, SneckoOil, Fairy, SmokeBomb,
});

mirror_enum!(PyPotionRarity from PotionRarity, "PotionRarity", {
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
    pub id: usize,
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

pub(crate) fn snapshot_potion(id: usize, entity: &Entity) -> PyPotion {
    PyPotion {
        id,
        name: entity.potion_name.into(),
        rarity: entity.potion_rarity.into(),
        requires_target: entity_requires_target(entity),
        combat_only: entity.potion_combat_only,
        effects: entity.potion_effects.iter().map(snapshot_effect).collect(),
    }
}
