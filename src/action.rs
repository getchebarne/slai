// Action handling: player input -> effects.

use crate::consts::{MAP_HEIGHT, REST_SITE_HEAL_FACTOR};
use crate::effect::{Effect, EffectKind};
use crate::state::{GameState, Map};
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
        idx_row: usize,
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
        Action::CardRewardSkip => Ok(handle_card_reward_skip(state)),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::MapNodeSelect { idx_row } => Ok(handle_map_node_select(state, idx_row)),
        Action::RestSiteRest => Ok(handle_rest_site_rest(state)),
    }
}

fn rest_site_exit(map: &mut Map) -> Effect {
    if map.y_current == Some(MAP_HEIGHT - 1) {
        map.y_current = Some(MAP_HEIGHT);
        map.x_current = Some(0);
        Effect {
            kind: EffectKind::RoomEnter,
            source: None,
            target: None,
        }
    } else {
        Effect {
            kind: EffectKind::AwaitMapNode,
            source: None,
            target: None,
        }
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

    // Resolver target if needed
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

fn handle_card_discard(state: &mut GameState, idx_hand: usize) -> Result<Vec<Effect>, String> {
    let card_id = validate_idx(&state.hand, idx_hand)?;

    // TODO: revisit halting effects
    state.effect_queue.pop_front();

    // Return effect to discard the card
    Ok(vec![Effect {
        kind: EffectKind::CardDiscard,
        source: None,
        target: Some(card_id),
    }])
}

fn handle_map_node_select(state: &mut GameState, idx_row: usize) -> Vec<Effect> {
    // TODO: revisit halting effects
    state.effect_queue.pop_front();

    // Get next y-coordinate
    let y_next = match state.map.y_current {
        None => 0,
        Some(y_current) => y_current + 1,
    };

    // Update coordinates
    state.map.y_current = Some(y_next);
    state.map.x_current = Some(idx_row);

    // Return effect to enter the room
    vec![Effect {
        kind: EffectKind::RoomEnter,
        source: None,
        target: None,
    }]
}

fn handle_card_reward_select(
    state: &mut GameState,
    idx_reward: usize,
) -> Result<Vec<Effect>, String> {
    validate_idx(&state.card_rewards, idx_reward)?;
    // TODO: revisit halting effects
    state.effect_queue.pop_front();

    // Return effects to select the card reward and then halt the queue,
    // awaiting for the player's map node selection
    Ok(vec![
        Effect {
            kind: EffectKind::CardRewardSelect { idx_reward },
            source: None,
            target: None,
        },
        Effect {
            kind: EffectKind::AwaitMapNode,
            source: None,
            target: None,
        },
    ])
}

fn handle_card_reward_skip(state: &mut GameState) -> Vec<Effect> {
    // TODO: revisit halting effects
    state.effect_queue.pop_front();

    // Return effects to clear the card rewards and then halt the queue,
    // awaiting for the player's map node selection
    vec![
        Effect {
            kind: EffectKind::CardRewardClear,
            source: None,
            target: None,
        },
        Effect {
            kind: EffectKind::AwaitMapNode,
            source: None,
            target: None,
        },
    ]
}

fn handle_rest_site_rest(state: &mut GameState) -> Vec<Effect> {
    // Get character's vitals
    let id_character = state.character;
    let (vitals, _) = state.entities[id_character.0 as usize].kind.combatant_ref();

    // Calculate heal amount
    let heal_amt = (REST_SITE_HEAL_FACTOR * vitals.health_max as f32) as u16;

    // Create heal effect
    let heal_effect = Effect {
        kind: EffectKind::HealthGain { amount: heal_amt },
        source: None,
        target: Some(id_character),
    };

    // Return effects to heal the character and exit the rest site
    vec![heal_effect, rest_site_exit(&mut state.map)]
}

fn handle_rest_site_card_upgrade(
    state: &mut GameState,
    idx_deck: usize,
) -> Result<Vec<Effect>, String> {
    validate_idx(&state.deck, idx_deck)?;

    // Return effects to upgrade the card and exit the rest site
    Ok(vec![
        Effect {
            kind: EffectKind::CardUpgrade { deck_idx: idx_deck },
            source: None,
            target: None,
        },
        rest_site_exit(&mut state.map),
    ])
}
