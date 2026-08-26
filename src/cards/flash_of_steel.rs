use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FLASH_OF_STEEL: CardTemplate = make_card_template(
    CardName::FlashOfSteel,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 3,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FLASH_OF_STEEL_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = FLASH_OF_STEEL.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 6,
            lifesteal: false,
        }; // +3 damage
        effects
    },
    ..FLASH_OF_STEEL
};
