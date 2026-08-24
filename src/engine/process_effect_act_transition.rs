use crate::consts::UNKNOWN_CHANCE_BASE_MONSTER;
use crate::consts::UNKNOWN_CHANCE_BASE_SHOP;
use crate::consts::UNKNOWN_CHANCE_BASE_TREASURE;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::POOL_EVENT_SHRINES;
use crate::events::pools_for_act;
use crate::game::GameState;
use crate::map::generate_map;
use crate::monsters::encounters::generate_act_monsters;
use crate::monsters::encounters::pick_boss;
use crate::types::DeltaSign;

pub fn process_effect_act_transition(state: &mut GameState) {
    // Increase current Act number
    state.act += 1;

    // Heal to full; A5+ heals 75% of the missing health instead
    let vitals = &state.entities[state.id_character].vitals;
    let missing = vitals.health_max - vitals.health;
    let amount = if state.ascension >= 5 {
        (missing as f32 * 0.75).round() as u16
    } else {
        missing
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    // Fresh map; stale Room entities stay in the arena, unreachable via `id_rooms`
    let (id_rooms, location) = generate_map(&mut state.rng, &mut state.entities, state.ascension);
    state.id_rooms = id_rooms;
    state.location = location;

    // Re-roll the act's Encounter Pools and Boss
    state.encounter_pool_normal.clear();
    state.encounter_pool_elite.clear();
    generate_act_monsters(
        state.act,
        &mut state.encounter_pool_normal,
        &mut state.encounter_pool_elite,
        &mut state.rng,
    );
    state.encounter_boss = pick_boss(state.act, &mut state.rng);

    // Events: the Act list replaces, shrines re-add fresh, run-scoped specials carry over
    let (pool_events, pool_special_additions) = pools_for_act(state.act);
    state.pool_events = pool_events.to_vec();
    state
        .pool_event_special
        .retain(|name| !POOL_EVENT_SHRINES.contains(name));
    state
        .pool_event_special
        .extend_from_slice(POOL_EVENT_SHRINES);
    state
        .pool_event_special
        .extend_from_slice(pool_special_additions);

    // ?-Room drift and the Potion swing reset between acts
    state.unknown_chance_monster = UNKNOWN_CHANCE_BASE_MONSTER;
    state.unknown_chance_shop = UNKNOWN_CHANCE_BASE_SHOP;
    state.unknown_chance_treasure = UNKNOWN_CHANCE_BASE_TREASURE;
    state.potion_drop_mod = 0;
}
