use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_upgradable;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::RelicName;

// Forge
const OPTION_FORGE: &[Effect] = &[EFFECT_DECK_UPGRADE_PICK_1, EVENT_CONSUME_EFFECT];

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
    EVENT_CONSUME_EFFECT,
];

// Leave
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Forge] Upgrade a card.", OPTION_FORGE),
    make_entity_event_option(
        "[Rummage] Obtain Warped Tongs. Become Cursed - Pain.",
        OPTION_RUMMAGE,
    ),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_upgradable(state),
        1 | 2 => true,
        _ => unreachable!("Ominous forge option out of range: {idx}"),
    }
}
