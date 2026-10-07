use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::cards::get_card_template;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::utils::card_name_healing;
use crate::utils::place_card;
use crate::utils::push_entity;
use rand::Rng;

// Random Silent Skill into hand, free-to-play-once
pub fn process_effect_distraction_add(state: &mut GameState) {
    let mut buf = [CardName::Strike; 64];
    let mut num = 0;
    for pool in [
        POOL_COMMON_GREEN_CARD,
        POOL_UNCOMMON_GREEN_CARD,
        POOL_RARE_GREEN_CARD,
    ] {
        for &name in pool {
            if card_name_healing(name) {
                continue;
            }
            if get_card_template(name, false).kind != CardKind::Skill {
                continue;
            }
            buf[num] = name;
            num += 1;
        }
    }
    if num == 0 {
        return;
    }

    let card_name = buf[state.rng.random_range(0..num)];
    let id_card = push_entity(&mut state.entities, get_card(card_name, false));
    let placed = place_card(state, id_card, CardPile::Hand);

    // Costs 0 this turn
    state.effect_queue.push_front(Effect {
        kind: EffectKind::SetCostOverride {
            amount: 0,
            only_reduce: false,
            random: false,
            scope: CostScope::Turn,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });

    // Accuracy: a Card reaching the hand resets every Shiv in it
    if placed
        && has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Accuracy,
        )
    {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
