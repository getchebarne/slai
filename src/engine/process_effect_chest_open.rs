use std::collections::VecDeque;

use rand::Rng;
use strum::EnumCount;

use crate::consts::{
    CHEST_LARGE, CHEST_MEDIUM, CHEST_SMALL, ChestParams, MAP_HEIGHT, MAP_WIDTH, TierThresholds,
};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::engine::process_effect_relic_reward_roll::add_relic_reward_for_roll;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::room_at_mut;
use crate::types::{ChestKind, RelicName};

pub fn process_effect_chest_open(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    id_character: usize,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let Location::Overworld { y, x } = location else {
        panic!("ChestOpen outside Overworld");
    };
    let room = room_at_mut(id_rooms, entities, y, x).expect("ChestOpen room missing");
    let kind = room
        .room_chest_kind
        .expect("ChestOpen with no chest_kind on room");
    room.room_chest_opened = true;

    let params = match kind {
        ChestKind::Small => CHEST_SMALL,
        ChestKind::Medium => CHEST_MEDIUM,
        ChestKind::Large => CHEST_LARGE,
    };

    // One shared roll for both gold-yes/no and relic-tier
    let roll = rng.random_range(0..100) as u8;

    if roll < params.gold_chance {
        let amount = roll_gold_amount(rng, params);
        effect_queue.push_back(Effect {
            kind: EffectKind::GoldGain { amount },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    let thresholds = TierThresholds {
        th_common: params.th_common,
        th_uncommon: params.th_uncommon,
    };
    add_relic_reward_for_roll(roll, thresholds, id_relics, id_relic_rewards, entities, rng);

    DispatchResult::Continue
}

fn roll_gold_amount(rng: &mut impl Rng, params: ChestParams) -> u16 {
    let base = params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
