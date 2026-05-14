use std::collections::VecDeque;

use rand::Rng;
use strum::EnumCount;

use crate::consts::CHEST_LARGE;
use crate::consts::CHEST_MEDIUM;
use crate::consts::CHEST_SMALL;
use crate::consts::ChestParams;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::room_at_mut;
use crate::types::ChestKind;
use crate::types::RelicName;
use crate::utils::add_relic_reward_for_roll;

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
    // Validate we're in the Overworld
    let Location::Overworld { y, x } = location else {
        panic!("ChestOpen outside Overworld");
    };

    // Get room mut & chest kind
    let room = room_at_mut(id_rooms, entities, y, x).expect("ChestOpen room missing");
    let chest_kind = room
        .room_chest_kind
        .expect("ChestOpen with no chest_kind on room");

    // Mark chest as opened
    room.room_chest_opened = true;

    // Get roll parameters according to chest kind
    let chest_params = match chest_kind {
        ChestKind::Small => CHEST_SMALL,
        ChestKind::Medium => CHEST_MEDIUM,
        ChestKind::Large => CHEST_LARGE,
    };

    // Roll. Shared roll for both gold-yes/no and relic-tier
    let roll = rng.random_range(0..100) as u8;

    // Gold check
    if roll < chest_params.gold_chance {
        let amount = roll_gold_amount(rng, chest_params);
        effect_queue.push_back(Effect {
            kind: EffectKind::GoldGain { amount },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Relic roll
    add_relic_reward_for_roll(
        roll,
        chest_params.th_common,
        chest_params.th_uncommon,
        id_relics,
        id_relic_rewards,
        entities,
        rng,
    );

    DispatchResult::Continue
}

fn roll_gold_amount(rng: &mut impl Rng, params: ChestParams) -> u16 {
    let base = params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
