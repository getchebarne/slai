use rand::Rng;

use crate::consts::MAX_SIZE_DECK;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::grant_relic;

pub fn process_effect_relic_grant_specific(
    state: &mut GameState,
    name: RelicName,
    fallback_circlet: bool,
) {
    let owns_target = state.id_relics[name as usize].is_some();
    let target = match (owns_target, fallback_circlet) {
        (false, _) => name,
        (true, true) => RelicName::Circlet,
        (true, false) => return,
    };
    grant_relic(target, &mut state.id_relics, &mut state.entities);
    // Keyed on `target` so a Circlet fallback never procs the requested relic
    queue_pickup_effects(state, target);
}

// On-pickup effects; every acquisition path funnels through this handler
fn queue_pickup_effects(state: &mut GameState, name: RelicName) {
    let id_character = state.id_character;
    match name {
        RelicName::DollysMirror => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDuplicate,
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Deck {
                        filter: CandidatePoolDeckFilter::Any,
                    },
                    selection_kind: SelectionKind::Input { count: 1 },
                },
            });
        }
        RelicName::LeesWaffle => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: HealthDeltaAmount::Relative {
                        numerator: 1,
                        denominator: 1,
                    },
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::MaxHealthDelta {
                    sign: DeltaSign::Gain,
                    amount: HealthDeltaAmount::Absolute(7),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        RelicName::Strawberry => stat_pickup(state, id_character, 7),
        RelicName::Pear => stat_pickup(state, id_character, 10),
        RelicName::Mango => stat_pickup(state, id_character, 14),
        RelicName::OldCoin => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Gain,
                    kind: GoldDeltaKind::Fixed(300),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }
        RelicName::WarPaint => upgrade_random_of_kind(state, CardKind::Skill),
        RelicName::Whetstone => upgrade_random_of_kind(state, CardKind::Attack),
        RelicName::EmptyCage => {
            // Two sequential halting picks; auto-resolve covers small decks
            for _ in 0..2 {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardPurge,
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Deck {
                            filter: CandidatePoolDeckFilter::Purgeable,
                        },
                        selection_kind: SelectionKind::Input { count: 1 },
                    },
                });
            }
        }
        RelicName::PandorasBox => {
            // Every starter Strike/Defend becomes a random card (no player choice)
            let mut starters = [0usize; MAX_SIZE_DECK];
            let mut n = 0;
            for &id in &state.id_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    starters[n] = id;
                    n += 1;
                }
            }
            for &id in &starters[..n] {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardTransform,
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
        _ => {}
    }
}

// Max HP first so the heal lands under the new ceiling
fn stat_pickup(state: &mut GameState, id_character: usize, amount: u16) {
    state.effect_queue.push_front(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}

// War Paint / Whetstone: up to 2 random unupgraded deck cards of `kind`
fn upgrade_random_of_kind(state: &mut GameState, kind: CardKind) {
    let mut ids = [0usize; MAX_SIZE_DECK];
    let mut n = 0;
    for &id in &state.id_deck {
        let card = &state.entities[id];
        if card.card_kind == kind && !card.card_upgraded {
            ids[n] = id;
            n += 1;
        }
    }
    for _ in 0..2.min(n) {
        let idx = state.rng.random_range(0..n);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(ids[idx])),
        });
        ids[idx] = ids[n - 1];
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::action::handle_action;
    use crate::action::recompute_legal_actions;
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::GameState;
    use crate::game::create_game_state;
    use crate::types::CardKind;
    use crate::types::CardName;
    use crate::types::RelicName;
    use crate::types::RewardKind;
    use crate::utils::push_entity;

    fn grant(state: &mut GameState, name: RelicName) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RelicGrantSpecific {
                name,
                fallback_circlet: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(state);
    }

    #[test]
    fn lees_waffle_grants_max_hp_and_heals_to_full() {
        let mut state = create_game_state(0, 42, false);
        let id_character = state.id_character;
        let max_before = state.entities[id_character].vitals.health_max;
        state.entities[id_character].vitals.health = 40;
        grant(&mut state, RelicName::LeesWaffle);
        let vitals = &state.entities[id_character].vitals;
        assert_eq!(vitals.health_max, max_before + 7);
        assert_eq!(vitals.health, vitals.health_max);
    }

    #[test]
    fn dollys_mirror_duplicates_the_picked_deck_card() {
        let mut state = create_game_state(0, 42, false);
        let deck_before = state.id_deck.len();
        grant(&mut state, RelicName::DollysMirror);
        // Grant halts on the deck pick
        assert!(state.effect_pending.is_some());
        let picked_name = state.entities[state.id_deck[0]].card_name;
        recompute_legal_actions(&mut state);
        handle_action(&mut state, Action::CardDuplicate { idx: 0 }).unwrap();
        process_effect_queue(&mut state);
        assert_eq!(state.id_deck.len(), deck_before + 1);
        let added = *state.id_deck.last().unwrap();
        assert_eq!(state.entities[added].card_name, picked_name);
    }

    #[test]
    fn stat_pickups_raise_max_hp_and_heal() {
        for (relic, amount) in [
            (RelicName::Strawberry, 7),
            (RelicName::Pear, 10),
            (RelicName::Mango, 14),
        ] {
            let mut state = create_game_state(0, 42, false);
            let id_character = state.id_character;
            state.entities[id_character].vitals.health = 40;
            let max_before = state.entities[id_character].vitals.health_max;
            grant(&mut state, relic);
            let vitals = &state.entities[id_character].vitals;
            assert_eq!(vitals.health_max, max_before + amount);
            assert_eq!(vitals.health, 40 + amount);
        }
    }

    #[test]
    fn old_coin_grants_gold() {
        let mut state = create_game_state(0, 42, false);
        let gold_before = state.entities[state.id_character].character_gold;
        grant(&mut state, RelicName::OldCoin);
        assert_eq!(
            state.entities[state.id_character].character_gold,
            gold_before + 300
        );
    }

    #[test]
    fn potion_belt_adds_two_slots() {
        let mut state = create_game_state(0, 42, false);
        assert_eq!(state.potion_slots_max, 3);
        grant(&mut state, RelicName::PotionBelt);
        assert_eq!(state.potion_slots_max, 5);
    }

    #[test]
    fn war_paint_and_whetstone_upgrade_two_of_their_kind() {
        for (relic, kind) in [
            (RelicName::WarPaint, CardKind::Skill),
            (RelicName::Whetstone, CardKind::Attack),
        ] {
            let mut state = create_game_state(0, 42, false);
            grant(&mut state, relic);
            let upgraded: Vec<usize> = state
                .id_deck
                .iter()
                .copied()
                .filter(|&id| state.entities[id].card_upgraded)
                .collect();
            assert_eq!(upgraded.len(), 2);
            for id in upgraded {
                assert_eq!(state.entities[id].card_kind, kind);
            }
        }
    }

    #[test]
    fn empty_cage_removes_two_picked_cards() {
        let mut state = create_game_state(0, 42, false);
        let deck_before = state.id_deck.len();
        grant(&mut state, RelicName::EmptyCage);
        for _ in 0..2 {
            assert!(state.effect_pending.is_some());
            recompute_legal_actions(&mut state);
            handle_action(&mut state, Action::CardPurge { idx: 0 }).unwrap();
            process_effect_queue(&mut state);
        }
        assert!(state.effect_pending.is_none());
        assert_eq!(state.id_deck.len(), deck_before - 2);
    }

    #[test]
    fn pandoras_box_transforms_every_starter() {
        let mut state = create_game_state(0, 42, false);
        let deck_before = state.id_deck.len();
        grant(&mut state, RelicName::PandorasBox);
        assert_eq!(state.id_deck.len(), deck_before);
        let starters = state
            .id_deck
            .iter()
            .filter(|&&id| {
                matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                )
            })
            .count();
        assert_eq!(starters, 0);
    }

    #[test]
    fn reward_take_claim_fires_the_pickup_hook() {
        let mut state = create_game_state(0, 42, false);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health = 40;
        let max_before = state.entities[id_character].vitals.health_max;
        let id_relic = push_entity(
            &mut state.entities,
            crate::relics::get_relic(RelicName::Strawberry),
        );
        state.reward_id_relic = Some(id_relic);
        state.screen = crate::types::Screen::Reward;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RewardTake {
                kind: RewardKind::Relic,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(&mut state);
        assert_eq!(
            state.entities[id_character].vitals.health_max,
            max_before + 7
        );
        assert_eq!(state.entities[id_character].vitals.health, 47);
    }
}
