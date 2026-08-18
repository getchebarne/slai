use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_2;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::deck_has_two_transformable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;

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
    EVENT_CONSUME_EFFECT,
];

const OPTION_TRANSFORM: [Effect; 2] = [EFFECT_DECK_TRANSFORM_PICK_2, EVENT_CONSUME_EFFECT];

const OPTION_MUTAGENS: [Effect; 2] = [
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::MutagenicStrength,
            fallback_circlet: true,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[EventOptionTemplate] = &[
    make_event_option_template("[Ingest Mutagens] Obtain J.A.X.", &OPTION_JAX),
    make_event_option_template("[Sign Up] Transform 2 Cards.", &OPTION_TRANSFORM),
    make_event_option_template("[Experiment] Obtain Mutagenic Strength.", &OPTION_MUTAGENS),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        // Transforming two requires two transformable Cards
        1 => deck_has_two_transformable(state),
        _ => true,
    }
}
