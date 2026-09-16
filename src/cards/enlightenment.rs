use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static ENLIGHTENMENT: CardTemplate = make_card_template(
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
// Upgraded: the turn cut still lands, plus an independent cut to the printed cost
pub static ENLIGHTENMENT_PLUS: CardTemplate = make_card_template(
    CardName::Enlightenment,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    true,
    false,
    false,
    false,
    &[
        Effect {
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
        },
        Effect {
            kind: EffectKind::SetCostOverride {
                amount: 1,
                only_reduce: true,
                random: false,
                scope: CostScope::Combat,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
