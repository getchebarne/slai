// Action handling: player input -> effects.

use crate::effect::Effect;
use crate::process::REST_SITE_HEAL_FACTOR;
use crate::state::GameState;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    PlayCard { hand_idx: usize },
    EndTurn,
    SelectMonster { monster_idx: u8 },
    SelectMapNode { column: usize },
    SelectCardReward { reward_idx: usize },
    SkipCardReward,
    Rest,
    Upgrade { deck_idx: usize },
}

pub fn handle_action(state: &mut GameState, action: Action) -> Vec<Effect> {
    match (state.fsm, action) {
        (Fsm::CombatDefault, Action::PlayCard { hand_idx }) => {
            handle_play_card(state, hand_idx)
        }
        (Fsm::CombatDefault, Action::EndTurn) => {
            vec![Effect::TurnEnd { actor: ActorId::Character }]
        }
        (Fsm::CombatAwaitTarget, Action::SelectMonster { monster_idx }) => {
            handle_select_monster(state, monster_idx)
        }
        (Fsm::CombatAwaitDiscard, Action::PlayCard { hand_idx }) => {
            handle_select_discard(state, hand_idx)
        }
        (Fsm::Map, Action::SelectMapNode { column }) => {
            handle_select_map_node(state, column)
        }
        (Fsm::CardReward, Action::SelectCardReward { reward_idx }) => {
            handle_card_reward_select(state, reward_idx)
        }
        (Fsm::CardReward, Action::SkipCardReward) => {
            handle_card_reward_skip(state)
        }
        (Fsm::RestSite, Action::Rest) => {
            handle_rest(state)
        }
        (Fsm::RestSite, Action::Upgrade { deck_idx }) => {
            handle_upgrade(state, deck_idx)
        }
        _ => panic!("Invalid action {:?} in state {:?}", action, state.fsm),
    }
}

fn handle_play_card(state: &mut GameState, hand_idx: usize) -> Vec<Effect> {
    let card_idx = state.hand[hand_idx];
    let card = &state.combat_cards[card_idx];

    // Energy check
    assert!(
        card.cost <= state.energy.current,
        "Not enough energy: need {}, have {}",
        card.cost, state.energy.current
    );

    if card.requires_target() {
        // If only one monster, auto-target (fast_mode behavior)
        if state.monsters.len() == 1 {
            vec![
                Effect::TargetSet { monster_idx: 0 },
                Effect::CardActiveClear,
                Effect::CardPlay { card_idx },
                Effect::TargetClear,
            ]
        } else {
            vec![Effect::CardActiveSet { card_idx }]
        }
    } else {
        vec![Effect::CardPlay { card_idx }]
    }
}

fn handle_select_monster(state: &mut GameState, monster_idx: u8) -> Vec<Effect> {
    let card_idx = state.card_active.expect("No active card for monster select");
    vec![
        Effect::TargetSet { monster_idx },
        Effect::CardActiveClear,
        Effect::CardPlay { card_idx },
        Effect::TargetClear,
    ]
}

fn handle_select_discard(state: &mut GameState, hand_idx: usize) -> Vec<Effect> {
    let card_idx = state.hand[hand_idx];
    // Remove the AwaitDiscard from the front of the queue
    state.effect_queue.pop_front();
    vec![Effect::CardDiscard { card_idx }]
}

fn handle_select_map_node(state: &mut GameState, column: usize) -> Vec<Effect> {
    // Remove the AwaitMapNode from the front of the queue
    state.effect_queue.pop_front();

    // Set map active
    let y = if state.map.active_y.is_none() {
        0
    } else {
        state.map.active_y.unwrap() + 1
    };

    state.map.active_y = Some(y);
    state.map.active_x = Some(column);

    // Reset card rewards
    state.card_rewards.clear();

    vec![Effect::RoomEnter]
}

fn handle_card_reward_select(state: &mut GameState, reward_idx: usize) -> Vec<Effect> {
    // Remove AwaitCardReward from queue
    state.effect_queue.pop_front();

    vec![
        Effect::CardRewardSelect { reward_idx },
        Effect::AwaitMapNode,
    ]
}

fn handle_card_reward_skip(state: &mut GameState) -> Vec<Effect> {
    // Remove AwaitCardReward from queue
    state.effect_queue.pop_front();

    vec![
        Effect::CardRewardClear,
        Effect::AwaitMapNode,
    ]
}

fn handle_rest(state: &mut GameState) -> Vec<Effect> {
    let heal = (REST_SITE_HEAL_FACTOR * state.character.vitals.health_max as f32) as u16;

    let is_last_floor = state.map.active_y == Some(crate::map::MAP_HEIGHT - 1);

    let mut effects = vec![
        Effect::HealthGain { target: ActorId::Character, amount: heal },
    ];

    if is_last_floor {
        // Boss fight: go directly to boss room
        state.map.active_y = Some(state.map.boss_room_y);
        state.map.active_x = Some(0); // boss doesn't have a real column
        effects.push(Effect::RoomEnter);
    } else {
        effects.push(Effect::AwaitMapNode);
    }

    effects
}

fn handle_upgrade(state: &mut GameState, deck_idx: usize) -> Vec<Effect> {
    let is_last_floor = state.map.active_y == Some(crate::map::MAP_HEIGHT - 1);

    let mut effects = vec![
        Effect::CardUpgrade { deck_idx },
    ];

    if is_last_floor {
        state.map.active_y = Some(state.map.boss_room_y);
        state.map.active_x = Some(0);
        effects.push(Effect::RoomEnter);
    } else {
        effects.push(Effect::AwaitMapNode);
    }

    effects
}
