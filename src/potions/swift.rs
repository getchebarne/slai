use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static SWIFT: PotionTemplate = PotionTemplate {
    name: PotionName::Swift,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::CardDraw { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    }],
};
