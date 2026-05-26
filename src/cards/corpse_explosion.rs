use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
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

pub static CORPSE_EXPLOSION: Entity = make_entity_card(
    CardName::CorpseExplosion,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 6,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::Picked,
                },
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::CorpseExplosion,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::Picked,
                },
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CORPSE_EXPLOSION_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = CORPSE_EXPLOSION.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 9, // +3 poison
        };
        a
    },
    ..CORPSE_EXPLOSION
};
