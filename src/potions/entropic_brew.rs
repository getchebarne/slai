use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

// PotionAddRandom no-ops on a full belt, so slots-max copies fill exactly the free slots
static ADD_RANDOM: Effect = Effect {
    kind: EffectKind::PotionAddRandom { limited: true },
    id_source: None,
    target: Target::Direct(None),
};
pub static ENTROPIC_BREW: Entity = make_entity_potion(
    PotionName::EntropicBrew,
    PotionRarity::Rare,
    false,
    false,
    &[ADD_RANDOM; POTION_SLOTS_MAX],
);
