use crate::cards::make_entity_card;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
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

pub static PREPARED: Entity = make_entity_card(
    CardName::Prepared,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PREPARED_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = PREPARED.card_effects;
        effects[0].kind = EffectKind::CardDraw { count: 2 }; // +1 Card
        effects[1].target = Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 2 }, // +1 Card
        };
        effects
    },
    ..PREPARED
};
