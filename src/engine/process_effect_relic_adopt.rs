use rand::Rng;

use crate::consts::CAULDRON_POTION_COUNT;
use crate::consts::MAX_SIZE_DECK;
use crate::consts::ORRERY_BUNDLE_COUNT;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EventKind;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::card_is_upgradable;
use crate::utils::increase_max_hp;
use crate::utils::mode_replace;
use crate::utils::mode_top;
use crate::utils::pick_relic_from_pool;
use crate::utils::push_entity;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");

    // Flag Relic as owned and stamp its acquisition order
    let name = state.entities[id_relic].relic_name;
    state.id_relics[name as usize] = Some(id_relic);
    state.entities[id_relic].relic_seq = state.relic_seq_next;
    state.relic_seq_next += 1;

    // Queue the Relic's pickup effects
    queue_pickup_effects(state, name);
}

// On-pickup effects; every acquisition path queues RelicAdopt, so they all land here
// TODO: add consants
fn queue_pickup_effects(state: &mut GameState, name: RelicName) {
    let id_character = state.id_character;

    match name {
        // Necronomicon arrives bound to its curse
        RelicName::Necronomicon => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Necronomicurse,
                    pile: CardPile::Deck,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Dolly's Mirror: choose a deck Card and obtain a copy of it
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

        // Lee's Waffle: gain 7 max HP and heal to full
        RelicName::LeesWaffle => {
            // Executes in reverse: MaxHealthDelta, then HealthDelta (full heal)
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

        // Strawberry / Pear / Mango: gain max HP (healed)
        RelicName::Strawberry => increase_max_hp(state, id_character, 7),
        RelicName::Pear => increase_max_hp(state, id_character, 10),
        RelicName::Mango => increase_max_hp(state, id_character, 14),

        // Old Coin: gain 300 gold
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

        // Potion Belt: gain 2 Potion slots
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }

        // War Paint / Whetstone: upgrade 2 random Skills / Attacks
        RelicName::WarPaint => upgrade_random_cards(state, 2, Some(CardKind::Skill)),
        RelicName::Whetstone => upgrade_random_cards(state, 2, Some(CardKind::Attack)),

        // Empty Cage: remove 2 Cards from the deck
        RelicName::EmptyCage => {
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

        // Pandora's Box: every starter Strike / Defend becomes a random Card
        RelicName::PandorasBox => {
            let mut id_starter = [0usize; MAX_SIZE_DECK];
            let mut id_starter_num = 0;
            for &id in &state.id_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    id_starter[id_starter_num] = id;
                    id_starter_num += 1;
                }
            }
            for &id in &id_starter[..id_starter_num] {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardTransform { upgraded: false },
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
            }
        }

        // Astrolabe: choose 3 Cards to transform; the results are upgraded
        RelicName::Astrolabe => {
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

        // Calling Bell: gain Curse of the Bell plus a Common, an Uncommon, and a Rare Relic
        RelicName::CallingBell => {
            // The bell arrives from a reward screen or Neow's consumed blessing
            assert!(
                matches!(
                    mode_top(&state.mode_stack),
                    Mode::Reward { .. }
                        | Mode::Event {
                            kind: EventKind::Neow,
                            consumed: true,
                            ..
                        }
                ),
                "Calling Bell adopts from a reward screen or Neow"
            );

            // Roll one Relic for each rarity
            let mut reward_id_relics = Vec::with_capacity(3);
            for pool in [POOL_COMMON_RELIC, POOL_UNCOMMON_RELIC, POOL_RARE_RELIC] {
                if let Some(name) = pick_relic_from_pool(pool, &state.id_relics, &mut state.rng) {
                    reward_id_relics.push(push_entity(&mut state.entities, get_relic(name)));
                }
            }

            // Set `Mode::Reward`
            mode_replace(
                &mut state.mode_stack,
                Mode::Reward {
                    reward_id_cards: Vec::new(),
                    reward_id_relics,
                    reward_id_potions: Vec::new(),
                    reward_gold: None,
                    reward_relics_exclusive: false,
                },
            );
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

        // Tiny House: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion
        RelicName::TinyHouse => {
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
            increase_max_hp(state, id_character, 5);
            upgrade_random_cards(state, 1, None);
        }

        // Ring of the Serpent: replaces the starter; SnakeRing's combat-start draw is lost
        RelicName::RingOfTheSerpent => {
            state.id_relics[RelicName::SnakeRing as usize] = None;
        }

        // Orrery: a 5-bundle Reward frame pushed over the shop; the stock resumes on exit
        RelicName::Orrery => queue_reward_roll(
            state,
            RewardSource::Cards {
                bundles: ORRERY_BUNDLE_COUNT,
            },
        ),

        // Bottled Flame / Lightning / Tornado: bottle a Card of the kind
        RelicName::BottledFlame => queue_bottle_pick(state, CandidateFilter::KindAttack),
        RelicName::BottledLightning => queue_bottle_pick(state, CandidateFilter::KindSkill),
        RelicName::BottledTornado => queue_bottle_pick(state, CandidateFilter::KindPower),

        // Cauldron: brews 5 Potions, staged as a Reward frame over the shop
        RelicName::Cauldron => queue_reward_roll(
            state,
            RewardSource::Potions {
                count: CAULDRON_POTION_COUNT as u8,
                uniform: false,
            },
        ),
        _ => {}
    }
}

// Bottle a deck Card of the given kind; an empty pool auto-resolves to no pick (Relic inert)
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

// Upgrade `count` random upgradable Cards, optionally kind-filtered; without replacement
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

// Shop relics stage their roll as a Reward frame over the stock
fn queue_reward_roll(state: &mut GameState, source: RewardSource) {
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RewardRoll { source },
        id_source: None,
        target: Target::Direct(None),
    });
}
