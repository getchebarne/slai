use crate::cards::get_card;
use crate::game::GameState;

pub fn process_effect_card_upgrade(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardUpgrade requires id_target");
    let card = &state.entities[id_target];

    // Early return if already upgraded (e.g., Apotheosis)
    if card.card_upgraded {
        return;
    }

    // Get upgraded variant
    let name = card.card_name;
    let card_upgraded = get_card(name, true);

    // Snapshot runtime-preserved fields: cost, cost override, bottled status
    let cost = if card_upgraded.card_cost == get_card(name, false).card_cost {
        card.card_cost
    } else {
        card_upgraded.card_cost
    };
    let cost_override = card.card_cost_override;
    let bottled = card.card_bottled;

    // Overwrite non-upgraded variant with upgraded one
    state.entities[id_target] = card_upgraded;

    // Stamp preserved fields
    state.entities[id_target].card_cost = cost;
    state.entities[id_target].card_cost_override = cost_override;
    state.entities[id_target].card_bottled = bottled;
}
