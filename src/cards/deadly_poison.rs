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

pub static DEADLY_POISON: Entity = make_entity_card(
    CardName::DeadlyPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 5,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DEADLY_POISON_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = DEADLY_POISON.card_effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 7, // +2 poison
        };
        effects
    },
    ..DEADLY_POISON
};
