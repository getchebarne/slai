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

pub static MIND_BLAST: CardTemplate = make_card_template(
    CardName::MindBlast,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamageMindBlast,
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MIND_BLAST_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    cost: 1, // -1 cost
    ..MIND_BLAST
};
