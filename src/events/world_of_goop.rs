use rand::Rng;

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

// Leave's gold loss roll; A15+ forfeits more
const GOLD_LOSS_MIN: u16 = 20;
const GOLD_LOSS_MAX: u16 = 50;
const GOLD_LOSS_MIN_A15: u16 = 35;
const GOLD_LOSS_MAX_A15: u16 = 75;

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(11),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(75),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Leave: forfeit the gold rolled on entry
const fn leave(gold: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Catalog layout: Gather, then Leave for every loss up to the max (capping can go below the min)
const IDX_GATHER: usize = 0;
const IDX_LEAVE: usize = 1;
const EOTS_LEN: usize = IDX_LEAVE + GOLD_LOSS_MAX as usize + 1;
const EOTS_LEN_A15: usize = IDX_LEAVE + GOLD_LOSS_MAX_A15 as usize + 1;

const fn eots<const N: usize>() -> [EventOptionTemplate; N] {
    let mut eots = [make_event_option_template(OPTION_GATHER); N];
    let mut idx = IDX_LEAVE;
    while idx < N {
        let option_leave = leave((idx - IDX_LEAVE) as u16);
        eots[idx] = make_event_option_template(&option_leave);
        idx += 1;
    }
    eots
}

static EOTS_BASE: [EventOptionTemplate; EOTS_LEN] = eots();
static EOTS_A15: [EventOptionTemplate; EOTS_LEN_A15] = eots();

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 {
        &EOTS_BASE
    } else {
        &EOTS_A15
    }
}

// The loss rolls on entry, capped at the gold held, so Leave shows the exact amount
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    let (min, max) = if state.ascension < 15 {
        (GOLD_LOSS_MIN, GOLD_LOSS_MAX)
    } else {
        (GOLD_LOSS_MIN_A15, GOLD_LOSS_MAX_A15)
    };
    let gold = state.entities[state.id_character].character_gold;
    let gold_loss = state.rng.random_range(min..=max).min(gold);
    let eots = catalog(state.ascension);
    let options = [eots[IDX_GATHER], eots[IDX_LEAVE + gold_loss as usize]];
    bake_options(state, &options)
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
