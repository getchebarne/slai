use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static CALTROPS: Entity = make_entity_card(
    CardName::Caltrops,
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
            kind: ModifierKind::Thorns,
            stacks: 3,
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
);
// Upgraded
pub static CALTROPS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = CALTROPS.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Thorns,
            stacks: 5, // +2 stacks
        };
        a
    },
    ..CALTROPS
};
