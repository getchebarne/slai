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
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::utils::place_card;
use crate::utils::push_entity;

pub fn process_effect_card_add_random(
    state: &mut GameState,
    color: CardColor,
    kind: Option<CardKind>,
    pile: CardPile,
    count: u8,
    cost_zero: Option<CostScope>,
    upgraded: bool,
) {
    let pool: Vec<&Entity> = ALL_CARDS
        .iter()
        .filter(|c| c.card_color == color)
        .filter(|c| kind.is_none_or(|k| c.card_kind == k))
        .filter(|c| {
            matches!(
                c.card_rarity,
                CardRarity::Common | CardRarity::Uncommon | CardRarity::Rare
            )
        })
        .map(|c| &**c)
        .collect();

    for _ in 0..count {
        let name = pool[state.rng.random_range(0..pool.len())].card_name;
        let id_card = push_entity(&mut state.entities, get_card(name, upgraded));
        place_card(state, id_card, pile);
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
