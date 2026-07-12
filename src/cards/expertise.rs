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

pub static EXPERTISE: Entity = make_entity_card(
    CardName::Expertise,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardDrawUpTo { amount: 6 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
    "Draw cards until you have {magic} in your hand.",
);
// Upgraded
pub static EXPERTISE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = EXPERTISE.card_effects;
        a[0].kind = EffectKind::CardDrawUpTo { amount: 7 }; // +1 draw
        a
    },
    ..EXPERTISE
};
