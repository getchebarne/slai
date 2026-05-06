use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

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
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static WELL_LAID_PLANS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = WELL_LAID_PLANS.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Retain,
            stacks: 2, // +1 stack
        };
        a
    },
    ..WELL_LAID_PLANS
};
