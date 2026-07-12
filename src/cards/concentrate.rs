use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
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

pub static CONCENTRATE: Entity = make_entity_card(
    CardName::Concentrate,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                selection_kind: SelectionKind::Input { count: 3 },
            },
        },
        Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    "Discard 3 cards. Gain 2 Energy.",
);
// Upgraded
pub static CONCENTRATE_PLUS: Entity = Entity {
    card_upgraded: true,
    description: "Discard 2 cards. Gain 2 Energy.",
    card_effects: {
        let mut a = CONCENTRATE.card_effects;
        a[0].target = Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            selection_kind: SelectionKind::Input { count: 2 }, // -1 discard
        };
        a
    },
    ..CONCENTRATE
};
