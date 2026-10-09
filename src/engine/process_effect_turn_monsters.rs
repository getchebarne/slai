use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::TurnMonstersStage;
use crate::game::GameState;
use crate::types::Combat;

// The Character's turn is over: the Monsters' half of the round runs in stages, each waiting for everything queued before it
pub fn process_effect_turn_monsters(state: &mut GameState, stage: TurnMonstersStage) {
    assert!(
        state.combat.active,
        "process_effect_turn_monsters outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &state.combat;

    match stage {
        // Behind what the turn end left (Dead Branch's Card): every Monster's turn starts, its Poison tick with it
        TurnMonstersStage::TurnStarts => {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::TurnStartMonster,
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
            state.phase_queue.push_back(Effect {
                kind: EffectKind::TurnMonsters {
                    stage: TurnMonstersStage::Moves,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Once the Poison ticks have resolved, the Monsters still standing act one at a time; the round end follows the last
        TurnMonstersStage::Moves => {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.phase_queue.push_back(Effect {
                    kind: EffectKind::MoveExecute,
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
            state.phase_queue.push_back(Effect {
                kind: EffectKind::TurnMonsters {
                    stage: TurnMonstersStage::RoundEnd,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Every Monster's turn ends, the ones that joined mid-round included, then the Character's next turn starts
        TurnMonstersStage::RoundEnd => {
            for id_monster in id_monsters.iter().flatten().copied() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::TurnEndMonster,
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
            state.effect_queue.push_back(Effect {
                kind: EffectKind::TurnStartCharacter,
                id_source: None,
                target: Target::Direct(None),
            });
        }
    }
}
