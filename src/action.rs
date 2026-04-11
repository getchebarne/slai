// Action handling: player input -> effects.

use crate::consts::{MAP_WIDTH, REST_SITE_HEAL_FACTOR};
use crate::effect::{Effect, EffectKind};
use crate::state::GameState;
use crate::types::{EntityId, Phase};
use crate::utils::get_alive_monster_ids;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    CardDiscard {
        idx_hand: usize,
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
        Action::CardDiscard { .. } => Phase::CombatAwaitDiscard,
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
        target: Some(state.character),
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
                        target: Some(id_monster_target),
                    },
                    Effect {
                        kind: EffectKind::CardPlay,
                        source: None,
                        target: Some(id_card),
                    },
                    Effect {
                        kind: EffectKind::TargetClear,
                        source: None,
                        target: None,
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
            target: Some(id_card),
        }])
    }
}

fn handle_card_discard(state: &GameState, idx_hand: usize) -> Result<Vec<Effect>, String> {
    let id_card = validate_idx(&state.hand, idx_hand)?;

    // Return effect to discard the card
    Ok(vec![Effect {
        kind: EffectKind::CardDiscard,
        source: None,
        target: Some(id_card),
    }])
}

fn handle_map_node_select(state: &mut GameState, idx_column: usize) -> Result<Vec<Effect>, String> {
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

    // Validate node exists
    if state.map.nodes[y_next][idx_column].is_none() {
        return Err(format!("No node at ({}, {})", y_next, idx_column));
    }

    // Validate edge from current node (skip for first move)
    if let Some(y) = state.map.y_current {
        let x = state.map.x_current.unwrap();
        let current_node = state.map.nodes[y][x].as_ref().unwrap();
        if !current_node.has_edge(idx_column) {
            return Err(format!(
                "No edge from ({}, {}) to ({}, {})",
                y, x, y_next, idx_column
            ));
        }
    }

    // Update coordinates
    state.map.y_current = Some(y_next);
    state.map.x_current = Some(idx_column);

    // Return effect to enter the room
    Ok(vec![Effect {
        kind: EffectKind::RoomEnter,
        source: None,
        target: None,
    }])
}

fn handle_card_reward_select(state: &GameState, idx_reward: usize) -> Result<Vec<Effect>, String> {
    validate_idx(&state.card_rewards, idx_reward)?;

    // The CardRewardSelect handler adds the chosen card to the deck, enqueues
    // CardRewardClear, and that handler halts on AwaitMapNode.
    Ok(vec![Effect {
        kind: EffectKind::CardRewardSelect { idx_reward },
        source: None,
        target: None,
    }])
}

fn handle_card_reward_skip() -> Vec<Effect> {
    // CardRewardClear halts on AwaitMapNode once the rewards are cleared.
    vec![Effect {
        kind: EffectKind::CardRewardClear,
        source: None,
        target: None,
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
            target: Some(id_character),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            source: None,
            target: None,
        },
    ]
}

fn handle_rest_site_card_upgrade(
    state: &GameState,
    idx_deck: usize,
) -> Result<Vec<Effect>, String> {
    validate_idx(&state.deck, idx_deck)?;

    // Upgrade, then let the RestSiteExit handler decide whether to halt or enter boss
    Ok(vec![
        Effect {
            kind: EffectKind::CardUpgrade { idx_deck },
            source: None,
            target: None,
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            source: None,
            target: None,
        },
    ])
}
