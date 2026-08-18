use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_ENTROPIC_BREW: PotionTemplate = PotionTemplate {
    name: PotionName::EntropicBrew,
    rarity: PotionRarity::Rare,
    combat_only: false,
    effects: &[Effect {
        kind: EffectKind::PotionAddRandom { limited: true },
        id_source: None,
        target: Target::Direct(None),
    }; POTION_SLOTS_MAX],
};
