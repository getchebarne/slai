use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_pick;
use crate::potions::PotionTemplate;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static SKILL: PotionTemplate = PotionTemplate {
    name: PotionName::Skill,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
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
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand, 1, true),
    ],
};
// Doubled
pub static SKILL_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
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
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand, 2, true),
    ],
    ..SKILL
};
