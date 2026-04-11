// Action handling: player input -> effects.

use crate::consts::{MAP_WIDTH, REST_SITE_HEAL_FACTOR};
use crate::effect::{Effect, EffectKind, SelectionKind, Targeting};
use crate::engine::resolve_candidates;
use crate::state::GameState;
use crate::types::{EntityId, Phase};
use crate::utils::get_alive_monster_ids;

#[derive(Debug, Clone)]
pub enum Action {
    InputResolve {
        indices: Vec<usize>,
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

fn validate_phase(current_phase: Phase, action: &Action) -> Result<(), String> {
    let expected = match action {
        Action::InputResolve { .. } => Phase::CombatAwaitInput,
        Action::CardPlay { .. } | Action::EndTurn => Phase::CombatDefault,
        Action::RestSiteCardUpgrade { .. } | Action::RestSiteRest => Phase::RestSite,
        Action::CardRewardSelect { .. } | Action::CardRewardSkip => Phase::CardReward,
        Action::MapNodeSelect { .. } => Phase::Map,
    };
    if current_phase != expected {
        return Err(format!(
            "{:?} invalid in phase {:?} (expected {:?})",
            action, current_phase, expected
        ));
    }
    Ok(())
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    validate_phase(state.phase, &action)?;

    match action {
        Action::InputResolve { indices } => handle_input_resolve(state, indices),
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
    }
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
        targeting: Targeting::Direct(Some(state.character)),
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
                        targeting: Targeting::Direct(Some(id_monster_target)),
                    },
                    Effect {
                        kind: EffectKind::CardPlay,
                        source: None,
                        targeting: Targeting::Direct(Some(id_card)),
                    },
                    Effect {
                        kind: EffectKind::TargetClear,
                        source: None,
                        targeting: Targeting::Direct(None),
                    },
                ])
            }
            None => Err(format!(
                "Card {:?} requires a target: provide idx_monster",
                card.name
            )),
        }
    } else {
        // Return effect to play the card
        Ok(vec![Effect {
            kind: EffectKind::CardPlay,
            source: None,
            targeting: Targeting::Direct(Some(id_card)),
        }])
    }
}

fn handle_input_resolve(state: &GameState, indices: Vec<usize>) -> Result<Vec<Effect>, String> {
    // Peek the unresolved halt effect at the queue front. step() will pop it
    // after we return Ok.
    let unresolved = state
        .effect_queue
        .front()
        .ok_or_else(|| "No halt effect at queue front".to_string())?;

    let (candidates, count) = match unresolved.targeting {
        Targeting::Resolve {
            candidates,
            selection: SelectionKind::Input { count },
        } => (candidates, count),
        _ => return Err("Queue front is not an unresolved input prompt".into()),
    };

    if indices.len() != count as usize {
        return Err(format!("Expected {} picks, got {}", count, indices.len()));
    }

    // Re-resolve candidates against current state.
    let alive = get_alive_monster_ids(state);
    let src_id = unresolved.source.unwrap_or(state.character);
    let ids = resolve_candidates(
        candidates,
        src_id,
        state.character,
        &state.hand,
        state.card_target,
        &alive,
        &state.map,
        &state.entities,
        &state.card_rewards,
    );

    let mut effects = Vec::with_capacity(indices.len());
    for &idx in &indices {
        let target = *ids
            .get(idx)
            .ok_or_else(|| format!("Invalid index {}: {} candidates", idx, ids.len()))?;
        effects.push(Effect {
            kind: unresolved.kind,
            source: unresolved.source,
            targeting: Targeting::Direct(Some(target)),
        });
    }

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

    // Return a Direct SelectMapNode effect. Its dispatch arm will update
    // state.map.y_current/x_current and push a RoomEnter effect.
    Ok(vec![Effect::direct(
        EffectKind::SelectMapNode,
        None,
        Some(target_id),
    )])
}

fn handle_card_reward_select(state: &GameState, idx_reward: usize) -> Result<Vec<Effect>, String> {
    let id_card = validate_idx(&state.card_rewards, idx_reward)?;

    // Direct SelectCardReward: handler adds the target card to the deck and
    // enqueues CardRewardClear, which in turn enqueues SelectMapNode.
    Ok(vec![Effect::direct(
        EffectKind::SelectCardReward,
        None,
        Some(id_card),
    )])
}

fn handle_card_reward_skip() -> Vec<Effect> {
    // CardRewardClear halts on AwaitMapNode once the rewards are cleared.
    vec![Effect {
        kind: EffectKind::CardRewardClear,
        source: None,
        targeting: Targeting::Direct(None),
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
            targeting: Targeting::Direct(Some(id_character)),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            source: None,
            targeting: Targeting::Direct(None),
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
