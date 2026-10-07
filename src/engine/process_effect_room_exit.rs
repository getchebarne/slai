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
        Focus::Reward => {
            // A boss chest left with its pick untaken closes again, holding the same Relics
            if state.reward.relics_exclusive && !state.reward.id_relics.is_empty() {
                state.chest.chest_opened = false;
            }
            state.reward.active = false;
        }
        Focus::Combat => unreachable!("RoomExit during combat"),
        Focus::Shop => state.shop.active = false,
        Focus::Chest => state.chest.active = false,
        Focus::RestSite => state.rest_site.active = false,
        Focus::Event => state.event.active = false,
        Focus::Map => unreachable!("RoomExit with no context to close"),
    }

    // Closing a Reward overlay reveals its live host; the Room itself is not
    // left until every context is closed, so the exit logic below stays out
    if context_focus(state) != Focus::Map {
        return;
    }

    // A mid-run Boss Room opens onto its treasure Room, whose exit starts the next act
    match state.location {
        Location::BossRoom if state.act < ACT_FINAL => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RoomEnter {
                    location: Location::BossTreasure,
                    landing: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            return;
        }
        Location::BossTreasure => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ActTransition,
                id_source: None,
                target: Target::Direct(None),
            });
            return;
        }
        _ => {}
    }

    // Final-row rest Room enters the boss instead of returning to the map
    if matches!(state.location, Location::Overworld { y, .. } if y == MAP_HEIGHT - 1)
        && get_active_room_kind(&state.id_rooms, state.location, &state.entities)
            == Some(RoomKind::RestSite)
    {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RoomEnter {
                location: Location::BossRoom,
                landing: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
