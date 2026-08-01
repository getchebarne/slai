use crate::consts::GIRYA_LIFT_MAX;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::get_card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::event_option_available;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::potions::find_free_slot;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::utils::card_filter_matches;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    CardDiscard {
        idx: usize,
    },
    CardDiscover {
        idx: usize,
    },
    CardExhaust {
        idx: usize,
    },
    CardDuplicate {
        idx: usize,
    },
    CardMoveToHand {
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
        idx: usize,
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
    RestToke,
    RewardSingingBowl,
    RewardTakeCard {
        idx: usize,
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
    ShopPurge {
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
        Action::CardDiscard { idx } => handle_pending_pick_hand(state, idx),
        Action::CardExhaust { idx } => handle_pending_pick_hand(state, idx),
        Action::CardMoveToHand { idx } => handle_card_move_to_hand_pick(state, idx),
        Action::PickSkip => handle_pick_skip(state),
        Action::CardDiscover { idx } => handle_card_discover(state, idx),
        Action::CardDuplicate { idx } => resolve_pending_pick_deck(state, idx),
        Action::CardNightmare { idx } => handle_pending_pick_hand(state, idx),
        Action::CardPlay {
            idx_card,
            idx_monster,
        } => handle_card_play(state, idx_card, idx_monster),
        Action::CardPurge { idx } => resolve_pending_pick_deck(state, idx),
        Action::CardRetain { idx } => handle_pending_pick_hand(state, idx),
        Action::CardSetup { idx } => handle_pending_pick_hand(state, idx),
        Action::CardTransform { idx } => resolve_pending_pick_deck(state, idx),
        Action::CardUpgrade { idx } => handle_card_upgrade(state, idx),
        Action::ChestOpen => handle_chest_open(state),
        Action::EventOptionSelect { idx } => handle_event_option_select(state, idx),
        Action::PotionDiscard { idx } => handle_potion_discard(state, idx),
        Action::PotionUse {
            idx_potion,
            idx_monster,
        } => handle_potion_use(state, idx_potion, idx_monster),
        Action::Rest => handle_rest(state),
        Action::RestDig => handle_rest_dig(state),
        Action::RestLift => handle_rest_lift(state),
        Action::RestToke => handle_rest_toke(state),
        Action::RewardSingingBowl => handle_reward_singing_bowl(state),
        Action::RewardTakeCard { idx } => handle_reward_take_card(state, idx),
        Action::RewardTakeGold => handle_reward_take_gold(state),
        Action::RewardTakePotion { idx } => handle_reward_take_potion(state, idx),
        Action::RewardTakeRelic { idx } => handle_reward_take_relic(state, idx),
        Action::RoomExit => handle_room_exit(state),
        Action::RoomSelect { idx } => handle_room_select(state, idx),
        Action::ShopBuyCard { idx } => handle_shop_buy_card(state, idx),
        Action::ShopBuyPotion { idx } => handle_shop_buy_potion(state, idx),
        Action::ShopBuyRelic { idx } => handle_shop_buy_relic(state, idx),
        Action::ShopPurge { idx } => handle_shop_purge(state, idx),
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

    // `state.effect_pending` takes precedence over `state.mode`
    if let Some(effect_pending) = state.effect_pending.as_ref() {
        // Copy out the halt's shape so the &mut dispatch below can't alias the borrow
        let effect_pending_kind = effect_pending.kind;
        let filter = pending_card_filter(effect_pending);
        let up_to = matches!(
            effect_pending.target,
            Target::Resolve {
                selection_kind: SelectionKind::InputUpTo { .. },
                ..
            }
        );
        fill_legal_actions_effect_pending(state, effect_pending_kind, filter);
        if up_to {
            state.legal_actions.push(Action::PickSkip);
        }
        return;
    }
    match state.mode {
        Mode::Combat { .. } => fill_legal_actions_screen_combat(state),
        Mode::CombatEnded => unreachable!("CombatEnded rests only at game_over"),
        Mode::Reward { .. } => fill_legal_actions_screen_reward(state),
        Mode::Event { .. } => fill_legal_actions_screen_event(state),
        Mode::Shop { .. } => fill_legal_actions_screen_shop(state),
        Mode::Map => fill_legal_actions_screen_map(state),
        Mode::RestSite => fill_legal_actions_screen_rest_site(state),
        Mode::Chest => fill_legal_actions_screen_chest(state),
        Mode::ChestOpened => fill_legal_actions_screen_chest_opened(state),
    }
}

// Discard / retain / setup / nightmare picks all resolve a pending hand pick
fn handle_pending_pick_hand(state: &mut GameState, idx: usize) {
    let Mode::Combat { id_hand, .. } = &state.mode else {
        unreachable!("Hand pick outside Combat mode")
    };
    let id_card = id_hand[idx];
    resolve_pending_pick(state, id_card);
}

// idx is an absolute id_pile_draw index
fn handle_card_move_to_hand_pick(state: &mut GameState, idx: usize) {
    let Mode::Combat { id_pile_draw, .. } = &state.mode else {
        unreachable!("draw-pile pick outside Combat mode")
    };
    let id_card = id_pile_draw[idx];
    resolve_pending_pick(state, id_card);
}

// Ends an InputUpTo halt early; remaining picks are forfeited
fn handle_pick_skip(state: &mut GameState) {
    state
        .effect_pending
        .take()
        .expect("PickSkip requires a pending effect");
}

fn handle_card_discover(state: &mut GameState, idx: usize) {
    let Mode::Combat { id_discover, .. } = &state.mode else {
        unreachable!("handle_card_discover outside Combat mode")
    };
    let id_card = id_discover[idx];
    resolve_pending_pick(state, id_card);
}

fn handle_card_play(state: &mut GameState, idx_card: usize, idx_monster: Option<usize>) {
    let Mode::Combat {
        id_hand,
        id_monsters,
        ..
    } = &state.mode
    else {
        unreachable!("handle_card_play outside Combat mode")
    };
    let id_card = id_hand[idx_card];
    if state.entities[id_card].requires_target {
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = id_monsters
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

fn handle_card_upgrade(state: &mut GameState, idx: usize) {
    // Dual-mode: a pending CardUpgrade resolves a deck pick; at a rest site it triggers a direct upgrade
    if state.effect_pending.is_some() {
        resolve_pending_pick_deck(state, idx);
        return;
    }
    let id_card = state.id_deck[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    push_rest_site_consume(state);
}

fn handle_chest_open(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::ChestOpen,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_event_option_select(state: &mut GameState, idx: usize) {
    let Mode::Event { id_options, .. } = &state.mode else {
        unreachable!("EventOptionSelect outside Event mode")
    };
    let id_option = id_options[idx];
    let effects = state.entities[id_option].event_option_effects;
    for effect in effects {
        state.effect_buf.push(Effect {
            id_source: Some(id_option),
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
    let id_potion = state.id_potions[idx_potion].expect("enumerated potion slot is occupied");
    if state.entities[id_potion].requires_target {
        let Mode::Combat { id_monsters, .. } = &state.mode else {
            unreachable!("Targeted potion use outside Combat mode")
        };
        let idx_monster =
            idx_monster.expect("Missing `idx_monster` when `requires_target` is true");
        let id_monster_target = id_monsters
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

// Marks the site used; every rest-site option ends with this
fn push_rest_site_consume(state: &mut GameState) {
    let id_room = current_room_id(state);
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSiteConsume,
        id_source: None,
        target: Target::Direct(Some(id_room)),
    });
}

// TODO: add `EffectKind::Rest`
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

    // Dream Catcher: resting also offers a card reward (Rest only, not Smith)
    if has_relic(&state.id_relics, RelicName::DreamCatcher) {
        state.effect_buf.push(Effect {
            kind: EffectKind::RewardRollCards,
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

// Peace Pipe: spend the rest on purging a card (halting deck pick)
fn handle_rest_toke(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Purgeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
    push_rest_site_consume(state);
}

// Shovel: spend the rest on a random relic (granted directly, not staged)
fn handle_rest_dig(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::RelicGrantRandom,
        id_source: None,
        target: Target::Direct(None),
    });
    push_rest_site_consume(state);
}

// Singing Bowl: forfeit the card reward for +2 max HP
fn handle_reward_singing_bowl(state: &mut GameState) {
    state.effect_buf.push(Effect {
        kind: EffectKind::SingingBowlProc,
        id_source: None,
        target: Target::Direct(None),
    });
}

fn handle_reward_take_card(state: &mut GameState, idx: usize) {
    let Mode::Reward {
        reward_id_cards, ..
    } = &state.mode
    else {
        unreachable!("RewardTakeCard outside Reward mode")
    };
    let id_card = reward_id_cards[idx];
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

fn handle_reward_take_potion(state: &mut GameState, idx: usize) {
    let Mode::Reward {
        reward_id_potions, ..
    } = &state.mode
    else {
        unreachable!("RewardTakePotion outside Reward mode")
    };
    let id_potion = reward_id_potions[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Potion,
        },
        id_source: None,
        target: Target::Direct(Some(id_potion)),
    });
}

fn handle_reward_take_relic(state: &mut GameState, idx: usize) {
    let Mode::Reward {
        reward_id_relics, ..
    } = &state.mode
    else {
        unreachable!("RewardTakeRelic outside Reward mode")
    };
    let id_relic = reward_id_relics[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::RewardTake {
            kind: RewardKind::Relic,
        },
        id_source: None,
        target: Target::Direct(Some(id_relic)),
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

fn handle_shop_buy_card(state: &mut GameState, idx: usize) {
    let Mode::Shop { shop_id_cards, .. } = &state.mode else {
        unreachable!("ShopBuyCard outside Shop mode")
    };
    let id_card = shop_id_cards[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopBuyCard,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}

fn handle_shop_buy_potion(state: &mut GameState, idx: usize) {
    let Mode::Shop {
        shop_id_potions, ..
    } = &state.mode
    else {
        unreachable!("ShopBuyPotion outside Shop mode")
    };
    let id_potion = shop_id_potions[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopBuyPotion,
        id_source: None,
        target: Target::Direct(Some(id_potion)),
    });
}

fn handle_shop_buy_relic(state: &mut GameState, idx: usize) {
    let Mode::Shop { shop_id_relics, .. } = &state.mode else {
        unreachable!("ShopBuyRelic outside Shop mode")
    };
    let id_relic = shop_id_relics[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopBuyRelic,
        id_source: None,
        target: Target::Direct(Some(id_relic)),
    });
}

fn handle_shop_purge(state: &mut GameState, idx: usize) {
    let id_card = state.id_deck[idx];
    state.effect_buf.push(Effect {
        kind: EffectKind::ShopPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}

fn fill_legal_actions_effect_pending(
    state: &mut GameState,
    kind: EffectKind,
    filter: Option<CandidatePoolCardFilter>,
) {
    match kind {
        // Discard/retain offer single-card picks; the handler re-raises the halt with a
        // decremented count, so discard-N becomes N single picks (see resolve_hand_pending)
        EffectKind::CardDiscard { .. } => {
            let Mode::Combat { id_hand, .. } = &state.mode else {
                unreachable!("Hand pick outside Combat mode")
            };
            for i in 0..id_hand.len() {
                state.legal_actions.push(Action::CardDiscard { idx: i });
            }
        }
        EffectKind::CardRetain => {
            let Mode::Combat { id_hand, .. } = &state.mode else {
                unreachable!("Hand pick outside Combat mode")
            };
            for i in 0..id_hand.len() {
                state.legal_actions.push(Action::CardRetain { idx: i });
            }
        }
        EffectKind::CardExhaust => {
            let Mode::Combat { id_hand, .. } = &state.mode else {
                unreachable!("Hand pick outside Combat mode")
            };
            for i in 0..id_hand.len() {
                state.legal_actions.push(Action::CardExhaust { idx: i });
            }
        }
        EffectKind::CardMove {
            pile: CardPile::Hand,
        } => {
            let Mode::Combat { id_pile_draw, .. } = &state.mode else {
                unreachable!("Draw-pile pick outside Combat mode")
            };
            let filter = filter.expect("draw-pile pick carries a card filter");
            for i in 0..id_pile_draw.len() {
                if card_filter_matches(filter, &state.entities[id_pile_draw[i]]) {
                    state.legal_actions.push(Action::CardMoveToHand { idx: i });
                }
            }
        }
        EffectKind::CardSetupPick { .. } => {
            let Mode::Combat { id_hand, .. } = &state.mode else {
                unreachable!("Hand pick outside Combat mode")
            };
            for i in 0..id_hand.len() {
                state.legal_actions.push(Action::CardSetup { idx: i });
            }
        }
        EffectKind::CardNightmarePick => {
            let Mode::Combat { id_hand, .. } = &state.mode else {
                unreachable!("Hand pick outside Combat mode")
            };
            for i in 0..id_hand.len() {
                state.legal_actions.push(Action::CardNightmare { idx: i });
            }
        }
        EffectKind::CardDiscoverPick { .. } => {
            let Mode::Combat { id_discover, .. } = &state.mode else {
                unreachable!("Discover pick outside Combat mode")
            };
            for i in 0..id_discover.len() {
                state.legal_actions.push(Action::CardDiscover { idx: i });
            }
        }
        // Bonfire's offer and bottling reuse `CardPurge` actions: same pool, same resolution shape
        EffectKind::CardPurge | EffectKind::BonfireOffer | EffectKind::CardBottle => {
            let filter = filter.expect("deck pick carries a card filter");
            for i in 0..state.id_deck.len() {
                if card_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardPurge { idx: i });
                }
            }
        }
        EffectKind::CardUpgrade => {
            let filter = filter.expect("deck pick carries a card filter");
            for i in 0..state.id_deck.len() {
                if card_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardUpgrade { idx: i });
                }
            }
        }
        EffectKind::CardDuplicate => {
            let filter = filter.expect("deck pick carries a card filter");
            for i in 0..state.id_deck.len() {
                if card_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardDuplicate { idx: i });
                }
            }
        }
        EffectKind::CardTransform { .. } => {
            let filter = filter.expect("deck pick carries a card filter");
            for i in 0..state.id_deck.len() {
                if card_filter_matches(filter, &state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardTransform { idx: i });
                }
            }
        }
        _ => unreachable!("effect_pending with non-halting kind: {:?}", kind),
    }
}

fn fill_legal_actions_screen_combat(state: &mut GameState) {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_monsters,
        energy,
        this_turn_discards,
        this_turn_cards_played,
        this_combat_damage_instances_taken,
        ..
    } = &state.mode
    else {
        unreachable!("Combat legality outside Combat mode")
    };
    let id_character = state.id_character;
    let entangled = has_modifier(
        &state.entities[id_character].modifiers,
        ModifierKind::Entangled,
    );
    let alive_count = id_monsters.iter().flatten().count();
    // Normality in hand caps the turn at 3 plays; blocks ANY further CardPlay
    let normality_blocks = *this_turn_cards_played >= 3
        && id_hand
            .iter()
            .any(|&id| state.entities[id].card_name == CardName::Normality);
    // Velvet Choker: no more than 6 cards per turn (increment is post-play, so exactly 6 land)
    let choker_blocks =
        *this_turn_cards_played >= 6 && has_relic(&state.id_relics, RelicName::VelvetChoker);
    for i in 0..id_hand.len() {
        if normality_blocks || choker_blocks {
            break;
        }
        let card = &state.entities[id_hand[i]];
        let restriction_ok = is_play_restriction_satisfied(
            card.card_play_restriction,
            card.card_kind,
            &id_pile_draw,
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
    let Mode::Reward {
        reward_id_cards,
        reward_id_relics,
        reward_id_potions,
        reward_gold,
        ..
    } = &state.mode
    else {
        unreachable!("Reward legality outside Reward mode")
    };
    for i in 0..reward_id_cards.len() {
        state.legal_actions.push(Action::RewardTakeCard { idx: i });
    }
    // Singing Bowl: the card reward can be forfeited for +2 max HP
    if !reward_id_cards.is_empty() && has_relic(&state.id_relics, RelicName::SingingBowl) {
        state.legal_actions.push(Action::RewardSingingBowl);
    }
    for i in 0..reward_id_relics.len() {
        state.legal_actions.push(Action::RewardTakeRelic { idx: i });
    }
    // Sozu: potion rewards can't be taken (mirrors the shop gate)
    if find_free_slot(&state.id_potions, state.potion_slots_max).is_some()
        && !has_relic(&state.id_relics, RelicName::Sozu)
    {
        for i in 0..reward_id_potions.len() {
            state
                .legal_actions
                .push(Action::RewardTakePotion { idx: i });
        }
    }
    if reward_gold.is_some() {
        state.legal_actions.push(Action::RewardTakeGold);
    }
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

fn fill_legal_actions_screen_event(state: &mut GameState) {
    let Mode::Event {
        kind,
        consumed,
        id_options,
        ..
    } = &state.mode
    else {
        unreachable!("Event legality outside Event mode")
    };
    if *consumed {
        state.legal_actions.push(Action::RoomExit);
    } else {
        let kind = *kind;
        let num_options = id_options.len();
        for i in 0..num_options {
            if event_option_available(state, kind, i) {
                state
                    .legal_actions
                    .push(Action::EventOptionSelect { idx: i });
            }
        }
    }
    push_potion_actions(state);
}

fn fill_legal_actions_screen_shop(state: &mut GameState) {
    let Mode::Shop {
        shop_id_cards,
        shop_id_relics,
        shop_id_potions,
        shop_purge_cost,
    } = &state.mode
    else {
        unreachable!("Shop legality outside Shop mode")
    };
    state.legal_actions.push(Action::RoomExit);
    let gold = state.entities[state.id_character].character_gold;
    let belt_has_room = find_free_slot(&state.id_potions, state.potion_slots_max).is_some();

    // Cards
    for i in 0..shop_id_cards.len() {
        if gold >= state.entities[shop_id_cards[i]].price {
            state.legal_actions.push(Action::ShopBuyCard { idx: i });
        }
    }

    // Relics
    for i in 0..shop_id_relics.len() {
        if gold >= state.entities[shop_id_relics[i]].price {
            state.legal_actions.push(Action::ShopBuyRelic { idx: i });
        }
    }

    // Potions (Sozu: unobtainable, so unbuyable)
    if belt_has_room && !has_relic(&state.id_relics, RelicName::Sozu) {
        for i in 0..shop_id_potions.len() {
            if gold >= state.entities[shop_id_potions[i]].price {
                state.legal_actions.push(Action::ShopBuyPotion { idx: i });
            }
        }
    }

    // Purge
    if !state.entities[current_room_id(state)].room_shop_purged && gold >= *shop_purge_cost {
        for i in 0..state.id_deck.len() {
            if card_is_purgeable(&state.entities[state.id_deck[i]]) {
                state.legal_actions.push(Action::ShopPurge { idx: i });
            }
        }
    }
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
        let mut any_option = false;

        // Coffee Dripper: Rest is unavailable
        if !has_relic(&state.id_relics, RelicName::CoffeeDripper) {
            state.legal_actions.push(Action::Rest);
            any_option = true;
        }

        // Fusion Hammer: Smith is unavailable
        if !has_relic(&state.id_relics, RelicName::FusionHammer) {
            // CardUpgrade idx is an absolute id_deck index; offer only upgradable cards
            for i in 0..state.id_deck.len() {
                if card_is_upgradable(&state.entities[state.id_deck[i]]) {
                    state.legal_actions.push(Action::CardUpgrade { idx: i });
                    any_option = true;
                }
            }
        }

        // Girya: Lift for +1 combat-start Strength (max 3)
        if let Some(id) = state.id_relics[RelicName::Girya as usize]
            && state.entities[id].relic_counter < GIRYA_LIFT_MAX
        {
            state.legal_actions.push(Action::RestLift);
            any_option = true;
        }

        // Peace Pipe: Toke to purge a card
        if has_relic(&state.id_relics, RelicName::PeacePipe)
            && state
                .id_deck
                .iter()
                .any(|&id| card_is_purgeable(&state.entities[id]))
        {
            state.legal_actions.push(Action::RestToke);
            any_option = true;
        }

        // Shovel: Dig for a random relic
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

fn fill_legal_actions_screen_chest(state: &mut GameState) {
    state.legal_actions.push(Action::ChestOpen);
    state.legal_actions.push(Action::RoomExit);
    push_potion_actions(state);
}

// An opened chest cannot be opened again (N'loth's eaten chests rest here)
fn fill_legal_actions_screen_chest_opened(state: &mut GameState) {
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
                // Wing Boots: with charges left, any next-row room is reachable
                let winged = state.id_relics[RelicName::WingBoots as usize]
                    .is_some_and(|id| state.entities[id].relic_counter > 0);
                for c in 0..MAP_WIDTH {
                    if (winged || has_edge(edges, c)) && state.id_rooms[y_next][c].is_some() {
                        state.legal_actions.push(Action::RoomSelect { idx: c });
                    }
                }
            }
        }
        Location::BossRoom => {}
    }
}

fn push_potion_actions(state: &mut GameState) {
    let (in_combat, alive_count) = match &state.mode {
        Mode::Combat { id_monsters, .. } => (true, id_monsters.iter().flatten().count()),
        _ => (false, 0),
    };
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

// Pops effect_pending, applies the picked entity as a Direct effect, and re-raises the
// halt with the remaining count against the pending's own pool
fn resolve_pending_pick(state: &mut GameState, id_picked: usize) {
    let effect_pending = state.effect_pending.take().unwrap();

    state.effect_buf.push(Effect {
        kind: effect_pending.kind,
        id_source: effect_pending.id_source,
        target: Target::Direct(Some(id_picked)),
    });

    // Re-raise the remaining count; the pick flushes ahead so the pool shrinks first
    let (candidate_pool, selection_kind) = match effect_pending.target {
        Target::Resolve {
            candidate_pool,
            selection_kind: SelectionKind::Input { count },
        } => (
            candidate_pool,
            SelectionKind::Input {
                count: count.saturating_sub(1),
            },
        ),
        Target::Resolve {
            candidate_pool,
            selection_kind: SelectionKind::InputUpTo { count },
        } => (
            candidate_pool,
            SelectionKind::InputUpTo {
                count: count.saturating_sub(1),
            },
        ),
        _ => panic!("pending pick carries an Input halt"),
    };
    let remaining = match selection_kind {
        SelectionKind::Input { count } | SelectionKind::InputUpTo { count } => count,
        _ => unreachable!(),
    };
    if remaining > 0 {
        state.effect_buf.push(Effect {
            kind: effect_pending.kind,
            id_source: effect_pending.id_source,
            target: Target::Resolve {
                candidate_pool,
                selection_kind,
            },
        });
    }
}

// Extract the card filter from a pending deck / draw-pile pick; None for other halts
fn pending_card_filter(effect: &Effect) -> Option<CandidatePoolCardFilter> {
    match effect.target {
        Target::Resolve {
            candidate_pool: CandidatePool::Deck { filter } | CandidatePool::PileDraw { filter },
            ..
        } => Some(filter),
        _ => None,
    }
}

// Resolves a pending deck pick; idx is an absolute id_deck index
fn resolve_pending_pick_deck(state: &mut GameState, idx: usize) {
    let id_card = state.id_deck[idx];
    resolve_pending_pick(state, id_card);
}
