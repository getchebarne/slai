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
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static TRANSMUTATION: CardTemplate = make_card_template(
    CardName::Transmutation,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::XCost { offset: 0 },
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 1,
            cost_zero: Some(CostScope::Turn),
            upgraded: false,
            rarity: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static TRANSMUTATION_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = TRANSMUTATION.effects;
        effects[0].kind = EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 1,
            cost_zero: Some(CostScope::Turn),
            upgraded: true, // Added Cards are upgraded
            rarity: None,
        };
        effects
    },
    ..TRANSMUTATION
};
