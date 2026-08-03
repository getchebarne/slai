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

pub static FLECHETTES: Entity = make_entity_card(
    CardName::Flechettes,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamageFlechettes { damage: 4 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLECHETTES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = FLECHETTES.card_effects;
        a[0].kind = EffectKind::DamageFlechettes { damage: 6 }; // +2 damage
        a
    },
    ..FLECHETTES
};
