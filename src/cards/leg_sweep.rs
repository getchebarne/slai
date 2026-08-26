use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static LEG_SWEEP: CardTemplate = make_card_template(
    CardName::LegSweep,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 11 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static LEG_SWEEP_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = LEG_SWEEP.effects;
        effects[0].kind = EffectKind::BlockGain { amount: 14 }; // +3 block
        effects[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 3, // +1 stack
        };
        effects
    },
    ..LEG_SWEEP
};
