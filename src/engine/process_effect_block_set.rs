use crate::consts::MAX_BLOCK;
use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;

pub fn process_effect_block_set(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    amount: u16,
) {
    let id_target = first_target(
        state,
        EffectKind::BlockSet { amount },
        id_source,
        id_targets,
    );
    let id_target = id_target.expect("BlockSet requires id_target");
    state.entities[id_target].vitals.block = amount.min(MAX_BLOCK);
}
