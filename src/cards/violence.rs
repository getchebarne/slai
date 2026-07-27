use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
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
use crate::types::CardPile;
use crate::types::CardRarity;

pub static VIOLENCE: Entity = make_entity_card(
    CardName::Violence,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardMove {
            pile: CardPile::Hand,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::PileDraw {
                filter: CandidatePoolCardFilter::Attack,
            },
            selection_kind: SelectionKind::Random { count: 3 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static VIOLENCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = VIOLENCE.card_effects;
        a[0].target = Target::Resolve {
            candidate_pool: CandidatePool::PileDraw {
                filter: CandidatePoolCardFilter::Attack,
            },
            selection_kind: SelectionKind::Random { count: 4 }, // +1 card
        };
        a
    },
    ..VIOLENCE
};
