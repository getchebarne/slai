use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static REFLEX: Entity = make_entity_card(
    CardName::Reflex,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[],
    &[Effect {
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    PlayRestriction::Never,
    "Unplayable. If this card is discarded from your hand, draw 2 cards.",
);
// Upgraded
pub static REFLEX_PLUS: Entity = Entity {
    card_upgraded: true,
    description: "Unplayable. If this card is discarded from your hand, draw 3 cards.",
    card_on_discard_effects: &[Effect {
        kind: EffectKind::CardDraw { count: 3 }, // +1 draw
        id_source: None,
        target: Target::Direct(None),
    }],
    ..REFLEX
};
