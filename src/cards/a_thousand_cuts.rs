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

pub static A_THOUSAND_CUTS: CardTemplate = make_card_template(
    CardName::AThousandCuts,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static A_THOUSAND_CUTS_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = A_THOUSAND_CUTS.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::ThousandCuts,
            stacks: 2, // +1 stack
        };
        effects
    },
    ..A_THOUSAND_CUTS
};
