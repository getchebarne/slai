use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::modifier_clear;
use crate::types::Mode;

// Smoke Bomb: leave combat with no rewards. Bypasses CombatEnd, so victory
// hooks (rewards, Meat on the Bone) never fire; queued effects keep running
pub fn process_effect_combat_escape(state: &mut GameState) {
    assert!(
        matches!(state.mode, Mode::Combat { .. }),
        "CombatEscape outside Combat mode"
    );
    modifier_clear(&mut state.entities[state.id_character].modifiers);
    state.mode = Mode::CombatEnded;
    state.effect_queue.push_back(Effect {
        kind: EffectKind::RoomExit,
        id_source: None,
        target: Target::Direct(None),
    });
}
