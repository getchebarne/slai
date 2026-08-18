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
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Static definition of a monster: everything spawn-time except per-monster rolls.
// Tier arrays are (min_ascension, value), ascending; the last satisfied entry wins
pub struct MonsterTemplate {
    pub name: MonsterName,
    pub kind: MonsterKind,
    pub health_tiers: &'static [(u8, (u16, u16))],
    pub block_start: u16,
    pub move_tiers: &'static [(u8, &'static [Move])],
    pub modifier_tiers: &'static [(u8, &'static [(ModifierKind, i16)])],
}

// None iff `tiers` is empty (the louses' rolled move tables)
pub fn pick_tier<T: Copy>(tiers: &'static [(u8, T)], ascension_level: u8) -> Option<T> {
    debug_assert!(
        tiers.windows(2).all(|w| w[0].0 < w[1].0),
        "tiers must ascend"
    );
    tiers
        .iter()
        .rev()
        .find(|&&(threshold, _)| ascension_level >= threshold)
        .map(|&(_, value)| value)
}

// Uniform instancer: one HP roll (skipped for fixed-HP monsters — a width-1
// random_range would still consume RNG), tiered moves and spawn modifiers
pub fn instance_monster_from_template(
    template: &MonsterTemplate,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> Entity {
    let (hp_min, hp_max) =
        pick_tier(template.health_tiers, ascension_level).expect("health_tiers is never empty");
    let health_max = if hp_min == hp_max {
        hp_min
    } else {
        rng.random_range(hp_min..=hp_max)
    };
    let mut modifiers = MODIFIERS_ZERO;
    for &(kind, stacks) in pick_tier(template.modifier_tiers, ascension_level).unwrap_or(&[]) {
        modifier_apply(&mut modifiers, kind, stacks);
    }
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
        MonsterName::Cultist => &cultist::TEMPLATE,
        MonsterName::JawWorm => &jaw_worm::TEMPLATE,
        MonsterName::TheGuardian => &the_guardian::TEMPLATE,
        MonsterName::FungiBeast => &fungi_beast::TEMPLATE,
        MonsterName::SlaverBlue => &slaver_blue::TEMPLATE,
        MonsterName::SlimeAcidSmall => &slime_acid_small::TEMPLATE,
        MonsterName::SlimeSpikeSmall => &slime_spike_small::TEMPLATE,
        MonsterName::GremlinFat => &gremlin_fat::TEMPLATE,
        MonsterName::GremlinNob => &gremlin_nob::TEMPLATE,
        MonsterName::GremlinThief => &gremlin_thief::TEMPLATE,
        MonsterName::GremlinTsundere => &gremlin_tsundere::TEMPLATE,
        MonsterName::GremlinWarrior => &gremlin_warrior::TEMPLATE,
        MonsterName::GremlinWizard => &gremlin_wizard::TEMPLATE,
        MonsterName::Hexaghost => &hexaghost::TEMPLATE,
        MonsterName::Lagavulin => &lagavulin::TEMPLATE,
        MonsterName::Looter => &looter::TEMPLATE,
        MonsterName::LouseDefensive => &louse_green::TEMPLATE,
        MonsterName::LouseNormal => &louse_red::TEMPLATE,
        MonsterName::Sentry => &sentry::TEMPLATE,
        MonsterName::SlaverRed => &slaver_red::TEMPLATE,
        MonsterName::SlimeAcidLarge => &slime_acid_large::TEMPLATE,
        MonsterName::SlimeAcidMedium => &slime_acid_medium::TEMPLATE,
        MonsterName::SlimeBoss => &slime_boss::TEMPLATE,
        MonsterName::SlimeSpikeLarge => &slime_spike_large::TEMPLATE,
        MonsterName::SlimeSpikeMedium => &slime_spike_medium::TEMPLATE,
        MonsterName::Byrd => &byrd::TEMPLATE,
        MonsterName::Centurion => &centurion::TEMPLATE,
        MonsterName::Chosen => &chosen::TEMPLATE,
        MonsterName::Healer => &healer::TEMPLATE,
        MonsterName::Mugger => &mugger::TEMPLATE,
        MonsterName::ShelledParasite => &shelled_parasite::TEMPLATE,
        MonsterName::SnakePlant => &snake_plant::TEMPLATE,
        MonsterName::Snecko => &snecko::TEMPLATE,
        MonsterName::SphericGuardian => &spheric_guardian::TEMPLATE,
        MonsterName::BookOfStabbing => &book_of_stabbing::TEMPLATE,
        MonsterName::GremlinLeader => &gremlin_leader::TEMPLATE,
        MonsterName::Taskmaster => &taskmaster::TEMPLATE,
        MonsterName::BronzeAutomaton => &bronze_automaton::TEMPLATE,
        MonsterName::BronzeOrb => &bronze_orb::TEMPLATE,
        MonsterName::Champ => &champ::TEMPLATE,
        MonsterName::TheCollector => &the_collector::TEMPLATE,
        MonsterName::TorchHead => &torch_head::TEMPLATE,
        MonsterName::BanditBear => &bandit_bear::TEMPLATE,
        MonsterName::BanditLeader => &bandit_leader::TEMPLATE,
        MonsterName::BanditPointy => &bandit_pointy::TEMPLATE,
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

pub fn get_move_history_slice(entity: &Entity) -> &[u8] {
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

// The repeated monster move shapes; each spells out one Effect array longhand
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
    assert!(
        effects.len() <= MAX_EFFECTS_PER_MOVE,
        "Move effects exceeds MAX_EFFECTS_PER_MOVE",
    );
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
