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

pub static PIERCING_WAIL: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::StrengthLoseTemp { stacks: 6 },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PIERCING_WAIL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = PIERCING_WAIL.card_effects;
        a[0].kind = EffectKind::StrengthLoseTemp { stacks: 8 };
        a
    },
    ..PIERCING_WAIL
};
