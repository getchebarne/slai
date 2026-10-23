use rand::Rng;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardKind;
use crate::utils::filter_candidates;

// The spawn's pick lists in draw order; a bottled Card is never staked
const CANDIDATE_FILTERS: [&[CandidateFilter]; 3] = [
    &[CandidateFilter::NotBottled, CandidateFilter::CardKindAttack],
    &[CandidateFilter::NotBottled, CandidateFilter::CardKindSkill],
    &[CandidateFilter::NotBottled, CandidateFilter::CardKindPower],
];

// Lose the staked Card of one kind
const fn lose(filters: &'static [CandidateFilter]) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::CardPileEventRoll,
                filters,
                selection_kind: SelectionKind::Single,
            },
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Lose the staked Skill
const OPTION_SKILL: [Effect; 2] = lose(&[CandidateFilter::CardKindSkill]);

// Lose the staked Power
const OPTION_POWER: [Effect; 2] = lose(&[CandidateFilter::CardKindPower]);

// Lose the staked Attack
const OPTION_ATTACK: [Effect; 2] = lose(&[CandidateFilter::CardKindAttack]);

// The type rows, then the leave row that is the whole menu when nothing is staked
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_SKILL),
    make_event_option_template(&OPTION_POWER),
    make_event_option_template(&OPTION_ATTACK),
    EOT_LEAVE,
];
const IDX_LEAVE: usize = 3;

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

// Stakes one uniform pick per kind present; a missing kind draws nothing and keeps its row, locked
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    for filters in CANDIDATE_FILTERS {
        let mut id_cards = state.id_card_pile_deck.clone();
        filter_candidates(filters, &mut id_cards, &state.entities, None);
        if !id_cards.is_empty() {
            let id_card = id_cards[state.rng.random_range(0..id_cards.len())];
            state.event.id_card_pile_event_roll.push(id_card);
        }
    }

    let eots = catalog(state.ascension);
    if state.event.id_card_pile_event_roll.is_empty() {
        bake_options(state, &[EOT_LEAVE])
    } else {
        bake_options(state, &eots[..IDX_LEAVE])
    }
}

// A row needs a staked Card of its kind; the leave-only menu is always open
pub fn option_available(state: &GameState, idx: usize) -> bool {
    let id_cards = &state.event.id_card_pile_event_roll;
    if id_cards.is_empty() {
        return true;
    }
    let kind = match idx {
        0 => CardKind::Skill,
        1 => CardKind::Power,
        2 => CardKind::Attack,
        _ => unreachable!("Falling option out of range: {idx}"),
    };
    id_cards
        .iter()
        .any(|&id| state.entities[id].card_kind == kind)
}
