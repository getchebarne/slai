use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: CardTemplate = make_card_template(
    CardName::Backstab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 11,
            instances: 1,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BACKSTAB_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = BACKSTAB.effects_play;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 15,
            instances: 1,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..BACKSTAB
};
