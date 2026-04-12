// Action handling: player input -> effects.

use crate::consts::{MAP_WIDTH, REST_SITE_HEAL_FACTOR};
use crate::effect::{Effect, EffectKind, Target};
use crate::state::GameState;
use crate::types::{EntityId, Phase};
use crate::utils::get_alive_monster_ids;

#[derive(Debug, Clone)]
pub enum Action {
    CardDiscard {
        idx_hand: Vec<usize>,
    },
    CardPlay {
        idx_hand: usize,
        idx_monster: Option<usize>,
    },
    RestSiteCardUpgrade {
        idx_deck: usize,
    },
    CardRewardSelect {
        idx_reward: usize,
    },
    CardRewardSkip,
    EndTurn,
    MapNodeSelect {
        idx_column: usize,
    },
    RestSiteRest,
}

fn validate_phase(action: &Action, current_phase: Phase) -> Result<(), String> {
    let valid = match (action, current_phase) {
        (Action::CardDiscard { idx_hand }, Phase::CombatAwaitDiscard { num }) => {
            idx_hand.len() == num as usize
        }
        (Action::CardPlay { .. } | Action::EndTurn, Phase::CombatDefault) => true,
        (Action::RestSiteCardUpgrade { .. } | Action::RestSiteRest, Phase::RestSite) => true,
        (Action::CardRewardSelect { .. } | Action::CardRewardSkip, Phase::CombatReward) => true,
        (Action::MapNodeSelect { .. }, Phase::Map) => true,
        _ => false,
    };
    if !valid {
        return Err(format!("{:?} invalid in phase {:?}", action, current_phase));
    }
    Ok(())
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    validate_phase(&action, state.phase)?;

    let effects = match action {
        Action::CardDiscard { idx_hand } => handle_card_discard(state, idx_hand),
        Action::CardPlay {
            idx_hand,
            idx_monster,
        } => handle_card_play(state, idx_hand, idx_monster),
        Action::RestSiteCardUpgrade { idx_deck } => handle_rest_site_card_upgrade(state, idx_deck),
        Action::CardRewardSelect { idx_reward } => handle_card_reward_select(state, idx_reward),
        Action::CardRewardSkip => Ok(handle_card_reward_skip()),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::MapNodeSelect { idx_column } => handle_map_node_select(state, idx_column),
        Action::RestSiteRest => Ok(handle_rest_site_rest(state)),
    }?;

    Ok(effects)
}

fn validate_idx(slice: &[EntityId], idx: usize) -> Result<EntityId, String> {
    slice
        .get(idx)
        .copied()
        .ok_or_else(|| format!("Invalid index {}: {} available", idx, slice.len()))
}

fn handle_end_turn(state: &GameState) -> Vec<Effect> {
    // Return effect to end the character's turn
    vec![Effect {
        kind: EffectKind::TurnEnd,
        source: None,
        target: Target::Direct(Some(state.character)),
    }]
}

fn handle_card_play(
    state: &mut GameState,
    idx_hand: usize,
    idx_monster: Option<usize>,
) -> Result<Vec<Effect>, String> {
    let id_card = validate_idx(&state.hand, idx_hand)?;
    let card = state.entities[id_card.0 as usize].kind.card_ref();

    // Check energy
    if card.cost > state.energy.current {
        return Err(format!(
            "Not enough energy to play {:?}: need {}, have {}",
            card.name, card.cost, state.energy.current
        ));
    }

    // Resolve target if needed
    if card.requires_target {
        match idx_monster {
            Some(idx_monster) => {
                let id_monsters_alive = get_alive_monster_ids(state);
                let id_monster_target = *id_monsters_alive
                    .get(idx_monster)
                    .ok_or_else(|| format!("Invalid monster index: {}", idx_monster))?;

                // Return effects to set the card's target, play the card, and then clear it
                Ok(vec![
                    Effect {
                        kind: EffectKind::TargetSet,
                        source: None,
                        target: Target::Direct(Some(id_monster_target)),
                    },
                    Effect {
                        kind: EffectKind::CardPlay,
                        source: None,
                        target: Target::Direct(Some(id_card)),
                    },
                    Effect {
                        kind: EffectKind::TargetClear,
                        source: None,
                        target: Target::Direct(None),
                    },
                    Effect::direct(EffectKind::AwaitCombatAction, None, None),
                ])
            }
            None => Err(format!(
                "Card {:?} requires a target: provide idx_monster",
                card.name
            )),
        }
    } else {
        // Return effect to play the card
        Ok(vec![
            Effect {
                kind: EffectKind::CardPlay,
                source: None,
                target: Target::Direct(Some(id_card)),
            },
            Effect::direct(EffectKind::AwaitCombatAction, None, None),
        ])
    }
}

fn handle_card_discard(state: &GameState, idx_hand: Vec<usize>) -> Result<Vec<Effect>, String> {
    let mut effects = Vec::with_capacity(idx_hand.len() + 1);
    for &idx in &idx_hand {
        let id = *state
            .hand
            .get(idx)
            .ok_or_else(|| format!("Invalid hand index {}: {} cards", idx, state.hand.len()))?;
        effects.push(Effect::direct(EffectKind::CardDiscard, None, Some(id)));
    }
    effects.push(Effect::direct(EffectKind::AwaitCombatAction, None, None));
    Ok(effects)
}

fn handle_map_node_select(state: &GameState, idx_column: usize) -> Result<Vec<Effect>, String> {
    if idx_column >= MAP_WIDTH {
        return Err(format!(
            "Invalid column {}: max is {}",
            idx_column,
            MAP_WIDTH - 1
        ));
    }

    // Compute next y-coordinate
    let y_next = match state.map.y_current {
        None => 0,
        Some(y) => y + 1,
    };

    // Validate node exists at (y_next, idx_column)
    let target_id = state.map.nodes[y_next][idx_column]
        .ok_or_else(|| format!("No node at ({}, {})", y_next, idx_column))?;

    // Validate edge from current node (skip for first move)
    if let Some(y) = state.map.y_current {
        let x = state.map.x_current.unwrap();
        let current_node = state
            .map
            .node_at(&state.entities, y, x)
            .expect("current map node missing");
        if !current_node.has_edge(idx_column) {
            return Err(format!(
                "No edge from ({}, {}) to ({}, {})",
                y, x, y_next, idx_column
            ));
        }
    }

    // Return a Direct MapNodeSelect effect. Its dispatch arm will update
    // state.map.y_current/x_current and push a RoomEnter effect.
    Ok(vec![Effect::direct(
        EffectKind::MapNodeSelect,
        None,
        Some(target_id),
    )])
}

fn handle_card_reward_select(state: &GameState, idx_reward: usize) -> Result<Vec<Effect>, String> {
    let id_card = validate_idx(&state.card_rewards, idx_reward)?;

    // Direct CardRewardSelect: handler adds the target card to the deck and
    // enqueues CardRewardClear, which in turn enqueues MapNodeSelect.
    Ok(vec![Effect::direct(
        EffectKind::CardRewardSelect,
        None,
        Some(id_card),
    )])
}

fn handle_card_reward_skip() -> Vec<Effect> {
    // CardRewardClear halts on AwaitMapNode once the rewards are cleared.
    vec![Effect {
        kind: EffectKind::CardRewardClear,
        source: None,
        target: Target::Direct(None),
    }]
}

fn handle_rest_site_rest(state: &GameState) -> Vec<Effect> {
    // Get character's vitals
    let id_character = state.character;
    let (vitals, _) = state.entities[id_character.0 as usize].kind.combatant_ref();

    // Calculate heal amount
    let heal_amt = (REST_SITE_HEAL_FACTOR * vitals.health_max as f32) as u16;

    // Heal, then let the RestSiteExit handler decide whether to halt or enter boss
    vec![
        Effect {
            kind: EffectKind::HealthGain { amount: heal_amt },
            source: None,
            target: Target::Direct(Some(id_character)),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            source: None,
            target: Target::Direct(None),
        },
    ]
}

fn handle_rest_site_card_upgrade(
    state: &GameState,
    idx_deck: usize,
) -> Result<Vec<Effect>, String> {
    let id_card = validate_idx(&state.deck, idx_deck)?;

    // Upgrade by entity id, then let the RestSiteExit handler decide whether
    // to halt (non-final row) or enter the boss room (final row).
    Ok(vec![
        Effect::direct(EffectKind::CardUpgrade, None, Some(id_card)),
        Effect::direct(EffectKind::RestSiteExit, None, None),
    ])
}
