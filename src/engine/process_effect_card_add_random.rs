use rand::Rng;

use crate::cards::ALL_CARDS;
use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::utils::place_card;
use crate::utils::push_entity;

#[allow(clippy::too_many_arguments)]
pub fn process_effect_card_add_random(
    state: &mut GameState,
    color: CardColor,
    kind: Option<CardKind>,
    pile: CardPile,
    count: u8,
    cost_zero: Option<CostScope>,
    upgraded: bool,
    rarity: Option<CardRarity>,
) {
    let pool: Vec<&Entity> = ALL_CARDS
        .iter()
        .filter(|c| c.card_color == color)
        .filter(|c| kind.is_none_or(|k| c.card_kind == k))
        .filter(|c| {
            rarity.map_or(
                matches!(
                    c.card_rarity,
                    CardRarity::Common | CardRarity::Uncommon | CardRarity::Rare
                ),
                |r| c.card_rarity == r,
            )
        })
        .filter(|c| {
            !matches!(
                c.card_name,
                CardName::AscendersBane | CardName::CurseOfTheBell
            )
        })
        .map(|c| &**c)
        .collect();

    for _ in 0..count {
        let name = pool[state.rng.random_range(0..pool.len())].card_name;
        let id_card = push_entity(&mut state.entities, get_card(name, upgraded));

        // Deck additions route through the obtain hook
        if pile == CardPile::Deck {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAddToDeck,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        } else {
            place_card(state, id_card, pile);
        }

        if let Some(scope) = cost_zero {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::SetCostOverride {
                    amount: 0,
                    only_reduce: false,
                    random: false,
                    scope,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }
}
