use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::utils::has_relic;

// While Philosopher's Stone is owned, a Monster gains 1 Strength as it spawns or splits, and a Darkling as it reincarnates
pub fn process_effect_philosopher_stone_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_monster = id_target.expect("PhilosopherStoneProc requires id_target");
    if has_relic(&state.id_relics, RelicName::PhilosopherStone) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
}
