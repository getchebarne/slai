use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::EFFECT_CARD_DISCOVER_PICK;
use crate::potions::PotionTemplate;
use crate::types::CardColor;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static COLORLESS: PotionTemplate = PotionTemplate {
    name: PotionName::Colorless,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Colorless,
                exclude: &[],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_CARD_DISCOVER_PICK,
    ],
};
