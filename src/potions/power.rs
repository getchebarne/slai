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

pub static POWER: PotionTemplate = PotionTemplate {
    name: PotionName::Power,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: Some(CardKind::Power),
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
pub static POWER_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: Some(CardKind::Power),
                color: CardColor::Green,
                exclude: &[],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand, 2, true),
    ],
    ..POWER
};
