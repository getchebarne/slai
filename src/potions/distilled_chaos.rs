use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static DISTILLED_CHAOS: PotionTemplate = PotionTemplate {
    name: PotionName::DistilledChaos,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::CardPlayFromDrawTop,
        id_source: None,
        target: Target::Direct(None),
    }; 3],
};
// Doubled
pub static DISTILLED_CHAOS_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::CardPlayFromDrawTop,
        id_source: None,
        target: Target::Direct(None),
    }; 6],
    ..DISTILLED_CHAOS
};
