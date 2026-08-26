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

pub static DEFEND: CardTemplate = make_card_template(
    CardName::Defend,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Basic,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 5 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DEFEND_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DEFEND.effects;
        effects[0].kind = EffectKind::BlockGain { amount: 8 }; // +3 block
        effects
    },
    ..DEFEND
};
