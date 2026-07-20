use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::DeltaSign;

const fn buy(cost: u16, count: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(cost),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        // Rolled potions land on the reward screen, where the belt is interactive
        // (discard-to-swap), matching the source's combatRewardScreen
        Effect {
            kind: EffectKind::RewardRollPotions { count },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_BUY_1: [Effect; 3] = buy(20, 1);
static OPTION_BUY_2: [Effect; 3] = buy(30, 2);
static OPTION_BUY_3: [Effect; 3] = buy(40, 3);

// Leave: free below A15; costs ceil(5% max HP) at A15+
const OPTION_LEAVE_BASE: &[Effect] = &[EVENT_CONSUME_EFFECT];
const OPTION_LEAVE_A15: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::RelativeCeil {
                numerator: 1,
                denominator: 20,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// The event only spawns with >= 50 gold, which covers every price
const LABELS_BASE: &[&str] = &[
    "[Buy 1 Potion] Lose 20 Gold.",
    "[Buy 2 Potions] Lose 30 Gold.",
    "[Buy 3 Potions] Lose 40 Gold.",
    "[Leave] Nothing happens.",
];
const LABELS_A15: &[&str] = &[
    "[Buy 1 Potion] Lose 20 Gold.",
    "[Buy 2 Potions] Lose 30 Gold.",
    "[Buy 3 Potions] Lose 40 Gold.",
    "[Leave] Lose 5% of your Max HP.",
];

pub fn labels(ascension: u8) -> &'static [&'static str] {
    if ascension < 15 {
        LABELS_BASE
    } else {
        LABELS_A15
    }
}

pub fn push_option_effects(buf: &mut Vec<Effect>, ascension: u8, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => &OPTION_BUY_1,
        1 => &OPTION_BUY_2,
        2 => &OPTION_BUY_3,
        3 if ascension < 15 => OPTION_LEAVE_BASE,
        3 => OPTION_LEAVE_A15,
        _ => unreachable!("the woman in blue option out of range: {idx}"),
    });
}
