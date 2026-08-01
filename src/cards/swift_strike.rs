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
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static SWIFT_STRIKE: Entity = make_entity_card(
    CardName::SwiftStrike,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 7 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters,
            filter: CandidateFilter::Picked,
            selection_kind: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SWIFT_STRIKE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SWIFT_STRIKE.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 10 }; // +3 damage
        a
    },
    ..SWIFT_STRIKE
};
