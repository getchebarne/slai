use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::potions::belt_has_room;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_potion_adopt(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::PotionAdopt, id_source, id_targets);
    let id_potion = id_target.expect("PotionAdopt requires id_target");

    // Sozu: the claim consumes but nothing is obtained
    if has_relic(&state.id_relics, RelicName::Sozu) {
        return;
    }

    assert!(
        belt_has_room(&state.id_potions, state.potion_slots_max),
        "PotionAdopt queuers check for belt room"
    );
    state.id_potions.push(id_potion);
}
