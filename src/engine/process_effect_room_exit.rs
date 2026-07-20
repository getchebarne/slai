use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::types::Mode;

pub fn process_effect_room_exit(state: &mut GameState) {
    match state.mode {
        // final-row rest site enters the boss instead of returning to the map
        Mode::RestSite if matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1) =>
        {
            state.location = Location::BossRoom;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RoomEnter,
                id_source: None,
                target: Target::Direct(None),
            });
            return;
        }
        // Reward and Shop memory die with the variant swap below; Event,
        // RestSite (non-final), Chest need no per-mode cleanup
        _ => {}
    }
    // Event combats exit via the Reward screen, so event working memory clears here, not per-screen
    state.event = None;
    state.mode = Mode::Map;
}
