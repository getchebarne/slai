use crate::cards::Card;
use crate::effect::{Candidates, EffectKind, EffectTemplate, SelectionKind, Targeting};
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ALL_OUT_ATTACK: Card = Card {
    name: CardName::AllOutAttack,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical { base: 10 },
            targeting: Some(Targeting {
                candidates: Candidates::Monsters,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDiscard,
            targeting: Some(Targeting {
                candidates: Candidates::Hand,
                selection: SelectionKind::Random { count: 1 },
            }),
        },
    ],
};
// Upgraded
pub static ALL_OUT_ATTACK_PLUS: Card = Card {
    name: CardName::AllOutAttack,
    kind: CardKind::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    requires_target: false,
    effects: &[
        EffectTemplate {
            kind: EffectKind::DamagePhysical {
                base: 14, // +4 damage
            },
            targeting: Some(Targeting {
                candidates: Candidates::Monsters,
                selection: SelectionKind::All,
            }),
        },
        EffectTemplate {
            kind: EffectKind::CardDiscard,
            targeting: Some(Targeting {
                candidates: Candidates::Hand,
                selection: SelectionKind::Random { count: 1 },
            }),
        },
    ],
};
