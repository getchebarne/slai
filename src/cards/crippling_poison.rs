use crate::effect::CandidateFilter;
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

pub static CRIPPLING_POISON: Entity = make_entity_card(
    CardName::CripplingPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 4,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CRIPPLING_POISON_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = CRIPPLING_POISON.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 7, // +3 poison
        };
        a
    },
    ..CRIPPLING_POISON
};
