use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static CRIPPLING_POISON: CardTemplate = make_card_template(
    CardName::CripplingPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Poison,
                stacks: 4,
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static CRIPPLING_POISON_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects_play: {
        let mut effects = CRIPPLING_POISON.effects_play;
        effects[0].kind = EffectKind::ModifierDelta {
            kind: ModifierKind::Poison,
            stacks: 7, // +3 poison
        };
        effects
    },
    ..CRIPPLING_POISON
};
