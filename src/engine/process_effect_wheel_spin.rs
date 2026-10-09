use rand::Rng;

use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::consts::WHEEL_GOLD_PER_ACT;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::RelicExclusion;
use crate::effect::RelicPick;
use crate::effect::Rounding;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

pub fn process_effect_wheel_spin(state: &mut GameState) {
    let id_character = state.id_character;

    // Uniform 1/6 across gold / Relic / full heal / decay / purge / health loss
    let effect = match state.rng.random_range(0..6) {
        0 => Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(WHEEL_GOLD_PER_ACT * state.act as u16),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        // The Relic is staged as a reward the player may leave
        1 => Effect {
            kind: EffectKind::RewardRollRelic {
                pick: RelicPick::Thresholds {
                    th_common: RELIC_TIER_TH_COMMON,
                    th_uncommon: RELIC_TIER_TH_UNCOMMON,
                },
                exclusion: RelicExclusion::Screenless,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        2 => Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 1,
                    rounding: Rounding::Truncate,
                    read_at: ReadAt::Now,
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
                candidate_pool: CandidatePool::Deck,
                filters: &[CandidateFilter::NotBottled, CandidateFilter::NotBoundCurse],
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
                        rounding: Rounding::Truncate,
                        read_at: ReadAt::Now,
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
