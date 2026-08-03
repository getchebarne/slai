use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_damage_card;
use crate::events::deck_has_purgeable;
use crate::game::GameState;
use crate::types::DeltaSign;

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(7),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_DECK_PURGE_PICK,
    EVENT_CONSUME_EFFECT,
];

// Attack
const OPTION_ATTACK: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Range { min: 50, max: 80 },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Pray] Remove a card from your deck. Lose 7 HP.",
        OPTION_PRAY,
    ),
    make_entity_event_option("[Destroy] Receive 50-80 Gold.", OPTION_ATTACK),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_damage_card(state, 10),
        2 => true,
        _ => unreachable!("Wing statue option out of range: {idx}"),
    }
}
