// Relic registry and accessors

mod snake_ring;

use crate::entity::Entity;
use crate::game::GameState;
use crate::types::RelicName;

pub fn get_relic(name: RelicName) -> Entity {
    match name {
        RelicName::SnakeRing => snake_ring::SNAKE_RING,
    }
}

pub fn has_relic(state: &GameState, name: RelicName) -> bool {
    state.id_relics[..state.relic_count as usize]
        .iter()
        .any(|&id| state.entities[id].relic_name == name)
}

pub fn relic_counter(state: &GameState, name: RelicName) -> Option<i16> {
    state.id_relics[..state.relic_count as usize]
        .iter()
        .find(|&&id| state.entities[id].relic_name == name)
        .map(|&id| state.entities[id].relic_counter)
}

pub fn relic_counter_mut<'a>(
    state: &'a mut GameState,
    name: RelicName,
) -> Option<&'a mut i16> {
    let n = state.relic_count as usize;
    for i in 0..n {
        let id = state.id_relics[i];
        if state.entities[id].relic_name == name {
            return Some(&mut state.entities[id].relic_counter);
        }
    }
    None
}
