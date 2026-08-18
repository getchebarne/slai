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

pub static TACTICIAN: CardTemplate = make_card_template(
    CardName::Tactician,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[],
    &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    PlayRestriction::Never,
);
pub static TACTICIAN_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    on_discard_effects: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        }, // +1 energy
        id_source: None,
        target: Target::Direct(None),
    }],
    ..TACTICIAN
};
