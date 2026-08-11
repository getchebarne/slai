use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::count_monsters_named;
use crate::monsters::the_collector::TORCH_HEAD_COUNT;
use crate::types::Mode;
use crate::types::MonsterName;
use crate::utils::mode_top;

// The Collector's Spawn and Revive: top the roster back up to two Torch Heads
pub fn process_effect_torch_head_spawn(state: &mut GameState) {
    let Mode::Combat { id_monsters, .. } = mode_top(&state.mode_stack) else {
        unreachable!("process_effect_torch_head_spawn outside Combat mode")
    };
    let alive = count_monsters_named(&state.entities, id_monsters, MonsterName::TorchHead);
    for _ in alive..TORCH_HEAD_COUNT {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MonsterSpawn {
                name: MonsterName::TorchHead,
                minion: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
