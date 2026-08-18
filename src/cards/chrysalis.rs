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

pub static CHRYSALIS: CardTemplate = make_card_template(
    CardName::Chrysalis,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Green,
            kind: Some(CardKind::Skill),
            pile: CardPile::Draw,
            count: 3,
            cost_zero: Some(CostScope::Combat),
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
pub static CHRYSALIS_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = CHRYSALIS.effects;
        effects[0].kind = EffectKind::CardAddRandom {
            color: CardColor::Green,
            kind: Some(CardKind::Skill),
            pile: CardPile::Draw,
            count: 5, // +2 Cards
            cost_zero: Some(CostScope::Combat),
            upgraded: false,
            rarity: None,
        };
        effects
    },
    ..CHRYSALIS
};
