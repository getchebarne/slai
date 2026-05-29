use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::get_input_count;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::event_option_gate_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::map::room_at;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::potions::find_free_slot;
use crate::types::CardKind;
use crate::types::DeltaSign;
use crate::types::RewardKind;
use crate::types::Screen;

#[derive(Debug, Clone)]
pub enum Action {
    CardDiscard {
        idxs: Vec<usize>,
    },
    CardDiscover {
        idx: usize,
    },
    CardDuplicate {
        idx: usize,
    },
    CardNightmare {
        idx: usize,
    },
    CardPlay {
        idx_card: usize,
        idx_monster: Option<usize>,
    },
    CardPurge {
        idx: usize,
    },
    CardRetain {
        idxs: Vec<usize>,
    },
    CardSetup {
        idx: usize,
    },
    CardTransform {
        idx: usize,
    },
    CardUpgrade {
        idx: usize,
    },
    ChestOpen,
    EventOptionSelect {
        idx: usize,
    },
    PotionDiscard {
        idx: usize,
    },
    PotionUse {
        idx_potion: usize,
        idx_monster: Option<usize>,
    },
    Rest,
    RewardSkip, // TODO: think about merging w/ `RoomExit`
    RewardTakeCard {
        idx: usize,
    },
    RewardTakeGold,
    RewardTakePotion,
    RewardTakeRelic,
    RoomExit,
    RoomSelect {
        idx: usize,
    },
    TurnEnd,
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    if state.game_over {
        return Err("GameOver".into());
    }

    let effects = match action {
        Action::CardDiscard { idxs } => handle_card_discard(state, idxs),
        Action::CardDiscover { idx } => handle_card_discover(state, idx),
        Action::CardDuplicate { idx } => handle_card_duplicate(state, idx),
        Action::CardNightmare { idx } => handle_card_nightmare(state, idx),
        Action::CardPlay {
            idx_card,
            idx_monster,
        } => handle_card_play(state, idx_card, idx_monster),
        Action::CardPurge { idx } => handle_card_purge(state, idx),
        Action::CardRetain { idxs } => handle_card_retain(state, idxs),
        Action::CardSetup { idx } => handle_card_setup(state, idx),
        Action::CardTransform { idx } => handle_card_transform(state, idx),
        Action::CardUpgrade { idx } => handle_card_upgrade(state, idx),
        Action::ChestOpen => handle_chest_open(state),
        Action::EventOptionSelect { idx } => handle_event_option_select(state, idx),
        Action::PotionDiscard { idx } => handle_potion_discard(state, idx),
        Action::PotionUse {
            idx_potion,
            idx_monster,
        } => handle_potion_use(state, idx_potion, idx_monster),
        Action::Rest => handle_rest(state),
        Action::RewardSkip => handle_reward_skip(state),
        Action::RewardTakeCard { idx } => handle_reward_take_card(state, idx),
        Action::RewardTakeGold => handle_reward_take_gold(state),
        Action::RewardTakePotion => handle_reward_take_potion(state),
        Action::RewardTakeRelic => handle_reward_take_relic(state),
        Action::RoomExit => handle_room_exit(state),
        Action::RoomSelect { idx } => handle_room_select(state, idx),
        Action::TurnEnd => handle_turn_end(state),
    }?;

    Ok(effects)
}

fn handle_card_discard(state: &mut GameState, idxs: Vec<usize>) -> Result<Vec<Effect>, String> {
    let pending = state
        .effect_pending
        .as_ref()
        .ok_or("CardDiscard: requires a pending effect")?;
    if !matches!(pending.kind, EffectKind::CardDiscard { .. }) {
        return Err(format!(
            "CardDiscard: pending kind {:?} mismatch",
            pending.kind
        ));
    }
    validate_count(&idxs, get_input_count(pending).unwrap_or(1) as usize)?;
    resolve_hand_pending(state, idxs)
}

fn handle_card_discover(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardDiscoverPick) => {}
        other => return Err(format!("CardDiscover: pending kind {:?} mismatch", other)),
    }
    let id_card = *state
        .id_discover
        .get(idx)
        .ok_or_else(|| format!("CardDiscover: idx {} out of range", idx))?;
    let pending = state
        .effect_pending
        .take()
        .ok_or("CardDiscover: no effect_pending")?;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDiscoverPick,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
    Ok(Vec::new())
}

fn handle_card_duplicate(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardDuplicate) => {}
        other => return Err(format!("CardDuplicate: pending kind {:?} mismatch", other)),
    }
    resolve_deck_pending(state, idx)
}

fn handle_card_nightmare(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardNightmarePick) => {}
        other => return Err(format!("CardNightmare: pending kind {:?} mismatch", other)),
    }
    resolve_hand_pending(state, vec![idx])
}

fn handle_card_play(
    state: &mut GameState,
    idx_card: usize,
    idx_monster: Option<usize>,
) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Combat {
        return Err(format!(
            "CardPlay: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    let id_card = lookup_idx(&state.id_hand, idx_card)?;
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
        state.energy.energy_current,
    );
    if effective_cost > state.energy.energy_current {
        return Err(format!(
            "Not enough energy to play {:?}: need {}, have {}",
            card.card_name, effective_cost, state.energy.energy_current
        ));
    }

    if card.requires_target {
        match idx_monster {
            Some(idx_monster) => {
                let id_monster_target = state
                    .id_monsters
                    .iter()
                    .flatten()
                    .copied()
                    .nth(idx_monster)
                    .ok_or_else(|| format!("Invalid monster index: {}", idx_monster))?;

                // TargetSet -> CardPlay -> TargetClear; no terminator (derive_phase handles rest)
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

fn handle_card_purge(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardPurge) => {}
        other => return Err(format!("CardPurge: pending kind {:?} mismatch", other)),
    }
    resolve_deck_pending(state, idx)
}

fn handle_card_retain(state: &mut GameState, idxs: Vec<usize>) -> Result<Vec<Effect>, String> {
    let pending = state
        .effect_pending
        .as_ref()
        .ok_or("CardRetain: requires a pending effect")?;
    if pending.kind != EffectKind::CardRetain {
        return Err(format!(
            "CardRetain: pending kind {:?} mismatch",
            pending.kind
        ));
    }
    validate_count(&idxs, get_input_count(pending).unwrap_or(1) as usize)?;
    resolve_hand_pending(state, idxs)
}

fn handle_card_setup(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardSetupPick) => {}
        other => return Err(format!("CardSetup: pending kind {:?} mismatch", other)),
    }
    resolve_hand_pending(state, vec![idx])
}

fn handle_card_transform(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardTransform) => {}
        other => return Err(format!("CardTransform: pending kind {:?} mismatch", other)),
    }
    resolve_deck_pending(state, idx)
}

fn handle_card_upgrade(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    // Dual-mode: resolves a pending CardUpgrade halt; at rest site triggers a direct upgrade + exit
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::CardUpgrade) => return resolve_deck_pending(state, idx),
        None if matches!(state.screen, Screen::RestSite) => {}
        _ => {
            return Err(format!(
                "CardUpgrade: invalid context (screen={:?}, pending={:?})",
                state.screen,
                state.effect_pending.as_ref().map(|p| p.kind)
            ));
        }
    }
    let id_card = upgradeable_deck_at(state, idx)?;
    Ok(vec![
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            id_source: None,
            target: Target::Direct(None),
        },
    ])
}

fn handle_chest_open(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Chest {
        return Err(format!(
            "ChestOpen: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    Ok(vec![Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    }])
}

fn handle_event_option_select(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Event {
        return Err(format!(
            "EventOptionSelect: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    let id_event = state
        .id_event
        .expect("Event screen implies id_event is set");
    let event = &state.entities[id_event];
    if idx >= event.event_options.len() {
        return Err(format!(
            "EventOptionSelect: idx {} out of range (options {})",
            idx,
            event.event_options.len()
        ));
    }
    let option = event.event_options[idx];
    if !event_option_gate_satisfied(option.gate, state, id_event) {
        return Err(format!(
            "EventOptionSelect: option {} gated out ({:?})",
            idx, option.gate
        ));
    }
    let effects: Vec<Effect> = option
        .effects
        .iter()
        .map(|e| Effect {
            id_source: Some(id_event),
            ..*e
        })
        .collect();
    Ok(effects)
}

fn handle_potion_discard(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        None | Some(EffectKind::RoomSelect) => {}
        Some(other) => return Err(format!("PotionDiscard: invalid during pending {:?}", other)),
    }
    let character = &mut state.entities[state.id_character];
    if idx >= character.potion_slots_max as usize {
        return Err(format!("PotionDiscard: idx {} out of range", idx));
    }
    if character.potion_slots[idx].is_none() {
        return Err(format!("PotionDiscard: slot {} is empty", idx));
    }
    character.potion_slots[idx] = None;
    Ok(Vec::new())
}

fn handle_potion_use(
    state: &mut GameState,
    idx_potion: usize,
    idx_monster: Option<usize>,
) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        None | Some(EffectKind::RoomSelect) => {}
        Some(other) => return Err(format!("PotionUse: invalid during pending {:?}", other)),
    }
    let character = &state.entities[state.id_character];
    if idx_potion >= character.potion_slots_max as usize {
        return Err(format!("PotionUse: idx_potion {} out of range", idx_potion));
    }
    let id_potion = character.potion_slots[idx_potion]
        .ok_or_else(|| format!("PotionUse: slot {} is empty", idx_potion))?;
    let potion = &state.entities[id_potion];

    if potion.potion_combat_only && !matches!(state.screen, Screen::Combat) {
        return Err(format!(
            "PotionUse: {:?} is combat-only",
            potion.potion_name
        ));
    }

    let requires_target = potion.requires_target;
    let id_monster_target = if requires_target {
        let idx = idx_monster
            .ok_or_else(|| "PotionUse: requires_target but idx_monster is None".to_string())?;
        Some(
            state
                .id_monsters
                .iter()
                .flatten()
                .copied()
                .nth(idx)
                .ok_or_else(|| format!("PotionUse: invalid monster index {}", idx))?,
        )
    } else {
        if idx_monster.is_some() {
            return Err("PotionUse: idx_monster supplied but potion is untargeted".into());
        }
        None
    };

    // Clear the slot before the effect chain runs
    state.entities[state.id_character].potion_slots[idx_potion] = None;

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

fn handle_rest(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::RestSite {
        return Err(format!(
            "Rest: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    let id_character = state.id_character;

    // Heal, then let the RestSiteExit handler decide whether to halt or enter boss
    Ok(vec![
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: HealthDeltaAmount::Relative {
                    numerator: 3,
                    denominator: 10,
                },
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        },
        Effect {
            kind: EffectKind::RestSiteExit,
            id_source: None,
            target: Target::Direct(None),
        },
    ])
}

fn handle_reward_skip(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Reward {
        return Err(format!(
            "RewardSkip: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    Ok(vec![Effect {
        kind: EffectKind::RewardSkip,
        id_source: None,
        target: Target::Direct(None),
    }])
}

fn handle_reward_take_card(state: &GameState, idx: usize) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Reward {
        return Err(format!(
            "RewardTakeCard: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    let id_card = lookup_idx(&state.reward_id_cards, idx)?;
    Ok(vec![Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Card,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    }])
}

fn handle_reward_take_gold(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Reward {
        return Err(format!(
            "RewardTakeGold: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    if state.reward_gold.is_none() {
        return Err("RewardTakeGold: no gold in reward pool".to_string());
    }
    Ok(vec![Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Gold,
        },
        id_source: None,
        target: Target::Direct(None),
    }])
}

fn handle_reward_take_potion(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Reward {
        return Err(format!(
            "RewardTakePotion: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    if state.reward_id_potion.is_none() {
        return Err("RewardTakePotion: no potion in reward pool".to_string());
    }
    let character = &state.entities[state.id_character];
    if find_free_slot(&character.potion_slots, character.potion_slots_max).is_none() {
        return Err("belt is full; discard a potion first".to_string());
    }
    Ok(vec![Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Potion,
        },
        id_source: None,
        target: Target::Direct(None),
    }])
}

fn handle_reward_take_relic(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Reward {
        return Err(format!(
            "RewardTakeRelic: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    if state.reward_id_relic.is_none() {
        return Err("RewardTakeRelic: no relic in reward pool".to_string());
    }
    Ok(vec![Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Relic,
        },
        id_source: None,
        target: Target::Direct(None),
    }])
}

fn handle_room_exit(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Shop {
        return Err(format!(
            "RoomExit: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    Ok(vec![Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::NextRowRooms,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }])
}

fn handle_room_select(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    match state.effect_pending.as_ref().map(|p| p.kind) {
        Some(EffectKind::RoomSelect) => {}
        other => return Err(format!("RoomSelect: pending kind {:?} mismatch", other)),
    }
    if idx >= MAP_WIDTH {
        return Err(format!("Invalid column {}: max is {}", idx, MAP_WIDTH - 1));
    }

    let y_next = match state.location {
        Location::Start => 0,
        Location::Overworld { y, .. } => y + 1,
        Location::BossRoom => return Err("Cannot pick a map node from the boss room".into()),
    };

    let id_room =
        state.id_rooms[y_next][idx].ok_or_else(|| format!("No room at ({}, {})", y_next, idx))?;

    if let Location::Overworld { y, x } = state.location {
        let current_room =
            room_at(&state.id_rooms, &state.entities, y, x).expect("current room missing");
        if !has_edge(current_room.edges, idx) {
            return Err(format!(
                "No edge from ({}, {}) to ({}, {})",
                y, x, y_next, idx
            ));
        }
    }

    let pending = state
        .effect_pending
        .take()
        .ok_or("RoomSelect: no effect_pending")?;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RoomSelect,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_room)),
    });
    Ok(Vec::new())
}

fn handle_turn_end(state: &GameState) -> Result<Vec<Effect>, String> {
    if state.effect_pending.is_some() || state.screen != Screen::Combat {
        return Err(format!(
            "TurnEnd: invalid (screen={:?}, pending={:?})",
            state.screen,
            state.effect_pending.as_ref().map(|p| p.kind)
        ));
    }
    Ok(vec![Effect {
        kind: EffectKind::TurnEnd,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    }])
}

// Mirrors validate() shape; empty when game_over
pub fn get_legal_actions(state: &GameState) -> Vec<Action> {
    if state.game_over {
        return Vec::new();
    }
    if let Some(pending) = state.effect_pending.as_ref() {
        return legal_actions_pending(state, pending);
    }
    match state.screen {
        Screen::Combat => legal_actions_combat(state),
        Screen::Reward => legal_actions_reward(state),
        Screen::Event => legal_actions_event(state),
        Screen::Shop => legal_actions_shop(state),
        Screen::Map => legal_actions_map(state),
        Screen::RestSite => legal_actions_rest_site(state),
        Screen::Chest => legal_actions_chest(state),
    }
}

fn legal_actions_pending(state: &GameState, pending: &Effect) -> Vec<Action> {
    let mut actions = Vec::new();
    match pending.kind {
        EffectKind::CardDiscard { .. } => {
            let num = get_input_count(pending).unwrap_or(1) as usize;
            for combo in hand_combinations(state.id_hand.len(), num) {
                actions.push(Action::CardDiscard { idxs: combo });
            }
        }
        EffectKind::CardRetain => {
            let num = get_input_count(pending).unwrap_or(1) as usize;
            for combo in hand_combinations(state.id_hand.len(), num) {
                actions.push(Action::CardRetain { idxs: combo });
            }
        }
        EffectKind::CardSetupPick => {
            for i in 0..state.id_hand.len() {
                actions.push(Action::CardSetup { idx: i });
            }
        }
        EffectKind::CardNightmarePick => {
            for i in 0..state.id_hand.len() {
                actions.push(Action::CardNightmare { idx: i });
            }
        }
        EffectKind::CardDiscoverPick => {
            for i in 0..state.id_discover.len() {
                actions.push(Action::CardDiscover { idx: i });
            }
        }
        EffectKind::CardPurge => {
            for i in 0..state.buf_candidates.len() {
                actions.push(Action::CardPurge { idx: i });
            }
        }
        EffectKind::CardUpgrade => {
            for i in 0..state.buf_candidates.len() {
                actions.push(Action::CardUpgrade { idx: i });
            }
        }
        EffectKind::CardDuplicate => {
            for i in 0..state.buf_candidates.len() {
                actions.push(Action::CardDuplicate { idx: i });
            }
        }
        EffectKind::CardTransform => {
            for i in 0..state.buf_candidates.len() {
                actions.push(Action::CardTransform { idx: i });
            }
        }
        EffectKind::RoomSelect => {
            push_room_select_actions(state, &mut actions);
            push_potion_actions(state, &mut actions);
        }
        _ => unreachable!("effect_pending with non-halting kind: {:?}", pending.kind),
    }
    actions
}

fn legal_actions_combat(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    let char_mods = &state.entities[state.id_character].modifiers;
    let entangled = modifier_has(char_mods, ModifierKind::Entangled);
    let alive_count = state.id_monsters.iter().flatten().count();
    for (i, &id_card) in state.id_hand.iter().enumerate() {
        let card = &state.entities[id_card];
        let restriction_ok =
            is_play_restriction_satisfied(card.card_play_restriction, &state.id_pile_draw);
        let entangled_blocks = entangled && card.card_kind == CardKind::Attack;
        if !restriction_ok || entangled_blocks {
            continue;
        }
        let cost = card_effective_cost(
            card,
            state.this_turn_discards,
            state.this_combat_damage_instances_taken,
            state.energy.energy_current,
        );
        if cost > state.energy.energy_current {
            continue;
        }
        if card.requires_target {
            for m in 0..alive_count {
                actions.push(Action::CardPlay {
                    idx_card: i,
                    idx_monster: Some(m),
                });
            }
        } else {
            actions.push(Action::CardPlay {
                idx_card: i,
                idx_monster: None,
            });
        }
    }
    push_potion_actions(state, &mut actions);
    actions.push(Action::TurnEnd);
    actions
}

fn legal_actions_reward(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    for i in 0..state.reward_id_cards.len() {
        actions.push(Action::RewardTakeCard { idx: i });
    }
    if state.reward_id_relic.is_some() {
        actions.push(Action::RewardTakeRelic);
    }
    if state.reward_id_potion.is_some() {
        let character = &state.entities[state.id_character];
        if find_free_slot(&character.potion_slots, character.potion_slots_max).is_some() {
            actions.push(Action::RewardTakePotion);
        }
    }
    if state.reward_gold.is_some() {
        actions.push(Action::RewardTakeGold);
    }
    actions.push(Action::RewardSkip);
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_event(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    let id_event = state.id_event.expect("Event context requires id_event");
    let event = &state.entities[id_event];
    for (i, opt) in event.event_options.iter().enumerate() {
        if event_option_gate_satisfied(opt.gate, state, id_event) {
            actions.push(Action::EventOptionSelect { idx: i });
        }
    }
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_shop(state: &GameState) -> Vec<Action> {
    let mut actions = vec![Action::RoomExit];
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_map(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    push_room_select_actions(state, &mut actions);
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_rest_site(state: &GameState) -> Vec<Action> {
    let mut actions = vec![Action::Rest];
    for i in 0..count_upgradeable_deck(state) {
        actions.push(Action::CardUpgrade { idx: i });
    }
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_chest(state: &GameState) -> Vec<Action> {
    let mut actions = vec![Action::ChestOpen];
    push_potion_actions(state, &mut actions);
    actions
}

fn push_room_select_actions(state: &GameState, actions: &mut Vec<Action>) {
    match state.location {
        Location::Start => {
            for c in 0..MAP_WIDTH {
                if state.id_rooms[0][c].is_some() {
                    actions.push(Action::RoomSelect { idx: c });
                }
            }
        }
        Location::Overworld { y, x } => {
            let y_next = y + 1;
            if y_next >= MAP_HEIGHT {
                return;
            }
            if let Some(id_current) = state.id_rooms[y][x] {
                let edges = state.entities[id_current].edges;
                for c in 0..MAP_WIDTH {
                    if has_edge(edges, c) && state.id_rooms[y_next][c].is_some() {
                        actions.push(Action::RoomSelect { idx: c });
                    }
                }
            }
        }
        Location::BossRoom => {}
    }
}

fn push_potion_actions(state: &GameState, actions: &mut Vec<Action>) {
    let character = &state.entities[state.id_character];
    let in_combat = matches!(state.screen, Screen::Combat);
    let alive_count = state.id_monsters.iter().flatten().count();
    for s in 0..character.potion_slots_max as usize {
        let Some(id_potion) = character.potion_slots[s] else {
            continue;
        };
        let potion = &state.entities[id_potion];
        if potion.potion_combat_only && !in_combat {
            actions.push(Action::PotionDiscard { idx: s });
            continue;
        }
        if potion.requires_target {
            if in_combat {
                for m in 0..alive_count {
                    actions.push(Action::PotionUse {
                        idx_potion: s,
                        idx_monster: Some(m),
                    });
                }
            }
        } else {
            actions.push(Action::PotionUse {
                idx_potion: s,
                idx_monster: None,
            });
        }
        actions.push(Action::PotionDiscard { idx: s });
    }
}

fn validate_count(idxs: &[usize], num: usize) -> Result<(), String> {
    if idxs.len() != num {
        return Err(format!("Expected {} idxs, got {}", num, idxs.len()));
    }
    Ok(())
}

fn count_upgradeable_deck(state: &GameState) -> usize {
    state
        .id_deck
        .iter()
        .filter(|&&id| !state.entities[id].card_upgraded)
        .count()
}

fn upgradeable_deck_at(state: &GameState, idx: usize) -> Result<usize, String> {
    state
        .id_deck
        .iter()
        .filter(|&&id| !state.entities[id].card_upgraded)
        .nth(idx)
        .copied()
        .ok_or_else(|| format!("CardUpgrade at RestSite: idx {} out of range", idx))
}

// All k-subsets of [0..n) as Vec<usize>. For HandSelect Discard/Retain
fn hand_combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    if k > n {
        return out;
    }
    let mut combo: Vec<usize> = (0..k).collect();
    loop {
        out.push(combo.clone());
        // Find rightmost position that can be incremented
        let mut i = k;
        while i > 0 {
            i -= 1;
            if combo[i] != i + n - k {
                combo[i] += 1;
                for j in i + 1..k {
                    combo[j] = combo[j - 1] + 1;
                }
                break;
            }
            if i == 0 {
                return out;
            }
        }
        if k == 0 {
            return out;
        }
    }
}

fn lookup_idx(slice: &[usize], idx: usize) -> Result<usize, String> {
    slice
        .get(idx)
        .copied()
        .ok_or_else(|| format!("Invalid index {}: {} available", idx, slice.len()))
}

// Pops effect_pending and re-enqueues it as Direct for each id; caller is responsible for the pending-kind check
fn resolve_hand_pending(state: &mut GameState, idxs: Vec<usize>) -> Result<Vec<Effect>, String> {
    let effect_pending = state.effect_pending.take().unwrap();

    // Reverse so push_front yields the original idxs order at the queue front
    for &idx in idxs.iter().rev() {
        let id_card = state
            .id_hand
            .get(idx)
            .copied()
            .unwrap_or_else(|| panic!("Invalid hand index {idx}"));
        state.effect_queue.push_front(Effect {
            kind: effect_pending.kind,
            id_source: effect_pending.id_source,
            target: Target::Direct(Some(id_card)),
        });
    }
    Ok(Vec::new())
}

// Pops effect_pending and re-enqueues it as Direct for the resolved deck-card id
fn resolve_deck_pending(state: &mut GameState, idx: usize) -> Result<Vec<Effect>, String> {
    let pending = state
        .effect_pending
        .take()
        .ok_or("Deck pick: no effect_pending")?;
    let id_card = *state
        .buf_candidates
        .get(idx)
        .ok_or_else(|| format!("Deck pick: idx {} out of range", idx))?;
    state.effect_queue.push_front(Effect {
        kind: pending.kind,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
    Ok(Vec::new())
}
