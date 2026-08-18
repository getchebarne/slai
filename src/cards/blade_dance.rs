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
use crate::types::CardPile;
use crate::types::CardRarity;

pub static BLADE_DANCE: CardTemplate = make_card_template(
    CardName::BladeDance,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BLADE_DANCE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = BLADE_DANCE.effects;
        effects[0].kind = EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 4, // +1 shiv
            upgraded: false,
        };
        effects
    },
    ..BLADE_DANCE
};
