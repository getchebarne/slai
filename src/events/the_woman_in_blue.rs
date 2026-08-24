use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
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
        EFFECT_EVENT_CONSUME,
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

// Buy one Potion: 20 gold
const OPTION_BUY_1: [Effect; 3] = buy(20, 1);

// Buy two Potions: 30 gold
const OPTION_BUY_2: [Effect; 3] = buy(30, 2);

// Buy three Potions: 40 gold
const OPTION_BUY_3: [Effect; 3] = buy(40, 3);

// Leave: free below A15; costs ceil(5% max HP) at A15+
const OPTION_LEAVE_BASE: &[Effect] = &[EFFECT_EVENT_CONSUME];

// Leave at A15+: costs max HP
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
    EFFECT_EVENT_CONSUME,
];

// The event only spawns with >= 50 gold, which covers every price
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_BUY_1),
    make_event_option_template(&OPTION_BUY_2),
    make_event_option_template(&OPTION_BUY_3),
    make_event_option_template(OPTION_LEAVE_BASE),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_BUY_1),
    make_event_option_template(&OPTION_BUY_2),
    make_event_option_template(&OPTION_BUY_3),
    make_event_option_template(OPTION_LEAVE_A15),
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
