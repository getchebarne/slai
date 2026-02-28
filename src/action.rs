// Action handling: player input -> effects.

use crate::effect::Effect;
use crate::consts::REST_SITE_HEAL_FACTOR;
use crate::state::{EntityKind, GameState};
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
            vec![Effect::TurnEnd { actor: EntityId(0) }]
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
    let card_id = state.hand[hand_idx];
    let card = state.entities[card_id.0 as usize].as_ref().expect("Missing card").kind.card_ref();

    assert!(
        card.cost <= state.energy.current,
        "Not enough energy: need {}, have {}",
        card.cost, state.energy.current
    );

    if card.requires_target() {
        let monster_count = state.entities.iter()
            .filter(|s| matches!(s, Some(e) if matches!(e.kind, EntityKind::Monster(..))))
            .count();
        if monster_count == 1 {
            let target = state.entities.iter().enumerate()
                .find(|(_, s)| matches!(s, Some(e) if matches!(e.kind, EntityKind::Monster(..))))
                .map(|(i, _)| EntityId(i as u32))
                .unwrap();
            vec![
                Effect::TargetSet { target },
                Effect::CardActiveClear,
                Effect::CardPlay { card_id },
                Effect::TargetClear,
            ]
        } else {
            vec![Effect::CardActiveSet { card_id }]
        }
    } else {
        vec![Effect::CardPlay { card_id }]
    }
}

fn handle_select_monster(state: &mut GameState, monster_idx: u8) -> Vec<Effect> {
    let card_id = state.card_active.expect("No active card for monster select");
    let target = state.entities.iter().enumerate()
        .filter(|(_, s)| matches!(s, Some(e) if matches!(e.kind, EntityKind::Monster(..))))
        .nth(monster_idx as usize)
        .map(|(i, _)| EntityId(i as u32))
        .expect("Invalid monster index");
    vec![
        Effect::TargetSet { target },
        Effect::CardActiveClear,
        Effect::CardPlay { card_id },
        Effect::TargetClear,
    ]
}

fn handle_select_discard(state: &mut GameState, hand_idx: usize) -> Vec<Effect> {
    let card_id = state.hand[hand_idx];
    state.effect_queue.pop_front();
    vec![Effect::CardDiscard { card_id }]
}

fn handle_select_map_node(state: &mut GameState, column: usize) -> Vec<Effect> {
    state.effect_queue.pop_front();

    let y = match state.map.active_y {
        None => 0,
        Some(prev) => prev + 1,
    };

    state.map.active_y = Some(y);
    state.map.active_x = Some(column);

    state.card_rewards.clear();

    vec![Effect::RoomEnter]
}

fn handle_card_reward_select(state: &mut GameState, reward_idx: usize) -> Vec<Effect> {
    state.effect_queue.pop_front();
    vec![
        Effect::CardRewardSelect { reward_idx },
        Effect::AwaitMapNode,
    ]
}

fn handle_card_reward_skip(state: &mut GameState) -> Vec<Effect> {
    state.effect_queue.pop_front();
    vec![
        Effect::CardRewardClear,
        Effect::AwaitMapNode,
    ]
}

fn handle_rest(state: &mut GameState) -> Vec<Effect> {
    let (vitals, _) = state.entities[0].as_ref().unwrap().kind.combatant_ref();
    let heal = (REST_SITE_HEAL_FACTOR * vitals.health_max as f32) as u16;

    let is_last_floor = state.map.active_y == Some(crate::consts::MAP_HEIGHT - 1);

    let mut effects = vec![
        Effect::HealthGain { target: EntityId(0), amount: heal },
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

fn handle_upgrade(state: &mut GameState, deck_idx: usize) -> Vec<Effect> {
    let is_last_floor = state.map.active_y == Some(crate::consts::MAP_HEIGHT - 1);

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
