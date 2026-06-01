use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::Target;
use crate::effect::get_input_count;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::event_option_gate_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::potions::find_free_slot;
use crate::types::CardKind;
use crate::types::DeltaSign;
use crate::types::RewardKind;
use crate::types::Screen;
use crate::utils::card_is_upgradable;
use crate::utils::deck_filter_matches;
use crate::utils::flush_effects_from_buf_to_queue_front;

#[derive(Debug, Clone, PartialEq)]
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

pub fn handle_action(state: &mut GameState, action: Action) -> Result<(), String> {
    if state.game_over {
        return Err("GameOver".into());
    }
    if !state.legal_actions.contains(&action) {
        return Err(format!("Illegal action {:?} in current state", action));
    }

    // Handlers push their effects into effect_buf; flush drains them to the queue front (reversed)
    state.effect_buf.clear();
    match action {
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
        Action::RewardTakeCard { idx } => handle_reward_take_card(state, idx),
        Action::RewardTakeGold => handle_reward_take_gold(state),
        Action::RewardTakePotion => handle_reward_take_potion(state),
        Action::RewardTakeRelic => handle_reward_take_relic(state),
        Action::RoomExit => handle_room_exit(state),
        Action::RoomSelect { idx } => handle_room_select(state, idx),
        Action::TurnEnd => handle_turn_end(state),
    }
    flush_effects_from_buf_to_queue_front(state);
    Ok(())
}

pub fn recompute_legal_actions(state: &mut GameState) {
    state.legal_actions.clear();
    if state.game_over {
        return;
    }

    // `state.effect_pending` takes precedence over `state.screen`
    if let Some(effect_pending) = state.effect_pending.as_ref() {
        // Copy out the halt's shape so the &mut dispatch below can't alias the borrow
        let effect_pending_kind = effect_pending.kind;
        let count = get_input_count(effect_pending).expect("Expected some input count, got `None`")
            as usize;
        let deck_filter = pending_deck_filter(effect_pending);
        fill_legal_actions_effect_pending(state, effect_pending_kind, count, deck_filter);
        return;
    }
    match state.screen {
        Screen::Combat => fill_legal_actions_screen_combat(state),
        Screen::Reward => fill_legal_actions_screen_reward(state),
        Screen::Event => fill_legal_actions_screen_event(state),
        Screen::Shop => fill_legal_actions_screen_shop(state),
        Screen::Map => fill_legal_actions_screen_map(state),
        Screen::RestSite => fill_legal_actions_screen_rest_site(state),
        Screen::Chest => fill_legal_actions_screen_chest(state),
    }
}

fn handle_card_discard(state: &mut GameState, idxs: Vec<usize>) {
    resolve_hand_pending(state, idxs);
}

fn handle_card_discover(state: &mut GameState, idx: usize) {
    let id_card = state.id_discover[idx];
    let pending = state
        .effect_pending
        .take()
        .expect("CardDiscover requires a pending effect");
    state.effect_buf.push(Effect {
        kind: EffectKind::CardDiscoverPick,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
}

fn handle_card_duplicate(state: &mut GameState, idx: usize) {
    resolve_deck_pending(state, idx);
}

fn handle_card_nightmare(state: &mut GameState, idx: usize) {
    resolve_hand_pending(state, vec![idx]);
}

fn handle_card_play(state: &mut GameState, idx_card: usize, idx_monster: Option<usize>) {
    let id_card = state.id_hand[idx_card];
    if state.entities[id_card].requires_target {
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = state
            .id_monsters
            .iter()
            .flatten()
            .copied()
            .nth(idx_monster)
            .expect("enumerated monster idx is valid");

        // TargetSet -> CardPlay -> TargetClear
        state.effect_buf.push(Effect {
            kind: EffectKind::TargetSet,
            id_source: None,
            target: Target::Direct(Some(id_monster_target)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlay,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::TargetClear,
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlay,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
}

fn handle_card_purge(state: &mut GameState, idx: usize) {
    resolve_deck_pending(state, idx);
}

fn handle_card_retain(state: &mut GameState, idxs: Vec<usize>) {
    resolve_hand_pending(state, idxs);
}

fn handle_card_setup(state: &mut GameState, idx: usize) {
    resolve_hand_pending(state, vec![idx]);
}

fn handle_card_transform(state: &mut GameState, idx: usize) {
    resolve_deck_pending(state, idx);
}

fn handle_card_upgrade(state: &mut GameState, idx: usize) {
    // Dual-mode: a pending CardUpgrade resolves a deck pick; at a rest site it triggers a direct upgrade
    if state.effect_pending.is_some() {
        resolve_deck_pending(state, idx);
        return;
    }
    // idx is an absolute id_deck index; membership guaranteed validity, assert upgradability
    let id_card = state.id_deck[idx];
    assert!(
        card_is_upgradable(&state.entities[id_card]),
        "CardUpgrade idx {idx} targets a non-upgradable deck card"
    );
    let id_room = current_room_id(state);
    state.effect_buf.push(Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSiteConsume,
        id_source: None,
        target: Target::Direct(Some(id_room)),
    });
}

fn handle_chest_open(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_event_option_select(state: &mut GameState, idx: usize) {
    let id_event = state
        .id_event
        .expect("Event screen implies id_event is set");
    let event_option = state.entities[id_event].event_options[idx];
    for effect in event_option.effects {
        state.effect_buf.push(Effect {
            id_source: Some(id_event),
            ..*effect
        });
    }
}

fn handle_potion_discard(state: &mut GameState, idx: usize) {
    let id_potion = state.id_potions[idx].expect("enumerated potion slot is occupied");

    state.effect_buf.push(Effect {
        kind: EffectKind::PotionDiscard,
        id_source: Some(id_potion),
        target: Target::Direct(Some(id_potion)),
    });
}

fn handle_potion_use(state: &mut GameState, idx_potion: usize, idx_monster: Option<usize>) {
    let id_potion =
        state.id_potions[idx_potion].expect("enumerated potion slot is occupied");
    if state.entities[id_potion].requires_target {
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = state
            .id_monsters
            .iter()
            .flatten()
            .copied()
            .nth(idx_monster)
            .expect("enumerated monster idx is valid");

        // TargetSet -> PotionUse -> TargetClear
        state.effect_buf.push(Effect {
            kind: EffectKind::TargetSet,
            id_source: None,
            target: Target::Direct(Some(id_monster_target)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::PotionUse,
            id_source: Some(id_potion),
            target: Target::Direct(Some(id_potion)),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::TargetClear,
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        state.effect_buf.push(Effect {
            kind: EffectKind::PotionUse,
            id_source: Some(id_potion),
            target: Target::Direct(Some(id_potion)),
        });
    }
}

fn handle_rest(state: &mut GameState) {
    let id_character = state.id_character;
    let id_room = current_room_id(state);

    // Heal, then RestSiteConsume marks the site used; the explicit RoomExit leaves it
    state.effect_buf.push(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Relative {
                numerator: 3,
                denominator: 10,
            },
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSiteConsume,
        id_source: None,
        target: Target::Direct(Some(id_room)),
    });
}

fn handle_reward_take_card(state: &mut GameState, idx: usize) {
    let id_card = state.reward_id_cards[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Card,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}

fn handle_reward_take_gold(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Gold,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_reward_take_potion(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Potion,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_reward_take_relic(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Relic,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_room_exit(state: &mut GameState) {
    // RoomExit's processor does the screen-specific cleanup and the transition to Map (or boss)
    state.effect_buf.push(Effect {
        kind: EffectKind::RoomExit,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_room_select(state: &mut GameState, idx: usize) {
    // Membership guarantees the column has a reachable room, so row < MAP_HEIGHT and the room exists
    let y_next = match state.location {
        Location::Start => 0,
        Location::Overworld { y, .. } => y + 1,
        Location::BossRoom => unreachable!("RoomSelect not enumerated from the boss room"),
    };
    let id_room = state.id_rooms[y_next][idx].expect("enumerated room exists");
    state.effect_buf.push(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Direct(Some(id_room)),
    });
}

fn handle_turn_end(state: &mut GameState) {
    let id_character = state.id_character;
    state.effect_buf.push(Effect {
        kind: EffectKind::TurnEnd,
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}

fn fill_legal_actions_effect_pending(
    state: &mut GameState,
    kind: EffectKind,
    num: usize,
    deck_filter: Option<CandidatePoolDeckFilter>,
) {
    match kind {
        EffectKind::CardDiscard { .. } => {
            for combo in hand_combinations(state.id_hand.len(), num) {
                state
                    .legal_actions
                    .push(Action::CardDiscard { idxs: combo });
            }
        }
        EffectKind::CardRetain => {
            for combo in hand_combinations(state.id_hand.len(), num) {
                state.legal_actions.push(Action::CardRetain { idxs: combo });
            }
        }
        EffectKind::CardSetupPick => {
            for i in 0..state.id_hand.len() {
                state.legal_actions.push(Action::CardSetup { idx: i });
            }
        }
        EffectKind::CardNightmarePick => {
            for i in 0..state.id_hand.len() {
                state.legal_actions.push(Action::CardNightmare { idx: i });
            }
        }
        EffectKind::CardDiscoverPick => {
            for i in 0..state.id_discover.len() {
                state.legal_actions.push(Action::CardDiscover { idx: i });
            }
        }
        EffectKind::CardPurge => {
            let filter = deck_filter.expect("deck pick carries a Deck filter");
            for i in 0..state.id_deck.len() {
                if deck_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardPurge { idx: i });
                }
            }
        }
        EffectKind::CardUpgrade => {
            let filter = deck_filter.expect("deck pick carries a Deck filter");
            for i in 0..state.id_deck.len() {
                if deck_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardUpgrade { idx: i });
                }
            }
        }
        EffectKind::CardDuplicate => {
            let filter = deck_filter.expect("deck pick carries a Deck filter");
            for i in 0..state.id_deck.len() {
                if deck_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardDuplicate { idx: i });
                }
            }
        }
        EffectKind::CardTransform => {
            let filter = deck_filter.expect("deck pick carries a Deck filter");
            for i in 0..state.id_deck.len() {
                if deck_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardTransform { idx: i });
                }
            }
        }
        _ => unreachable!("effect_pending with non-halting kind: {:?}", kind),
    }
}

fn fill_legal_actions_screen_combat(state: &mut GameState) {
    let id_character = state.id_character;
    let entangled = modifier_has(
        &state.entities[id_character].modifiers,
        ModifierKind::Entangled,
    );
    let alive_count = state.id_monsters.iter().flatten().count();
    for i in 0..state.id_hand.len() {
        let card = &state.entities[state.id_hand[i]];
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
        let requires_target = card.requires_target;
        if requires_target {
            for m in 0..alive_count {
                state.legal_actions.push(Action::CardPlay {
                    idx_card: i,
                    idx_monster: Some(m),
                });
            }
        } else {
            state.legal_actions.push(Action::CardPlay {
                idx_card: i,
                idx_monster: None,
            });
        }
    }
    push_potion_actions(state);
    state.legal_actions.push(Action::TurnEnd);
}

fn fill_legal_actions_screen_reward(state: &mut GameState) {
    for i in 0..state.reward_id_cards.len() {
        state.legal_actions.push(Action::RewardTakeCard { idx: i });
    }
    if state.reward_id_relic.is_some() {
        state.legal_actions.push(Action::RewardTakeRelic);
    }
    if state.reward_id_potion.is_some()
        && find_free_slot(&state.id_potions, state.potion_slots_max).is_some()
    {
        state.legal_actions.push(Action::RewardTakePotion);
    }
    if state.reward_gold.is_some() {
        state.legal_actions.push(Action::RewardTakeGold);
    }
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn fill_legal_actions_screen_event(state: &mut GameState) {
    let id_event = state.id_event.expect("Event context requires id_event");
    if state.entities[id_event].event_consumed {
        state.legal_actions.push(Action::RoomExit);
    } else {
        for i in 0..state.entities[id_event].event_options.len() {
            let gate = state.entities[id_event].event_options[i].gate;
            if event_option_gate_satisfied(gate, state, id_event) {
                state
                    .legal_actions
                    .push(Action::EventOptionSelect { idx: i });
            }
        }
    }
    push_potion_actions(state);
}

fn fill_legal_actions_screen_shop(state: &mut GameState) {
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn fill_legal_actions_screen_map(state: &mut GameState) {
    push_room_select_actions(state);
    push_potion_actions(state);
}

fn fill_legal_actions_screen_rest_site(state: &mut GameState) {
    if state.entities[current_room_id(state)].room_rest_site_done {
        state.legal_actions.push(Action::RoomExit);
    } else {
        state.legal_actions.push(Action::Rest);
        // CardUpgrade idx is an absolute id_deck index; offer only upgradable cards
        for i in 0..state.id_deck.len() {
            if card_is_upgradable(&state.entities[state.id_deck[i]]) {
                state.legal_actions.push(Action::CardUpgrade { idx: i });
            }
        }
    }
    push_potion_actions(state);
}

fn fill_legal_actions_screen_chest(state: &mut GameState) {
    state.legal_actions.push(Action::ChestOpen);
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn push_room_select_actions(state: &mut GameState) {
    match state.location {
        Location::Start => {
            for c in 0..MAP_WIDTH {
                if state.id_rooms[0][c].is_some() {
                    state.legal_actions.push(Action::RoomSelect { idx: c });
                }
            }
        }
        Location::Overworld { y, x } => {
            let y_next = y + 1;
            if y_next >= MAP_HEIGHT {
                return;
            }
            if let Some(id_current) = state.id_rooms[y][x] {
                let edges = state.entities[id_current].room_edges;
                for c in 0..MAP_WIDTH {
                    if has_edge(edges, c) && state.id_rooms[y_next][c].is_some() {
                        state.legal_actions.push(Action::RoomSelect { idx: c });
                    }
                }
            }
        }
        Location::BossRoom => {}
    }
}

fn push_potion_actions(state: &mut GameState) {
    let in_combat = matches!(state.screen, Screen::Combat);
    let alive_count = state.id_monsters.iter().flatten().count();
    let slots_max = state.potion_slots_max as usize;
    for s in 0..slots_max {
        let Some(id_potion) = state.id_potions[s] else {
            continue;
        };
        let potion = &state.entities[id_potion];
        let combat_only = potion.potion_combat_only;
        let requires_target = potion.requires_target;
        if combat_only && !in_combat {
            state.legal_actions.push(Action::PotionDiscard { idx: s });
            continue;
        }
        if requires_target {
            if in_combat {
                for m in 0..alive_count {
                    state.legal_actions.push(Action::PotionUse {
                        idx_potion: s,
                        idx_monster: Some(m),
                    });
                }
            }
        } else {
            state.legal_actions.push(Action::PotionUse {
                idx_potion: s,
                idx_monster: None,
            });
        }
        state.legal_actions.push(Action::PotionDiscard { idx: s });
    }
}

fn current_room_id(state: &GameState) -> usize {
    match state.location {
        Location::Overworld { y, x } => state.id_rooms[y][x].expect("current room must exist"),
        Location::Start | Location::BossRoom => panic!("no current room outside the overworld"),
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

// Pops effect_pending and re-enqueues it as Direct for each id
fn resolve_hand_pending(state: &mut GameState, idxs: Vec<usize>) {
    let effect_pending = state.effect_pending.take().unwrap();

    // Push in idxs order; flush reverses into the queue front, preserving order
    for &idx in &idxs {
        let id_card = state.id_hand[idx];
        state.effect_buf.push(Effect {
            kind: effect_pending.kind,
            id_source: effect_pending.id_source,
            target: Target::Direct(Some(id_card)),
        });
    }
}

// Pops effect_pending and re-enqueues it as Direct for the resolved deck-card id
// Extract the Deck filter from a pending deck-pick effect; None for non-deck halts
fn pending_deck_filter(effect: &Effect) -> Option<CandidatePoolDeckFilter> {
    match effect.target {
        Target::Resolve {
            candidate_pool: CandidatePool::Deck { filter },
            ..
        } => Some(filter),
        _ => None,
    }
}

fn resolve_deck_pending(state: &mut GameState, idx: usize) {
    let pending = state
        .effect_pending
        .take()
        .expect("deck pick requires a pending effect");
    // idx is an absolute id_deck index; assert it still matches the pool's filter
    let filter = pending_deck_filter(&pending).expect("deck pick has a Deck pool");
    let id_card = state.id_deck[idx];
    assert!(
        deck_filter_matches(filter, &state.entities[id_card]),
        "deck pick idx {idx} targets a card the filter rejects"
    );
    state.effect_buf.push(Effect {
        kind: pending.kind,
        id_source: pending.id_source,
        target: Target::Direct(Some(id_card)),
    });
}
