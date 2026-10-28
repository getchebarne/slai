use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTER_SELECTED;
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
    effects_play: {
        let mut effects = FLYING_KNEE.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 11,
            instances: 1,
            lifesteal: false,
            if_poisoned: false,
            scaled_at: ScaledAt::Now,
        }; // +3 damage
        effects
    },
    ..FLYING_KNEE
};
