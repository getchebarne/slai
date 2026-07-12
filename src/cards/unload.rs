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

pub static UNLOAD: Entity = make_entity_card(
    CardName::Unload,
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
            kind: EffectKind::DamagePhysical { amount: 14 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::Picked,
                },
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::UnloadDiscard,
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    "Deal {damage} damage. Discard all non-Attack cards in your hand.",
);
// Upgraded
pub static UNLOAD_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = UNLOAD.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 18 }; // +4 damage
        a
    },
    ..UNLOAD
};
