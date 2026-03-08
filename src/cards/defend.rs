use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DEFEND: Card = Card {
    name: CardName::Defend,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::BlockGain { amount: 5 },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
// Upgraded
pub static DEFEND_PLUS: Card = Card {
    name: CardName::Defend,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[EffectTemplate {
        kind: EffectKind::BlockGain {
            amount: 8, // +3 block
        },
        targeting: Some(Targeting {
            candidates: Candidates::Character,
            selection: SelectionKind::All,
        }),
    }],
};
