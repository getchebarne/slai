use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_purgeable;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::DeltaSign;

// The draw gate in `draw_event` requires this much gold before the event can spawn
pub const BEGGAR_COST_PURGE: u16 = 75;

const OPTION_GIVE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(BEGGAR_COST_PURGE),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_DECK_PURGE_PICK,
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Offer Gold] Lose 75 Gold. Remove a card from your deck.",
        OPTION_GIVE,
    ),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => {
            state.entities[state.id_character].character_gold >= BEGGAR_COST_PURGE
                && deck_has_purgeable(state)
        }
        _ => true,
    }
}
