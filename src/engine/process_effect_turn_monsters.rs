use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::types::Combat;

// The Character's turn is over: every Monster takes its turn, then the Character's next turn starts
pub fn process_effect_turn_monsters(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_monsters outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &state.combat;

    // A Vulnerable skips its first round end only if applied after the turn ended; Doubt's Weak and Shame's Frail still skip
    state.entities[state.id_character].modifiers.is_new[ModifierKind::Vulnerable as usize] = false;

    // Behind what the turn end left (Dead Branch's Card): Monster turn starts, Poison ticks, moves and rolls, turn ends
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::TurnStartMonster,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::PoisonTick,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::MoveExecute,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
        state.effect_queue.push_back(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: None,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
    for id_monster in id_monsters.iter().flatten().copied() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::TurnEndMonster,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    // Queue Character's turn start
    state.effect_queue.push_back(Effect {
        kind: EffectKind::TurnStartCharacter,
        id_source: None,
        target: Target::Direct(None),
    });
}
