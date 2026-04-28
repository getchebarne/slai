use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// Glass Knife: deal 8 dmg twice, then decrement this card instance's
// DamagePhysical amounts by 2 (saturating at 0). The two hits and the
// decrement all read/mutate the same per-instance card_effects array,
// so subsequent plays see the reduced damage.
pub static GLASS_KNIFE: Entity = make_entity_card(
    CardName::GlassKnife,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
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
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::GlassKnifeDecay { delta: -2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    PlayRestriction::Always,
);
// Upgraded: base 8 -> 12 (+4 per hit). Decay rate unchanged.
pub static GLASS_KNIFE_PLUS: Entity = make_entity_card(
    CardName::GlassKnife,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    true,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::GlassKnifeDecay { delta: -2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    PlayRestriction::Always,
);
