use std::collections::VecDeque;

use crate::consts::MAP_HEIGHT;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::DispatchResult;
use crate::state::{Map, Position};

pub fn process_effect_rest_site_exit(
    map: &mut Map,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let at_final_row = matches!(map.position, Position::Overworld { y, .. } if y == MAP_HEIGHT - 1);

    if at_final_row {
        map.position = Position::BossRoom;
        queue.push_front(Effect::direct(EffectKind::RoomEnter, None, None));
    } else {
        queue.push_front(Effect {
            kind: EffectKind::RoomSelect,
            source: None,
            target: Target::Resolve {
                candidates: CandidatePool::NextRowRooms,
                selection: SelectionKind::Input { count: 1 },
            },
        });
    }
    DispatchResult::Continue
}
