use rand::Rng;

use crate::consts::MAX_SIZE_DECK;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::card_is_upgradable;
use crate::utils::pick_from_pool;
use crate::utils::push_entity;

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
                    candidate_pool: CandidatePool::Deck,
                    filter: CandidateFilter::Any,
                    selection_kind: SelectionKind::Input { count: 1 },
                },
            });
        }
        RelicName::LeesWaffle => {
            // Executes in reverse:
            //     1. MaxHealthDelta
            //     2. HealthDelta (full heal)
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
        RelicName::WarPaint => upgrade_random_cards(state, 2, Some(CardKind::Skill)),
        RelicName::Whetstone => upgrade_random_cards(state, 2, Some(CardKind::Attack)),
        RelicName::EmptyCage => {
            // Two sequential halting picks; auto-resolve covers small decks
            for _ in 0..2 {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardPurge,
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Deck,
                        filter: CandidateFilter::Purgeable,
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
                    kind: EffectKind::CardTransform { upgraded: false },
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }
        RelicName::Astrolabe => {
            // Choose 3 to transform and upgrade; <=3 transformable auto-resolves
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardTransform { upgraded: true },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Deck,
                    filter: CandidateFilter::Transformable,
                    selection_kind: SelectionKind::Input { count: 3 },
                },
            });
        }
        RelicName::CallingBell => {
            // Curse of the Bell, then one Common, Uncommon, and Rare relic. A granted
            // Bottle's halting pick suspends the chain and resumes cleanly, so the
            // source's screenless-relic exclusion is unnecessary
            for pool in [POOL_RARE_RELIC, POOL_UNCOMMON_RELIC, POOL_COMMON_RELIC] {
                if let Some(name) = pick_from_pool(pool, &state.id_relics, &mut state.rng) {
                    let id = push_entity(&mut state.entities, get_relic(name));
                    state.effect_queue.push_front(Effect {
                        kind: EffectKind::RelicAdopt,
                        id_source: None,
                        target: Target::Direct(Some(id)),
                    });
                }
            }
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::CurseOfTheBell,
                    pile: CardPile::Deck,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RelicName::TinyHouse => {
            // Upgrade 1 random card, +5 max HP (healed), 50 gold, 1 random potion
            state.effect_queue.push_front(Effect {
                kind: EffectKind::PotionAddRandom { limited: false },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(50),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
            stat_pickup(state, id_character, 5);
            upgrade_random_cards(state, 1, None);
        }
        RelicName::RingOfTheSerpent => {
            // The boss upgrade replaces the starter; its combat-start draw is lost
            state.id_relics[RelicName::SnakeRing as usize] = None;
        }
        RelicName::Orrery => {
            // First of 4 card rewards; relic_counter drives the rest (room_exit chains them)
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RewardRollCards,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RelicName::BottledFlame => queue_bottle_pick(state, CandidateFilter::KindAttack),
        RelicName::BottledLightning => queue_bottle_pick(state, CandidateFilter::KindSkill),
        RelicName::BottledTornado => queue_bottle_pick(state, CandidateFilter::KindPower),
        RelicName::Cauldron => {
            // Brew 5 potions; overflow beyond belt space is lost (Java stages them as rewards)
            for _ in 0..5 {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::PotionAddRandom { limited: false },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
        _ => {}
    }
}

// Bottle a deck card of the given kind; an empty pool auto-resolves to no pick (relic inert)
fn queue_bottle_pick(state: &mut GameState, filter: CandidateFilter) {
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardBottle,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
}

// Max HP first so the heal lands under the new ceiling
fn stat_pickup(state: &mut GameState, id_character: usize, amount: u16) {
    // Executes in reverse:
    //     1. MaxHealthDelta
    //     2. HealthDelta
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

// Upgrade `count` random upgradable cards, optionally kind-filtered; without replacement
fn upgrade_random_cards(state: &mut GameState, count: usize, kind: Option<CardKind>) {
    let mut ids_valid = [0usize; MAX_SIZE_DECK];
    let mut num = 0;
    for &id in &state.id_deck {
        let card = &state.entities[id];
        if card_is_upgradable(card) && kind.is_none_or(|k| card.card_kind == k) {
            ids_valid[num] = id;
            num += 1;
        }
    }

    for _ in 0..count.min(num) {
        let idx = state.rng.random_range(0..num);
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
