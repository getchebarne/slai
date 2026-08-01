use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_upgradable;
use crate::game::GameState;
use crate::types::DeltaSign;

// Enter: 20% -> 30% max HP loss at A15
const fn enter(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                // Rounded, not truncated: the source rounds this one damage roll
                amount: Amount::RelativeRounded {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck,
                filter: CandidateFilter::Upgradeable,
                selection_kind: SelectionKind::Random { count: 2 },
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_ENTER_BASE: [Effect; 3] = enter(1, 5);
const OPTION_ENTER_A15: [Effect; 3] = enter(3, 10);

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option(
        "[Enter] Upgrade 2 random cards. Lose 20% of your max HP.",
        &OPTION_ENTER_BASE,
    ),
    make_entity_event_option("[Leave] Nothing happens.", OPTION_LEAVE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option(
        "[Enter] Upgrade 2 random cards. Lose 30% of your max HP.",
        &OPTION_ENTER_A15,
    ),
    make_entity_event_option("[Leave] Nothing happens.", OPTION_LEAVE),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_upgradable(state),
        1 => true,
        _ => unreachable!("Shining light option out of range: {idx}"),
    }
}
