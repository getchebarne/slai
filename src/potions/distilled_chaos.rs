use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_DISTILLED_CHAOS: PotionTemplate = PotionTemplate {
    name: PotionName::DistilledChaos,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::CardPlayFromDrawTop,
        id_source: None,
        target: Target::Direct(None),
    }; 3],
};
