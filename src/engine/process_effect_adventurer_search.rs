use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EventPayload;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::monsters::lagavulin;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::MonsterEncounter;

// Dead Adventurer search: escalating chance an elite returns
pub fn process_effect_adventurer_search(state: &mut GameState) {
    let Mode::Event {
        payload:
            EventPayload::DeadAdventurer {
                found_gold,
                found_nothing,
                found_relic,
                searches,
            },
        ..
    } = &mut state.mode
    else {
        unreachable!("AdventurerSearch outside a Dead Adventurer event")
    };

    // Calculate chance
    let base: u16 = if state.ascension < 15 { 25 } else { 35 };
    let chance = base + 25 * *searches as u16;

    if (state.rng.random_range(0..100) as u16) < chance {
        // The un-found loot folds into the fight's rewards, riding on CombatStart;
        // the spawn chain replaces this event with the combat
        let gold_extra = !*found_gold as u16 * 30;
        let relic_roll = !*found_relic;

        // Rolled at wake, not at spawn: the identity is hidden state with no
        // observable consequence before the fight
        let encounter = match state.rng.random_range(0..3) {
            0 => MonsterEncounter::ThreeSentries,
            1 => MonsterEncounter::GremlinNob,
            2 => MonsterEncounter::Lagavulin,
            roll => unreachable!("adventurer enemy roll out of range: {roll}"),
        };
        spawn_encounter_monsters(
            state,
            encounter,
            Some(Amount::Range {
                min: 25 + gold_extra,
                max: 35 + gold_extra,
            }),
            None,
            relic_roll,
        );

        // The event Lagavulin spawns awake: no sleep kit, opens with Siphon Soul
        if encounter == MonsterEncounter::Lagavulin {
            let target = Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::All,
                },
                selection_kind: SelectionKind::All,
            };
            for kind in [
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
    let num_unfound = !*found_gold as usize + !*found_nothing as usize + !*found_relic as usize;
    let mut idx = state.rng.random_range(0..num_unfound) as i8;
    if !*found_gold {
        if idx == 0 {
            *found_gold = true;
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
    if !*found_nothing {
        if idx == 0 {
            *found_nothing = true;
        }
        idx -= 1;
    }
    if !*found_relic && idx == 0 {
        *found_relic = true;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Advance the search count (drives the chance)
    *searches += 1;

    // All rewards found: the event is over
    if *found_gold && *found_nothing && *found_relic {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EventConsume,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
