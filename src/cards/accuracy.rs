use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ACCURACY: Entity = make_entity_card(
    CardName::Accuracy,
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
            kind: ModifierKind::Accuracy,
            stacks: 4,
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
pub static ACCURACY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ACCURACY.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Accuracy,
            stacks: 6, // +2 stacks
        };
        a
    },
    ..ACCURACY
};
