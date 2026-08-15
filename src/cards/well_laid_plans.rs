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

pub static WELL_LAID_PLANS: Entity = make_entity_card(
    CardName::WellLaidPlans,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
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
pub static WELL_LAID_PLANS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = WELL_LAID_PLANS.card_effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
            stacks: 2, // +1 stack
        };
        effects
    },
    ..WELL_LAID_PLANS
};
