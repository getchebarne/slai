use crate::game::GameState;

pub fn process_effect_gold_steal(id_source: Option<usize>, state: &mut GameState, amount: u8) {
    let id_source = id_source.expect("GoldSteal requires id_source");
    let id_character = state.id_character;
    let take = (amount as u16).min(state.entities[id_character].character_gold);
    if take == 0 {
        return;
    }
    state.entities[id_character].character_gold -= take;
    state.entities[id_source].monster_gold_stolen = state.entities[id_source]
        .monster_gold_stolen
        .saturating_add(take);
}
