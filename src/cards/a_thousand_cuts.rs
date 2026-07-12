use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static A_THOUSAND_CUTS: Entity = make_entity_card(
    CardName::AThousandCuts,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    "Whenever you play a card, deal 1 damage to ALL enemies.",
);
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: Entity = Entity {
    card_upgraded: true,
    description: "Whenever you play a card, deal 2 damage to ALL enemies.",
    card_effects: {
        let mut a = A_THOUSAND_CUTS.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 2, // +1 stack
        };
        a
    },
    ..A_THOUSAND_CUTS
};
