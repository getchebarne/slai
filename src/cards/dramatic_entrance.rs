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

pub static DRAMATIC_ENTRANCE: Entity = make_entity_card(
    CardName::DramaticEntrance,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    true,
    false,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 8 },
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
);
// Upgraded
pub static DRAMATIC_ENTRANCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DRAMATIC_ENTRANCE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 12 }; // +4 damage
        a
    },
    ..DRAMATIC_ENTRANCE
};
