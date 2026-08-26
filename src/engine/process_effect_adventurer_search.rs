use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::monsters::lagavulin;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::Focus;
use crate::types::MonsterEncounter;
use crate::utils::context_focus;

// Dead Adventurer search: escalating chance an elite returns
pub fn process_effect_adventurer_search(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Event,
        "AdventurerSearch outside the Event context"
    );
    assert!(
        matches!(state.event.name, EventName::DeadAdventurer),
        "AdventurerSearch outside a Dead Adventurer event"
    );

    // Calculate chance
    let base: u16 = if state.ascension < 15 { 25 } else { 35 };
    let chance = base + 25 * state.event.stage as u16;

    if (state.rng.random_range(0..100) as u16) < chance {
        let encounter = match state.rng.random_range(0..3) {
            0 => MonsterEncounter::ThreeSentries,
            1 => MonsterEncounter::GremlinNob,
            2 => MonsterEncounter::Lagavulin,
            roll => unreachable!("Adventurer enemy roll out of range: {roll}"),
        };
        spawn_encounter_monsters(state, encounter);

        // The event Lagavulin spawns awake: no sleep kit, opens with Siphon Soul
        if encounter == MonsterEncounter::Lagavulin {
            let target = TARGET_MONSTERS_ALL;
            for kind in [
                EffectKind::BlockSet { amount: 0 },
                EffectKind::ModifierRemove {
                    kind: ModifierKind::Asleep,
                },
                EffectKind::ModifierRemove {
                    kind: ModifierKind::Metallicize,
                },
                EffectKind::MoveUpdate {
                    move_override: Some(lagavulin::IDX_MOVE_SIPHON),
                },
            ] {
                state.effect_queue.push_back(Effect {
                    kind,
                    id_source: None,
                    target,
                });
            }
        }
        return;
    }

    // Draw uniformly among the not-yet-found rewards; the countdown walks them
    // in fixed Gold, Nothing, Relic order and lands on the drawn one
    let num_unfound = !state.event.found_gold as usize
        + !state.event.found_nothing as usize
        + !state.event.found_relic as usize;
    let mut idx = state.rng.random_range(0..num_unfound) as i8;
    if !state.event.found_gold {
        if idx == 0 {
            state.event.found_gold = true;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(30),
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        idx -= 1;
    }
    if !state.event.found_nothing {
        if idx == 0 {
            state.event.found_nothing = true;
        }
        idx -= 1;
    }
    if !state.event.found_relic && idx == 0 {
        state.event.found_relic = true;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RelicGrantRandom { tier: None },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Advance the search count (drives the chance)
    state.event.stage += 1;

    // All rewards found: the event is over
    if state.event.found_gold && state.event.found_nothing && state.event.found_relic {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EventConsume,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
