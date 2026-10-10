use crate::effect::Amount;
use crate::effect::CandidateFilter;
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
use crate::events::deck_has_upgradable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

// Enter: 20% -> 30% max HP loss at A15; the loss lands last, so a lethal Enter still upgrades and closes
const fn enter(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::CardPileDeck,
                filters: &[CandidateFilter::Upgradeable],
                selection_kind: SelectionKind::Random { count: 2 },
            },
        },
        EFFECT_EVENT_CONSUME,
        // The Boot and Torii never touch this loss: they act only on 1 to 5 damage, which needs max HP of 27 or less (18 or less at A15+), and in Act 1, the only act with this event, max HP never drops that low
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,

                // Rounded, not truncated: this one damage roll rounds
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
    ]
}

// Enter: 20% max HP upgrades two random Cards
const OPTION_ENTER_BASE: [Effect; 3] = enter(1, 5);

// Enter at A15+: 30% max HP
const OPTION_ENTER_A15: [Effect; 3] = enter(3, 10);

// Leave
static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(&OPTION_ENTER_BASE), EOT_LEAVE];
static EOTS_A15: &[EventOptionTemplate] =
    &[make_event_option_template(&OPTION_ENTER_A15), EOT_LEAVE];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_upgradable(state),
        1 => true,
        _ => unreachable!("Shining light option out of range: {idx}"),
    }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
