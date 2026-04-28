use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static TOOLS_OF_THE_TRADE: Entity = make_entity_card(
    CardName::ToolsOfTheTrade,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    1,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ToolsOfTheTrade,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
// Upgraded
pub static TOOLS_OF_THE_TRADE_PLUS: Entity = make_entity_card(
    CardName::ToolsOfTheTrade,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    0, // -1 cost
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ToolsOfTheTrade,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
);
