use crate::consts::MAP_HEIGHT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::types::Mode;
use crate::types::RelicName;

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
    // Orrery / Prayer Wheel: chain a pending card reward before leaving the screen
    if matches!(state.mode, Mode::Reward { .. }) {
        for name in [RelicName::Orrery, RelicName::PrayerWheel] {
            if let Some(id) = state.id_relics[name as usize]
                && state.entities[id].relic_counter > 0
            {
                let relic = &mut state.entities[id];
                relic.relic_counter -= 1;
                // Orrery burns out; Prayer Wheel re-arms every normal fight
                if name == RelicName::Orrery {
                    relic.relic_used_up = relic.relic_counter == 0;
                }
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::RewardRollCards,
                    id_source: None,
                    target: Target::Direct(None),
                });
                return;
            }
        }
    }

    state.mode = Mode::Map;
}
