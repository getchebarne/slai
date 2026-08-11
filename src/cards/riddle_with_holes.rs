use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 3,
        lifesteal: false,
    },
    id_source: None,
    target: TARGET_MONSTER_PICKED,
};
const HIT_PLUS: Effect = Effect {
    kind: EffectKind::DamagePhysical {
        amount: 4,
        lifesteal: false,
    }, // +1 damage
    id_source: None,
    target: TARGET_MONSTER_PICKED,
};

pub static RIDDLE_WITH_HOLES: Entity = make_entity_card(
    CardName::RiddleWithHoles,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[HIT, HIT, HIT, HIT, HIT],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static RIDDLE_WITH_HOLES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = RIDDLE_WITH_HOLES.card_effects;
        a[0] = HIT_PLUS;
        a[1] = HIT_PLUS;
        a[2] = HIT_PLUS;
        a[3] = HIT_PLUS;
        a[4] = HIT_PLUS;
        a
    },
    ..RIDDLE_WITH_HOLES
};
