use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::types::Screen;
use crate::utils::clear_shop_state;

pub fn process_effect_room_exit(state: &mut GameState) {
    match state.screen {
        // final-row rest site enters the boss instead of returning to the map
        Screen::RestSite if matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1) =>
        {
            state.location = Location::BossRoom;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RoomEnter,
                id_source: None,
                target: Target::Direct(None),
            });
            return;
        }
        Screen::Reward => {
            state.reward_id_cards.clear();
            state.reward_id_relic = None;
            state.reward_id_potion = None;
            state.reward_gold = None;
        }
        Screen::Shop => clear_shop_state(state),
        _ => {} // Event, RestSite (non-final), Chest: no per-screen cleanup
    }
    // Event combats exit via the Reward screen, so event working memory clears here, not per-screen
    state.id_event = None;
    state.id_event_picks.clear();
    state.event_gold_rolled = 0;
    state.event_rolls.clear();
    state.screen = Screen::Map;
}
