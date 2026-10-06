use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::has_edge;
use crate::types::RelicName;

// Direct form (post-resolve); Resolve form handled by process_effect
pub fn process_effect_room_select(id_target: Option<usize>, state: &mut GameState) {
    let id_room = id_target.expect("RoomSelect Direct form must have target");

    // Wing Boots: an off-path move burns a charge (checked before the location moves)
    if let Some(id_boots) = state.id_relics[RelicName::WingBoots as usize]
        && let Location::Overworld { y, x } = state.location
        && let Some(id_current) = state.id_rooms[y][x]
        && !has_edge(
            state.entities[id_current].room_edges,
            state.entities[id_room].room_x,
        )
    {
        let relic = &mut state.entities[id_boots];
        relic.relic_counter -= 1;
        relic.relic_used_up = relic.relic_counter == 0;
    }

    let room = &state.entities[id_room];
    state.location = Location::Overworld {
        y: room.room_y,
        x: room.room_x,
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RoomEnter,
        id_source: None,
        target: Target::Direct(None),
    });
}
