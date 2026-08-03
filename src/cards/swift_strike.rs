use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SWIFT_STRIKE: Entity = make_entity_card(
    CardName::SwiftStrike,
    CardKind::Attack,
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
        kind: EffectKind::DamagePhysical { amount: 7 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SWIFT_STRIKE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SWIFT_STRIKE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 10 }; // +3 damage
        a
    },
    ..SWIFT_STRIKE
};
