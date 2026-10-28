use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::utils::card_damage_delta;

// Ritual Dagger: a fatal blow permanently grows this copy and its deck original.
// Queued right after the Card's damage: the kill has fully resolved by the time
// this runs (mirrors HandOfGreedProc)
pub fn process_effect_ritual_dagger_proc(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    bump: u16,
) {
    let id_source = id_source.expect("RitualDaggerProc requires id_source");
    let id_target = id_target.expect("RitualDaggerProc requires id_target");
    assert!(state.entities[id_source].kind == EntityKind::Card);

    // Summons don't feed the blade
    if !state.entities[id_target].dead
        || has_modifier(&state.entities[id_target].modifiers, ModifierKind::Minion)
    {
        return;
    }

    card_damage_delta(&mut state.entities[id_source], bump as i16);

    // The combat copy is discarded at combat end; the deck original carries the growth
    let id_origin = state
        .combat
        .id_card_origins
        .iter()
        .find(|&&(id, _)| id == id_source)
        .map(|&(_, id_origin)| id_origin);
    if let Some(id_origin) = id_origin {
        card_damage_delta(&mut state.entities[id_origin], bump as i16);
    }
}
