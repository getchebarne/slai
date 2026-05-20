use strum::EnumCount;

use crate::engine::process_effect_relic_grant_random::grant_relic;
use crate::entity::Entity;
use crate::types::RelicName;

pub fn process_effect_relic_grant_specific(
    name: RelicName,
    fallback_circlet: bool,
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
) {
    let owns_target = id_relics[name as usize].is_some();
    let target = match (owns_target, fallback_circlet) {
        (false, _) => name,
        (true, true) => RelicName::Circlet,
        (true, false) => return,
    };
    grant_relic(target, id_relics, entities);
}
