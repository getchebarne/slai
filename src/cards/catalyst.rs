use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CATALYST: Entity = make_entity_card(
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
pub static CATALYST_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = CATALYST.card_effects;
        effects[0].kind = EffectKind::ModifierMultiply {
            kind: ModifierKind::Poison,
            factor: 3, // +1 factor
        };
        effects
    },
    ..CATALYST
};
