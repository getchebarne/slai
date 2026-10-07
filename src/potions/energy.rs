use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::DeltaSign;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static ENERGY: PotionTemplate = PotionTemplate {
    name: PotionName::Energy,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
// Doubled
pub static ENERGY_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 4,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    ..ENERGY
};
