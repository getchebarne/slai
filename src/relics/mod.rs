// Relic registry and accessors. Storage is a u128 bitmask paired with a
// name-indexed `[usize; N_RELICS]` table on GameState; per-relic state lives
// on the Entity (counter, used_up, effects-on-combat-start)

mod akabeko;
mod anchor;
mod bag_of_marbles;
mod bag_of_preparation;
mod blood_vial;
mod bronze_scales;
mod kunai;
mod ninja_scroll;
mod oddly_smooth_stone;
mod shuriken;
mod snake_ring;
mod thread_and_needle;
mod twisted_funnel;
mod vajra;

use crate::entity::Entity;
use crate::game::GameState;
use crate::types::RelicName;

pub fn get_relic(name: RelicName) -> Entity {
    match name {
        RelicName::SnakeRing => snake_ring::SNAKE_RING,
        RelicName::Akabeko => akabeko::AKABEKO,
        RelicName::Anchor => anchor::ANCHOR,
        RelicName::BagOfMarbles => bag_of_marbles::BAG_OF_MARBLES,
        RelicName::BagOfPreparation => bag_of_preparation::BAG_OF_PREPARATION,
        RelicName::BloodVial => blood_vial::BLOOD_VIAL,
        RelicName::BronzeScales => bronze_scales::BRONZE_SCALES,
        RelicName::Kunai => kunai::KUNAI,
        RelicName::NinjaScroll => ninja_scroll::NINJA_SCROLL,
        RelicName::OddlySmoothStone => oddly_smooth_stone::ODDLY_SMOOTH_STONE,
        RelicName::Shuriken => shuriken::SHURIKEN,
        RelicName::ThreadAndNeedle => thread_and_needle::THREAD_AND_NEEDLE,
        RelicName::TwistedFunnel => twisted_funnel::TWISTED_FUNNEL,
        RelicName::Vajra => vajra::VAJRA,
    }
}

pub fn has_relic(state: &GameState, name: RelicName) -> bool {
    state.relics_active & (1u128 << name as u32) != 0
}

pub fn relic_counter(state: &GameState, name: RelicName) -> Option<i16> {
    if !has_relic(state, name) {
        return None;
    }
    let id = state.id_relics[name as usize];
    Some(state.entities[id].relic_counter)
}

pub fn relic_counter_mut<'a>(
    state: &'a mut GameState,
    name: RelicName,
) -> Option<&'a mut i16> {
    if !has_relic(state, name) {
        return None;
    }
    let id = state.id_relics[name as usize];
    Some(&mut state.entities[id].relic_counter)
}

pub fn iter_owned_relics(active: u128) -> impl Iterator<Item = RelicName> {
    let mut bits = active;
    std::iter::from_fn(move || {
        if bits == 0 {
            return None;
        }
        let idx = bits.trailing_zeros() as u8;
        bits &= bits - 1;
        Some(RelicName::from_u8(idx))
    })
}
