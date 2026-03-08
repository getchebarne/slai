// Action handling: player input -> effects.

use crate::consts::{MAP_HEIGHT, REST_SITE_HEAL_FACTOR};
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
    CardUpgrade {
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
    Rest,
}

fn validate_phase(current_phase: Phase, action: &Action) -> Result<(), String> {
    let expected = match action {
        Action::CardDiscard { .. } => Phase::CombatAwaitDiscard,
        Action::CardPlay { .. } | Action::EndTurn => Phase::CombatDefault,
        Action::CardUpgrade { .. } | Action::Rest => Phase::RestSite,
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
        Action::CardUpgrade { idx_deck } => handle_card_upgrade(state, idx_deck),
        Action::CardRewardSelect { idx_reward } => handle_card_reward_select(state, idx_reward),
        Action::CardRewardSkip => Ok(handle_card_reward_skip(state)),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::MapNodeSelect { idx_row } => Ok(handle_map_node_select(state, idx_row)),
        Action::Rest => Ok(handle_rest(state)),
    }
}

fn handle_end_turn(state: &GameState) -> Vec<Effect> {
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
    if idx_hand >= state.hand.len() {
        return Err(format!(
            "Invalid hand index {}: hand has {} cards",
            idx_hand,
            state.hand.len()
        ));
    }

    // Get card
    let id_card = state.hand[idx_hand];
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
            Some(idx) => {
                let alive = get_alive_monster_ids(state);
                let target = *alive
                    .get(idx)
                    .ok_or_else(|| format!("Invalid monster index: {}", idx))?;
                Ok(vec![
                    Effect {
                        kind: EffectKind::TargetSet,
                        source: None,
                        target: Some(target),
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
        Ok(vec![Effect {
            kind: EffectKind::CardPlay,
            source: None,
            target: Some(id_card),
        }])
    }
}

fn handle_card_discard(state: &mut GameState, idx_hand: usize) -> Result<Vec<Effect>, String> {
    if idx_hand >= state.hand.len() {
        return Err(format!(
            "Invalid hand index {}: hand has {} cards",
            idx_hand,
            state.hand.len()
        ));
    }
    let card_id = state.hand[idx_hand];

    // TODO: revisit
    state.effect_queue.pop_front();
    Ok(vec![Effect {
        kind: EffectKind::CardDiscard,
        source: None,
        target: Some(card_id),
    }])
}

fn handle_map_node_select(state: &mut GameState, idx_row: usize) -> Vec<Effect> {
    state.effect_queue.pop_front();

    let y = match state.map.active_y {
        None => 0,
        Some(prev) => prev + 1,
    };

    state.map.active_y = Some(y);
    state.map.active_x = Some(idx_row);

    state.card_rewards.clear();

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
    if idx_reward >= state.card_rewards.len() {
        return Err(format!(
            "Invalid reward index {}: {} rewards available",
            idx_reward,
            state.card_rewards.len()
        ));
    }
    state.effect_queue.pop_front();
    Ok(vec![
        Effect {
            kind: EffectKind::CardRewardSelect {
                reward_idx: idx_reward,
            },
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
    state.effect_queue.pop_front();
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

fn handle_rest(state: &mut GameState) -> Vec<Effect> {
    let (vitals, _) = state.entities[0].kind.combatant_ref();
    let heal = (REST_SITE_HEAL_FACTOR * vitals.health_max as f32) as u16;

    let is_last_floor = state.map.active_y == Some(MAP_HEIGHT - 1);

    let mut effects = vec![Effect {
        kind: EffectKind::HealthGain { amount: heal },
        source: None,
        target: Some(EntityId(0)),
    }];

    if is_last_floor {
        state.map.active_y = Some(MAP_HEIGHT);
        state.map.active_x = Some(0);
        effects.push(Effect {
            kind: EffectKind::RoomEnter,
            source: None,
            target: None,
        });
    } else {
        effects.push(Effect {
            kind: EffectKind::AwaitMapNode,
            source: None,
            target: None,
        });
    }

    effects
}

fn handle_card_upgrade(state: &mut GameState, idx_deck: usize) -> Result<Vec<Effect>, String> {
    if idx_deck >= state.deck.len() {
        return Err(format!(
            "Invalid deck index {}: deck has {} cards",
            idx_deck,
            state.deck.len()
        ));
    }

    let is_last_floor = state.map.active_y == Some(MAP_HEIGHT - 1);

    let mut effects = vec![Effect {
        kind: EffectKind::CardUpgrade { deck_idx: idx_deck },
        source: None,
        target: None,
    }];

    if is_last_floor {
        state.map.active_y = Some(MAP_HEIGHT);
        state.map.active_x = Some(0);
        effects.push(Effect {
            kind: EffectKind::RoomEnter,
            source: None,
            target: None,
        });
    } else {
        effects.push(Effect {
            kind: EffectKind::AwaitMapNode,
            source: None,
            target: None,
        });
    }

    Ok(effects)
}
