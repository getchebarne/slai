use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static BULLET_TIME: CardTemplate = make_card_template(
    CardName::BulletTime,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
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
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NoDraw,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static BULLET_TIME_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    cost: 2, // -1 cost
    ..BULLET_TIME
};
