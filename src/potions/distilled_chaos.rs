use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

static PLAY_TOP: Effect = Effect {
    kind: EffectKind::CardPlayFromDrawTop,
    id_source: None,
    target: Target::Direct(None),
};
pub static DISTILLED_CHAOS: Entity = make_entity_potion(
    PotionName::DistilledChaos,
    PotionRarity::Uncommon,
    false,
    true,
    &[PLAY_TOP; 3],
);
