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

pub static REFLEX: CardTemplate = make_card_template(
    CardName::Reflex,
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
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    PlayRestriction::Never,
);
// Upgraded
pub static REFLEX_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    on_discard_effects: &[Effect {
        kind: EffectKind::CardDraw { count: 3 }, // +1 draw
        id_source: None,
        target: Target::Direct(None),
    }],
    ..REFLEX
};
