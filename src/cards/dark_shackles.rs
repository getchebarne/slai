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

pub static DARK_SHACKLES: CardTemplate = make_card_template(
    CardName::DarkShackles,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::StrengthLoseTemp { stacks: 9 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DARK_SHACKLES_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DARK_SHACKLES.effects;
        effects[0].kind = EffectKind::StrengthLoseTemp { stacks: 15 };
        effects
    },
    ..DARK_SHACKLES
};
