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

pub static PANIC_BUTTON: Entity = make_entity_card(
    CardName::PanicButton,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 30 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NoBlock,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PANIC_BUTTON_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = PANIC_BUTTON.card_effects;
        a[0].kind = EffectKind::BlockGain { amount: 40 }; // +10 block
        a
    },
    ..PANIC_BUTTON
};
