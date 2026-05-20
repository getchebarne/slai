use std::collections::VecDeque;

use crate::consts::MAP_HEIGHT;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::Location;

pub fn process_effect_rest_site_exit(
    location: &mut Location,
    effect_queue: &mut VecDeque<Effect>,
) {
    let at_final_row = matches!(*location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1);

    if at_final_row {
        *location = Location::BossRoom;
        effect_queue.push_front(Effect::direct(EffectKind::RoomEnter, None, None));
    } else {
        effect_queue.push_front(Effect {
            kind: EffectKind::RoomSelect,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::NextRowRooms,
                selection: SelectionKind::Input { count: 1 },
            },
        });
    }
}
