use crate::consts::GIRYA_LIFT_MAX;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::CardPlay;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::fill_buf_candidates;
use crate::events::event_option_available;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::map::has_edge;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::potions::belt_has_room;
use crate::types::CardKind;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::Focus;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::types::Reward;
use crate::types::RewardKind;
use crate::types::RoomKind;
use crate::types::Shop;
use crate::types::ShopSlot;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::context_focus;
use crate::utils::entity_requires_target;
use crate::utils::filter_candidates;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::get_card_effective_cost;
use crate::utils::has_relic;
use crate::utils::is_play_restriction_satisfied;
use crate::utils::play_cap_reached;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    CardPlay {
        id_card: usize,
        id_monster: Option<usize>,
    },
    ChestOpen,
    EventOptionSelect {
        id_option: usize,
    },
    PickSkip,
    PotionDiscard {
        id_potion: usize,
    },
    PotionUse {
        id_potion: usize,
        id_monster: Option<usize>,
    },
    Rest,
    RestDig,
    RestLift,
    RestSmith,
    RestToke,
    RewardSingingBowl {
        id_card: usize,
    },
    RewardTakeCard {
        id_card: usize,
    },
    RewardTakeGold,
    RewardTakeGoldStolen,
    RewardTakePotion {
        id_potion: usize,
    },
    RewardTakeRelic {
        id_relic: usize,
    },
    RoomExit,
    RoomSelect {
        id_room: usize,
    },
    ShopBuyCard {
        id_card: usize,
    },
    ShopBuyPotion {
        id_potion: usize,
    },
    ShopBuyRelic {
        id_relic: usize,
    },
    ShopPurge,
    TurnEnd,

    // A pending halt's selection: one variant per operation, naming the selected entity
    BonfireOffer {
        id_card: usize,
    },
    CardAdopt {
        id_card: usize,
    },
    CardBottle {
        id_card: usize,
    },
    CardDiscard {
        id_card: usize,
    },
    CardDiscoverPick {
        id_card: usize,
    },
    CardDuplicate {
        id_card: usize,
    },
    CardExhaust {
        id_card: usize,
    },
    CardMove {
        id_card: usize,
    },
    CardNightmarePick {
        id_card: usize,
    },
    CardPurge {
        id_card: usize,
    },
    CardRetain {
        id_card: usize,
    },
    CardSetupPick {
        id_card: usize,
    },
    CardTransform {
        id_card: usize,
    },
    CardUpgrade {
        id_card: usize,
    },
    MatchFlipSeen {
        id_card: usize,
    },
    RelicLose {
        id_relic: usize,
    },
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
            id_card,
            id_monster,
        } => handle_card_play(state, id_card, id_monster),
        Action::ChestOpen => handle_chest_open(state),
        Action::EventOptionSelect { id_option } => handle_event_option_select(state, id_option),
        Action::PickSkip => handle_pick_skip(state),
        Action::PotionDiscard { id_potion } => handle_potion_discard(state, id_potion),
        Action::PotionUse {
            id_potion,
            id_monster,
        } => handle_potion_use(state, id_potion, id_monster),
        Action::Rest => handle_rest(state),
        Action::RestDig => handle_rest_dig(state),
        Action::RestLift => handle_rest_lift(state),
        Action::RestSmith => handle_rest_smith(state),
        Action::RestToke => handle_rest_toke(state),
        Action::RewardSingingBowl { id_card } => handle_reward_singing_bowl(state, id_card),
        Action::RewardTakeCard { id_card } => {
            handle_reward_take(state, RewardKind::Card, Some(id_card))
        }
        Action::RewardTakeGold => handle_reward_take(state, RewardKind::Gold, None),
        Action::RewardTakeGoldStolen => handle_reward_take(state, RewardKind::GoldStolen, None),
        Action::RewardTakePotion { id_potion } => {
            handle_reward_take(state, RewardKind::Potion, Some(id_potion))
        }
        Action::RewardTakeRelic { id_relic } => {
            handle_reward_take(state, RewardKind::Relic, Some(id_relic))
        }
        Action::RoomExit => handle_room_exit(state),
        Action::RoomSelect { id_room } => handle_room_select(state, id_room),
        Action::ShopBuyCard { id_card } => handle_shop_buy(state, ShopSlot::Card, id_card),
        Action::ShopBuyPotion { id_potion } => handle_shop_buy(state, ShopSlot::Potion, id_potion),
        Action::ShopBuyRelic { id_relic } => handle_shop_buy(state, ShopSlot::Relic, id_relic),
        Action::ShopPurge => handle_shop_purge(state),
        Action::TurnEnd => handle_turn_end(state),
        Action::BonfireOffer { id_card: id }
        | Action::CardAdopt { id_card: id }
        | Action::CardBottle { id_card: id }
        | Action::CardDiscard { id_card: id }
        | Action::CardDiscoverPick { id_card: id }
        | Action::CardDuplicate { id_card: id }
        | Action::CardExhaust { id_card: id }
        | Action::CardMove { id_card: id }
        | Action::CardNightmarePick { id_card: id }
        | Action::CardPurge { id_card: id }
        | Action::CardRetain { id_card: id }
        | Action::CardSetupPick { id_card: id }
        | Action::CardTransform { id_card: id }
        | Action::CardUpgrade { id_card: id }
        | Action::MatchFlipSeen { id_card: id }
        | Action::RelicLose { id_relic: id } => handle_selection(state, id),
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
            filters,
            selection_kind,
        } = effect_pending.target
        else {
            unreachable!("effect_pending carries a Resolve target")
        };
        fill_legal_actions_effect_pending(state, effect_pending.kind, filters, candidate_pool);
        if matches!(selection_kind, SelectionKind::InputUpTo { .. }) {
            state.legal_actions.push(Action::PickSkip);
        }

        // A pick leaves the belt usable: any discard, and a drink that needs no target
        push_potion_actions(state);
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

// The operation of each halting kind; Smith's and Toke's selections share Upgrade's and Purge's
fn selection_action(kind: EffectKind, id: usize) -> Action {
    match kind {
        EffectKind::BonfireOffer => Action::BonfireOffer { id_card: id },
        EffectKind::CardAdopt { .. } => Action::CardAdopt { id_card: id },
        EffectKind::CardBottle => Action::CardBottle { id_card: id },
        EffectKind::CardDiscard { .. } => Action::CardDiscard { id_card: id },
        EffectKind::CardDiscoverPick { .. } => Action::CardDiscoverPick { id_card: id },
        EffectKind::CardDuplicate { .. } => Action::CardDuplicate { id_card: id },
        EffectKind::CardExhaust => Action::CardExhaust { id_card: id },
        EffectKind::CardMove { .. } => Action::CardMove { id_card: id },
        EffectKind::CardNightmarePick => Action::CardNightmarePick { id_card: id },
        EffectKind::CardPurge | EffectKind::RestToke => Action::CardPurge { id_card: id },
        EffectKind::CardRetain => Action::CardRetain { id_card: id },
        EffectKind::CardSetupPick { .. } => Action::CardSetupPick { id_card: id },
        EffectKind::CardTransform { .. } => Action::CardTransform { id_card: id },
        EffectKind::CardUpgrade | EffectKind::RestSmith => Action::CardUpgrade { id_card: id },
        EffectKind::MatchFlipSeen => Action::MatchFlipSeen { id_card: id },
        EffectKind::RelicLose => Action::RelicLose { id_relic: id },
        other => unreachable!("effect kind cannot halt: {:?}", other),
    }
}

// Stages the selected id; the last selection flushes every staged id
fn handle_selection(state: &mut GameState, id_selected: usize) {
    // Validate there's a pending `Effect` and, if so, take it
    let effect_pending = state
        .effect_pending
        .take()
        .expect("A selection requires a pending effect");

    // Validate the pending `Effect`'s target is `Target::Resolve`
    let Target::Resolve {
        candidate_pool,
        filters,
        selection_kind,
    } = effect_pending.target
    else {
        unreachable!("effect_pending carries a Resolve target")
    };

    // Stage the selected `Entity` ID; nothing applies until the selection closes
    state.id_card_pile_selected.push(id_selected);

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
            filters,
            selection_kind,
        },
    });
}

// One Direct effect per staged pick, in pick order
fn flush_pending_picks(state: &mut GameState, kind: EffectKind, id_source: Option<usize>) {
    for &id in &state.id_card_pile_selected {
        state.effect_buf.push(Effect {
            kind,
            id_source,
            target: Target::Direct(Some(id)),
        });
    }
    state.id_card_pile_selected.clear();
}

// Ends an InputUpTo halt early; staged picks apply, the remainder is forfeited
fn handle_pick_skip(state: &mut GameState) {
    let effect_pending = state
        .effect_pending
        .take()
        .expect("PickSkip requires a pending effect");

    // A skipped discover pick still runs, with no Card, so its rolled Cards are cleared
    if matches!(effect_pending.kind, EffectKind::CardDiscoverPick { .. }) {
        state.effect_buf.push(Effect {
            target: Target::Direct(None),
            ..effect_pending
        });
    }
    flush_pending_picks(state, effect_pending.kind, effect_pending.id_source);
}

fn handle_card_play(state: &mut GameState, id_card: usize, id_monster: Option<usize>) {
    assert!(state.combat.active, "handle_card_play outside combat");
    let energy = state.combat.energy.energy_current;

    // The play carries its own target: the picked Monster, if the Card needs one
    state.card_play_queue.push_back(CardPlay {
        id_card,
        id_target: id_monster,
        play_source: PlaySource::Hand,
        energy,
        cost_effective: get_card_effective_cost(&state.entities[id_card], energy),
        draw_pile_size: state.combat.id_card_pile_draw.len() as u16,
    });
}

fn handle_chest_open(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_event_option_select(state: &mut GameState, id_option: usize) {
    assert!(
        context_focus(state) == Focus::Event,
        "EventOptionSelect outside the Event context"
    );
    let effects = state.entities[id_option].event_option_effects;
    let effects_len = state.entities[id_option].event_option_effects_len as usize;
    for effect in &effects[..effects_len] {
        state.effect_buf.push(Effect {
            id_source: Some(id_option),
            ..*effect
        });
    }
}

fn handle_potion_discard(state: &mut GameState, id_potion: usize) {
    state.effect_buf.push(Effect {
        kind: EffectKind::PotionDiscard,
        id_source: Some(id_potion),
        target: Target::Direct(Some(id_potion)),
    });

    // At a pick, the pick halts again behind the Potion
    if let Some(effect_pending) = state.effect_pending.take() {
        state.effect_buf.push(effect_pending);
    }
}

fn handle_potion_use(state: &mut GameState, id_potion: usize, id_monster: Option<usize>) {
    // The use carries its own target: the picked Monster, if the Potion needs one
    state.effect_buf.push(Effect {
        kind: EffectKind::PotionUse {
            at_pick: state.effect_pending.is_some(),
        },
        id_source: Some(id_potion),
        target: Target::Direct(id_monster),
    });

    // At a pick, the pick halts again behind the Potion
    if let Some(effect_pending) = state.effect_pending.take() {
        state.effect_buf.push(effect_pending);
    }
}

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
                rounding: Rounding::Truncate,
                read_at: ReadAt::Now,
            },
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    push_rest_site_consume(state);

    // Rest Relic effects, in pickup order, after the consume
    let mut id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();

    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);
    for id_relic in id_relics {
        for &effect in state.entities[id_relic].relic_effects_on_rest {
            state.effect_buf.push(effect);
        }
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

// Peace Pipe: spend the rest on purging a Card; skipping the deck pick cancels back to the site
fn handle_rest_toke(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RestToke,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDeck,
            filters: &[CandidateFilter::NotBottled, CandidateFilter::NotBoundCurse],
            selection_kind: SelectionKind::InputUpTo { count: 1 },
        },
    });
}

// Smith mirrors Toke: the picked Card is upgraded, and a skip cancels back to the site
fn handle_rest_smith(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSmith,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDeck,
            filters: &[CandidateFilter::Upgradeable],
            selection_kind: SelectionKind::InputUpTo { count: 1 },
        },
    });
}

// Shovel: spend the rest on a random Relic, staged as a reward the player may leave
fn handle_rest_dig(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RestDig,
        id_source: None,
        target: Target::Direct(None),
    });
}

// Singing Bowl: forfeit one Card bundle for +2 max HP
fn handle_reward_singing_bowl(state: &mut GameState, id_card: usize) {
    state.effect_buf.push(Effect {
        kind: EffectKind::SingingBowlProc,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}

// One processor handles every kind
fn handle_reward_take(state: &mut GameState, kind: RewardKind, id_taken: Option<usize>) {
    assert!(state.reward.active, "RewardTake outside the Reward context");
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

fn handle_room_select(state: &mut GameState, id_room: usize) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Direct(Some(id_room)),
    });
}

fn handle_turn_end(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::TurnEndCharacter { landing: false },
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_shop_buy(state: &mut GameState, slot: ShopSlot, id_bought: usize) {
    assert!(
        context_focus(state) == Focus::Shop,
        "ShopBuy outside the Shop context"
    );
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

// Every filtered, unstaged candidate of the pending pool, as the pending kind's operation
fn fill_legal_actions_effect_pending(
    state: &mut GameState,
    kind: EffectKind,
    filters: &[CandidateFilter],
    pool: CandidatePool,
) {
    // Get `CandidatePool`'s instanced IDs
    let mut survivors = Vec::new();
    fill_buf_candidates(
        &mut survivors,
        pool,
        None,
        state.id_character,
        &state.combat,
        &state.event,
        &state.id_card_pile_deck,
    );

    // Apply the filters over the whole set; staged picks are out of the running
    filter_candidates(filters, &mut survivors, &state.entities, None);
    for id in survivors {
        if !state.id_card_pile_selected.contains(&id) {
            state.legal_actions.push(selection_action(kind, id));
        }
    }
}

fn fill_legal_actions_combat(state: &mut GameState) {
    let Combat {
        id_card_pile_hand,
        id_card_pile_draw,
        id_monsters,
        energy,
        this_turn_cards_played,
        ..
    } = &state.combat;
    let id_character = state.id_character;

    // Entangled: can't play `CardKind::Attack` Cards
    let entangled = has_modifier(
        &state.entities[id_character].modifiers,
        ModifierKind::Entangled,
    );

    let cap_reached = play_cap_reached(
        id_card_pile_hand,
        &state.entities,
        &state.id_relics,
        *this_turn_cards_played,
    );
    for &id_card in id_card_pile_hand {
        if cap_reached {
            break;
        }
        let card = &state.entities[id_card];
        let restriction_ok = is_play_restriction_satisfied(
            card.card_play_restriction,
            card.card_kind,
            &id_card_pile_draw,
            &state.entities,
            &state.id_relics,
        );
        let entangled_blocks = entangled && card.card_kind == CardKind::Attack;
        if !restriction_ok || entangled_blocks {
            continue;
        }
        let cost = get_card_effective_cost(card, energy.energy_current);
        if cost > energy.energy_current {
            continue;
        }
        let requires_target = entity_requires_target(card);
        if requires_target {
            for &id_monster in id_monsters.iter().flatten() {
                state.legal_actions.push(Action::CardPlay {
                    id_card,
                    id_monster: Some(id_monster),
                });
            }
        } else {
            state.legal_actions.push(Action::CardPlay {
                id_card,
                id_monster: None,
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
        gold_stolen,
        ..
    } = &state.reward;

    // Take actions for every Card in every bundle; Singing Bowl forfeits per bundle
    let singing_bowl = has_relic(&state.id_relics, RelicName::SingingBowl);
    for bundle in id_cards {
        for &id_card in bundle {
            state.legal_actions.push(Action::RewardTakeCard { id_card });
        }
        if singing_bowl && !bundle.is_empty() && !state.reward.cards_forced {
            state
                .legal_actions
                .push(Action::RewardSingingBowl { id_card: bundle[0] });
        }
    }
    for &id_relic in id_relics {
        state
            .legal_actions
            .push(Action::RewardTakeRelic { id_relic });
    }

    // Sozu: taking a Potion reward removes it and obtains nothing, so a full belt doesn't block it
    if belt_has_room(&state.id_potions, state.potion_slots_max)
        || has_relic(&state.id_relics, RelicName::Sozu)
    {
        for &id_potion in id_potions {
            state
                .legal_actions
                .push(Action::RewardTakePotion { id_potion });
        }
    }
    if gold.is_some() {
        state.legal_actions.push(Action::RewardTakeGold);
    }
    if gold_stolen.is_some() {
        state.legal_actions.push(Action::RewardTakeGoldStolen);
    }

    // The Library's grid has no cancel button
    if !state.reward.cards_forced {
        state.legal_actions.push(Action::RoomExit);
    }
    push_potion_actions(state);
}

fn fill_legal_actions_event(state: &mut GameState) {
    if state.event.consumed {
        state.legal_actions.push(Action::RoomExit);
    } else {
        for (idx, &id_option) in state.event.id_event_options.iter().enumerate() {
            if event_option_available(state, idx) {
                state
                    .legal_actions
                    .push(Action::EventOptionSelect { id_option });
            }
        }
    }
    push_potion_actions(state);
}

fn fill_legal_actions_shop(state: &mut GameState) {
    let Shop {
        id_cards_price,
        id_relics_price,
        id_potions_price,
        purge_cost,
        purged,
        ..
    } = &state.shop;

    // Exiting is always legal
    state.legal_actions.push(Action::RoomExit);

    // Snapshots
    let gold = state.entities[state.id_character].character_gold;
    let belt_room = belt_has_room(&state.id_potions, state.potion_slots_max);

    // Cards
    for &(id_card, price) in id_cards_price {
        if gold >= price {
            state.legal_actions.push(Action::ShopBuyCard { id_card });
        }
    }

    // Relics
    for &(id_relic, price) in id_relics_price {
        if gold >= price {
            state.legal_actions.push(Action::ShopBuyRelic { id_relic });
        }
    }

    // Potions (Sozu: unobtainable, so unbuyable)
    if belt_room && !has_relic(&state.id_relics, RelicName::Sozu) {
        for &(id_potion, price) in id_potions_price {
            if gold >= price {
                state
                    .legal_actions
                    .push(Action::ShopBuyPotion { id_potion });
            }
        }
    }

    // Purge: pay-then-pick, so one gated entry; the halt lists the candidates
    if !*purged
        && gold >= *purge_cost
        && state
            .id_card_pile_deck
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
                .id_card_pile_deck
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
                .id_card_pile_deck
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

// An opened chest cannot be opened again (the boss chest after its pick)
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
            for &id_room in state.id_rooms[0].iter().flatten() {
                state.legal_actions.push(Action::RoomSelect { id_room });
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
                    if (winged || has_edge(edges, x))
                        && let Some(id_room) = state.id_rooms[y_next][x]
                    {
                        state.legal_actions.push(Action::RoomSelect { id_room });
                    }
                }
            }
        }
        Location::BossRoom | Location::BossTreasure => {}
    }
}

fn push_potion_actions(state: &mut GameState) {
    let (in_combat, alive_count) = if context_focus(state) == Focus::Combat {
        (true, state.combat.id_monsters.iter().flatten().count())
    } else {
        (false, 0)
    };
    // A pick takes no aimed drink
    let at_pick = state.effect_pending.is_some();
    let turn_ended = in_combat && state.combat.turn_ended;

    // A Room's event holds until the next Room is entered, even after its screen closes
    let in_we_meet_again = state.room_kind_resolved == Some(RoomKind::EventRoom)
        && state.event.name == EventName::WeMeetAgain;
    for &id_potion in &state.id_potions {
        let potion = &state.entities[id_potion];

        // The We Meet Again Room locks every Potion
        if in_we_meet_again {
            continue;
        }

        // Once the turn has ended, a Potion can only be discarded
        if turn_ended {
            state
                .legal_actions
                .push(Action::PotionDiscard { id_potion });
            continue;
        }

        // Fairy in a Bottle is never drinkable; it procs from the death hook
        if potion.potion_name == PotionName::Fairy {
            state
                .legal_actions
                .push(Action::PotionDiscard { id_potion });
            continue;
        }

        // Smoke Bomb: can't escape from Boss fights
        if potion.potion_name == PotionName::SmokeBomb
            && in_combat
            && get_active_room_kind(&state.id_rooms, state.location, &state.entities)
                == Some(RoomKind::CombatBoss)
        {
            state
                .legal_actions
                .push(Action::PotionDiscard { id_potion });
            continue;
        }

        // Combat-only Potions need a combat with a Monster left in it
        if potion.potion_combat_only && (!in_combat || alive_count == 0) {
            state
                .legal_actions
                .push(Action::PotionDiscard { id_potion });
            continue;
        }

        // Target-requiring Potions
        if entity_requires_target(potion) {
            if in_combat && !at_pick {
                for &id_monster in state.combat.id_monsters.iter().flatten() {
                    state.legal_actions.push(Action::PotionUse {
                        id_potion,
                        id_monster: Some(id_monster),
                    });
                }
            }
        } else {
            state.legal_actions.push(Action::PotionUse {
                id_potion,
                id_monster: None,
            });
        }
        state
            .legal_actions
            .push(Action::PotionDiscard { id_potion });
    }
}
