use crate::game::GameState;
use crate::monsters::encounters::roll_boss;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::types::EventName;
use crate::types::Focus;
use crate::utils::context_focus;

// Mind Bloom's I am War: an Act 1 boss, rolled uniformly when chosen, fights over the event
pub fn process_effect_mind_bloom_fight(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Event,
        "MindBloomFight outside the Event context"
    );
    assert!(
        matches!(state.event.name, EventName::MindBloom),
        "MindBloomFight outside a Mind Bloom event"
    );

    // The event rolls its own boss; the act's `encounter_boss` is left alone
    let encounter = roll_boss(1, &mut state.rng);
    spawn_encounter_monsters(state, encounter, false);
}
