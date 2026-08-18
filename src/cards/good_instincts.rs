use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static GOOD_INSTINCTS: CardTemplate = make_card_template(
    CardName::GoodInstincts,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 6 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static GOOD_INSTINCTS_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = GOOD_INSTINCTS.effects;
        effects[0].kind = EffectKind::BlockGain { amount: 9 }; // +3 block
        effects
    },
    ..GOOD_INSTINCTS
};
