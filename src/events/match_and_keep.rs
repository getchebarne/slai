use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_CURSE_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::cards::get_card_template;
use crate::consts::MATCH_AND_KEEP_ATTEMPTS;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::types::CardName;
use crate::utils::push_entity;
use crate::utils::shuffle;

// Flip the next Card never flipped; the board is shuffled, so every unseen Card is alike
const OPTION_FLIP_UNSEEN: &[Effect] = &[Effect {
    kind: EffectKind::MatchFlipUnseen,
    id_source: None,
    target: Target::Direct(None),
}];

// Flip a face-down Card seen before, picked from the seen ones
const OPTION_FLIP_SEEN: &[Effect] = &[Effect {
    kind: EffectKind::MatchFlipSeen,
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::CardPileEventRoll,
        filters: &[],
        selection_kind: SelectionKind::Input { count: 1 },
    },
}];

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_FLIP_UNSEEN),
    make_event_option_template(OPTION_FLIP_SEEN),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

// Deals the board face down; the flips play out in the MatchFlipUnseen and MatchFlipSeen processors
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    let ascension = state.ascension;
    let rng = &mut state.rng;

    // A Rare, an Uncommon and a Common Silent Card, a colorless Uncommon, a Curse and Neutralize
    let names = [
        POOL_RARE_GREEN_CARD[rng.random_range(0..POOL_RARE_GREEN_CARD.len())],
        POOL_UNCOMMON_GREEN_CARD[rng.random_range(0..POOL_UNCOMMON_GREEN_CARD.len())],
        POOL_COMMON_GREEN_CARD[rng.random_range(0..POOL_COMMON_GREEN_CARD.len())],
        // A15+ deals a second Curse in place of the colorless Uncommon
        if ascension < 15 {
            POOL_UNCOMMON_COLORLESS_CARD[rng.random_range(0..POOL_UNCOMMON_COLORLESS_CARD.len())]
        } else {
            POOL_CURSE_CARD[rng.random_range(0..POOL_CURSE_CARD.len())]
        },
        POOL_CURSE_CARD[rng.random_range(0..POOL_CURSE_CARD.len())],
        CardName::Neutralize,
    ];

    // Each Card is dealt twice, with the eggs' upgrades on show
    for name in names {
        let upgraded = egg_upgrades_kind(get_card_template(name, false).kind, &state.id_relics);
        for _ in 0..2 {
            let id_card = push_entity(&mut state.entities, get_card(name, upgraded));
            state.event.id_card_match_unseen.push(id_card);
        }
    }
    shuffle(&mut state.event.id_card_match_unseen, &mut state.rng);
    state.event.match_attempts = MATCH_AND_KEEP_ATTEMPTS;
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => !state.event.id_card_match_unseen.is_empty(),
        1 => !state.event.id_card_pile_event_roll.is_empty(),
        _ => unreachable!("Match and Keep! option out of range: {idx}"),
    }
}
