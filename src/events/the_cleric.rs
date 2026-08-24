use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_purgeable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

const COST_HEAL: u16 = 35;
const COST_PURIFY_BASE: u16 = 50;
const COST_PURIFY_A15: u16 = 75;

// Heal
const OPTION_HEAL: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(COST_HEAL),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 4,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_EVENT_CONSUME,
];

// Purify: +25 gold cost at A15
const fn purify(cost: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(cost),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_DECK_PURGE_PICK_1,
        EFFECT_EVENT_CONSUME,
    ]
}

// Purify: 50 gold purges a Card
const OPTION_PURIFY_BASE: [Effect; 3] = purify(COST_PURIFY_BASE);

// Purify at A15+: 75 gold
const OPTION_PURIFY_A15: [Effect; 3] = purify(COST_PURIFY_A15);

// Leave
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_HEAL),
    make_event_option_template(&OPTION_PURIFY_BASE),
    EOT_LEAVE,
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_HEAL),
    make_event_option_template(&OPTION_PURIFY_A15),
    EOT_LEAVE,
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    let gold = state.entities[state.id_character].character_gold;
    match idx {
        0 => gold >= COST_HEAL,
        1 => {
            let cost = if state.ascension < 15 {
                COST_PURIFY_BASE
            } else {
                COST_PURIFY_A15
            };
            gold >= cost && deck_has_purgeable(state)
        }
        2 => true,
        _ => unreachable!("The cleric option out of range: {idx}"),
    }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
