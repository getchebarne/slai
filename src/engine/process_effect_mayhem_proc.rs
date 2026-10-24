use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::monsters_alive;
use crate::utils::shuffle;

// Mayhem: the stack's target is a living Monster picked as a Random selection picks,
// by shuffling, and fixed before the draw; its draw-top play queues behind the draw,
// targetless when none is left
pub fn process_effect_mayhem_proc(state: &mut GameState) {
    let mut id_alive: Vec<usize> =
        monsters_alive(&state.entities, &state.combat.id_monsters).collect();
    shuffle(&mut id_alive, &mut state.rng);
    state.effect_queue.push_back(Effect {
        kind: EffectKind::CardPlayFromCardPileDrawTop,
        id_source: None,
        target: Target::Direct(id_alive.first().copied()),
    });
}
