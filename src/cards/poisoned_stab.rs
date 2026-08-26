use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static POISONED_STAB: CardTemplate = make_card_template(
    CardName::PoisonedStab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 6,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: +2 damage, +1 poison
pub static POISONED_STAB_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = POISONED_STAB.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 8,
            lifesteal: false,
        }; // +2 damage
        effects[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 4, // +1 poison
        };
        effects
    },
    ..POISONED_STAB
};
