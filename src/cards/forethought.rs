use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::consts::MAX_SIZE_HAND;
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

pub static FORETHOUGHT: CardTemplate = make_card_template(
    CardName::Forethought,
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
        kind: EffectKind::CardSetupPick {
            free: true,
            bottom: true,
        },
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
pub static FORETHOUGHT_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = FORETHOUGHT.effects;
        effects[0].target = Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::InputUpTo {
                count: MAX_SIZE_HAND as u16,
            },
        }; // Any number of Cards
        effects
    },
    ..FORETHOUGHT
};
