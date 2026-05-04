use crate::consts::{MAP_WIDTH, MAX_MONSTERS, REST_SITE_HEAL_FACTOR};
use crate::effect::{DiscardSource, Effect, EffectKind, Target};
use crate::entity::{card_effective_cost, is_play_restriction_satisfied};
use crate::map::{has_edge, room_at};
use crate::modifier::{ModifierKind, modifier_has};
use crate::game::{GameState, Location};
use crate::types::{CardKind, Phase};
use crate::utils::fill_alive_monster_ids;

#[derive(Debug, Clone)]
pub enum Action {
    CardDiscard {
        indices_hand: Vec<usize>,
    },
    CardRetain {
        indices_hand: Vec<usize>,
    },
    CardSetup {
        idx_hand: usize,
    },
    CardNightmare {
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
    RoomSelect {
        idx_column: usize,
    },
    RestSiteRest,
}

fn validate_phase(action: &Action, current_phase: Phase) -> Result<(), String> {
    let valid = match (action, current_phase) {
        (Action::CardDiscard { indices_hand }, Phase::CombatAwaitDiscard { num }) => {
            indices_hand.len() == num as usize
        }
        (Action::CardRetain { indices_hand }, Phase::CombatAwaitRetain { num }) => {
            indices_hand.len() == num as usize
        }
        (Action::CardSetup { .. }, Phase::CombatAwaitSetup) => true,
        (Action::CardNightmare { .. }, Phase::CombatAwaitNightmare) => true,
        (Action::CardPlay { .. } | Action::EndTurn, Phase::CombatDefault) => true,
        (Action::RestSiteCardUpgrade { .. } | Action::RestSiteRest, Phase::RestSite) => true,
        (Action::CardRewardSelect { .. } | Action::CardRewardSkip, Phase::CombatReward) => true,
        (Action::RoomSelect { .. }, Phase::Map) => true,
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
        Action::CardDiscard { indices_hand } => handle_card_discard(state, indices_hand),
        Action::CardRetain { indices_hand } => handle_card_retain(state, indices_hand),
        Action::CardSetup { idx_hand } => handle_card_setup(state, idx_hand),
        Action::CardNightmare { idx_hand } => handle_card_nightmare(state, idx_hand),
        Action::CardPlay {
            idx_hand,
            idx_monster,
        } => handle_card_play(state, idx_hand, idx_monster),
        Action::RestSiteCardUpgrade { idx_deck } => handle_rest_site_card_upgrade(state, idx_deck),
        Action::CardRewardSelect { idx_reward } => handle_card_reward_select(state, idx_reward),
        Action::CardRewardSkip => Ok(handle_card_reward_skip()),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::RoomSelect { idx_column } => handle_room_select(state, idx_column),
        Action::RestSiteRest => Ok(handle_rest_site_rest(state)),
    }?;

    Ok(effects)
}

fn lookup_idx(slice: &[usize], idx: usize) -> Result<usize, String> {
    slice
        .get(idx)
        .copied()
        .ok_or_else(|| format!("Invalid index {}: {} available", idx, slice.len()))
}

fn handle_end_turn(state: &GameState) -> Vec<Effect> {
    // Return effect to end the character's turn
    vec![Effect {
        kind: EffectKind::TurnEnd,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    }]
}

fn handle_card_play(
    state: &mut GameState,
    idx_hand: usize,
    idx_monster: Option<usize>,
) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_hand, idx_hand)?;
    let card = &state.entities[id_card];

    if !is_play_restriction_satisfied(card.card_play_restriction, &state.id_pile_draw) {
        return Err(format!(
            "Card {:?} not playable right now (restriction: {:?})",
            card.card_name, card.card_play_restriction,
        ));
    }

    // Entangled: blocks playing Attack cards
    if card.card_kind == CardKind::Attack
        && modifier_has(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Entangled,
        )
    {
        return Err(format!(
            "Card {:?} cannot be played while Entangled",
            card.card_name,
        ));
    }

    let effective_cost = card_effective_cost(
        card,
        state.this_turn_discards,
        state.this_combat_damage_instances_taken,
        state.energy.current,
    );
    if effective_cost > state.energy.current {
        return Err(format!(
            "Not enough energy to play {:?}: need {}, have {}",
            card.card_name, effective_cost, state.energy.current
        ));
    }

    if card.card_requires_target {
        match idx_monster {
            Some(idx_monster) => {
                // Stack locals
                let mut buf_alive = [0usize; MAX_MONSTERS];

                let n = fill_alive_monster_ids(state, &mut buf_alive);
                let id_monster_target = *buf_alive[..n]
                    .get(idx_monster)
                    .ok_or_else(|| format!("Invalid monster index: {}", idx_monster))?;

                // Set the card's target, play the card, then clear the target
                // No trailing terminator: process_queue derives the resting
                // phase (CombatDefault) from state once the chain drains
                Ok(vec![
                    Effect {
                        kind: EffectKind::TargetSet,
                        id_source: None,
                        target: Target::Direct(Some(id_monster_target)),
                    },
                    Effect {
                        kind: EffectKind::CardPlay,
                        id_source: None,
                        target: Target::Direct(Some(id_card)),
                    },
                    Effect {
                        kind: EffectKind::TargetClear,
                        id_source: None,
                        target: Target::Direct(None),
                    },
                ])
            }
            None => Err(format!(
                "Card {:?} requires a target: provide idx_monster",
                card.card_name
            )),
        }
    } else {
        Ok(vec![Effect {
            kind: EffectKind::CardPlay,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        }])
    }
}

fn handle_card_discard(state: &GameState, indices_hand: Vec<usize>) -> Result<Vec<Effect>, String> {
    let mut effects = Vec::with_capacity(indices_hand.len());
    for &idx in &indices_hand {
        let id_card = *state
            .id_hand
            .get(idx)
            .ok_or_else(|| format!("Invalid hand index {}: {} cards", idx, state.id_hand.len()))?;
        effects.push(Effect::direct(
            EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            None,
            Some(id_card),
        ));
    }
    Ok(effects)
}

fn handle_card_setup(state: &GameState, idx_hand: usize) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_hand, idx_hand)?;
    Ok(vec![Effect::direct(
        EffectKind::CardSetupPick,
        None,
        Some(id_card),
    )])
}

fn handle_card_nightmare(state: &GameState, idx_hand: usize) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_hand, idx_hand)?;
    Ok(vec![Effect::direct(
        EffectKind::CardNightmarePick,
        None,
        Some(id_card),
    )])
}

fn handle_card_retain(state: &GameState, indices_hand: Vec<usize>) -> Result<Vec<Effect>, String> {
    let mut effects = Vec::with_capacity(indices_hand.len());
    for &idx in &indices_hand {
        let id_card = *state
            .id_hand
            .get(idx)
            .ok_or_else(|| format!("Invalid hand index {}: {} cards", idx, state.id_hand.len()))?;
        effects.push(Effect::direct(EffectKind::CardRetain, None, Some(id_card)));
    }
    Ok(effects)
}

fn handle_room_select(state: &GameState, idx_column: usize) -> Result<Vec<Effect>, String> {
    if idx_column >= MAP_WIDTH {
        return Err(format!(
            "Invalid column {}: max is {}",
            idx_column,
            MAP_WIDTH - 1
        ));
    }

    // Compute next y-coordinate from current position
    let y_next = match state.location {
        Location::Start => 0,
        Location::Overworld { y, .. } => y + 1,
        Location::BossRoom => return Err("Cannot pick a map node from the boss room".into()),
    };

    let id_room = state.id_rooms[y_next][idx_column]
        .ok_or_else(|| format!("No room at ({}, {})", y_next, idx_column))?;

    if let Location::Overworld { y, x } = state.location {
        let current_room =
            room_at(&state.id_rooms, &state.entities, y, x).expect("current room missing");
        if !has_edge(current_room.edges, idx_column) {
            return Err(format!(
                "No edge from ({}, {}) to ({}, {})",
                y, x, y_next, idx_column
            ));
        }
    }

    // Return a Direct RoomSelect effect. Its dispatch arm will update
    // state.location and push a RoomEnter effect
    Ok(vec![Effect::direct(
        EffectKind::RoomSelect,
        None,
        Some(id_room),
    )])
}

fn handle_card_reward_select(state: &GameState, idx_reward: usize) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_card_rewards, idx_reward)?;

    // Direct CardRewardSelect: handler adds the target card to the deck and
    // enqueues CardRewardClear, which in turn enqueues RoomSelect
    Ok(vec![Effect::direct(
        EffectKind::CardRewardSelect,
        None,
        Some(id_card),
    )])
}

fn handle_card_reward_skip() -> Vec<Effect> {
    // CardRewardClear halts on AwaitMapNode once the rewards are cleared
    vec![Effect {
        kind: EffectKind::CardRewardClear,
        id_source: None,
        target: Target::Direct(None),
    }]
}

fn handle_rest_site_rest(state: &GameState) -> Vec<Effect> {
    let id_character = state.id_character;
    let health_max = state.entities[id_character].vitals.health_max;
    let heal_amt = (REST_SITE_HEAL_FACTOR * health_max as f32) as u16;

    // Heal, then let the RestSiteExit handler decide whether to halt or enter boss
    vec![
        Effect {
            kind: EffectKind::HealthGain { amount: heal_amt },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}

fn handle_rest_site_card_upgrade(
    state: &GameState,
    idx_deck: usize,
) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_deck, idx_deck)?;

    // Upgrade by entity id, then let the RestSiteExit handler decide whether
    // to halt (non-final row) or enter the boss room (final row)
    Ok(vec![
        Effect::direct(EffectKind::CardUpgrade, None, Some(id_card)),
        Effect::direct(EffectKind::RestSiteExit, None, None),
    ])
}
