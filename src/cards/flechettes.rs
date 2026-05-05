use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static FLECHETTES: Entity = make_entity_card(
    CardName::Flechettes,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::FlechettesDamage { damage: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::CardTarget,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLECHETTES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = FLECHETTES.card_effects;
        a[0].kind = EffectKind::FlechettesDamage { damage: 6 }; // +2 damage
        a
    },
    ..FLECHETTES
};
