use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;

pub static BLADE_DANCE: Entity = make_entity_card(
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
pub static BLADE_DANCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = BLADE_DANCE.card_effects;
        a[0].kind = EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 4, // +1 shiv
            upgraded: false,
        };
        a
    },
    ..BLADE_DANCE
};
