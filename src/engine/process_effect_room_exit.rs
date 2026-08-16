use crate::consts::ACT_FINAL;
use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::types::Focus;
use crate::types::RoomKind;
use crate::utils::context_focus;

pub fn process_effect_room_exit(state: &mut GameState) {
    // Close the focused context
    match context_focus(state) {
        Focus::Reward => state.reward.active = false,
        Focus::Combat => unreachable!("RoomExit during combat"),
        Focus::Shop => state.shop.active = false,
        Focus::Chest => state.chest.active = false,
        Focus::RestSite => state.rest_site.active = false,
        Focus::Event => state.event.active = false,
        Focus::Map => unreachable!("RoomExit with no context to close"),
    }

    // Closing a Reward overlay reveals its live host; the room itself is not
    // left until every context is closed, so the exit logic below stays out
    if context_focus(state) != Focus::Map {
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
