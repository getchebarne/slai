use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_upgradable;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;

// Forge
const OPTION_FORGE: &[Effect] = &[EFFECT_DECK_UPGRADE_PICK_1, EFFECT_EVENT_CONSUME];

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
        kind: EffectKind::CardAdd {
            card_name: CardName::Pain,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Leave
pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_FORGE),
    make_event_option_template(OPTION_RUMMAGE),
    EOT_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_upgradable(state),
        1 | 2 => true,
        _ => unreachable!("Ominous forge option out of range: {idx}"),
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
