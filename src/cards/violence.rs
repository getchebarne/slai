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
use crate::types::CardPile;
use crate::types::CardRarity;

pub static VIOLENCE: CardTemplate = make_card_template(
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
    &[Effect {
        kind: EffectKind::CardMove {
            pile: CardPile::Hand,
            cost_zero: None,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::PileDraw,
            filter: CandidateFilter::KindAttack,
            selection_kind: SelectionKind::Random { count: 3 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static VIOLENCE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = VIOLENCE.effects;
        effects[0].target = Target::Resolve {
            candidate_pool: CandidatePool::PileDraw,
            filter: CandidateFilter::KindAttack,
            selection_kind: SelectionKind::Random { count: 4 }, // +1 Card
        };
        effects
    },
    ..VIOLENCE
};
