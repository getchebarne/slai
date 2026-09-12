use pyo3::prelude::*;
use strum::EnumCount;
use strum::IntoEnumIterator;

use crate::cards::CardTemplate;
use crate::cards::card_template;
use crate::entity::Intent;
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
use super::modifier::PyModifierKind;
use super::monster::PyIntentKind;
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
    pub effects_turn_start: Vec<PyEffect>,
    pub effects_turn_end: Vec<PyEffect>,
    pub effects_combat_end: Vec<PyEffect>,
    pub effects_on_pickup: Vec<PyEffect>,
    pub effects_on_rest: Vec<PyEffect>,
    pub counter_reset: i16,
    pub effects_counter: Vec<PyEffect>,
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
    name = "ModifierSpawnTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyModifierSpawnTemplate {
    pub kind: PyModifierKind,
    pub stacks_min: i16,
    pub stacks_max: i16,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "MonsterMoveTemplate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyMonsterMoveTemplate {
    pub name: String,
    pub intent: PyIntentKind,
    pub damage: Option<u16>, // Raw damage, before Strength / Weak / Vulnerable / etc. scaling
    pub instances: Option<u8>,
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
    pub block_start: u16,
    pub moves: Vec<PyMonsterMoveTemplate>,
    pub modifier_spawns: Vec<PyModifierSpawnTemplate>,
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

// Every constructible Card in enum declaration order
#[pyfunction]
pub fn get_card_templates() -> Vec<PyCardTemplate> {
    let mut out = Vec::with_capacity(2 * CardName::COUNT);
    for name in CardName::iter() {
        // Normal
        out.push(template_card(card_template(name, false)));

        // Upgraded
        let upgraded = card_template(name, true);
        if upgraded.upgraded {
            out.push(template_card(upgraded));
        }
    }
    out
}

// Every Relic in enum declaration order
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
                effects_turn_start: template
                    .effects_turn_start
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
                effects_turn_end: template
                    .effects_turn_end
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
                effects_combat_end: template
                    .effects_combat_end
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
                effects_on_pickup: template
                    .effects_on_pickup
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
                effects_on_rest: template
                    .effects_on_rest
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
                counter_reset: template.counter_reset,
                effects_counter: template
                    .effects_counter
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
            }
        })
        .collect()
}

// Every Potion in enum declaration order
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

// Every Monster in enum declaration order
#[pyfunction]
pub fn get_monster_templates(ascension: u8) -> Vec<PyMonsterTemplate> {
    MonsterName::iter()
        .map(|name| {
            let template = monster_template(name);
            let moveset =
                pick_tier(template.move_tiers, ascension).expect("`move_tiers` is never empty");
            let moves = moveset
                .iter()
                .flat_map(|moveset| moveset.iter())
                .map(|move_| {
                    let (damage, instances) = match move_.intent {
                        Intent::Attack { damage, instances }
                        | Intent::AttackBlock { damage, instances }
                        | Intent::AttackBuff { damage, instances }
                        | Intent::AttackDebuff { damage, instances } => {
                            (Some(damage), Some(instances))
                        }
                        _ => (None, None),
                    };
                    PyMonsterMoveTemplate {
                        name: move_.name.to_string(),
                        intent: move_.intent.into(),
                        damage,
                        instances,
                        effects: move_.effects[..move_.effects_len as usize]
                            .iter()
                            .map(snapshot_effect)
                            .collect(),
                    }
                })
                .collect();
            PyMonsterTemplate {
                name: name.into(),
                kind: template.kind.into(),
                hp_range: pick_tier(template.health_tiers, ascension)
                    .expect("health_tiers is never empty"),
                block_start: template.block_start,
                moves,
                modifier_spawns: pick_tier(template.modifier_tiers, ascension)
                    .unwrap_or(&[])
                    .iter()
                    .map(|spawn| PyModifierSpawnTemplate {
                        kind: spawn.kind.into(),
                        stacks_min: spawn.stacks_min,
                        stacks_max: spawn.stacks_max,
                    })
                    .collect(),
            }
        })
        .collect()
}

// Every Event Option variant reachable at the passed ascension
#[pyfunction]
pub fn get_event_option_templates(ascension: u8) -> Vec<PyEventOptionTemplate> {
    let mut out = Vec::new();
    for name in EventName::iter() {
        for template in options_catalog(name, ascension) {
            out.push(PyEventOptionTemplate {
                event: name.into(),
                effects: template.effects[..template.effects_len as usize]
                    .iter()
                    .map(snapshot_effect)
                    .collect(),
            });
        }
    }
    out
}
