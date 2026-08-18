use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 4,
        lifesteal: false,
    },
    id_source: None,
    target: TARGET_MONSTERS_ALL,
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 6,
        lifesteal: false,
    }, // +2 damage
    id_source: None,
    target: TARGET_MONSTERS_ALL,
};

pub static DAGGER_SPRAY: CardTemplate = make_card_template(
    CardName::DaggerSpray,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DAGGER_SPRAY_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DAGGER_SPRAY.effects;
        effects[0] = HIT_PLUS;
        effects[1] = HIT_PLUS;
        effects
    },
    ..DAGGER_SPRAY
};
