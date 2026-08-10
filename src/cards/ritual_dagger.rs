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

// On fatal, permanently gains damage (+3, +5 upgraded); see `process_effect_damage_deal.rs`
pub static RITUAL_DAGGER: Entity = make_entity_card(
    CardName::RitualDagger,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Special,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 15 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: only the on-kill bump grows (3 -> 5); base damage is unchanged
pub static RITUAL_DAGGER_PLUS: Entity = Entity {
    card_upgraded: true,
    ..RITUAL_DAGGER
};
