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

pub static FINISHER: Entity = make_entity_card(
    CardName::Finisher,
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
        kind: EffectKind::DamageFinisher { damage: 6 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FINISHER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = FINISHER.card_effects;
        effects[0].kind = EffectKind::DamageFinisher { damage: 8 }; // +2 damage
        effects
    },
    ..FINISHER
};
