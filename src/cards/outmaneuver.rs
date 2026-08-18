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

pub static OUTMANEUVER: CardTemplate = make_card_template(
    CardName::Outmaneuver,
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
            kind: ModifierKind::NextTurnEnergy,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static OUTMANEUVER_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = OUTMANEUVER.effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 3, // +1 next-turn-energy
        };
        effects
    },
    ..OUTMANEUVER
};
