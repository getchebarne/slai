use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static TRIP: Entity = make_entity_card(
    CardName::Trip,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Vulnerable,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static TRIP_PLUS: Entity = Entity {
    card_upgraded: true,
    requires_target: false,
    card_effects: {
        let mut a = TRIP.card_effects;
        a[0].target = TARGET_MONSTERS_ALL; // Targets all monsters
        a
    },
    ..TRIP
};
