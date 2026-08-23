use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::EFFECT_CARD_DISCOVER_PICK;
use crate::potions::PotionTemplate;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static SKILL: PotionTemplate = PotionTemplate {
    name: PotionName::Skill,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: Some(CardKind::Skill),
                color: CardColor::Green,
                exclude: &[],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_CARD_DISCOVER_PICK,
    ],
};
