use crate::effect::EffectTemplate;
use crate::effect::SelectionKind;
use crate::effect::TargetKind;
use crate::cards::Card;
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
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 10,
            target: TargetKind::AllMonsters,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Random,
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
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 14, // +4 damage
            target: TargetKind::AllMonsters,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Random,
        },
    ],
};
