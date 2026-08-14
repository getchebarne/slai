use crate::consts::ACT_FINAL;
use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::types::RoomKind;

pub fn process_effect_room_exit(state: &mut GameState) {
    // Pop the top frame; Map (or a suspended frame) resumes underneath
    assert!(
        state.frame_stack.len() > 1,
        "RoomExit with no room frame to pop"
    );
    state.frame_stack.pop();

    // Closing a Reward overlay reveals its live host; the room itself is not
    // left until only Map remains, so the exit logic below stays out of it
    if state.frame_stack.len() > 1 {
        return;
    }

    // Exiting a mid-run Boss room starts the next act
    if matches!(state.location, Location::BossRoom) && state.act < ACT_FINAL {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ActTransition,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // Final-row rest room enters the boss instead of returning to the map
    if matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1)
        && get_active_room_kind(&state.id_rooms, state.location, &state.entities)
            == Some(RoomKind::RestSite)
    {
        state.location = Location::BossRoom;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RoomEnter,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
