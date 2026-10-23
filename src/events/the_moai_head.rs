use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::SelectionKind;
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
use crate::utils::has_relic;

// Jump Inside: lose a fraction of the max HP the event opened with, then heal to the new max
const fn jump_inside(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                    rounding: Rounding::HalfUp,
                    read_at: ReadAt::EventOpen,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
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
            target: TARGET_CHARACTER,
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Jump Inside: 12.5% max HP
const OPTION_JUMP_INSIDE_BASE: [Effect; 3] = jump_inside(1, 8);

// Jump Inside at A15+: 18% max HP
const OPTION_JUMP_INSIDE_A15: [Effect; 3] = jump_inside(18, 100);

// Offer the Idol: lose it, then gain 333 gold
const OPTION_IDOL: [Effect; 3] = [
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::RelicEventRoll,
            filters: &[],
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(333),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_JUMP_INSIDE_BASE),
    make_event_option_template(&OPTION_IDOL),
    EOT_LEAVE,
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_JUMP_INSIDE_A15),
    make_event_option_template(&OPTION_IDOL),
    EOT_LEAVE,
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        1 => has_relic(&state.id_relics, RelicName::GoldenIdol),
        _ => true,
    }
}

// The Idol offer consumes the staked Relic; availability gates it on ownership
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    if let Some(id) = state.id_relics[RelicName::GoldenIdol as usize] {
        state.event.id_relic_event_roll.push(id);
    }
    bake_options(state, catalog(state.ascension))
}
