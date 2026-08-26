use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub static ADRENALINE: CardTemplate = make_card_template(
    CardName::Adrenaline,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 1,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ADRENALINE_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = ADRENALINE.effects;
        effects[0].kind = EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        }; // +1 energy gain
        effects
    },
    ..ADRENALINE
};
