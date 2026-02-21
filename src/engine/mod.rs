pub mod process_effect_add_shivs;
pub mod process_effect_calculated_gamble;
pub mod process_effect_card_active_clear;
pub mod process_effect_card_active_set;
pub mod process_effect_card_discard;
pub mod process_effect_card_discard_all;
pub mod process_effect_card_draw;
pub mod process_effect_card_exhaust;
pub mod process_effect_card_play;
pub mod process_effect_card_remove;
pub mod process_effect_card_upgrade;

use crate::effect::{Effect, EffectTemplate, SelectionKind, TargetKind};
use crate::monsters::Monster;
use crate::state::GameState;
use crate::types::ActorId;

pub enum ProcessEffectResult {
    Continue { bot: Vec<Effect>, top: Vec<Effect> },
    Pass,
    Halt,
    Pause,
}

fn resolve_target_kind(
    target_kind: TargetKind,
    source: ActorId,
    card_target: Option<u8>,
    monsters: &Vec<Monster>,
) -> Vec<ActorId> {
    match target_kind {
        TargetKind::CardTarget => vec![ActorId::Monster(card_target.unwrap())],
        TargetKind::Character => vec![ActorId::Character],
        TargetKind::AllMonsters => (0..monsters.len())
            .map(|i| ActorId::Monster(i as u8))
            .collect(),
        TargetKind::Source => vec![source],
    }
}

/// Instantiate an EffectTemplate into one or more runtime Effects.
fn instantiate_templates(
    templates: &[EffectTemplate],
    source: ActorId,
    card_target: Option<u8>,
    monsters: &Vec<Monster>,
) -> Vec<Effect> {
    let mut out = Vec::new();
    for tmpl in templates {
        match *tmpl {
            EffectTemplate::DamagePhysical { base, target } => {
                for actor in resolve_target_kind(target, source, card_target, monsters) {
                    out.push(Effect::DamagePhysical {
                        source,
                        target: actor,
                        base,
                    });
                }
            }
            EffectTemplate::BlockGain { amount, target } => {
                for actor in resolve_target_kind(target, source, card_target, monsters) {
                    out.push(Effect::BlockGain {
                        target: actor,
                        amount,
                        from_card: true,
                    });
                }
            }
            EffectTemplate::ModifierGain {
                kind,
                stacks,
                target,
            } => {
                for actor in resolve_target_kind(target, source, card_target, monsters) {
                    out.push(Effect::ModifierGain {
                        target: actor,
                        kind,
                        stacks,
                    });
                }
            }
            EffectTemplate::ModifierRemove { kind, target } => {
                for actor in resolve_target_kind(target, source, card_target, monsters) {
                    out.push(Effect::ModifierRemove {
                        target: actor,
                        kind,
                    });
                }
            }
            EffectTemplate::EnergyGain { amount } => {
                out.push(Effect::EnergyGain { amount });
            }
            EffectTemplate::AddShivs { count } => {
                out.push(Effect::AddShivs { count });
            }
            EffectTemplate::CardDraw { count } => {
                out.push(Effect::CardDraw { count });
            }
            EffectTemplate::CardDiscard { selection } => {
                match selection {
                    SelectionKind::Input => {
                        out.push(Effect::AwaitDiscard);
                    }
                    SelectionKind::Random => {
                        // Random discard handled at processing time
                        out.push(Effect::CardDiscardAll); // placeholder: actually random 1
                        // We'll handle this properly in the discard handler
                    }
                }
            }
            EffectTemplate::CalculatedGamble => {
                out.push(Effect::CalculatedGamble);
            }
        }
    }
    out
}

// pub fn process_effect(effect: Effect, game_state: &mut GameState) -> ProcessEffectResult {
//     match effect {
//         Effect::CardDraw { count } =>
//     }
// }
