use std::collections::VecDeque;

use crate::consts::MAP_HEIGHT;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::DispatchResult;
use crate::state::Location;

pub fn process_effect_rest_site_exit(
    location: &mut Location,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let at_final_row = matches!(*location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1);

    if at_final_row {
        *location = Location::BossRoom;
        queue.push_front(Effect::direct(EffectKind::RoomEnter, None, None));
    } else {
        queue.push_front(Effect {
            kind: EffectKind::RoomSelect,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::NextRowRooms,
                selection: SelectionKind::Input { count: 1 },
            },
        });
    }
    DispatchResult::Continue
}
