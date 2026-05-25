use crate::consts::MAP_HEIGHT;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::types::Screen;

pub fn process_effect_rest_site_exit(state: &mut GameState) {
    let at_final_row =
        matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1);

    if at_final_row {
        state.location = Location::BossRoom;
        state
            .effect_queue
            .push_front(effect_direct(EffectKind::RoomEnter, None, None));
    } else {
        state.active = Screen::Map;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RoomSelect,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::NextRowRooms,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        });
    }
}
