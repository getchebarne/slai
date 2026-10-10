use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::utils::card_name_bound_curse;

// J.A.X.: gain the Card
const OPTION_JAX: [Effect; 2] = [
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Jax,
            card_pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Transform: two chosen Cards; bottled Cards are offered, only the bound curses are not
const OPTION_TRANSFORM: [Effect; 2] = [
    // Consume first, so the pick lands over the spent event
    EFFECT_EVENT_CONSUME,
    Effect {
        kind: EffectKind::CardTransform { upgraded: false },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDeck,
            filters: &[CandidateFilter::NotBoundCurse],
            selection_kind: SelectionKind::Input { count: 2 },
        },
    },
];

// Mutagens: swap the Golden Idol for Toolbox
const OPTION_MUTAGENS: [Effect; 2] = [
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::MutagenicStrength,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_JAX),
    make_event_option_template(&OPTION_TRANSFORM),
    make_event_option_template(&OPTION_MUTAGENS),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        // Transforming two requires two Cards the pick offers
        1 => state
            .id_card_pile_deck
            .iter()
            .filter(|&&id| !card_name_bound_curse(state.entities[id].card_name))
            .nth(1)
            .is_some(),
        _ => true,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
