use crate::game::GameState;
use crate::types::Key;

// A key is held from its claim to the run's end; a source offers its key only while it isn't held
pub fn process_effect_key_obtain(state: &mut GameState, key: Key) {
    let key_held = match key {
        Key::Emerald => &mut state.key_emerald,
        Key::Ruby => &mut state.key_ruby,
        Key::Sapphire => &mut state.key_sapphire,
    };
    assert!(!*key_held, "KeyObtain for the {key:?} Key, already held");
    *key_held = true;
}
