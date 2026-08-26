use crate::consts::GIRYA_LIFT_MAX;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::event_option_available;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::map::has_edge;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::potions::find_free_slot;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::Event;
use crate::types::Focus;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::types::Reward;
use crate::types::RewardKind;
use crate::types::RoomKind;
use crate::types::Shop;
use crate::types::ShopSlot;
use crate::utils::candidate_matches;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::context_focus;
use crate::utils::entity_requires_target;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::get_card_effective_cost;
use crate::utils::has_relic;
use crate::utils::is_play_restriction_satisfied;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    CardPlay {
        idx_card: usize,
        idx_monster: Option<usize>,
    },
    ChestOpen,
    EffectPendingResolve {
        idx: usize,
    },
    EventOptionSelect {
        idx: usize,
    },
    PickSkip,
    PotionDiscard {
        idx: usize,
    },
    PotionUse {
        idx_potion: usize,
        idx_monster: Option<usize>,
    },
    Rest,
    RestDig,
    RestLift,
    RestSmith,
    RestToke,
    RewardSingingBowl {
        idx_bundle: usize,
    },
    RewardTakeCard {
        idx_bundle: usize,
        idx_card: usize,
    },
    RewardTakeGold,
    RewardTakePotion {
        idx: usize,
    },
    RewardTakeRelic {
        idx: usize,
    },
    RoomExit,
    RoomSelect {
        idx: usize,
    },
    ShopBuyCard {
        idx: usize,
    },
    ShopBuyPotion {
        idx: usize,
    },
    ShopBuyRelic {
        idx: usize,
    },
    ShopPurge,
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
        Action::CardPlay {
            idx_card,
            idx_monster,
        } => handle_card_play(state, idx_card, idx_monster),
        Action::ChestOpen => handle_chest_open(state),
        Action::EffectPendingResolve { idx } => handle_effect_pending_resolve(state, idx),
        Action::EventOptionSelect { idx } => handle_event_option_select(state, idx),
        Action::PickSkip => handle_pick_skip(state),
        Action::PotionDiscard { idx } => handle_potion_discard(state, idx),
        Action::PotionUse {
            idx_potion,
            idx_monster,
        } => handle_potion_use(state, idx_potion, idx_monster),
        Action::Rest => handle_rest(state),
        Action::RestDig => handle_rest_dig(state),
        Action::RestLift => handle_rest_lift(state),
        Action::RestSmith => handle_rest_smith(state),
        Action::RestToke => handle_rest_toke(state),
        Action::RewardSingingBowl { idx_bundle } => handle_reward_singing_bowl(state, idx_bundle),
        Action::RewardTakeCard {
            idx_bundle,
            idx_card,
        } => handle_reward_take(state, RewardKind::Card, idx_bundle, idx_card),
        Action::RewardTakeGold => handle_reward_take(state, RewardKind::Gold, 0, 0),
        Action::RewardTakePotion { idx } => handle_reward_take(state, RewardKind::Potion, 0, idx),
        Action::RewardTakeRelic { idx } => handle_reward_take(state, RewardKind::Relic, 0, idx),
        Action::RoomExit => handle_room_exit(state),
        Action::RoomSelect { idx } => handle_room_select(state, idx),
        Action::ShopBuyCard { idx } => handle_shop_buy(state, ShopSlot::Card, idx),
        Action::ShopBuyPotion { idx } => handle_shop_buy(state, ShopSlot::Potion, idx),
        Action::ShopBuyRelic { idx } => handle_shop_buy(state, ShopSlot::Relic, idx),
        Action::ShopPurge => handle_shop_purge(state),
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

    // `state.effect_pending` takes precedence over the context focus
    if let Some(effect_pending) = state.effect_pending {
        // effect_pending is written only on the Target::Resolve halt path
        let Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        } = effect_pending.target
        else {
            unreachable!("effect_pending carries a Resolve target")
        };
        fill_legal_actions_effect_pending(state, filter, candidate_pool);
        if matches!(selection_kind, SelectionKind::InputUpTo { .. }) {
            state.legal_actions.push(Action::PickSkip);
        }
        return;
    }
    match context_focus(state) {
        Focus::Combat => fill_legal_actions_combat(state),
        Focus::Reward => fill_legal_actions_reward(state),
        Focus::Event => fill_legal_actions_event(state),
        Focus::Shop => fill_legal_actions_shop(state),
        Focus::Map => fill_legal_actions_map(state),
        Focus::RestSite => fill_legal_actions_rest_site(state),
        Focus::Chest => fill_legal_actions_chest(state),
    }
}

// idx indexes the collection named by the pending effect's candidate pool
fn handle_effect_pending_resolve(state: &mut GameState, idx: usize) {
    // Validate there's a pending `Effect` and, if so, take it
    let effect_pending = state
        .effect_pending
        .take()
        .expect("EffectPendingResolve requires a pending effect");

    // Validate the pending `Effect`'s target is `Target::Resolve`
    let Target::Resolve {
        candidate_pool,
        filter,
        selection_kind,
    } = effect_pending.target
    else {
        unreachable!("effect_pending carries a Resolve target")
    };

    // Stage the selected `Entity` ID; nothing applies until the selection closes
    let id_selected = pool_collection(
        candidate_pool,
        &state.combat,
        &state.event,
        &state.id_card_deck,
    )[idx];
    state.effect_pending_picks.push(id_selected);

    let remaining = match selection_kind {
        SelectionKind::Input { count } | SelectionKind::InputUpTo { count } => {
            count.saturating_sub(1)
        }
        _ => panic!("Pending pick carries an Input halt"),
    };
    if remaining == 0 {
        flush_pending_picks(state, effect_pending.kind, effect_pending.id_source);
        return;
    }

    // Re-park with the remaining count; staged picks stay out of the pool
    let selection_kind = match selection_kind {
        SelectionKind::Input { .. } => SelectionKind::Input { count: remaining },
        _ => SelectionKind::InputUpTo { count: remaining },
    };
    state.effect_buf.push(Effect {
        kind: effect_pending.kind,
        id_source: effect_pending.id_source,
        target: Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        },
    });
}

// One Direct effect per staged pick, in pick order
fn flush_pending_picks(state: &mut GameState, kind: EffectKind, id_source: Option<usize>) {
    for &id in &state.effect_pending_picks {
        state.effect_buf.push(Effect {
            kind,
            id_source,
            target: Target::Direct(Some(id)),
        });
    }
    state.effect_pending_picks.clear();
}

// The indexable collection behind each halting pool
fn pool_collection<'a>(
    pool: CandidatePool,
    combat: &'a Combat,
    event: &'a Event,
    id_card_deck: &'a [usize],
) -> &'a [usize] {
    match pool {
        CandidatePool::Hand => {
            assert!(combat.active, "Hand pick outside combat");
            &combat.id_card_hand
        }
        CandidatePool::Discover => {
            assert!(combat.active, "Discover pick outside combat");
            &combat.id_card_discover
        }
        CandidatePool::PileDraw => {
            assert!(combat.active, "Pile pick outside combat");
            &combat.id_card_draw
        }
        CandidatePool::PileDiscard => {
            assert!(combat.active, "Pile pick outside combat");
            &combat.id_card_discard
        }
        CandidatePool::PileExhaust => {
            assert!(combat.active, "Pile pick outside combat");
            &combat.id_card_exhaust
        }
        CandidatePool::Deck => id_card_deck,
        CandidatePool::EventRollCard => {
            assert!(event.active, "Event roll pick outside an event");
            &event.id_roll_card
        }
        CandidatePool::EventRollRelic => {
            assert!(event.active, "Event roll pick outside an event");
            &event.id_roll_relic
        }
        CandidatePool::EventRollPotion => {
            assert!(event.active, "Event roll pick outside an event");
            &event.id_roll_potion
        }
        other => unreachable!("pick over non-indexable pool: {:?}", other),
    }
}

// Ends an InputUpTo halt early; staged picks apply, the remainder is forfeited
fn handle_pick_skip(state: &mut GameState) {
    let effect_pending = state
        .effect_pending
        .take()
        .expect("PickSkip requires a pending effect");
    flush_pending_picks(state, effect_pending.kind, effect_pending.id_source);
}

fn handle_card_play(state: &mut GameState, idx_card: usize, idx_monster: Option<usize>) {
    assert!(state.combat.active, "handle_card_play outside combat");
    let id_card = state.combat.id_card_hand[idx_card];
    if entity_requires_target(&state.entities[id_card]) {
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = state
            .combat
            .id_monsters
            .iter()
            .flatten()
            .copied()
            .nth(idx_monster)
            .expect("Enumerated monster idx is valid");

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

fn handle_chest_open(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_event_option_select(state: &mut GameState, idx: usize) {
    assert!(
        context_focus(state) == Focus::Event,
        "EventOptionSelect outside the Event context"
    );
    let id_option = state.event.id_event_options[idx];
    let effects = state.entities[id_option].event_option_effects;
    let effects_len = state.entities[id_option].event_option_effects_len as usize;
    for effect in &effects[..effects_len] {
        state.effect_buf.push(Effect {
            id_source: Some(id_option),
            ..*effect
        });
    }
}

fn handle_potion_discard(state: &mut GameState, idx: usize) {
    let id_potion = state.id_potions[idx].expect("Enumerated Potion slot is occupied");
    state.effect_buf.push(Effect {
        kind: EffectKind::PotionDiscard,
        id_source: Some(id_potion),
        target: Target::Direct(Some(id_potion)),
    });
}

fn handle_potion_use(state: &mut GameState, idx_potion: usize, idx_monster: Option<usize>) {
    let id_potion = state.id_potions[idx_potion].expect("Enumerated Potion slot is occupied");
    if entity_requires_target(&state.entities[id_potion]) {
        assert!(state.combat.active, "Targeted Potion use outside combat");
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = state
            .combat
            .id_monsters
            .iter()
            .flatten()
            .copied()
            .nth(idx_monster)
            .expect("Enumerated monster idx is valid");

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

// Marks the site used; every rest-site option ends with this
fn push_rest_site_consume(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSiteConsume,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_rest(state: &mut GameState) {
    let id_character = state.id_character;

    // Heal, then RestSiteConsume marks the site used; the explicit RoomExit leaves it
    state.effect_buf.push(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 3,
                denominator: 10,
            },
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    // Regal Pillow: resting heals 15 more
    if has_relic(&state.id_relics, RelicName::RegalPillow) {
        state.effect_buf.push(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(15),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }
    push_rest_site_consume(state);

    // Dream Catcher: resting also offers a Card reward (Rest only, not Smith)
    if has_relic(&state.id_relics, RelicName::DreamCatcher) {
        state.effect_buf.push(Effect {
            kind: EffectKind::RewardRollCards {
                bundles: 1,
                rare_only: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}

// Girya: spend the rest on +1 combat-start Strength
fn handle_rest_lift(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::GiryaLift,
        id_source: None,
        target: Target::Direct(None),
    });
    push_rest_site_consume(state);
}

// Peace Pipe: spend the rest on purging a Card (halting deck pick)
fn handle_rest_toke(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Purgeable,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
    push_rest_site_consume(state);
}

// Smith mirrors Toke: queue the deck pick, let the halt resolve it
fn handle_rest_smith(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Upgradeable,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
    push_rest_site_consume(state);
}

// Shovel: spend the rest on a random Relic (granted directly, not staged)
fn handle_rest_dig(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    });
    push_rest_site_consume(state);
}

// Singing Bowl: forfeit one Card bundle for +2 max HP
fn handle_reward_singing_bowl(state: &mut GameState, idx_bundle: usize) {
    state.effect_buf.push(Effect {
        kind: EffectKind::SingingBowlProc {
            idx_bundle: idx_bundle as u8,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}

// One processor handles all four kinds; the action layer only resolves idx -> id
fn handle_reward_take(state: &mut GameState, kind: RewardKind, idx_bundle: usize, idx: usize) {
    assert!(state.reward.active, "RewardTake outside the Reward context");
    let id_taken = match kind {
        RewardKind::Card => Some(state.reward.id_cards[idx_bundle][idx]),
        RewardKind::Relic => Some(state.reward.id_relics[idx]),
        RewardKind::Potion => Some(state.reward.id_potions[idx]),
        RewardKind::Gold => None,
    };
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake { kind },
        id_source: None,
        target: Target::Direct(id_taken),
    });
}

fn handle_room_exit(state: &mut GameState) {
    // RoomExit's processor does the per-context cleanup and the transition to Map (or boss)
    state.effect_buf.push(Effect {
        kind: EffectKind::RoomExit,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_room_select(state: &mut GameState, idx: usize) {
    // Membership guarantees the column has a reachable Room, so row < MAP_HEIGHT and the Room exists
    let y_next = match state.location {
        Location::Start => 0,
        Location::Overworld { y, .. } => y + 1,
        Location::BossRoom => unreachable!("RoomSelect not enumerated from the boss room"),
    };
    let id_room = state.id_rooms[y_next][idx].expect("Enumerated room exists");
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

fn handle_shop_buy(state: &mut GameState, slot: ShopSlot, idx: usize) {
    assert!(
        context_focus(state) == Focus::Shop,
        "ShopBuy outside the Shop context"
    );
    let id_bought = match slot {
        ShopSlot::Card => state.shop.cards[idx].0,
        ShopSlot::Relic => state.shop.relics[idx].0,
        ShopSlot::Potion => state.shop.potions[idx].0,
    };
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopBuy { slot },
        id_source: None,
        target: Target::Direct(Some(id_bought)),
    });
}

fn handle_shop_purge(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopPurge,
        id_source: None,
        target: Target::Direct(None),
    });
}

// One uniform arm: idx enumerates the pending pool's collection, filter-gated
fn fill_legal_actions_effect_pending(
    state: &mut GameState,
    filter: CandidateFilter,
    pool: CandidatePool,
) {
    // Get `CandidatePool`'s instanced IDs
    let id_collection = pool_collection(pool, &state.combat, &state.event, &state.id_card_deck);

    // Apply `CandidateFilter`; staged picks are out of the running
    for (idx, &id) in id_collection.iter().enumerate() {
        if !state.effect_pending_picks.contains(&id)
            && candidate_matches(filter, id, &state.entities[id], None, None)
        {
            state
                .legal_actions
                .push(Action::EffectPendingResolve { idx });
        }
    }
}

fn fill_legal_actions_combat(state: &mut GameState) {
    let Combat {
        id_card_hand,
        id_card_draw,
        id_monsters,
        energy,
        this_turn_discards,
        this_turn_cards_played,
        this_combat_damage_instances_taken,
        ..
    } = &state.combat;
    let id_character = state.id_character;

    // Entangled: can't play `CardKind::Attack` Cards
    let entangled = has_modifier(
        &state.entities[id_character].modifiers,
        ModifierKind::Entangled,
    );

    // Normality in hand caps the turn at 3 plays; blocks ANY further CardPlay
    let normality_blocks = *this_turn_cards_played >= 3
        && id_card_hand
            .iter()
            .any(|&id| state.entities[id].card_name == CardName::Normality);

    // Velvet Choker: no more than 6 Cards per turn (increment is post-play, so exactly 6 land)
    let choker_blocks =
        *this_turn_cards_played >= 6 && has_relic(&state.id_relics, RelicName::VelvetChoker);
    for idx in 0..id_card_hand.len() {
        if normality_blocks || choker_blocks {
            break;
        }
        let card = &state.entities[id_card_hand[idx]];
        let restriction_ok = is_play_restriction_satisfied(
            card.card_play_restriction,
            card.card_kind,
            &id_card_draw,
            &state.id_relics,
        );
        let entangled_blocks = entangled && card.card_kind == CardKind::Attack;
        if !restriction_ok || entangled_blocks {
            continue;
        }
        let cost = get_card_effective_cost(
            card,
            *this_turn_discards,
            *this_combat_damage_instances_taken,
            energy.energy_current,
        );
        if cost > energy.energy_current {
            continue;
        }
        let requires_target = entity_requires_target(card);
        let alive_count = id_monsters.iter().flatten().count();
        if requires_target {
            for m in 0..alive_count {
                state.legal_actions.push(Action::CardPlay {
                    idx_card: idx,
                    idx_monster: Some(m),
                });
            }
        } else {
            state.legal_actions.push(Action::CardPlay {
                idx_card: idx,
                idx_monster: None,
            });
        }
    }
    push_potion_actions(state);
    state.legal_actions.push(Action::TurnEnd);
}

fn fill_legal_actions_reward(state: &mut GameState) {
    let Reward {
        id_cards,
        id_relics,
        id_potions,
        gold,
        ..
    } = &state.reward;

    // Take actions for every Card in every bundle; Singing Bowl forfeits per bundle
    let singing_bowl = has_relic(&state.id_relics, RelicName::SingingBowl);
    for (idx_bundle, bundle) in id_cards.iter().enumerate() {
        for idx_card in 0..bundle.len() {
            state.legal_actions.push(Action::RewardTakeCard {
                idx_bundle,
                idx_card,
            });
        }
        if singing_bowl && !bundle.is_empty() {
            state
                .legal_actions
                .push(Action::RewardSingingBowl { idx_bundle });
        }
    }
    for idx in 0..id_relics.len() {
        state
            .legal_actions
            .push(Action::RewardTakeRelic { idx: idx });
    }

    // Sozu: Potion rewards can't be taken (mirrors the shop gate)
    if find_free_slot(&state.id_potions, state.potion_slots_max).is_some()
        && !has_relic(&state.id_relics, RelicName::Sozu)
    {
        for idx in 0..id_potions.len() {
            state
                .legal_actions
                .push(Action::RewardTakePotion { idx: idx });
        }
    }
    if gold.is_some() {
        state.legal_actions.push(Action::RewardTakeGold);
    }
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn fill_legal_actions_event(state: &mut GameState) {
    if state.event.consumed {
        state.legal_actions.push(Action::RoomExit);
    } else {
        let num_options = state.event.id_event_options.len();
        for idx in 0..num_options {
            if event_option_available(state, idx) {
                state
                    .legal_actions
                    .push(Action::EventOptionSelect { idx: idx });
            }
        }
    }
    push_potion_actions(state);
}

fn fill_legal_actions_shop(state: &mut GameState) {
    let Shop {
        cards,
        relics,
        potions,
        purge_cost,
        purged,
        ..
    } = &state.shop;

    // Exiting is always legal
    state.legal_actions.push(Action::RoomExit);

    // Snapshots
    let gold = state.entities[state.id_character].character_gold;
    let belt_has_room = find_free_slot(&state.id_potions, state.potion_slots_max).is_some();

    // Cards
    for (idx, &(_, price)) in cards.iter().enumerate() {
        if gold >= price {
            state.legal_actions.push(Action::ShopBuyCard { idx: idx });
        }
    }

    // Relics
    for (idx, &(_, price)) in relics.iter().enumerate() {
        if gold >= price {
            state.legal_actions.push(Action::ShopBuyRelic { idx: idx });
        }
    }

    // Potions (Sozu: unobtainable, so unbuyable)
    if belt_has_room && !has_relic(&state.id_relics, RelicName::Sozu) {
        for (idx, &(_, price)) in potions.iter().enumerate() {
            if gold >= price {
                state.legal_actions.push(Action::ShopBuyPotion { idx: idx });
            }
        }
    }

    // Purge: pay-then-pick, so one gated entry; the halt lists the candidates
    if !*purged
        && gold >= *purge_cost
        && state
            .id_card_deck
            .iter()
            .any(|&id| card_is_purgeable(&state.entities[id]))
    {
        state.legal_actions.push(Action::ShopPurge);
    }
    push_potion_actions(state);
}

fn fill_legal_actions_map(state: &mut GameState) {
    push_room_select_actions(state);
    push_potion_actions(state);
}

fn fill_legal_actions_rest_site(state: &mut GameState) {
    if state.rest_site.consumed {
        state.legal_actions.push(Action::RoomExit);
    } else {
        let mut any_option = false;

        // Coffee Dripper: Rest is unavailable
        if !has_relic(&state.id_relics, RelicName::CoffeeDripper) {
            state.legal_actions.push(Action::Rest);
            any_option = true;
        }

        // Fusion Hammer: Smith is unavailable
        if !has_relic(&state.id_relics, RelicName::FusionHammer)
            && state
                .id_card_deck
                .iter()
                .any(|&id| card_is_upgradable(&state.entities[id]))
        {
            state.legal_actions.push(Action::RestSmith);
            any_option = true;
        }

        // Girya: Lift for +1 combat-start Strength (max 3)
        if let Some(id) = state.id_relics[RelicName::Girya as usize]
            && state.entities[id].relic_counter < GIRYA_LIFT_MAX
        {
            state.legal_actions.push(Action::RestLift);
            any_option = true;
        }

        // Peace Pipe: Toke to purge a Card
        if has_relic(&state.id_relics, RelicName::PeacePipe)
            && state
                .id_card_deck
                .iter()
                .any(|&id| card_is_purgeable(&state.entities[id]))
        {
            state.legal_actions.push(Action::RestToke);
            any_option = true;
        }

        // Shovel: Dig for a random Relic
        if has_relic(&state.id_relics, RelicName::Shovel) {
            state.legal_actions.push(Action::RestDig);
            any_option = true;
        }

        // Every option gated: allow leaving so the site can't soft-lock
        if !any_option {
            state.legal_actions.push(Action::RoomExit);
        }
    }
    push_potion_actions(state);
}

// An opened chest cannot be opened again (N'loth's eaten chests rest opened)
fn fill_legal_actions_chest(state: &mut GameState) {
    if !state.chest.chest_opened {
        state.legal_actions.push(Action::ChestOpen);
    }
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn push_room_select_actions(state: &mut GameState) {
    match state.location {
        Location::Start => {
            for x in 0..MAP_WIDTH {
                if state.id_rooms[0][x].is_some() {
                    state.legal_actions.push(Action::RoomSelect { idx: x });
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

                // Wing Boots: with charges left, any next-row Room is reachable
                let winged = state.id_relics[RelicName::WingBoots as usize]
                    .is_some_and(|id| state.entities[id].relic_counter > 0);
                for x in 0..MAP_WIDTH {
                    if (winged || has_edge(edges, x)) && state.id_rooms[y_next][x].is_some() {
                        state.legal_actions.push(Action::RoomSelect { idx: x });
                    }
                }
            }
        }
        Location::BossRoom => {}
    }
}

fn push_potion_actions(state: &mut GameState) {
    let (in_combat, alive_count) = if context_focus(state) == Focus::Combat {
        (true, state.combat.id_monsters.iter().flatten().count())
    } else {
        (false, 0)
    };
    let slots_max = state.potion_slots_max as usize;
    for s in 0..slots_max {
        let Some(id_potion) = state.id_potions[s] else {
            continue;
        };
        let potion = &state.entities[id_potion];

        // Fairy in a Bottle is never drinkable; it procs from the death hook
        if potion.potion_name == PotionName::Fairy {
            state.legal_actions.push(Action::PotionDiscard { idx: s });
            continue;
        }

        // Smoke Bomb: can't escape from Boss fights
        if potion.potion_name == PotionName::SmokeBomb
            && in_combat
            && get_active_room_kind(&state.id_rooms, state.location, &state.entities)
                == Some(RoomKind::CombatBoss)
        {
            state.legal_actions.push(Action::PotionDiscard { idx: s });
            continue;
        }

        // Combat-only Potions
        if potion.potion_combat_only && !in_combat {
            state.legal_actions.push(Action::PotionDiscard { idx: s });
            continue;
        }

        // Target-requiring Potions
        if entity_requires_target(potion) {
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
