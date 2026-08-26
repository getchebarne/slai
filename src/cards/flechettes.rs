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

pub static FLECHETTES: CardTemplate = make_card_template(
    CardName::Flechettes,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamageFlechettes { damage: 4 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLECHETTES_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = FLECHETTES.effects;
        effects[0].kind = EffectKind::DamageFlechettes { damage: 6 }; // +2 damage
        effects
    },
    ..FLECHETTES
};
