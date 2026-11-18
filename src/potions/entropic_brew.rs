use crate::consts::MAX_POTION_SLOTS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static ENTROPIC_BREW: PotionTemplate = PotionTemplate {
    name: PotionName::EntropicBrew,
    rarity: PotionRarity::Rare,
    combat_only: false,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::PotionAddRandom {
            limited: true,
            uniform: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }; MAX_POTION_SLOTS],
};
