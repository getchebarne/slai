pub mod cultist;
pub mod fungi_beast;
pub mod gremlin_fat;
pub mod gremlin_nob;
pub mod gremlin_thief;
pub mod gremlin_tsundere;
pub mod gremlin_warrior;
pub mod gremlin_wizard;
pub mod jaw_worm;
pub mod lagavulin;
pub mod looter;
pub mod louse_green;
pub mod louse_red;
pub mod sentry;
pub mod slaver_blue;
pub mod slaver_red;
pub mod slime_acid_large;
pub mod slime_acid_medium;
pub mod slime_acid_small;
pub mod slime_boss;
pub mod slime_spike_large;
pub mod slime_spike_medium;
pub mod slime_spike_small;
pub mod the_guardian;

use crate::entity::{Entity, get_move_history_slice};
use crate::types::MonsterName;
use rand::Rng;

pub fn spawn_monster(monster_name: MonsterName, ascension_level: u8, rng: &mut impl Rng) -> Entity {
    match monster_name {
        MonsterName::Cultist => cultist::spawn_cultist(ascension_level, rng),
        MonsterName::JawWorm => jaw_worm::spawn_jaw_worm(ascension_level, rng),
        MonsterName::TheGuardian => the_guardian::spawn_the_guardian(ascension_level),
        MonsterName::FungiBeast => fungi_beast::spawn_fungi_beast(ascension_level, rng),
        MonsterName::SlaverBlue => slaver_blue::spawn_slaver_blue(ascension_level, rng),
        MonsterName::SlimeAcidSmall => {
            slime_acid_small::spawn_slime_acid_small(ascension_level, rng)
        }
        MonsterName::SlimeSpikeSmall => {
            slime_spike_small::spawn_slime_spike_small(ascension_level, rng)
        }
        MonsterName::GremlinFat => gremlin_fat::spawn_gremlin_fat(ascension_level, rng),
        MonsterName::GremlinNob => gremlin_nob::spawn_gremlin_nob(ascension_level, rng),
        MonsterName::GremlinThief => gremlin_thief::spawn_gremlin_thief(ascension_level, rng),
        MonsterName::GremlinTsundere => {
            gremlin_tsundere::spawn_gremlin_tsundere(ascension_level, rng)
        }
        MonsterName::GremlinWarrior => gremlin_warrior::spawn_gremlin_warrior(ascension_level, rng),
        MonsterName::GremlinWizard => gremlin_wizard::spawn_gremlin_wizard(ascension_level, rng),
        MonsterName::Lagavulin => lagavulin::spawn_lagavulin(ascension_level, rng),
        MonsterName::Looter => looter::spawn_looter(ascension_level, rng),
        MonsterName::LouseDefensive => louse_green::spawn_louse_green(ascension_level, rng),
        MonsterName::LouseNormal => louse_red::spawn_louse_red(ascension_level, rng),
        MonsterName::Sentry => sentry::spawn_sentry(ascension_level, rng),
        MonsterName::SlaverRed => slaver_red::spawn_slaver_red(ascension_level, rng),
        MonsterName::SlimeAcidLarge => {
            slime_acid_large::spawn_slime_acid_large(ascension_level, rng)
        }
        MonsterName::SlimeAcidMedium => {
            slime_acid_medium::spawn_slime_acid_medium(ascension_level, rng)
        }
        MonsterName::SlimeBoss => slime_boss::spawn_slime_boss(ascension_level),
        MonsterName::SlimeSpikeLarge => {
            slime_spike_large::spawn_slime_spike_large(ascension_level, rng)
        }
        MonsterName::SlimeSpikeMedium => {
            slime_spike_medium::spawn_slime_spike_medium(ascension_level, rng)
        }
    }
}

// True if completing `move_idx` marks the end of one of this monster's
// attack/defense cycles. Callers increment `Entity::cycle_count` on true
pub fn is_cycle_boundary(name: MonsterName, move_idx: u8) -> bool {
    match name {
        MonsterName::TheGuardian => move_idx == the_guardian::IDX_MOVE_TWIN_SLAM as u8,
        _ => false,
    }
}

pub fn get_next_move(
    entity: &Entity,
    entity_id: usize,
    id_alive_monsters: &[usize],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let history = get_move_history_slice(entity);
    match entity.monster_name {
        MonsterName::Cultist => cultist::get_next_move_cultist(entity.move_current, history),
        MonsterName::JawWorm => {
            jaw_worm::get_next_move_jaw_worm(entity.move_current, history, entity.moves, rng)
        }
        MonsterName::TheGuardian => the_guardian::get_next_move_the_guardian_full(
            entity.move_current,
            history,
            &entity.modifiers,
        ),
        MonsterName::FungiBeast => {
            fungi_beast::get_next_move_fungi_beast(entity.move_current, history, rng)
        }
        MonsterName::SlaverBlue => slaver_blue::get_next_move_slaver_blue(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeAcidSmall => slime_acid_small::get_next_move_slime_acid_small(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeSpikeSmall => slime_spike_small::get_next_move_slime_spike_small(),
        MonsterName::GremlinFat => gremlin_fat::get_next_move_gremlin_fat(),
        MonsterName::GremlinNob => {
            gremlin_nob::get_next_move_gremlin_nob(history, ascension_level, rng)
        }
        MonsterName::GremlinThief => gremlin_thief::get_next_move_gremlin_thief(),
        MonsterName::GremlinWizard => gremlin_wizard::get_next_move_gremlin_wizard(
            entity.move_current,
            history,
            ascension_level,
        ),
        MonsterName::GremlinTsundere => {
            let other_alive_count = id_alive_monsters
                .iter()
                .filter(|&&id| id != entity_id)
                .count() as u8;
            gremlin_tsundere::get_next_move_gremlin_tsundere(entity.move_current, other_alive_count)
        }
        MonsterName::GremlinWarrior => gremlin_warrior::get_next_move_gremlin_warrior(),
        MonsterName::Lagavulin => {
            lagavulin::get_next_move_lagavulin(entity.move_current, history, &entity.modifiers)
        }
        MonsterName::Looter => looter::get_next_move_looter(entity.move_current, history, rng),
        MonsterName::LouseDefensive => louse_green::get_next_move_louse_green(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::LouseNormal => {
            louse_red::get_next_move_louse_red(entity.move_current, history, ascension_level, rng)
        }
        MonsterName::Sentry => sentry::get_next_move_sentry(
            entity.move_current,
            history,
            entity_id,
            id_alive_monsters,
        ),
        MonsterName::SlaverRed => slaver_red::get_next_move_slaver_red(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeAcidLarge => slime_acid_large::get_next_move_slime_acid_large(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeAcidMedium => slime_acid_medium::get_next_move_slime_acid_medium(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeBoss => {
            slime_boss::get_next_move_slime_boss(entity.move_current, history)
        }
        MonsterName::SlimeSpikeLarge => slime_spike_large::get_next_move_slime_spike_large(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeSpikeMedium => slime_spike_medium::get_next_move_slime_spike_medium(
            entity.move_current,
            history,
            ascension_level,
            rng,
        ),
    }
}
