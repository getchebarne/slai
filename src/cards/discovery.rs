use crate::effect::CandidatePool;
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
use crate::types::CostScope;

pub static DISCOVERY: Entity = make_entity_card(
    CardName::Discovery,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Green,
                exclude: &[],
                count: 3,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscoverPick {
                cost_zero: Some(CostScope::Turn),
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Discover,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DISCOVERY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_exhaust: false, // Doesn't exhaust
    ..DISCOVERY
};
