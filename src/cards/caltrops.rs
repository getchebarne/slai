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

pub static CALTROPS: Entity = make_entity_card(
    CardName::Caltrops,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Thorns,
            stacks: 3,
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
    "Whenever you are attacked, deal 3 damage back.",
);
// Upgraded
pub static CALTROPS_PLUS: Entity = Entity {
    card_upgraded: true,
    description: "Whenever you are attacked, deal 5 damage back.",
    card_effects: {
        let mut a = CALTROPS.card_effects;
        a[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Thorns,
            stacks: 5, // +2 stacks
        };
        a
    },
    ..CALTROPS
};
