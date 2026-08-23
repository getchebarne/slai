use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;
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
        // Consume first: the staged Reward overlays this frame until RoomExit
        // Rolled Potions land on the Reward context, where the belt is interactive
        // (discard-to-swap), matching the source's combatRewardScreen
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RewardRollPotions {
                count,
                uniform: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTION_BUY_1: [Effect; 3] = buy(20, 1);
const OPTION_BUY_2: [Effect; 3] = buy(30, 2);
const OPTION_BUY_3: [Effect; 3] = buy(40, 3);

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
        target: TARGET_CHARACTER,
    },
    EVENT_CONSUME_EFFECT,
];

// The event only spawns with >= 50 gold, which covers every price
static OPTIONS_BASE: &[&[Effect]] = &[
    opt(&OPTION_BUY_1),
    opt(&OPTION_BUY_2),
    opt(&OPTION_BUY_3),
    opt(OPTION_LEAVE_BASE),
];
static OPTIONS_A15: &[&[Effect]] = &[
    opt(&OPTION_BUY_1),
    opt(&OPTION_BUY_2),
    opt(&OPTION_BUY_3),
    opt(OPTION_LEAVE_A15),
];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
