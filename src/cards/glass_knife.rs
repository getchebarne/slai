use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::TARGET_SOURCE;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static GLASS_KNIFE: CardTemplate = make_card_template(
    CardName::GlassKnife,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
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
                instances: 2,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::GlassKnifeDecay { delta: -2 },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static GLASS_KNIFE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = GLASS_KNIFE.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 12,
            instances: 2,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..GLASS_KNIFE
};
