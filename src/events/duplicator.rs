use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::CardDuplicate,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Any,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

pub const LABELS: &[&str] = &[
    "[Pray] Choose a card. Add a copy of it to your deck.",
    "[Leave] Nothing happens.",
];

pub fn push_option_effects(buf: &mut Vec<Effect>, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => OPTION_PRAY,
        1 => OPTION_LEAVE,
        _ => unreachable!("duplicator option out of range: {idx}"),
    });
}
