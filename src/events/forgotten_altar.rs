use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

const OPTION_IDOL: [Effect; 3] = [
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRelicPicks,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::BloodyIdol,
            fallback_circlet: true,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Damage resolves before the max-HP gain so the fraction reads the old maximum
const fn sacrifice(numerator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::RelativeRounded {
                    numerator,
                    denominator: 100,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EVENT_CONSUME_EFFECT,
    ]
}

const OPTION_DECAY: [Effect; 2] = [
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Decay,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

const OPTION_SACRIFICE_BASE: [Effect; 3] = sacrifice(25);
const OPTION_SACRIFICE_A15: [Effect; 3] = sacrifice(35);

static OPTIONS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template("[Offer: Golden Idol] Obtain Bloody Idol.", &OPTION_IDOL),
    make_event_option_template(
        "[Sacrifice] Gain 5 Max HP. Lose 25% of your Max HP.",
        &OPTION_SACRIFICE_BASE,
    ),
    make_event_option_template("[Desecrate] Obtain Decay.", &OPTION_DECAY),
];
static OPTIONS_A15: &[EventOptionTemplate] = &[
    make_event_option_template("[Offer: Golden Idol] Obtain Bloody Idol.", &OPTION_IDOL),
    make_event_option_template(
        "[Sacrifice] Gain 5 Max HP. Lose 35% of your Max HP.",
        &OPTION_SACRIFICE_A15,
    ),
    make_event_option_template("[Desecrate] Obtain Decay.", &OPTION_DECAY),
];

pub fn options(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => has_relic(&state.id_relics, RelicName::GoldenIdol),
        _ => true,
    }
}

// The Idol trade targets the staked Relic; availability gates it on ownership
pub fn spawn_event_forgotten_altar(state: &mut GameState) -> Vec<usize> {
    if let Some(id) = state.id_relics[RelicName::GoldenIdol as usize] {
        state.event.id_relic_picks.push(id);
    }
    bake_options(state, options(state.ascension))
}
