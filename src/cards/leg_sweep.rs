use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static LEG_SWEEP: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 11,
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
    ],
};
// Upgraded
pub static LEG_SWEEP_PLUS: Card = Card {
    name: CardName::LegSweep,
    kind: CardKind::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: true,
    effects: &[
        EffectTemplate {
            kind: EffectKind::BlockGain {
                amount: 14, // +3 block
                from_card: true,
            },
            targeting: Some(Targeting {
                candidates: Candidates::Character,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 3, // +1 stack
            },
            targeting: Some(Targeting {
                candidates: Candidates::CardTarget,
                selection: SelectionKind::All,
            }),
        },
    ],
};
