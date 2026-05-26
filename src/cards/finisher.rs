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

pub static FINISHER: Entity = make_entity_card(
    CardName::Finisher,
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
        kind: EffectKind::FinisherDamage { damage: 6 },
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
);
// Upgraded
pub static FINISHER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = FINISHER.card_effects;
        a[0].kind = EffectKind::FinisherDamage { damage: 8 }; // +2 damage
        a
    },
    ..FINISHER
};
