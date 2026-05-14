use crate::consts::MAP_WIDTH;
use crate::consts::MAX_MONSTERS;
use crate::consts::REST_SITE_HEAL_FACTOR;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::map::room_at;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::potions::find_free_slot;
use crate::types::CardKind;
use crate::types::Phase;
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
    RelicRewardSelect,
    RelicRewardSkip,
    PotionRewardSelect,
    PotionRewardSkip,
    GoldRewardTake,
    RewardSkip,
    EndTurn,
    RoomSelect {
        idx_column: usize,
    },
    RestSiteRest,
    RoomSkip,
    ChestOpen,
    PotionUse {
        idx_slot: usize,
        idx_monster: Option<usize>,
    },
    PotionDiscard {
        idx_slot: usize,
    },
    CardDiscoverPick {
        idx_option: usize,
    },
}

fn validate_phase(action: &Action, current_phase: &Phase) -> Result<(), String> {
    let valid = match (action, current_phase) {
        (Action::CardDiscard { indices_hand }, Phase::CombatAwaitDiscard { num }) => {
            indices_hand.len() == *num as usize
        }
        (Action::CardRetain { indices_hand }, Phase::CombatAwaitRetain { num }) => {
            indices_hand.len() == *num as usize
        }
        (Action::CardSetup { .. }, Phase::CombatAwaitSetup) => true,
        (Action::CardNightmare { .. }, Phase::CombatAwaitNightmare) => true,
        (Action::CardPlay { .. } | Action::EndTurn, Phase::CombatDefault) => true,
        (Action::RestSiteCardUpgrade { .. } | Action::RestSiteRest, Phase::RestSite) => true,
        (
            Action::CardRewardSelect { .. }
            | Action::CardRewardSkip
            | Action::RelicRewardSelect
            | Action::RelicRewardSkip
            | Action::PotionRewardSelect
            | Action::PotionRewardSkip
            | Action::GoldRewardTake
            | Action::RewardSkip,
            Phase::Reward { .. },
        ) => true,
        (Action::RoomSelect { .. }, Phase::Map) => true,
        (Action::RoomSkip, Phase::EventRoom | Phase::Shop) => true,
        (Action::ChestOpen, Phase::Chest) => true,
        // PotionUse: combat-only potions checked in handler (need entity lookup)
        (Action::PotionUse { .. }, Phase::CombatDefault) => true,
        (Action::PotionUse { .. }, Phase::Map | Phase::RestSite | Phase::Chest | Phase::EventRoom | Phase::Shop) => true,
        (Action::PotionDiscard { .. }, p) if !matches!(p, Phase::GameOver) => true,
        (Action::CardDiscoverPick { .. }, Phase::CombatAwaitDiscover { .. }) => true,
        _ => false,
    };
    if !valid {
        return Err(format!("{:?} invalid in phase {:?}", action, current_phase));
    }
    Ok(())
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    validate_phase(&action, &state.phase)?;

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
        Action::RelicRewardSelect => handle_relic_reward_select(state),
        Action::RelicRewardSkip => Ok(handle_relic_reward_skip()),
        Action::PotionRewardSelect => handle_potion_reward_select(state),
        Action::PotionRewardSkip => Ok(handle_potion_reward_skip()),
        Action::GoldRewardTake => handle_gold_reward_take(state),
        Action::RewardSkip => Ok(handle_reward_skip()),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::RoomSelect { idx_column } => handle_room_select(state, idx_column),
        Action::RestSiteRest => Ok(handle_rest_site_rest(state)),
        Action::RoomSkip => Ok(handle_room_skip()),
        Action::ChestOpen => Ok(handle_chest_open()),
        Action::PotionUse { idx_slot, idx_monster } => {
            handle_potion_use(state, idx_slot, idx_monster)
        }
        Action::PotionDiscard { idx_slot } => handle_potion_discard(state, idx_slot),
        Action::CardDiscoverPick { idx_option } => handle_card_discover_pick(state, idx_option),
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
    let Phase::Reward { id_cards, .. } = &state.phase else {
        return Err(format!("CardRewardSelect invalid in phase {:?}", state.phase));
    };
    let id_card = lookup_idx(id_cards, idx_reward)?;
    let card = &state.entities[id_card];
    let card_name = card.card_name;
    let upgraded = card.card_upgraded;

    Ok(vec![
        Effect::direct(
            EffectKind::CardAddToDeck {
                card_name,
                upgraded,
            },
            None,
            None,
        ),
        Effect::direct(EffectKind::CardRewardClear, None, None),
    ])
}

fn handle_card_reward_skip() -> Vec<Effect> {
    // CardRewardClear halts on AwaitMapNode once the rewards are cleared
    vec![Effect {
        kind: EffectKind::CardRewardClear,
        id_source: None,
        target: Target::Direct(None),
    }]
}

fn handle_relic_reward_select(state: &GameState) -> Result<Vec<Effect>, String> {
    let Phase::Reward { id_relic, .. } = &state.phase else {
        return Err(format!("RelicRewardSelect invalid in phase {:?}", state.phase));
    };
    if id_relic.is_none() {
        return Err("RelicRewardSelect: no relic in reward pool".to_string());
    }
    Ok(vec![Effect::direct(EffectKind::RelicRewardSelect, None, None)])
}

fn handle_relic_reward_skip() -> Vec<Effect> {
    vec![Effect {
        kind: EffectKind::RelicRewardClear,
        id_source: None,
        target: Target::Direct(None),
    }]
}

fn handle_potion_reward_select(state: &GameState) -> Result<Vec<Effect>, String> {
    let Phase::Reward { id_potion, .. } = &state.phase else {
        return Err(format!("PotionRewardSelect invalid in phase {:?}", state.phase));
    };
    if id_potion.is_none() {
        return Err("PotionRewardSelect: no potion in reward pool".to_string());
    }
    let character = &state.entities[state.id_character];
    if find_free_slot(&character.potion_slots, character.potion_slots_max).is_none() {
        return Err("belt is full; discard a potion first".to_string());
    }
    Ok(vec![Effect::direct(EffectKind::PotionRewardSelect, None, None)])
}

fn handle_potion_reward_skip() -> Vec<Effect> {
    vec![Effect::direct(EffectKind::PotionRewardClear, None, None)]
}

fn handle_gold_reward_take(state: &GameState) -> Result<Vec<Effect>, String> {
    let Phase::Reward { gold, .. } = &state.phase else {
        return Err(format!("GoldRewardTake invalid in phase {:?}", state.phase));
    };
    if gold.is_none() {
        return Err("GoldRewardTake: no gold in reward pool".to_string());
    }
    Ok(vec![Effect::direct(EffectKind::GoldRewardTake, None, None)])
}

fn handle_reward_skip() -> Vec<Effect> {
    vec![Effect::direct(EffectKind::RewardSkip, None, None)]
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

fn handle_room_skip() -> Vec<Effect> {
    vec![Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::NextRowRooms,
            selection: SelectionKind::Input { count: 1 },
        },
    }]
}

fn handle_chest_open() -> Vec<Effect> {
    vec![Effect::direct(EffectKind::ChestOpen, None, None)]
}

fn handle_potion_use(
    state: &mut GameState,
    idx_slot: usize,
    idx_monster: Option<usize>,
) -> Result<Vec<Effect>, String> {
    let character = &state.entities[state.id_character];
    if idx_slot >= character.potion_slots_max as usize {
        return Err(format!("PotionUse: idx_slot {} out of range", idx_slot));
    }
    let id_potion = character.potion_slots[idx_slot]
        .ok_or_else(|| format!("PotionUse: slot {} is empty", idx_slot))?;
    let potion = &state.entities[id_potion];

    if potion.potion_combat_only && !matches!(state.phase, Phase::CombatDefault) {
        return Err(format!(
            "PotionUse: {:?} is combat-only, current phase {:?}",
            potion.potion_name, state.phase
        ));
    }

    let requires_target = potion.potion_requires_target;
    let id_monster_target = if requires_target {
        let mut buf_alive = [0usize; MAX_MONSTERS];
        let n = fill_alive_monster_ids(state, &mut buf_alive);
        let idx = idx_monster
            .ok_or_else(|| "PotionUse: requires_target but idx_monster is None".to_string())?;
        Some(
            *buf_alive[..n]
                .get(idx)
                .ok_or_else(|| format!("PotionUse: invalid monster index {}", idx))?,
        )
    } else {
        if idx_monster.is_some() {
            return Err("PotionUse: idx_monster supplied but potion is untargeted".into());
        }
        None
    };

    // Clear the slot before the effect chain runs
    state.entities[state.id_character].potion_slots[idx_slot] = None;

    let mut chain = Vec::with_capacity(3);
    if let Some(id) = id_monster_target {
        chain.push(Effect {
            kind: EffectKind::TargetSet,
            id_source: None,
            target: Target::Direct(Some(id)),
        });
    }
    chain.push(Effect {
        kind: EffectKind::PotionUse,
        id_source: Some(id_potion),
        target: Target::Direct(Some(id_potion)),
    });
    if requires_target {
        chain.push(Effect {
            kind: EffectKind::TargetClear,
            id_source: None,
            target: Target::Direct(None),
        });
    }
    Ok(chain)
}

fn handle_potion_discard(
    state: &mut GameState,
    idx_slot: usize,
) -> Result<Vec<Effect>, String> {
    let character = &mut state.entities[state.id_character];
    if idx_slot >= character.potion_slots_max as usize {
        return Err(format!("PotionDiscard: idx_slot {} out of range", idx_slot));
    }
    if character.potion_slots[idx_slot].is_none() {
        return Err(format!("PotionDiscard: slot {} is empty", idx_slot));
    }
    character.potion_slots[idx_slot] = None;
    Ok(Vec::new())
}

fn handle_card_discover_pick(
    state: &mut GameState,
    idx_option: usize,
) -> Result<Vec<Effect>, String> {
    let id_card = *state
        .id_card_discover
        .get(idx_option)
        .ok_or_else(|| format!("CardDiscoverPick: idx_option {} out of range", idx_option))?;
    let card = &mut state.entities[id_card];
    card.card_free_to_play_once = true;
    state.id_hand.push(id_card);
    state.id_card_discover.clear();
    Ok(Vec::new())
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
