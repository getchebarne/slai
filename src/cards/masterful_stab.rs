use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// MasterfulStab: 0-cost attack, 12 (16+) damage, target enemy.
// Cost grows by 1 per damage event the character takes this combat.
// Per StS `tookDamage()` — modeled here as `CardCostKind::GrowsOnDamageInstanceTaken`
// reading `state.instances_of_damage_taken_this_combat`.
pub static MASTERFUL_STAB: Entity = {
    let mut e = make_entity_card(
        CardName::MasterfulStab,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        false,
        false,
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
    );
    e.card_cost_kind = CardCostKind::GrowsOnDamageInstanceTaken;
    e
};
// Upgraded: damage 12 -> 16
pub static MASTERFUL_STAB_PLUS: Entity = {
    let mut e = make_entity_card(
        CardName::MasterfulStab,
        CardKind::Attack,
        CardColor::Green,
        CardRarity::Uncommon,
        0,
        true,
        false,
        false,
        true,
        &[Effect {
            kind: EffectKind::DamagePhysical { amount: 16 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::All,
            },
        }],
    );
    e.card_cost_kind = CardCostKind::GrowsOnDamageInstanceTaken;
    e
};
