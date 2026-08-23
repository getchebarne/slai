use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static CUNNING: PotionTemplate = PotionTemplate {
    name: PotionName::Cunning,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 3,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
