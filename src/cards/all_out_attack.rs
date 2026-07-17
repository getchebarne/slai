use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::DiscardSource;
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

pub static ALL_OUT_ATTACK: Entity = make_entity_card(
    CardName::AllOutAttack,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::All,
                },
                selection_kind: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand {
                    filter: CandidatePoolCardFilter::Any,
                },
                selection_kind: SelectionKind::Random { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ALL_OUT_ATTACK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ALL_OUT_ATTACK.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 14 }; // +4 damage
        a
    },
    ..ALL_OUT_ATTACK
};
