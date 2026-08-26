use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::RelicName;

// Touch: gold gain first, then health loss; -25 gold gain at A15
const fn touch(gold: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 10,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Touch: 75 gold and a face
const OPTION_TOUCH_BASE: [Effect; 3] = touch(75);

// Touch at A15+: 50 gold
const OPTION_TOUCH_A15: [Effect; 3] = touch(50);

// Trade: gain a random unowned face Relic
const FACE_POOL: &[RelicName] = &[
    RelicName::CultistHeadpiece,
    RelicName::FaceOfCleric,
    RelicName::GremlinVisage,
    RelicName::NlothsHungryFace,
    RelicName::SsserpentHead,
];

// Trade: one of the face Relics
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantPool { pool: FACE_POOL },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Leave
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_TOUCH_BASE),
    make_event_option_template(OPTION_TRADE),
    EOT_LEAVE,
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_TOUCH_A15),
    make_event_option_template(OPTION_TRADE),
    EOT_LEAVE,
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
