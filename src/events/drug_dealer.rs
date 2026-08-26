use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_2;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_two_transformable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;

// J.A.X.: gain the Card
const OPTION_JAX: [Effect; 2] = [
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Jax,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Transform: two chosen Cards
const OPTION_TRANSFORM: [Effect; 2] = [EFFECT_DECK_TRANSFORM_PICK_2, EFFECT_EVENT_CONSUME];

// Mutagens: swap the Golden Idol for Toolbox
const OPTION_MUTAGENS: [Effect; 2] = [
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::MutagenicStrength,
            fallback_circlet: true,
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
        // Transforming two requires two transformable Cards
        1 => deck_has_two_transformable(state),
        _ => true,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
