use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static BACKSTAB: Entity = make_entity_card(
    CardName::Backstab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 11 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::Picked,
            },
            selection_kind: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    "Innate. Deal {damage} damage. Exhaust.",
);
// Upgraded
pub static BACKSTAB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = BACKSTAB.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 15 }; // +4 damage
        a
    },
    ..BACKSTAB
};
