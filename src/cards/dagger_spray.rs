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

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 4 },
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::All,
    },
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 6 }, // +2 damage
    id_source: None,
    target: Target::Resolve {
        candidates: CandidatePool::Monsters,
        selection: SelectionKind::All,
    },
};

pub static DAGGER_SPRAY: Entity = make_entity_card(
    CardName::DaggerSpray,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DAGGER_SPRAY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = DAGGER_SPRAY.card_effects;
        a[0] = HIT_PLUS;
        a[1] = HIT_PLUS;
        a
    },
    ..DAGGER_SPRAY
};
