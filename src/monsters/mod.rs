pub mod byrd;
pub mod centurion;
pub mod chosen;
pub mod cultist;
pub mod encounters;
pub mod fungi_beast;
pub mod gremlin_fat;
pub mod gremlin_nob;
pub mod gremlin_thief;
pub mod gremlin_tsundere;
pub mod gremlin_warrior;
pub mod gremlin_wizard;
pub mod healer;
pub mod hexaghost;
pub mod jaw_worm;
pub mod lagavulin;
pub mod looter;
pub mod louse_green;
pub mod louse_red;
pub mod mugger;
pub mod sentry;
pub mod shelled_parasite;
pub mod slaver_blue;
pub mod slaver_red;
pub mod slime_acid_large;
pub mod slime_acid_medium;
pub mod slime_acid_small;
pub mod slime_boss;
pub mod slime_spike_large;
pub mod slime_spike_medium;
pub mod slime_spike_small;
pub mod snake_plant;
pub mod snecko;
pub mod spheric_guardian;
pub mod the_guardian;

use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_MOVES_PER_MONSTER;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::effect::ZERO_EFFECT;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::ZERO_ENTITY;
use crate::entity::ZERO_MOVE;
use crate::entity::get_move_history_slice;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

pub fn spawn_monster(monster_name: MonsterName, ascension_level: u8, rng: &mut impl Rng) -> Entity {
    match monster_name {
        MonsterName::Cultist => cultist::spawn_monster_cultist(ascension_level, rng),
        MonsterName::JawWorm => jaw_worm::spawn_monster_jaw_worm(ascension_level, rng),
        MonsterName::TheGuardian => the_guardian::spawn_monster_the_guardian(ascension_level),
        MonsterName::FungiBeast => fungi_beast::spawn_monster_fungi_beast(ascension_level, rng),
        MonsterName::SlaverBlue => slaver_blue::spawn_monster_slaver_blue(ascension_level, rng),
        MonsterName::SlimeAcidSmall => {
            slime_acid_small::spawn_monster_slime_acid_small(ascension_level, rng)
        }
        MonsterName::SlimeSpikeSmall => {
            slime_spike_small::spawn_monster_slime_spike_small(ascension_level, rng)
        }
        MonsterName::GremlinFat => gremlin_fat::spawn_monster_gremlin_fat(ascension_level, rng),
        MonsterName::GremlinNob => gremlin_nob::spawn_monster_gremlin_nob(ascension_level, rng),
        MonsterName::GremlinThief => {
            gremlin_thief::spawn_monster_gremlin_thief(ascension_level, rng)
        }
        MonsterName::GremlinTsundere => {
            gremlin_tsundere::spawn_monster_gremlin_tsundere(ascension_level, rng)
        }
        MonsterName::GremlinWarrior => {
            gremlin_warrior::spawn_monster_gremlin_warrior(ascension_level, rng)
        }
        MonsterName::GremlinWizard => {
            gremlin_wizard::spawn_monster_gremlin_wizard(ascension_level, rng)
        }
        MonsterName::Hexaghost => hexaghost::spawn_monster_hexaghost(ascension_level),
        MonsterName::Lagavulin => lagavulin::spawn_monster_lagavulin(ascension_level, rng),
        MonsterName::Looter => looter::spawn_monster_looter(ascension_level, rng),
        MonsterName::LouseDefensive => louse_green::spawn_monster_louse_green(ascension_level, rng),
        MonsterName::LouseNormal => louse_red::spawn_monster_louse_red(ascension_level, rng),
        MonsterName::Sentry => sentry::spawn_monster_sentry(ascension_level, rng),
        MonsterName::SlaverRed => slaver_red::spawn_monster_slaver_red(ascension_level, rng),
        MonsterName::SlimeAcidLarge => {
            slime_acid_large::spawn_monster_slime_acid_large(ascension_level, rng)
        }
        MonsterName::SlimeAcidMedium => {
            slime_acid_medium::spawn_monster_slime_acid_medium(ascension_level, rng)
        }
        MonsterName::SlimeBoss => slime_boss::spawn_monster_slime_boss(ascension_level),
        MonsterName::SlimeSpikeLarge => {
            slime_spike_large::spawn_monster_slime_spike_large(ascension_level, rng)
        }
        MonsterName::SlimeSpikeMedium => {
            slime_spike_medium::spawn_monster_slime_spike_medium(ascension_level, rng)
        }
        MonsterName::Byrd => byrd::spawn_monster_byrd(ascension_level, rng),
        MonsterName::Centurion => centurion::spawn_monster_centurion(ascension_level, rng),
        MonsterName::Chosen => chosen::spawn_monster_chosen(ascension_level, rng),
        MonsterName::Healer => healer::spawn_monster_healer(ascension_level, rng),
        MonsterName::Mugger => mugger::spawn_monster_mugger(ascension_level, rng),
        MonsterName::ShelledParasite => {
            shelled_parasite::spawn_monster_shelled_parasite(ascension_level, rng)
        }
        MonsterName::SnakePlant => snake_plant::spawn_monster_snake_plant(ascension_level, rng),
        MonsterName::Snecko => snecko::spawn_monster_snecko(ascension_level, rng),
        MonsterName::SphericGuardian => {
            spheric_guardian::spawn_monster_spheric_guardian(ascension_level, rng)
        }
    }
}

// True if `move_idx` ends a cycle; callers bump monster_cycle_count
pub fn is_cycle_boundary(name: MonsterName, move_idx: u8) -> bool {
    match name {
        MonsterName::TheGuardian => move_idx == the_guardian::IDX_MOVE_TWIN_SLAM as u8,
        _ => false,
    }
}

// Takes the whole arena: some AIs read other monsters' state (ally counts, party HP)
pub fn get_next_move(
    entities: &[Entity],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let entity = &entities[entity_id];
    let history = get_move_history_slice(entity);
    match entity.monster_name {
        // Single-move monsters
        MonsterName::SlimeSpikeSmall
        | MonsterName::GremlinFat
        | MonsterName::GremlinThief
        | MonsterName::GremlinWarrior => 0,
        MonsterName::Cultist => cultist::get_next_move_cultist(entity.monster_move_current),
        MonsterName::JawWorm => {
            jaw_worm::get_next_move_jaw_worm(entity.monster_move_current, history, rng)
        }
        MonsterName::TheGuardian => the_guardian::get_next_move_the_guardian_full(
            entity.monster_move_current,
            history,
            &entity.modifiers,
        ),
        MonsterName::FungiBeast => fungi_beast::get_next_move_fungi_beast(history, rng),
        MonsterName::SlaverBlue => {
            slaver_blue::get_next_move_slaver_blue(history, ascension_level, rng)
        }
        MonsterName::SlimeAcidSmall => slime_acid_small::get_next_move_slime_acid_small(
            entity.monster_move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::GremlinNob => {
            gremlin_nob::get_next_move_gremlin_nob(history, ascension_level, rng)
        }
        MonsterName::GremlinWizard => gremlin_wizard::get_next_move_gremlin_wizard(
            entity.monster_move_current,
            history,
            ascension_level,
        ),
        MonsterName::Hexaghost => {
            hexaghost::get_next_move_hexaghost(entity.monster_move_current, history)
        }
        MonsterName::GremlinTsundere => {
            let other_alive_count = id_monsters
                .iter()
                .flatten()
                .filter(|&&id| id != entity_id)
                .count() as u8;
            gremlin_tsundere::get_next_move_gremlin_tsundere(
                entity.monster_move_current,
                other_alive_count,
            )
        }
        MonsterName::Lagavulin => lagavulin::get_next_move_lagavulin(
            entity.monster_move_current,
            history,
            &entity.modifiers,
        ),
        MonsterName::Looter => {
            looter::get_next_move_looter(entity.monster_move_current, history, rng)
        }
        MonsterName::LouseDefensive => {
            louse_green::get_next_move_louse_green(history, ascension_level, rng)
        }
        MonsterName::LouseNormal => {
            louse_red::get_next_move_louse_red(history, ascension_level, rng)
        }
        MonsterName::Sentry => sentry::get_next_move_sentry(
            entity.monster_move_current,
            history,
            entity_id,
            id_monsters,
        ),
        MonsterName::SlaverRed => slaver_red::get_next_move_slaver_red(
            entity.monster_move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SlimeAcidLarge => {
            slime_acid_large::get_next_move_slime_acid_large(history, ascension_level, rng)
        }
        MonsterName::SlimeAcidMedium => {
            slime_acid_medium::get_next_move_slime_acid_medium(history, ascension_level, rng)
        }
        MonsterName::SlimeBoss => {
            slime_boss::get_next_move_slime_boss(entity.monster_move_current, history)
        }
        MonsterName::SlimeSpikeLarge => {
            slime_spike_medium::get_next_move_slime_spike(history, ascension_level, rng)
        }
        MonsterName::SlimeSpikeMedium => {
            slime_spike_medium::get_next_move_slime_spike(history, ascension_level, rng)
        }
        MonsterName::Byrd => byrd::get_next_move_byrd(
            entity.monster_move_current,
            history,
            &entity.modifiers,
            rng,
        ),
        MonsterName::Centurion => {
            centurion::get_next_move_centurion(history, entity_id, id_monsters, rng)
        }
        MonsterName::Chosen => chosen::get_next_move_chosen(history, ascension_level, rng),
        MonsterName::Healer => {
            healer::get_next_move_healer(history, entities, id_monsters, ascension_level, rng)
        }
        // Same script as the Looter; move indices line up one-to-one
        MonsterName::Mugger => {
            looter::get_next_move_looter(entity.monster_move_current, history, rng)
        }
        MonsterName::ShelledParasite => shelled_parasite::get_next_move_shelled_parasite(
            entity.monster_move_current,
            history,
            ascension_level,
            rng,
        ),
        MonsterName::SnakePlant => {
            snake_plant::get_next_move_snake_plant(history, ascension_level, rng)
        }
        MonsterName::Snecko => {
            snecko::get_next_move_snecko(entity.monster_move_current, history, rng)
        }
        MonsterName::SphericGuardian => spheric_guardian::get_next_move_spheric_guardian(history),
    }
}

// The repeated monster move shapes; each spells out one Effect array longhand
pub const fn make_move_attack(name: &'static str, damage: u16, instances: u8) -> Move {
    let mut effects = [ZERO_EFFECT; MAX_EFFECTS_PER_MOVE];
    let mut i = 0;
    while i < instances as usize {
        effects[i] = Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source: None,
            target: TARGET_CHARACTER,
        };
        i += 1;
    }
    Move {
        name,
        effects,
        effects_len: instances,
        intent: Intent::Attack { damage, instances },
    }
}

pub const fn make_move_buff(name: &'static str, kind: ModifierKind, stacks: i16) -> Move {
    make_move(
        name,
        &[Effect {
            kind: EffectKind::ModifierGain { kind, stacks },
            id_source: None,
            target: TARGET_SOURCE,
        }],
        Intent::Buff,
    )
}

pub const fn make_move_debuff(
    name: &'static str,
    kind: ModifierKind,
    stacks: i16,
    intent: Intent,
) -> Move {
    make_move(
        name,
        &[Effect {
            kind: EffectKind::ModifierGain { kind, stacks },
            id_source: None,
            target: TARGET_CHARACTER,
        }],
        intent,
    )
}

pub const fn make_move_attack_debuff(
    name: &'static str,
    damage: u16,
    kind: ModifierKind,
    stacks: i16,
) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::DamagePhysical { amount: damage },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain { kind, stacks },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 1,
        },
    )
}

pub const fn make_move_attack_card_add(
    name: &'static str,
    damage: u16,
    card_name: CardName,
    count: u16,
    upgraded: bool,
) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::DamagePhysical { amount: damage },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::CardAdd {
                    card_name,
                    pile: CardPile::Discard,
                    count,
                    upgraded,
                },
                id_source: None,
                target: Target::Direct(None),
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 1,
        },
    )
}

pub const fn make_move_block(name: &'static str, block: u16) -> Move {
    make_move(
        name,
        &[Effect {
            kind: EffectKind::BlockGain { amount: block },
            id_source: None,
            target: TARGET_SOURCE,
        }],
        Intent::Block,
    )
}

// Self-buff then block; the order matches the jaw_worm sites this serves
pub const fn make_move_block_buff(name: &'static str, block: u16, strength: i16) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: strength,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::BlockGain { amount: block },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::BlockBuff,
    )
}

pub const fn make_move_split(name: &'static str, first: MonsterName, second: MonsterName) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::MonsterSplit { name: first },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::MonsterSplit { name: second },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::MonsterEscape,
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::Unknown,
    )
}

pub const fn make_move(name: &'static str, effects: &[Effect], intent: Intent) -> Move {
    assert!(
        effects.len() <= MAX_EFFECTS_PER_MOVE,
        "Move effects exceeds MAX_EFFECTS_PER_MOVE",
    );
    let mut arr = [ZERO_EFFECT; MAX_EFFECTS_PER_MOVE];
    let mut i = 0;
    while i < effects.len() {
        arr[i] = effects[i];
        i += 1;
    }
    Move {
        name,
        effects: arr,
        effects_len: effects.len() as u8,
        intent,
    }
}

pub const fn make_entity_monster(
    name: MonsterName,
    monster_kind: MonsterKind,
    vitals: Vitals,
    modifiers: Modifiers,
    moves: &[Move],
) -> Entity {
    assert!(
        moves.len() <= MAX_MOVES_PER_MONSTER,
        "monster_moves exceeds MAX_MOVES_PER_MONSTER",
    );
    let mut arr = [ZERO_MOVE; MAX_MOVES_PER_MONSTER];
    let mut i = 0;
    while i < moves.len() {
        arr[i] = moves[i];
        i += 1;
    }
    Entity {
        kind: EntityKind::Monster,
        vitals,
        modifiers,
        monster_name: name,
        monster_kind,
        monster_moves: arr,
        ..ZERO_ENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::push_move_history;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    // Act-2 normals are unreachable in-game until the act seam lands; this guards
    // their spawn tables and AI against panics and out-of-range move indices
    #[test]
    fn act2_normal_ai_smoke() {
        let names = [
            MonsterName::Byrd,
            MonsterName::Centurion,
            MonsterName::Chosen,
            MonsterName::Healer,
            MonsterName::Mugger,
            MonsterName::ShelledParasite,
            MonsterName::SnakePlant,
            MonsterName::Snecko,
            MonsterName::SphericGuardian,
        ];
        let mut rng = SmallRng::seed_from_u64(7);
        for ascension in [0, 2, 7, 17, 18, 20] {
            for name in names {
                let mut entities = vec![spawn_monster(name, ascension, &mut rng)];
                let mut id_monsters = [None; MAX_MONSTERS];
                id_monsters[0] = Some(0);
                for _ in 0..50 {
                    let idx = get_next_move(&entities, 0, &id_monsters, ascension, &mut rng);
                    assert!(
                        idx < MAX_MOVES_PER_MONSTER
                            && !entities[0].monster_moves[idx].name.is_empty(),
                        "{name:?} rolled invalid move idx {idx}"
                    );
                    entities[0].monster_move_current = Some(idx);
                    push_move_history(&mut entities[0], idx as u8);
                }
            }
        }
    }
}
