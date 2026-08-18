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

pub static MASTER_OF_STRATEGY: CardTemplate = make_card_template(
    CardName::MasterOfStrategy,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardDraw { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MASTER_OF_STRATEGY_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = MASTER_OF_STRATEGY.effects;
        effects[0].kind = EffectKind::CardDraw { count: 4 }; // +1 draw
        effects
    },
    ..MASTER_OF_STRATEGY
};
