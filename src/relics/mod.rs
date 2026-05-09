// Relic registry and accessors. Storage is a name-indexed
// `[Option<usize>; RelicName::COUNT]` on GameState; per-relic state lives on
// the Entity (counter, used_up, effects-on-combat-start)

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

use strum::EnumCount;

use crate::entity::Entity;
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

pub fn iter_owned_relics(
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> impl Iterator<Item = (RelicName, usize)> + '_ {
    id_relics
        .iter()
        .enumerate()
        .filter_map(|(i, &opt)| opt.map(|id| (RelicName::from_u8(i as u8), id)))
}
