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

pub static SKEWER: Entity = make_entity_card(
    CardName::Skewer,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    false,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 7 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::MonsterPicked,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SKEWER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = SKEWER.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 10 }; // +3 damage
        a
    },
    ..SKEWER
};
