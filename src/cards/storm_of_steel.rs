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

pub static STORM_OF_STEEL: CardTemplate = make_card_template(
    CardName::StormOfSteel,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::StormOfSteelProc { upgraded: false },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static STORM_OF_STEEL_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = STORM_OF_STEEL.effects;
        effects[0].kind = EffectKind::StormOfSteelProc { upgraded: true }; // Shivs are upgraded
        effects
    },
    ..STORM_OF_STEEL
};
