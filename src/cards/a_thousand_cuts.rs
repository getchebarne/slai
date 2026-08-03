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

pub static A_THOUSAND_CUTS: Entity = make_entity_card(
    CardName::AThousandCuts,
    CardKind::Power,
    CardColor::Green,
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
            kind: ModifierKind::ThousandCuts,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = A_THOUSAND_CUTS.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 2, // +1 stack
        };
        a
    },
    ..A_THOUSAND_CUTS
};
