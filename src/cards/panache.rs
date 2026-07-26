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

pub static PANACHE: Entity = make_entity_card(
    CardName::Panache,
    CardKind::Power,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Panache,
            stacks: 10,
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
);
// Upgraded
pub static PANACHE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = PANACHE.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Panache,
            stacks: 14, // +4 damage
        };
        a
    },
    ..PANACHE
};
