use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

// Offer: pick a card to purge. An empty purgeable pool auto-resolves to nothing
const OPTION_OFFER: &[Effect] = &[
    Effect {
        kind: EffectKind::BonfireOffer,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Purgeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

pub const LABELS: &[&str] = &["[Offer] Remove a card; its rarity decides the spirits' blessing."];

pub fn push_option_effects(buf: &mut Vec<Effect>, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => OPTION_OFFER,
        _ => unreachable!("bonfire spirits option out of range: {idx}"),
    });
}
