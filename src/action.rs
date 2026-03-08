// Action handling: player input -> effects.

use crate::consts::REST_SITE_HEAL_FACTOR;
use crate::effect::{Effect, EffectKind};
use crate::state::GameState;
use crate::types::{EntityId, Phase};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    CardDiscard {
        hand_idx: usize,
    },
    CardPlay {
        hand_idx: usize,
        monster_idx: Option<usize>,
    },
    CardUpgrade {
        deck_idx: usize,
    },
    CardRewardSelect {
        reward_idx: usize,
    },
    CardRewardSkip,
    EndTurn,
    MapNodeSelect {
        column: usize,
    },
    Rest,
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    let phase = state.phase;
    match action {
        Action::CardDiscard { hand_idx } => {
            require_phase(phase, Phase::CombatAwaitDiscard, &action)?;
            handle_card_discard(state, hand_idx)
        }
        Action::CardPlay {
            hand_idx,
            monster_idx,
        } => {
            require_phase(phase, Phase::CombatDefault, &action)?;
            handle_card_play(state, hand_idx, monster_idx)
        }
        Action::CardUpgrade { deck_idx } => {
            require_phase(phase, Phase::RestSite, &action)?;
            handle_card_upgrade(state, deck_idx)
        }
        Action::CardRewardSelect { reward_idx } => {
            require_phase(phase, Phase::CardReward, &action)?;
            handle_card_reward_select(state, reward_idx)
        }
        Action::CardRewardSkip => {
            require_phase(phase, Phase::CardReward, &action)?;
            Ok(handle_card_reward_skip(state))
        }
        Action::EndTurn => {
            require_phase(phase, Phase::CombatDefault, &action)?;
            Ok(handle_end_turn(state))
        }
        Action::MapNodeSelect { column } => {
            require_phase(phase, Phase::Map, &action)?;
            Ok(handle_map_node_select(state, column))
        }
        Action::Rest => {
            require_phase(phase, Phase::RestSite, &action)?;
            Ok(handle_rest(state))
        }
    }
}

fn require_phase(current: Phase, expected: Phase, action: &Action) -> Result<(), String> {
    if current != expected {
        return Err(format!(
            "{:?} invalid in phase {:?} (expected {:?})",
            action, current, expected
        ));
    }
    Ok(())
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
    hand_idx: usize,
    monster_idx: Option<usize>,
) -> Result<Vec<Effect>, String> {
    if hand_idx >= state.hand.len() {
        return Err(format!(
            "Invalid hand index {}: hand has {} cards",
            hand_idx,
            state.hand.len()
        ));
    }

    let card_id = state.hand[hand_idx];
    let card = state.entities[card_id.0 as usize].kind.card_ref();

    if card.cost > state.energy.current {
        return Err(format!(
            "Not enough energy to play {:?}: need {}, have {}",
            card.name, card.cost, state.energy.current
        ));
    }

    if card.requires_target {
        match monster_idx {
            Some(idx) => {
                let alive = state.alive_monster_ids();
                let target = *alive
                    .get(idx)
                    .ok_or_else(|| format!("Invalid monster index: {}", idx))?;
                Ok(vec![
                    Effect { kind: EffectKind::TargetSet, source: None, target: Some(target) },
                    Effect { kind: EffectKind::CardPlay, source: None, target: Some(card_id) },
                    Effect { kind: EffectKind::TargetClear, source: None, target: None },
                ])
            }
            None => Err(format!(
                "Card {:?} requires a target: provide monster_idx",
                card.name
            )),
        }
    } else {
        Ok(vec![Effect { kind: EffectKind::CardPlay, source: None, target: Some(card_id) }])
    }
}

fn handle_card_discard(state: &mut GameState, hand_idx: usize) -> Result<Vec<Effect>, String> {
    if hand_idx >= state.hand.len() {
        return Err(format!(
            "Invalid hand index {}: hand has {} cards",
            hand_idx,
            state.hand.len()
        ));
    }
    let card_id = state.hand[hand_idx];

    // TODO: revisit
    state.effect_queue.pop_front();
    Ok(vec![Effect { kind: EffectKind::CardDiscard, source: None, target: Some(card_id) }])
}

fn handle_map_node_select(state: &mut GameState, column: usize) -> Vec<Effect> {
    state.effect_queue.pop_front();

    let y = match state.map.active_y {
        None => 0,
        Some(prev) => prev + 1,
    };

    state.map.active_y = Some(y);
    state.map.active_x = Some(column);

    state.card_rewards.clear();

    vec![Effect { kind: EffectKind::RoomEnter, source: None, target: None }]
}

fn handle_card_reward_select(
    state: &mut GameState,
    reward_idx: usize,
) -> Result<Vec<Effect>, String> {
    if reward_idx >= state.card_rewards.len() {
        return Err(format!(
            "Invalid reward index {}: {} rewards available",
            reward_idx,
            state.card_rewards.len()
        ));
    }
    state.effect_queue.pop_front();
    Ok(vec![
        Effect { kind: EffectKind::CardRewardSelect { reward_idx }, source: None, target: None },
        Effect { kind: EffectKind::AwaitMapNode, source: None, target: None },
    ])
}

fn handle_card_reward_skip(state: &mut GameState) -> Vec<Effect> {
    state.effect_queue.pop_front();
    vec![
        Effect { kind: EffectKind::CardRewardClear, source: None, target: None },
        Effect { kind: EffectKind::AwaitMapNode, source: None, target: None },
    ]
}

fn handle_rest(state: &mut GameState) -> Vec<Effect> {
    let (vitals, _) = state.entities[0].kind.combatant_ref();
    let heal = (REST_SITE_HEAL_FACTOR * vitals.health_max as f32) as u16;

    let is_last_floor = state.map.active_y == Some(crate::consts::MAP_HEIGHT - 1);

    let mut effects = vec![Effect {
        kind: EffectKind::HealthGain { amount: heal },
        source: None,
        target: Some(EntityId(0)),
    }];

    if is_last_floor {
        state.map.active_y = Some(state.map.boss_room_y);
        state.map.active_x = Some(0);
        effects.push(Effect { kind: EffectKind::RoomEnter, source: None, target: None });
    } else {
        effects.push(Effect { kind: EffectKind::AwaitMapNode, source: None, target: None });
    }

    effects
}

fn handle_card_upgrade(state: &mut GameState, deck_idx: usize) -> Result<Vec<Effect>, String> {
    if deck_idx >= state.deck.len() {
        return Err(format!(
            "Invalid deck index {}: deck has {} cards",
            deck_idx,
            state.deck.len()
        ));
    }

    let is_last_floor = state.map.active_y == Some(crate::consts::MAP_HEIGHT - 1);

    let mut effects = vec![Effect { kind: EffectKind::CardUpgrade { deck_idx }, source: None, target: None }];

    if is_last_floor {
        state.map.active_y = Some(state.map.boss_room_y);
        state.map.active_x = Some(0);
        effects.push(Effect { kind: EffectKind::RoomEnter, source: None, target: None });
    } else {
        effects.push(Effect { kind: EffectKind::AwaitMapNode, source: None, target: None });
    }

    Ok(effects)
}
