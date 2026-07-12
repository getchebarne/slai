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

pub static DIE_DIE_DIE: Entity = make_entity_card(
    CardName::DieDieDie,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 13 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::All,
            },
            selection_kind: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    "Deal {damage} damage to ALL enemies. Exhaust.",
);
// Upgraded
pub static DIE_DIE_DIE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DIE_DIE_DIE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 17 }; // +4 damage
        a
    },
    ..DIE_DIE_DIE
};
