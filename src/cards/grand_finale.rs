use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static GRAND_FINALE: Entity = make_entity_card(
    CardName::GrandFinale,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 50 },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::DrawPileEmpty,
);
// Upgraded
pub static GRAND_FINALE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = GRAND_FINALE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 60 }; // +10 damage
        a
    },
    ..GRAND_FINALE
};
