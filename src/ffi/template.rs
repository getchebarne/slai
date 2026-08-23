use pyo3::prelude::*;
use strum::EnumCount;
use strum::IntoEnumIterator;

use crate::cards::CardTemplate;
use crate::cards::card_template;
use crate::events::options_catalog;
use crate::monsters::monster_template;
use crate::monsters::pick_tier;
use crate::potions::get_potion;
use crate::relics::relic_template;
use crate::types::CardName;
use crate::types::EventName;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::utils::effects_require_target;
use crate::utils::entity_requires_target;

use super::card::PyCardColor;
use super::card::PyCardCostKind;
use super::card::PyCardKind;
use super::card::PyCardName;
use super::card::PyCardRarity;
use super::card::PyPlayRestriction;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::event::PyEventName;
use super::monster::PyMonsterKind;
use super::monster::PyMonsterName;
use super::potion::PyPotionName;
use super::potion::PyPotionRarity;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CardTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardTemplate {
    pub name: PyCardName,
    pub cost_base: u8,
    pub cost_kind: PyCardCostKind,

    // Categorical fields
    pub kind: PyCardKind,
    pub color: PyCardColor,
    pub rarity: PyCardRarity,
    pub play_restriction: PyPlayRestriction,

    // Boolean fields
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub requires_target: bool,

    // Effects
    pub effects: Vec<PyEffect>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "RelicTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyRelicTemplate {
    pub name: PyRelicName,
    pub tier: PyRelicTier,
    pub effects_combat_start: Vec<PyEffect>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "PotionTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyPotionTemplate {
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "MonsterTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyMonsterTemplate {
    pub name: PyMonsterName,
    pub kind: PyMonsterKind,
    pub hp_range: (u16, u16),
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EventOptionTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEventOptionTemplate {
    pub event: PyEventName,
    pub effects: Vec<PyEffect>,
}

fn template_card(card: &CardTemplate) -> PyCardTemplate {
    let effects = &card.effects[..card.effects_len as usize];
    PyCardTemplate {
        name: card.name.into(),
        cost_base: card.cost,
        cost_kind: card.cost_kind.into(),
        kind: card.kind.into(),
        color: card.color.into(),
        rarity: card.rarity.into(),
        play_restriction: card.play_restriction.into(),
        upgraded: card.upgraded,
        exhaust: card.exhaust,
        ethereal: card.ethereal,
        innate: card.innate,
        requires_target: effects_require_target(effects),
        effects: effects.iter().map(snapshot_effect).collect(),
    }
}

/// Every constructible card in enum declaration order, base and upgraded forms
/// adjacent where the card is upgradeable. State-free and deterministic.
#[pyfunction]
pub fn get_card_templates() -> Vec<PyCardTemplate> {
    let mut out = Vec::with_capacity(2 * CardName::COUNT);
    for name in CardName::iter() {
        out.push(template_card(card_template(name, false)));
        let upgraded = card_template(name, true);
        if upgraded.upgraded {
            out.push(template_card(upgraded));
        }
    }
    out
}

/// Every relic in enum declaration order. State-free and deterministic.
#[pyfunction]
pub fn get_relic_templates() -> Vec<PyRelicTemplate> {
    RelicName::iter()
        .map(|name| {
            let template = relic_template(name);
            PyRelicTemplate {
                name: template.name.into(),
                tier: template.tier.into(),
                effects_combat_start: template
                    .effects_combat_start
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
            }
        })
        .collect()
}

/// Every potion in enum declaration order. State-free and deterministic.
#[pyfunction]
pub fn get_potion_templates() -> Vec<PyPotionTemplate> {
    PotionName::iter()
        .map(|name| {
            let entity = get_potion(name);
            PyPotionTemplate {
                name: entity.potion_name.into(),
                rarity: entity.potion_rarity.into(),
                requires_target: entity_requires_target(&entity),
                combat_only: entity.potion_combat_only,
                effects: entity.potion_effects.iter().map(snapshot_effect).collect(),
            }
        })
        .collect()
}

/// Every monster in enum declaration order with the HP-roll bounds (inclusive)
/// resolved for the passed ascension. State-free and deterministic.
#[pyfunction]
pub fn get_monster_templates(ascension: u8) -> Vec<PyMonsterTemplate> {
    MonsterName::iter()
        .map(|name| {
            let template = monster_template(name);
            PyMonsterTemplate {
                name: name.into(),
                kind: template.kind.into(),
                hp_range: pick_tier(template.health_tiers, ascension)
                    .expect("health_tiers is never empty"),
            }
        })
        .collect()
}

/// Every event option variant reachable at the passed ascension, in EventName
/// declaration order, option-table order within an event. Spawn-composed menus
/// enumerate by evaluation from the spawn's own builders: We Meet Again's ask
/// covers 0 (unrolled) plus 50..=150, Neow's product covers all 37 variants.
/// State-free and deterministic.
#[pyfunction]
pub fn get_event_option_templates(ascension: u8) -> Vec<PyEventOptionTemplate> {
    let mut out = Vec::new();
    for name in EventName::iter() {
        for effects in options_catalog(name, ascension) {
            out.push(PyEventOptionTemplate {
                event: name.into(),
                effects: effects.iter().map(snapshot_effect).collect(),
            });
        }
    }
    out
}
