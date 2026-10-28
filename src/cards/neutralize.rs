use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_MONSTER_SELECTED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static NEUTRALIZE: CardTemplate = make_card_template(
    CardName::Neutralize,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Basic,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 3,
                instances: 1,
                lifesteal: false,
                if_poisoned: false,
                scaled_at: ScaledAt::Now,
            },
            id_source: None,
            target: TARGET_MONSTER_SELECTED,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_MONSTER_SELECTED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static NEUTRALIZE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = NEUTRALIZE.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 4,
            instances: 1,
            lifesteal: false,
            if_poisoned: false,
            scaled_at: ScaledAt::Now,
        }; // +1 damage
        effects[1].kind = EffectKind::ModifierDelta {
            kind: ModifierKind::Weak,
            stacks: 2, // +1 stack
        };
        effects
    },
    ..NEUTRALIZE
};
