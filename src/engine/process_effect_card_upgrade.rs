use crate::cards::get_card;
use crate::game::GameState;

pub fn process_effect_card_upgrade(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardUpgrade requires id_target");
    let card = &state.entities[id_target];
    if card.card_upgraded {
        return;
    }

    let name = card.card_name;
    let plus = get_card(name, true);
    // Runtime-field preserve list: engine-mutated card state must survive the
    // def swap. Combat-modified cost is kept unless the upgrade itself changes
    // base cost (StS upgradeBaseCost)
    let cost = if plus.card_cost == get_card(name, false).card_cost {
        card.card_cost
    } else {
        plus.card_cost
    };
    let over = card.card_cost_override;
    state.entities[id_target] = plus;
    state.entities[id_target].card_cost = cost;
    state.entities[id_target].card_cost_override = over;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::CostOverride;
    use crate::game::create_game_state;
    use crate::types::CardName;
    use crate::types::CostScope;
    use crate::utils::push_entity;

    #[test]
    fn upgrade_preserves_runtime_cost_state() {
        let mut state = create_game_state(0, 7, false);
        let id = push_entity(&mut state.entities, get_card(CardName::Strike, false));
        state.entities[id].card_cost = 0; // Combat-scope zero
        state.entities[id].card_cost_override = Some(CostOverride {
            amount: 0,
            scope: CostScope::Turn,
        });
        process_effect_card_upgrade(Some(id), &mut state);
        let card = &state.entities[id];
        assert!(card.card_upgraded);
        assert_eq!(card.card_cost, 0);
        assert_eq!(
            card.card_cost_override,
            Some(CostOverride {
                amount: 0,
                scope: CostScope::Turn,
            })
        );
    }

    #[test]
    fn upgrade_that_changes_base_cost_wins_over_combat_modification() {
        let mut state = create_game_state(0, 7, false);
        let id = push_entity(&mut state.entities, get_card(CardName::Madness, false));
        state.entities[id].card_cost = 3;
        process_effect_card_upgrade(Some(id), &mut state);
        assert_eq!(state.entities[id].card_cost, 0); // Madness+ base cost
    }
}
