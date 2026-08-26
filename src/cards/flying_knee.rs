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

pub static FLYING_KNEE: CardTemplate = make_card_template(
    CardName::FlyingKnee,
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
                amount: 8,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NextTurnEnergy,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLYING_KNEE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = FLYING_KNEE.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 11,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..FLYING_KNEE
};
