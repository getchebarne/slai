use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

pub fn process_effect_wheel_spin(state: &mut GameState) {
    let id_character = state.id_character;

    // Uniform 1/6 across gold / relic / full heal / decay / purge / health loss
    let effect = match state.rng.random_range(0..6) {
        0 => Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(100),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        1 => Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        },
        2 => Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 1,
                },
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        },
        3 => Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Decay,
                pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        4 => Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolCardFilter::Purgeable,
                },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        _ => {
            let (numerator, denominator) = if state.ascension < 15 {
                (1, 10)
            } else {
                (3, 20)
            };
            Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Loss,
                    amount: Amount::Relative {
                        numerator,
                        denominator,
                    },
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            }
        }
    };

    // Push
    state.effect_queue.push_front(effect);
}
