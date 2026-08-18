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

pub static PIERCING_WAIL: CardTemplate = make_card_template(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::StrengthLoseTemp { stacks: 6 },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PIERCING_WAIL_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = PIERCING_WAIL.effects;
        effects[0].kind = EffectKind::StrengthLoseTemp { stacks: 8 };
        effects
    },
    ..PIERCING_WAIL
};
