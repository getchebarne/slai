use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FOOTWORK: Entity = make_entity_card(
    CardName::Footwork,
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
            kind: ModifierKind::Dexterity,
            stacks: 2,
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
pub static FOOTWORK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = FOOTWORK.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 3, // +1 dexterity
        };
        a
    },
    ..FOOTWORK
};
