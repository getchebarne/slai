use crate::cards::make_entity_card;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static ENLIGHTENMENT: Entity = make_entity_card(
    CardName::Enlightenment,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::SetCostOverride {
            amount: 1,
            only_reduce: true,
            random: false,
            scope: CostScope::Turn,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::All,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ENLIGHTENMENT_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ENLIGHTENMENT.card_effects;
        a[0].kind = EffectKind::SetCostOverride {
            amount: 1,
            only_reduce: true,
            random: false,
            scope: CostScope::Combat, // Lasts the rest of combat
        };
        a
    },
    ..ENLIGHTENMENT
};
