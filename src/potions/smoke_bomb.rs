use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SMOKE_BOMB: PotionTemplate = PotionTemplate {
    name: PotionName::SmokeBomb,
    rarity: PotionRarity::Rare,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::CombatEnd {
            escaped_character: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
