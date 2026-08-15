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

pub static DARK_SHACKLES: Entity = make_entity_card(
    CardName::DarkShackles,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::StrengthLoseTemp { stacks: 9 },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DARK_SHACKLES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = DARK_SHACKLES.card_effects;
        effects[0].kind = EffectKind::StrengthLoseTemp { stacks: 15 };
        effects
    },
    ..DARK_SHACKLES
};
