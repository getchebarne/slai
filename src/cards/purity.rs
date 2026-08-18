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

pub static PURITY: CardTemplate = make_card_template(
    CardName::Purity,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardExhaust,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::InputUpTo { count: 3 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PURITY_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = PURITY.effects;
        effects[0].target = Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::InputUpTo { count: 5 }, // +2 Cards
        };
        effects
    },
    ..PURITY
};
