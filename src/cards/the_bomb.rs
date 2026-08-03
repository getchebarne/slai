use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static THE_BOMB: Entity = make_entity_card(
    CardName::TheBomb,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::TheBomb,
            stacks: 40,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static THE_BOMB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = THE_BOMB.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::TheBomb,
            stacks: 50, // +10 stacks
        };
        a
    },
    ..THE_BOMB
};
