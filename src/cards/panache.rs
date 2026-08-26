use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static PANACHE: CardTemplate = make_card_template(
    CardName::Panache,
    CardKind::Power,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Panache,
            stacks: 10,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PANACHE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = PANACHE.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Panache,
            stacks: 14, // +4 damage
        };
        effects
    },
    ..PANACHE
};
