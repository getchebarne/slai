use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ESCAPE_PLAN: CardTemplate = make_card_template(
    CardName::EscapePlan,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::EscapePlanCheck { block: 3 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ESCAPE_PLAN_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = ESCAPE_PLAN.effects;
        effects[1].kind = EffectKind::EscapePlanCheck { block: 5 }; // +2 block
        effects
    },
    ..ESCAPE_PLAN
};
