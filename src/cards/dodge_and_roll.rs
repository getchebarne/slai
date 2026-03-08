use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DODGE_AND_ROLL: Card = Card {
    name: CardName::DodgeAndRoll,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain { amount: 4 },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnBlock,
                stacks: 4,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
    ],
};
// Upgraded
pub static DODGE_AND_ROLL_PLUS: Card = Card {
    name: CardName::DodgeAndRoll,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 6, // +2 block
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnBlock,
                stacks: 6, // +2 next-turn-block
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
    ],
};
