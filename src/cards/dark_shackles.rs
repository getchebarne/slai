use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
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
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -9,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Shackled,
                stacks: 9,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DARK_SHACKLES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DARK_SHACKLES.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: -15, // -6 strength
        };
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Shackled,
            stacks: 15, // +6 strength
        };
        a
    },
    ..DARK_SHACKLES
};
