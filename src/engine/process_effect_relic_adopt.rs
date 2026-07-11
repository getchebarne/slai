use rand::Rng;

use crate::consts::MAX_SIZE_DECK;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");

    // Flag Relic as owned
    let name = state.entities[id_relic].relic_name;
    state.id_relics[name as usize] = Some(id_relic);

    // Queue the Relic's pickup effects
    queue_pickup_effects(state, name);
}

// On-pickup effects; every acquisition path queues RelicAdopt, so they all land here
// TODO: add consants
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
                    amount: Amount::Relative {
                        // Full health
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
                    amount: Amount::Absolute(7),
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
                    amount: Amount::Absolute(300),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }
        RelicName::WarPaint => upgrade_two_random_cards_from_kind(state, CardKind::Skill),
        RelicName::Whetstone => upgrade_two_random_cards_from_kind(state, CardKind::Attack),
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
            let mut num = 0;
            for &id in &state.id_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    starters[num] = id;
                    num += 1;
                }
            }
            for &id in &starters[..num] {
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
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}

fn upgrade_two_random_cards_from_kind(state: &mut GameState, kind: CardKind) {
    let mut ids_valid = [0usize; MAX_SIZE_DECK];

    // Get valid cards
    let mut num = 0;
    for &id in &state.id_deck {
        let card = &state.entities[id];
        if card.card_kind == kind && !card.card_upgraded {
            ids_valid[num] = id;
            num += 1;
        }
    }

    // Sample
    for _ in 0..2.min(num) {
        let idx = state.rng.random_range(0..num);

        // Queue effect
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(ids_valid[idx])),
        });

        // Without replacement
        ids_valid[idx] = ids_valid[num - 1];
        num -= 1;
    }
}
