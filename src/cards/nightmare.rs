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

pub static NIGHTMARE: CardTemplate = make_card_template(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static NIGHTMARE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    cost: 2, // -1 cost
    ..NIGHTMARE
};
