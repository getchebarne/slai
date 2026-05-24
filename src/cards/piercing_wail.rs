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

pub static PIERCING_WAIL: Entity = make_entity_card(
    CardName::PiercingWail,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -6,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                selection_kind: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Shackled,
                stacks: 6,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                selection_kind: SelectionKind::All,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PIERCING_WAIL_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = PIERCING_WAIL.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: -8, // -2 strength
        };
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Shackled,
            stacks: 8, // +2 strength
        };
        a
    },
    ..PIERCING_WAIL
};
