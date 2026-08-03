use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;

pub static JACK_OF_ALL_TRADES: Entity = make_entity_card(
    CardName::JackOfAllTrades,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 1,
            cost_zero: None,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static JACK_OF_ALL_TRADES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = JACK_OF_ALL_TRADES.card_effects;
        a[0].kind = EffectKind::CardAddRandom {
            color: CardColor::Colorless,
            kind: None,
            pile: CardPile::Hand,
            count: 2, // +1 Card
            cost_zero: None,
            upgraded: false,
        };
        a
    },
    ..JACK_OF_ALL_TRADES
};
