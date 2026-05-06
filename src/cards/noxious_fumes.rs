use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity, Tag};

pub static NOXIOUS_FUMES: Entity = make_entity_card(
    CardName::NoxiousFumes,
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
            kind: ModifierKind::NoxiousFumes,
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
    &[Tag::Poison],
);
// Upgraded
pub static NOXIOUS_FUMES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = NOXIOUS_FUMES.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NoxiousFumes,
            stacks: 3, // +1 poison
        };
        a
    },
    ..NOXIOUS_FUMES
};
