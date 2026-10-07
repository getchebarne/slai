use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::potions::PotionTemplate;
use crate::types::DeltaSign;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Never drinkable; the death hook in `process_effect_death` revives with this heal
pub static FAIRY: PotionTemplate = PotionTemplate {
    name: PotionName::Fairy,
    rarity: PotionRarity::Rare,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::RelativeMinOne {
                numerator: 30,
                denominator: 100,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static FAIRY_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::RelativeMinOne {
                numerator: 60,
                denominator: 100,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..FAIRY
};
