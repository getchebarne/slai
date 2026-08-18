use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static DEADLY_POISON: CardTemplate = make_card_template(
    CardName::DeadlyPoison,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 5,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DEADLY_POISON_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = DEADLY_POISON.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 7, // +2 poison
        };
        effects
    },
    ..DEADLY_POISON
};
