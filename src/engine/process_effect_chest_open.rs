use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{
    CHEST_LARGE, CHEST_MEDIUM, CHEST_SMALL, ChestParams, MAP_HEIGHT, MAP_WIDTH, TierWeights,
};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::game::Location;
use crate::map::room_at_mut;
use crate::types::ChestKind;

pub fn process_effect_chest_open(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    id_character: usize,
    entities: &mut [Entity],
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let Location::Overworld { y, x } = location else {
        panic!("ChestOpen outside Overworld");
    };
    let room = room_at_mut(id_rooms, entities, y, x).expect("ChestOpen room missing");
    let kind = room.room_chest_kind.expect("ChestOpen with no chest_kind on room");
    room.room_chest_kind = None;

    let params = match kind {
        ChestKind::Small => CHEST_SMALL,
        ChestKind::Medium => CHEST_MEDIUM,
        ChestKind::Large => CHEST_LARGE,
    };

    if (rng.random_range(0..100) as u8) < params.gold_chance {
        let amount = roll_gold(rng, params);
        effect_queue.push_back(Effect {
            kind: EffectKind::GoldGain { amount },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    let weights = TierWeights {
        common_threshold: params.common_threshold,
        uncommon_threshold: params.uncommon_threshold,
    };
    effect_queue.push_back(Effect {
        kind: EffectKind::RelicRewardRoll { weights },
        id_source: None,
        target: Target::Direct(None),
    });

    DispatchResult::Continue
}

fn roll_gold(rng: &mut impl Rng, params: ChestParams) -> u16 {
    let base = params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
