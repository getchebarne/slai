use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ALCHEMIZE: CardTemplate = make_card_template(
    CardName::Alchemize,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::PotionAddRandom { limited: true },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ALCHEMIZE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    cost: 0, // -1 cost
    ..ALCHEMIZE
};
