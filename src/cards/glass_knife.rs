use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static GLASS_KNIFE: Entity = make_entity_card(
    CardName::GlassKnife,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::GlassKnifeDecay { delta: -2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static GLASS_KNIFE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = GLASS_KNIFE.card_effects;
        let upgraded_kind = EffectKind::DamagePhysical { amount: 12 }; // +4 damage
        a[0].kind = upgraded_kind;
        a[1].kind = upgraded_kind;
        a
    },
    ..GLASS_KNIFE
};
