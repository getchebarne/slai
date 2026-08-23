use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::opt;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

pub const COST_RELIC: u16 = 85;

const OPTION_PAY: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(COST_RELIC),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

const OPTION_STEAL: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shame,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_PAY), opt(OPTION_STEAL), OPTION_LEAVE];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => state.entities[state.id_character].character_gold >= COST_RELIC,
        _ => true,
    }
}
