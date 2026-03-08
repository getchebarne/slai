// View layer: decoupled state snapshot for Python consumption.

use pyo3::prelude::*;

use crate::cards::Card;
use crate::consts::FACTOR_VULN;
use crate::effect::{EffectKind, EffectTemplate};
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::monsters::Intent;
use crate::state::{Entity, GameState};
use crate::types::EntityId;

// ---------------------------------------------------------------------------
// View types (PyO3 classes)
// ---------------------------------------------------------------------------

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewEffectTemplate {
    pub effect_type: String,
    pub value: Option<i32>,
    pub target: Option<String>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewCard {
    pub name: String,
    pub kind: String,
    pub color: String,
    pub rarity: String,
    pub cost: u8,
    pub upgraded: bool,
    pub exhaust: bool,
    pub innate: bool,
    pub is_active: bool,
    pub requires_target: bool,
    pub effects: Vec<ViewEffectTemplate>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewModifier {
    pub kind: String,
    pub stacks: i16,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewCharacter {
    pub name: String,
    pub health_current: u16,
    pub health_max: u16,
    pub block_current: u16,
    pub modifiers: Vec<ViewModifier>,
    pub card_reward_roll_offset: i8,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewIntent {
    pub damage: Option<u16>,
    pub instances: Option<u8>,
    pub block: bool,
    pub buff: bool,
    pub debuff: bool,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMonster {
    pub name: String,
    pub health_current: u16,
    pub health_max: u16,
    pub block_current: u16,
    pub modifiers: Vec<ViewModifier>,
    pub intent: ViewIntent,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewEnergy {
    pub current: u8,
    pub max: u8,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMapNode {
    pub room_type: String,
    pub x_next: Vec<usize>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMap {
    pub nodes: Vec<Vec<Option<ViewMapNode>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewGameState {
    pub character: ViewCharacter,
    pub monsters: Vec<ViewMonster>,
    pub deck: Vec<ViewCard>,
    pub hand: Vec<ViewCard>,
    pub pile_draw: Vec<ViewCard>,
    pub pile_disc: Vec<ViewCard>,
    pub pile_exhaust: Vec<ViewCard>,
    pub reward_combat: Vec<ViewCard>,
    pub energy: ViewEnergy,
    pub map: ViewMap,
    pub phase: String,
}

// ---------------------------------------------------------------------------
// Build functions
// ---------------------------------------------------------------------------

pub fn build_view(state: &GameState) -> ViewGameState {
    ViewGameState {
        character: build_view_character(state),
        monsters: build_view_monsters(state),
        deck: state
            .deck
            .iter()
            .map(|&id| {
                let card = state.entities[id.0 as usize].kind.card_ref();
                build_view_card_template(card, false)
            })
            .collect(),
        hand: state
            .hand
            .iter()
            .map(|&id| {
                let card = state.entities[id.0 as usize].kind.card_ref();
                build_view_card_template(card, false)
            })
            .collect(),
        pile_draw: build_view_pile(&state.entities, &state.draw_pile),
        pile_disc: build_view_pile(&state.entities, &state.discard_pile),
        pile_exhaust: build_view_pile(&state.entities, &state.exhaust_pile),
        reward_combat: state
            .card_rewards
            .iter()
            .map(|&id| {
                let card = state.entities[id.0 as usize].kind.card_ref();
                build_view_card_template(card, false)
            })
            .collect(),
        energy: ViewEnergy {
            current: state.energy.current,
            max: state.energy.max,
        },
        map: build_view_map(state),
        phase: format!("{:?}", state.phase),
    }
}

fn build_view_pile(entities: &[Entity], pile: &[EntityId]) -> Vec<ViewCard> {
    pile.iter()
        .map(|&id| {
            let card = entities[id.0 as usize].kind.card_ref();
            build_view_card_template(card, false)
        })
        .collect()
}

fn build_view_character(state: &GameState) -> ViewCharacter {
    let c = state.entities[0].kind.character_ref();
    ViewCharacter {
        name: "Silent".to_string(),
        health_current: c.vitals.health,
        health_max: c.vitals.health_max,
        block_current: c.vitals.block,
        modifiers: build_view_modifiers(&c.modifiers),
        card_reward_roll_offset: c.reward_roll_offset,
    }
}

fn build_view_monsters(state: &GameState) -> Vec<ViewMonster> {
    let (_, character_modifiers) = state.entities[0].kind.combatant_ref();

    state
        .alive_monster_ids()
        .iter()
        .map(|&mid| {
            let m = state.entities[mid.0 as usize].kind.monster_ref();

            let intent = if let Some(move_idx) = m.move_current {
                let mv = &m.moves[move_idx];
                let (base_damage, instances, block, buff, debuff) = match mv.intent {
                    Intent::Attack { damage, instances } => {
                        (Some(damage), Some(instances), false, false, false)
                    }
                    Intent::AttackBlock { damage, instances } => {
                        (Some(damage), Some(instances), true, false, false)
                    }
                    Intent::AttackBuff { damage, instances } => {
                        (Some(damage), Some(instances), false, true, false)
                    }
                    Intent::Block => (None, None, true, false, false),
                    Intent::BlockBuff => (None, None, true, true, false),
                    Intent::Buff => (None, None, false, true, false),
                    Intent::Debuff => (None, None, false, false, true),
                    Intent::DebuffPowerful => (None, None, false, false, true),
                };

                let damage = base_damage.map(|d| {
                    let mut dmg = d as f32;
                    if modifier_has(&m.modifiers, ModifierKind::Strength) {
                        dmg += modifier_stacks(&m.modifiers, ModifierKind::Strength) as f32;
                    }
                    if modifier_has(&m.modifiers, ModifierKind::Weak) {
                        dmg *= 0.75;
                    }
                    if modifier_has(character_modifiers, ModifierKind::Vulnerable) {
                        dmg *= FACTOR_VULN;
                    }
                    dmg as u16
                });

                ViewIntent {
                    damage,
                    instances,
                    block,
                    buff,
                    debuff,
                }
            } else {
                ViewIntent {
                    damage: None,
                    instances: None,
                    block: false,
                    buff: false,
                    debuff: false,
                }
            };

            ViewMonster {
                name: m.name.as_str().to_string(),
                health_current: m.vitals.health,
                health_max: m.vitals.health_max,
                block_current: m.vitals.block,
                modifiers: build_view_modifiers(&m.modifiers),
                intent,
            }
        })
        .collect()
}

fn build_view_modifiers(mods: &crate::modifier::Modifiers) -> Vec<ViewModifier> {
    let mut out = Vec::new();
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = ModifierKind::from_u8(idx as u8);
        out.push(ViewModifier {
            kind: format!("{:?}", kind),
            stacks: mods.stacks[idx],
        });
    }
    out
}

fn build_view_card_template(card: &Card, is_active: bool) -> ViewCard {
    ViewCard {
        name: if card.upgraded {
            format!("{}+", card.name.as_str())
        } else {
            card.name.as_str().to_string()
        },
        kind: format!("{:?}", card.kind),
        color: format!("{:?}", card.color),
        rarity: format!("{:?}", card.rarity),
        cost: card.cost,
        upgraded: card.upgraded,
        exhaust: card.exhaust,
        innate: card.innate,
        is_active,
        requires_target: card.requires_target,
        effects: card.effects.iter().map(view_effect_template).collect(),
    }
}

fn view_effect_template(tmpl: &EffectTemplate) -> ViewEffectTemplate {
    let target_str = tmpl.targeting.map(|t| format!("{:?}", t.candidates));
    match tmpl.kind {
        EffectKind::DamagePhysical { base } => ViewEffectTemplate {
            effect_type: "DamagePhysical".to_string(),
            value: Some(base as i32),
            target: target_str,
        },
        EffectKind::BlockGain { amount, .. } => ViewEffectTemplate {
            effect_type: "BlockGain".to_string(),
            value: Some(amount as i32),
            target: target_str,
        },
        EffectKind::ModifierGain { kind, stacks } => ViewEffectTemplate {
            effect_type: format!("ModifierGain_{:?}", kind),
            value: Some(stacks as i32),
            target: target_str,
        },
        EffectKind::ModifierRemove { kind } => ViewEffectTemplate {
            effect_type: format!("ModifierRemove_{:?}", kind),
            value: None,
            target: target_str,
        },
        EffectKind::EnergyGain { amount } => ViewEffectTemplate {
            effect_type: "EnergyGain".to_string(),
            value: Some(amount as i32),
            target: None,
        },
        EffectKind::AddShivs { count } => ViewEffectTemplate {
            effect_type: "AddShivs".to_string(),
            value: Some(count as i32),
            target: None,
        },
        EffectKind::CardDraw { count } => ViewEffectTemplate {
            effect_type: "CardDraw".to_string(),
            value: Some(count as i32),
            target: None,
        },
        EffectKind::CardDiscard => {
            let selection_str = tmpl.targeting.map(|t| format!("{:?}", t.selection));
            ViewEffectTemplate {
                effect_type: format!("CardDiscard_{}", selection_str.unwrap_or_default()),
                value: None,
                target: None,
            }
        },
        EffectKind::CalculatedGamble => ViewEffectTemplate {
            effect_type: "CalculatedGamble".to_string(),
            value: None,
            target: None,
        },
        _ => ViewEffectTemplate {
            effect_type: format!("{:?}", tmpl.kind),
            value: None,
            target: None,
        },
    }
}

fn build_view_map(state: &GameState) -> ViewMap {
    let nodes = state
        .map
        .nodes
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| {
                    n.as_ref().map(|node| ViewMapNode {
                        room_type: format!("{:?}", node.room_type),
                        x_next: node.x_next.clone(),
                    })
                })
                .collect()
        })
        .collect();

    ViewMap {
        nodes,
        y_current: state.map.active_y,
        x_current: state.map.active_x,
    }
}
