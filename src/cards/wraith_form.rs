use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static WRAITH_FORM: Entity = make_entity_card(
    CardName::WraithForm,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Intangible,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::WraithForm,
                stacks: 1,
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
pub static WRAITH_FORM_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = WRAITH_FORM.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Intangible,
            stacks: 3, // +1 stack
        };
        a
    },
    ..WRAITH_FORM
};
