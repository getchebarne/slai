use crate::consts::TIME_WARP_PLAYS;
use crate::consts::TIME_WARP_STRENGTH;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::effect::Target;
use crate::effect::TurnEndCharacterStage;
use crate::game::GameState;
use crate::modifier::ModifierKind;

// Time Warp counts a Card play; the 12th starts the count over, ends the Character's turn early and gives every Monster Strength
pub fn process_effect_time_warp_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("TimeWarpProc requires id_target");
    assert!(
        state.combat.active,
        "process_effect_time_warp_proc outside the Combat frame"
    );

    let count = &mut state.entities[id_target].modifiers.stacks[ModifierKind::TimeWarp as usize];
    *count += 1;
    if *count < TIME_WARP_PLAYS {
        return;
    }

    // Reset count
    *count = 0;

    // Each waiting draw-top play goes to its pile untriggered with its free play spent; every other waiting play is dropped
    for card_play in state.card_play_queue.drain(..) {
        if card_play.play_source == PlaySource::DrawTop {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::CardPlayRelocate,
                id_source: None,
                target: Target::Direct(Some(card_play.id_card)),
            });
            state.effect_queue.push_back(Effect {
                kind: EffectKind::CardFreePlaySpend,
                id_source: None,
                target: Target::Direct(Some(card_play.id_card)),
            });
        }
    }

    // The turn end waits for everything queued and every play still to come, ahead of a landing already waiting
    state.combat.turn_end_pending = true;
    state.phase_queue.push_front(Effect {
        kind: EffectKind::TurnEndCharacter {
            stage: TurnEndCharacterStage::FirstPass,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    // Every living Monster gains Strength behind the relocations
    for id_monster in state.combat.id_monsters.iter().flatten().copied() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: TIME_WARP_STRENGTH,
            },
            id_source: Some(id_monster),
            target: Target::Direct(Some(id_monster)),
        });
    }
}
