use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CATALYST: CardTemplate = make_card_template(
    CardName::Catalyst,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 2,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: triples instead of doubles
pub static CATALYST_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = CATALYST.effects;
        effects[0].kind = EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 3, // +1 factor
        };
        effects
    },
    ..CATALYST
};
