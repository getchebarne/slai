use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::REST_SITE_HEAL_FACTOR;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::input_count;
use crate::engine::enqueue_direct_targets;
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
use crate::types::RewardKind;
use crate::types::Screen;

#[derive(Debug, Clone)]
pub enum Action {
    HandSelect {
        idxs: Vec<usize>,
    },
    CardPlay {
        idx_hand: usize,
        idx_monster: Option<usize>,
    },
    RestSiteCardUpgrade {
        idx_deck: usize,
    },
    RewardTakeCard {
        idx_reward: usize,
    },
    RewardTakeRelic,
    RewardTakePotion,
    RewardTakeGold,
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
    CardDiscoverSelect {
        idx_option: usize,
    },
    EventChoice {
        idx_option: usize,
    },
    DeckSelect {
        idx_option: usize,
    },
}

fn validate(action: &Action, state: &GameState) -> Result<(), String> {
    if state.game_over {
        return Err("GameOver".into());
    }

    if let Some(pending) = state.pending_effect.as_ref() {
        return match pending.kind {
            EffectKind::CardDiscard { .. }
            | EffectKind::CardRetain
            | EffectKind::CardSetupPick
            | EffectKind::CardNightmarePick => {
                validate_hand_select(action, input_count(pending).unwrap())
            }
            EffectKind::CardDiscoverPick => validate_discover(action),
            EffectKind::CardPurge
            | EffectKind::CardUpgrade
            | EffectKind::CardDuplicate
            | EffectKind::CardTransform => validate_deck_select(action),
            EffectKind::RoomSelect => validate_room_select(action),
            _ => unreachable!("pending_effect with non-halting kind: {:?}", pending.kind),
        };
    }

    match state.screen {
        Screen::Combat => validate_combat(action),
        Screen::Reward => validate_reward(action),
        Screen::Event => validate_event(action),
        Screen::Shop => validate_shop(action),
        Screen::Map => validate_map(action),
        Screen::RestSite => validate_rest_site(action, state),
        Screen::Chest => validate_chest(action),
    }
}

fn validate_hand_select(action: &Action, num: u16) -> Result<(), String> {
    let Action::HandSelect { idxs } = action else {
        return Err(format!("Expected HandSelect, got {:?}", action));
    };
    if idxs.len() != num as usize {
        return Err(format!(
            "HandSelect expects {} idxs, got {}",
            num,
            idxs.len()
        ));
    }
    Ok(())
}

fn validate_room_select(action: &Action) -> Result<(), String> {
    match action {
        Action::RoomSelect { .. } | Action::PotionUse { .. } | Action::PotionDiscard { .. } => {
            Ok(())
        }
        _ => Err(format!(
            "Expected RoomSelect/PotionUse/PotionDiscard, got {:?}",
            action
        )),
    }
}

fn validate_discover(action: &Action) -> Result<(), String> {
    match action {
        Action::CardDiscoverSelect { .. } => Ok(()),
        _ => Err(format!("Expected CardDiscoverSelect, got {:?}", action)),
    }
}

fn validate_deck_select(action: &Action) -> Result<(), String> {
    match action {
        Action::DeckSelect { .. } => Ok(()),
        _ => Err(format!("Expected DeckSelect, got {:?}", action)),
    }
}

fn validate_combat(action: &Action) -> Result<(), String> {
    match action {
        Action::CardPlay { .. }
        | Action::EndTurn
        | Action::PotionUse { .. }
        | Action::PotionDiscard { .. } => Ok(()),
        _ => Err(format!("Action {:?} invalid in Combat context", action)),
    }
}

fn validate_reward(action: &Action) -> Result<(), String> {
    match action {
        Action::RewardTakeCard { .. }
        | Action::RewardTakeRelic
        | Action::RewardTakePotion
        | Action::RewardTakeGold
        | Action::RewardSkip
        | Action::PotionDiscard { .. } => Ok(()),
        _ => Err(format!("Action {:?} invalid in Reward context", action)),
    }
}

fn validate_event(action: &Action) -> Result<(), String> {
    match action {
        Action::EventChoice { .. } | Action::PotionUse { .. } | Action::PotionDiscard { .. } => {
            Ok(())
        }
        _ => Err(format!("Action {:?} invalid in Event context", action)),
    }
}

fn validate_shop(action: &Action) -> Result<(), String> {
    match action {
        Action::RoomSkip | Action::PotionUse { .. } | Action::PotionDiscard { .. } => Ok(()),
        _ => Err(format!("Action {:?} invalid in Shop context", action)),
    }
}

fn validate_map(action: &Action) -> Result<(), String> {
    match action {
        Action::RoomSelect { .. } | Action::PotionUse { .. } | Action::PotionDiscard { .. } => {
            Ok(())
        }
        _ => Err(format!("Action {:?} invalid on Map", action)),
    }
}

fn validate_rest_site(action: &Action, state: &GameState) -> Result<(), String> {
    match action {
        Action::PotionUse { .. } | Action::PotionDiscard { .. } => Ok(()),
        Action::RestSiteRest => Ok(()),
        Action::RestSiteCardUpgrade { idx_deck } => {
            if *idx_deck >= state.id_deck.len() {
                return Err(format!(
                    "RestSiteCardUpgrade: idx_deck {} out of range",
                    idx_deck
                ));
            }
            Ok(())
        }
        _ => Err(format!("Action {:?} invalid at RestSite", action)),
    }
}

fn validate_chest(action: &Action) -> Result<(), String> {
    match action {
        Action::ChestOpen | Action::PotionUse { .. } | Action::PotionDiscard { .. } => Ok(()),
        _ => Err(format!("Action {:?} invalid at Chest", action)),
    }
}

pub fn handle_action(state: &mut GameState, action: Action) -> Result<Vec<Effect>, String> {
    validate(&action, state)?;

    let effects = match action {
        Action::HandSelect { idxs } => handle_hand_select(state, idxs),
        Action::CardPlay {
            idx_hand,
            idx_monster,
        } => handle_card_play(state, idx_hand, idx_monster),
        Action::RestSiteCardUpgrade { idx_deck } => handle_rest_site_card_upgrade(state, idx_deck),
        Action::RewardTakeCard { idx_reward } => handle_reward_take_card(state, idx_reward),
        Action::RewardTakeRelic => handle_reward_take_relic(state),
        Action::RewardTakePotion => handle_reward_take_potion(state),
        Action::RewardTakeGold => handle_reward_take_gold(state),
        Action::RewardSkip => Ok(handle_reward_skip()),
        Action::EndTurn => Ok(handle_end_turn(state)),
        Action::RoomSelect { idx_column } => handle_room_select(state, idx_column),
        Action::RestSiteRest => Ok(handle_rest_site_rest(state)),
        Action::RoomSkip => Ok(handle_room_skip()),
        Action::ChestOpen => Ok(handle_chest_open()),
        Action::PotionUse {
            idx_slot,
            idx_monster,
        } => handle_potion_use(state, idx_slot, idx_monster),
        Action::PotionDiscard { idx_slot } => handle_potion_discard(state, idx_slot),
        Action::CardDiscoverSelect { idx_option } => handle_card_discover_select(state, idx_option),
        Action::EventChoice { idx_option } => handle_event_choice(state, idx_option),
        Action::DeckSelect { idx_option } => handle_deck_select(state, idx_option),
    }?;

    Ok(effects)
}

// Mirrors validate() shape; empty when game_over
pub fn get_legal_actions(state: &GameState) -> Vec<Action> {
    if state.game_over {
        return Vec::new();
    }
    if let Some(pending) = state.pending_effect.as_ref() {
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
        EffectKind::CardDiscard { .. } | EffectKind::CardRetain => {
            let num = input_count(pending).unwrap_or(1) as usize;
            for combo in hand_combinations(state.id_hand.len(), num) {
                actions.push(Action::HandSelect { idxs: combo });
            }
        }
        EffectKind::CardSetupPick | EffectKind::CardNightmarePick => {
            for i in 0..state.id_hand.len() {
                actions.push(Action::HandSelect { idxs: vec![i] });
            }
        }
        EffectKind::CardDiscoverPick => {
            for i in 0..state.id_pick.len() {
                actions.push(Action::CardDiscoverSelect { idx_option: i });
            }
        }
        EffectKind::CardPurge
        | EffectKind::CardUpgrade
        | EffectKind::CardDuplicate
        | EffectKind::CardTransform => {
            // buf_candidates was populated by resolve_or_halt at halt time
            for i in 0..state.buf_candidates.len() {
                actions.push(Action::DeckSelect { idx_option: i });
            }
        }
        EffectKind::RoomSelect => {
            push_room_select_actions(state, &mut actions);
            push_potion_actions(state, &mut actions);
        }
        _ => unreachable!("pending_effect with non-halting kind: {:?}", pending.kind),
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
            state.energy.current,
        );
        if cost > state.energy.current {
            continue;
        }
        if card.requires_target {
            for m in 0..alive_count {
                actions.push(Action::CardPlay {
                    idx_hand: i,
                    idx_monster: Some(m),
                });
            }
        } else {
            actions.push(Action::CardPlay {
                idx_hand: i,
                idx_monster: None,
            });
        }
    }
    push_potion_actions(state, &mut actions);
    actions.push(Action::EndTurn);
    actions
}

fn legal_actions_reward(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    for i in 0..state.reward_id_cards.len() {
        actions.push(Action::RewardTakeCard { idx_reward: i });
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
    push_potion_discard_actions(state, &mut actions);
    actions
}

fn legal_actions_event(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();
    let id_event = state.id_event.expect("Event context requires id_event");
    let event = &state.entities[id_event];
    for (i, opt) in event.event_options.iter().enumerate() {
        if event_option_gate_satisfied(opt.gate, state, id_event) {
            actions.push(Action::EventChoice { idx_option: i });
        }
    }
    push_potion_actions(state, &mut actions);
    actions
}

fn legal_actions_shop(state: &GameState) -> Vec<Action> {
    let mut actions = vec![Action::RoomSkip];
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
    let mut actions = vec![Action::RestSiteRest];
    for (i, &id_card) in state.id_deck.iter().enumerate() {
        if !state.entities[id_card].card_upgraded {
            actions.push(Action::RestSiteCardUpgrade { idx_deck: i });
        }
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
                    actions.push(Action::RoomSelect { idx_column: c });
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
                        actions.push(Action::RoomSelect { idx_column: c });
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
            actions.push(Action::PotionDiscard { idx_slot: s });
            continue;
        }
        if potion.requires_target {
            if in_combat {
                for m in 0..alive_count {
                    actions.push(Action::PotionUse {
                        idx_slot: s,
                        idx_monster: Some(m),
                    });
                }
            }
        } else {
            actions.push(Action::PotionUse {
                idx_slot: s,
                idx_monster: None,
            });
        }
        actions.push(Action::PotionDiscard { idx_slot: s });
    }
}

fn push_potion_discard_actions(state: &GameState, actions: &mut Vec<Action>) {
    let character = &state.entities[state.id_character];
    for s in 0..character.potion_slots_max as usize {
        if character.potion_slots[s].is_some() {
            actions.push(Action::PotionDiscard { idx_slot: s });
        }
    }
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
    if !matches!(state.screen, Screen::Combat) {
        return Err("CardPlay outside Combat context".into());
    }
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

fn handle_hand_select(state: &mut GameState, idxs: Vec<usize>) -> Result<Vec<Effect>, String> {
    if !matches!(state.screen, Screen::Combat) {
        return Err("HandSelect outside Combat context".into());
    }
    let pending = state
        .pending_effect
        .take()
        .ok_or("HandSelect: no pending_effect")?;
    let id_cards: Vec<usize> =
        idxs.iter()
            .map(|&idx| {
                state.id_hand.get(idx).copied().ok_or_else(|| {
                    format!("Invalid hand index {}: {} cards", idx, state.id_hand.len())
                })
            })
            .collect::<Result<_, _>>()?;
    enqueue_direct_targets(
        pending.id_source,
        &id_cards,
        pending.kind,
        &mut state.effect_queue,
    );
    Ok(Vec::new())
}

fn handle_room_select(state: &mut GameState, idx_column: usize) -> Result<Vec<Effect>, String> {
    if idx_column >= MAP_WIDTH {
        return Err(format!(
            "Invalid column {}: max is {}",
            idx_column,
            MAP_WIDTH - 1
        ));
    }

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

    let pending = state
        .pending_effect
        .take()
        .ok_or("RoomSelect: no pending_effect")?;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RoomSelect,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_room)),
    });
    Ok(Vec::new())
}

fn handle_reward_take_card(state: &GameState, idx_reward: usize) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Reward));
    let id_card = lookup_idx(&state.reward_id_cards, idx_reward)?;
    Ok(vec![Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Card,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    }])
}

fn handle_reward_take_relic(state: &GameState) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Reward));
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

fn handle_reward_take_potion(state: &GameState) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Reward));
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

fn handle_reward_take_gold(state: &GameState) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Reward));
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

fn handle_reward_skip() -> Vec<Effect> {
    vec![Effect {
        kind: EffectKind::RewardSkip,
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
            kind: EffectKind::HealthDelta {
                sign: HealthDeltaSign::Gain,
                amount: HealthDeltaAmount::Absolute(heal_amt),
            },
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
            candidate_pool: CandidatePool::NextRowRooms,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }]
}

fn handle_chest_open() -> Vec<Effect> {
    vec![Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    }]
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

fn handle_potion_discard(state: &mut GameState, idx_slot: usize) -> Result<Vec<Effect>, String> {
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

fn handle_card_discover_select(
    state: &mut GameState,
    idx_option: usize,
) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Combat));
    let id_card = *state
        .id_pick
        .get(idx_option)
        .ok_or_else(|| format!("CardDiscoverSelect: idx_option {} out of range", idx_option))?;
    let pending = state
        .pending_effect
        .take()
        .ok_or("CardDiscoverSelect: no pending_effect")?;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDiscoverPick,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
    Ok(Vec::new())
}

fn handle_rest_site_card_upgrade(
    state: &GameState,
    idx_deck: usize,
) -> Result<Vec<Effect>, String> {
    let id_card = lookup_idx(&state.id_deck, idx_deck)?;

    // Upgrade by id; RestSiteExit decides halt vs boss-room transition
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

fn handle_event_choice(state: &mut GameState, idx_option: usize) -> Result<Vec<Effect>, String> {
    debug_assert!(matches!(state.screen, Screen::Event));
    let id_event = state.id_event.expect("validate guarantees Event context");
    let event = &state.entities[id_event];
    if idx_option >= event.event_options.len() {
        return Err(format!(
            "EventChoice: idx_option {} out of range (options {})",
            idx_option,
            event.event_options.len()
        ));
    }
    let option = event.event_options[idx_option];
    if !event_option_gate_satisfied(option.gate, state, id_event) {
        return Err(format!(
            "EventChoice: option {} gated out ({:?})",
            idx_option, option.gate
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

fn handle_deck_select(state: &mut GameState, idx_option: usize) -> Result<Vec<Effect>, String> {
    let pending = state
        .pending_effect
        .take()
        .ok_or("DeckSelect: no pending_effect")?;
    if !matches!(
        pending.kind,
        EffectKind::CardPurge
            | EffectKind::CardUpgrade
            | EffectKind::CardDuplicate
            | EffectKind::CardTransform
    ) {
        return Err(format!(
            "DeckSelect: pending_effect kind is {:?}, expected a deck-pick halt",
            pending.kind
        ));
    }
    let id_card = *state
        .buf_candidates
        .get(idx_option)
        .ok_or_else(|| format!("DeckSelect: idx_option {} out of range", idx_option))?;
    state.effect_queue.push_front(Effect {
        kind: pending.kind,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
    Ok(Vec::new())
}
