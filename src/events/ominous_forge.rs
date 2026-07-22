use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_upgradable;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::RelicName;

// Forge
const OPTION_FORGE: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Upgradeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Rummage
const OPTION_RUMMAGE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::WarpedTongs,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Pain,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

pub const OPTIONS: &[(&str, &[Effect])] = &[
    ("[Forge] Upgrade a card.", OPTION_FORGE),
    (
        "[Rummage] Obtain Warped Tongs. Become Cursed - Pain.",
        OPTION_RUMMAGE,
    ),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_upgradable(state),
        1 | 2 => true,
        _ => unreachable!("ominous forge option out of range: {idx}"),
    }
}
