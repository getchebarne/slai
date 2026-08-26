pub mod bandit_bear;
pub mod bandit_leader;
pub mod bandit_pointy;
pub mod book_of_stabbing;
pub mod bronze_automaton;
pub mod bronze_orb;
pub mod byrd;
pub mod centurion;
pub mod champ;
pub mod chosen;
pub mod cultist;
pub mod encounters;
pub mod fungi_beast;
pub mod gremlin_fat;
pub mod gremlin_leader;
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
pub mod taskmaster;
pub mod the_collector;
pub mod the_guardian;
pub mod torch_head;

use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_MOVE_HISTORY;
use crate::effect::EFFECT_ZERO;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_apply;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use strum::EnumCount;

use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Static definition of a Monster: everything spawn-time except per-Monster rolls
pub struct MonsterTemplate {
    pub name: MonsterName,
    pub kind: MonsterKind,
    pub health_tiers: &'static [(u8, (u16, u16))],
    pub block_start: u16,
    pub move_tiers: &'static [(u8, &'static [Move])],
    pub modifier_tiers: &'static [(u8, &'static [(ModifierKind, i16)])],
}

// Every MonsterTemplate, for compile-time validation and template enumeration
pub const ALL_MONSTERS: &[&'static MonsterTemplate] = &[
    &bandit_bear::BANDIT_BEAR,
    &bandit_leader::BANDIT_LEADER,
    &bandit_pointy::BANDIT_POINTY,
    &book_of_stabbing::BOOK_OF_STABBING,
    &bronze_automaton::BRONZE_AUTOMATON,
    &bronze_orb::BRONZE_ORB,
    &byrd::BYRD,
    &centurion::CENTURION,
    &champ::CHAMP,
    &chosen::CHOSEN,
    &cultist::CULTIST,
    &fungi_beast::FUNGI_BEAST,
    &gremlin_fat::GREMLIN_FAT,
    &gremlin_leader::GREMLIN_LEADER,
    &gremlin_nob::GREMLIN_NOB,
    &gremlin_thief::GREMLIN_THIEF,
    &gremlin_tsundere::GREMLIN_TSUNDERE,
    &gremlin_warrior::GREMLIN_WARRIOR,
    &gremlin_wizard::GREMLIN_WIZARD,
    &healer::HEALER,
    &hexaghost::HEXAGHOST,
    &jaw_worm::JAW_WORM,
    &lagavulin::LAGAVULIN,
    &looter::LOOTER,
    &louse_green::LOUSE_DEFENSIVE,
    &louse_red::LOUSE_NORMAL,
    &mugger::MUGGER,
    &sentry::SENTRY,
    &shelled_parasite::SHELLED_PARASITE,
    &slaver_blue::SLAVER_BLUE,
    &slaver_red::SLAVER_RED,
    &slime_acid_large::SLIME_ACID_LARGE,
    &slime_acid_medium::SLIME_ACID_MEDIUM,
    &slime_acid_small::SLIME_ACID_SMALL,
    &slime_boss::SLIME_BOSS,
    &slime_spike_large::SLIME_SPIKE_LARGE,
    &slime_spike_medium::SLIME_SPIKE_MEDIUM,
    &slime_spike_small::SLIME_SPIKE_SMALL,
    &snake_plant::SNAKE_PLANT,
    &snecko::SNECKO,
    &spheric_guardian::SPHERIC_GUARDIAN,
    &taskmaster::TASKMASTER,
    &the_collector::THE_COLLECTOR,
    &the_guardian::THE_GUARDIAN,
    &torch_head::TORCH_HEAD,
];
const _: () = assert!(ALL_MONSTERS.len() == MonsterName::COUNT);

// Tier tables are looked up by descending scan, so they must ascend by threshold
const fn assert_tiers_ascend<T: Copy>(tiers: &[(u8, T)]) {
    let mut i = 1;
    while i < tiers.len() {
        assert!(tiers[i - 1].0 < tiers[i].0, "monster tiers must ascend");
        i += 1;
    }
}
const _: () = {
    let mut i = 0;
    while i < ALL_MONSTERS.len() {
        assert_tiers_ascend(ALL_MONSTERS[i].health_tiers);
        assert_tiers_ascend(ALL_MONSTERS[i].move_tiers);
        assert_tiers_ascend(ALL_MONSTERS[i].modifier_tiers);
        i += 1;
    }
};

// None iff `tiers` is empty (the louses' rolled move tables)
pub fn pick_tier<T: Copy>(tiers: &'static [(u8, T)], ascension_level: u8) -> Option<T> {
    tiers
        .iter()
        .rev()
        .find(|&&(threshold, _)| ascension_level >= threshold)
        .map(|&(_, value)| value)
}

fn instance_monster_from_template(
    template: &MonsterTemplate,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Entity {
    // Health
    let (health_min, health_max) =
        pick_tier(template.health_tiers, ascension_level).expect("health_tiers is never empty");
    let health_max = if health_min == health_max {
        health_min
    } else {
        rng.random_range(health_min..=health_max)
    };

    // Modifiers
    let mut modifiers = MODIFIERS_ZERO;
    for &(kind, stacks) in pick_tier(template.modifier_tiers, ascension_level).unwrap_or(&[]) {
        modifier_apply(&mut modifiers, kind, stacks);
    }

    // Instance
    make_entity_monster(
        template.name,
        template.kind,
        Vitals {
            health: health_max,
            health_max,
            block: template.block_start,
        },
        modifiers,
        pick_tier(template.move_tiers, ascension_level)
            .expect("hand-spawned monster (louse); use spawn_monster"),
    )
}

pub fn spawn_monster(monster_name: MonsterName, ascension_level: u8, rng: &mut impl Rng) -> Entity {
    match monster_name {
        // Rolled bite damage + Curl Up keep the louses on hand-written spawns
        MonsterName::LouseDefensive => louse_green::spawn_monster_louse_green(ascension_level, rng),
        MonsterName::LouseNormal => louse_red::spawn_monster_louse_red(ascension_level, rng),
        other => instance_monster_from_template(monster_template(other), ascension_level, rng),
    }
}

// Named arms: a new MonsterName fails compilation here until it has a template
pub fn monster_template(monster_name: MonsterName) -> &'static MonsterTemplate {
    match monster_name {
        MonsterName::Cultist => &cultist::CULTIST,
        MonsterName::JawWorm => &jaw_worm::JAW_WORM,
        MonsterName::TheGuardian => &the_guardian::THE_GUARDIAN,
        MonsterName::FungiBeast => &fungi_beast::FUNGI_BEAST,
        MonsterName::SlaverBlue => &slaver_blue::SLAVER_BLUE,
        MonsterName::SlimeAcidSmall => &slime_acid_small::SLIME_ACID_SMALL,
        MonsterName::SlimeSpikeSmall => &slime_spike_small::SLIME_SPIKE_SMALL,
        MonsterName::GremlinFat => &gremlin_fat::GREMLIN_FAT,
        MonsterName::GremlinNob => &gremlin_nob::GREMLIN_NOB,
        MonsterName::GremlinThief => &gremlin_thief::GREMLIN_THIEF,
        MonsterName::GremlinTsundere => &gremlin_tsundere::GREMLIN_TSUNDERE,
        MonsterName::GremlinWarrior => &gremlin_warrior::GREMLIN_WARRIOR,
        MonsterName::GremlinWizard => &gremlin_wizard::GREMLIN_WIZARD,
        MonsterName::Hexaghost => &hexaghost::HEXAGHOST,
        MonsterName::Lagavulin => &lagavulin::LAGAVULIN,
        MonsterName::Looter => &looter::LOOTER,
        MonsterName::LouseDefensive => &louse_green::LOUSE_DEFENSIVE,
        MonsterName::LouseNormal => &louse_red::LOUSE_NORMAL,
        MonsterName::Sentry => &sentry::SENTRY,
        MonsterName::SlaverRed => &slaver_red::SLAVER_RED,
        MonsterName::SlimeAcidLarge => &slime_acid_large::SLIME_ACID_LARGE,
        MonsterName::SlimeAcidMedium => &slime_acid_medium::SLIME_ACID_MEDIUM,
        MonsterName::SlimeBoss => &slime_boss::SLIME_BOSS,
        MonsterName::SlimeSpikeLarge => &slime_spike_large::SLIME_SPIKE_LARGE,
        MonsterName::SlimeSpikeMedium => &slime_spike_medium::SLIME_SPIKE_MEDIUM,
        MonsterName::Byrd => &byrd::BYRD,
        MonsterName::Centurion => &centurion::CENTURION,
        MonsterName::Chosen => &chosen::CHOSEN,
        MonsterName::Healer => &healer::HEALER,
        MonsterName::Mugger => &mugger::MUGGER,
        MonsterName::ShelledParasite => &shelled_parasite::SHELLED_PARASITE,
        MonsterName::SnakePlant => &snake_plant::SNAKE_PLANT,
        MonsterName::Snecko => &snecko::SNECKO,
        MonsterName::SphericGuardian => &spheric_guardian::SPHERIC_GUARDIAN,
        MonsterName::BookOfStabbing => &book_of_stabbing::BOOK_OF_STABBING,
        MonsterName::GremlinLeader => &gremlin_leader::GREMLIN_LEADER,
        MonsterName::Taskmaster => &taskmaster::TASKMASTER,
        MonsterName::BronzeAutomaton => &bronze_automaton::BRONZE_AUTOMATON,
        MonsterName::BronzeOrb => &bronze_orb::BRONZE_ORB,
        MonsterName::Champ => &champ::CHAMP,
        MonsterName::TheCollector => &the_collector::THE_COLLECTOR,
        MonsterName::TorchHead => &torch_head::TORCH_HEAD,
        MonsterName::BanditBear => &bandit_bear::BANDIT_BEAR,
        MonsterName::BanditLeader => &bandit_leader::BANDIT_LEADER,
        MonsterName::BanditPointy => &bandit_pointy::BANDIT_POINTY,
    }
}

// Weighted gremlin pool: Warrior/Thief/Fat twice, Tsundere/Wizard once
pub const GREMLIN_POOL: [MonsterName; 8] = [
    MonsterName::GremlinWarrior,
    MonsterName::GremlinWarrior,
    MonsterName::GremlinThief,
    MonsterName::GremlinThief,
    MonsterName::GremlinFat,
    MonsterName::GremlinFat,
    MonsterName::GremlinTsundere,
    MonsterName::GremlinWizard,
];

pub fn pick_gremlin(rng: &mut impl Rng) -> MonsterName {
    GREMLIN_POOL[rng.random_range(0..GREMLIN_POOL.len())]
}

pub fn push_move_history(entity: &mut Entity, move_idx: u8) {
    let len = entity.monster_move_history_len as usize;
    if len < MAX_MOVE_HISTORY {
        entity.monster_move_history[len] = move_idx;
        entity.monster_move_history_len += 1;
    } else {
        // Marathon combat: drop the oldest, keep the last `MAX_MOVE_HISTORY` moves
        entity.monster_move_history.copy_within(1.., 0);
        entity.monster_move_history[MAX_MOVE_HISTORY - 1] = move_idx;
    }
}

fn get_move_history_slice(entity: &Entity) -> &[u8] {
    &entity.monster_move_history[..entity.monster_move_history_len as usize]
}

pub fn count_monsters_named(
    entities: &[Entity],
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    name: MonsterName,
) -> usize {
    id_monsters
        .iter()
        .flatten()
        .filter(|&&id| entities[id].monster_name == name)
        .count()
}

// True if `move_idx` ends a cycle; callers bump monster_cycle_count
pub fn is_cycle_boundary(name: MonsterName, move_idx: u8) -> bool {
    match name {
        MonsterName::TheGuardian => move_idx == the_guardian::IDX_MOVE_TWIN_SLAM as u8,
        _ => false,
    }
}

// Takes the whole arena: some AIs read other Monsters' state (ally counts, party HP)
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
        // Single-move Monsters
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
        MonsterName::Byrd => {
            byrd::get_next_move_byrd(entity.monster_move_current, history, &entity.modifiers, rng)
        }
        MonsterName::Centurion => {
            centurion::get_next_move_centurion(history, entity_id, id_monsters, rng)
        }
        MonsterName::Chosen => chosen::get_next_move_chosen(history, ascension_level, rng),
        MonsterName::Healer => {
            healer::get_next_move_healer(history, entities, id_monsters, ascension_level, rng)
        }
        // Same script as the Looter
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
        MonsterName::BookOfStabbing => {
            book_of_stabbing::get_next_move_book_of_stabbing(history, ascension_level, rng)
        }
        MonsterName::GremlinLeader => {
            gremlin_leader::get_next_move_gremlin_leader(history, entity_id, id_monsters, rng)
        }
        MonsterName::Taskmaster => 0,
        MonsterName::BronzeAutomaton => {
            bronze_automaton::get_next_move_bronze_automaton(history, ascension_level)
        }
        MonsterName::BronzeOrb => bronze_orb::get_next_move_bronze_orb(history, rng),
        MonsterName::Champ => champ::get_next_move_champ(
            history,
            entity.vitals.health,
            entity.vitals.health_max,
            ascension_level,
            rng,
        ),
        MonsterName::TheCollector => {
            the_collector::get_next_move_the_collector(history, entities, id_monsters, rng)
        }
        MonsterName::TorchHead => 0,
        MonsterName::BanditBear => {
            bandit_bear::get_next_move_bandit_bear(entity.monster_move_current, history)
        }
        MonsterName::BanditLeader => bandit_leader::get_next_move_bandit_leader(
            entity.monster_move_current,
            history,
            ascension_level,
        ),
        MonsterName::BanditPointy => 0,
    }
}

// The repeated Monster move shapes; each spells out one Effect array longhand
pub const fn move_attack(name: &'static str, damage: u16, instances: u8) -> Move {
    let mut effects = [EFFECT_ZERO; MAX_EFFECTS_PER_MOVE];
    let mut idx = 0;
    while idx < instances as usize {
        effects[idx] = Effect {
            kind: EffectKind::DamagePhysical {
                amount: damage,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        };
        idx += 1;
    }
    Move {
        name,
        effects,
        effects_len: instances,
        intent: Intent::Attack { damage, instances },
    }
}

pub const fn move_buff(name: &'static str, kind: ModifierKind, stacks: i16) -> Move {
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

pub const fn move_debuff(
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

pub const fn move_attack_debuff(
    name: &'static str,
    damage: u16,
    kind: ModifierKind,
    stacks: i16,
) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    lifesteal: false,
                },
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

pub const fn move_attack_card_add(
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
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    lifesteal: false,
                },
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

pub const fn move_block(name: &'static str, block: u16) -> Move {
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
pub const fn move_block_buff(name: &'static str, block: u16, strength: i16) -> Move {
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

pub const fn move_split(name: &'static str, first: MonsterName, second: MonsterName) -> Move {
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

// `move` is a reserved keyword, so the base constructor keeps its make_ prefix
pub const fn make_move(name: &'static str, effects: &[Effect], intent: Intent) -> Move {
    let mut arr = [EFFECT_ZERO; MAX_EFFECTS_PER_MOVE];
    let mut idx = 0;
    while idx < effects.len() {
        arr[idx] = effects[idx];
        idx += 1;
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
    moves: &'static [Move],
) -> Entity {
    Entity {
        kind: EntityKind::Monster,
        vitals,
        modifiers,
        monster_name: name,
        monster_kind,
        monster_moves: moves,
        ..ENTITY_ZERO
    }
}
