use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::action::Action;
use crate::game::GameState;
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
    pub doubled: bool,
    pub effects: Vec<PyEffect>,
    pub price: Option<u16>, // While the Shop offers it
    pub usable: bool,       // Drinkable now: a PotionUse for it is legal
}

pub(crate) fn snapshot_potion(state: &GameState, id: usize) -> PyPotion {
    let entity = &state.entities[id];
    PyPotion {
        id,
        name: entity.potion_name.into(),
        rarity: entity.potion_rarity.into(),
        requires_target: entity_requires_target(entity),
        combat_only: entity.potion_combat_only,
        doubled: entity.potion_doubled,
        effects: entity.potion_effects.iter().map(snapshot_effect).collect(),
        price: entity.price,
        usable: state.legal_actions.iter().any(
            |action| matches!(action, Action::PotionUse { id_potion, .. } if *id_potion == id),
        ),
    }
}
