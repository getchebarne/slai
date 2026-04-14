use crate::consts::MAP_HEIGHT;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::ProcessEffectResult;
use crate::state::{Map, Position};

pub fn process_effect_rest_site_exit(map: &mut Map) -> ProcessEffectResult {
    let at_final_row = matches!(map.position, Position::Overworld { y, .. } if y == MAP_HEIGHT - 1);

    if at_final_row {
        // Final row — advance into the boss room and let RoomEnter do the setup
        map.position = Position::BossRoom;
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect::direct(EffectKind::RoomEnter, None, None)],
            bot: Vec::new(),
        }
    } else {
        // Non-final row — halt and wait for the player to pick the next map node
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::RoomSelect,
                source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::NextRowRooms,
                    selection: SelectionKind::Input { count: 1 },
                },
            }],
            bot: Vec::new(),
        }
    }
}
