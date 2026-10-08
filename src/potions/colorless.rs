use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_pick;
use crate::potions::PotionTemplate;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static COLORLESS: PotionTemplate = PotionTemplate {
    name: PotionName::Colorless,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
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
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand, 1, false),
    ],
};
// Doubled
pub static COLORLESS_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
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
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand, 2, false),
    ],
    ..COLORLESS
};
