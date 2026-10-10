use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static GAMBLERS_BREW: PotionTemplate = PotionTemplate {
    name: PotionName::GamblersBrew,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::Gamble {
            choose_discards: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
