use crate::cards::card_template;
use crate::cards::get_card;
use crate::effect::EffectKind;
use crate::game::GameState;

pub fn process_effect_card_upgrade(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardUpgrade requires id_target");
    let card = state.entities[id_target];

    // Early return if already upgraded (e.g., Apotheosis)
    if card.card_upgraded {
        return;
    }

    // Get upgraded variant
    let name = card.card_name;
    let base = card_template(name, false);
    let mut card_upgraded = get_card(name, true);

    // upgrade() adds to the live instance, so in-combat growth and decay survive it
    let slots = (card_upgraded.card_effects_len as usize)
        .min(base.effects_len as usize)
        .min(card.card_effects_len as usize);
    for idx in 0..slots {
        let delta = match (base.effects[idx].kind, card.card_effects[idx].kind) {
            (
                EffectKind::DamagePhysical {
                    amount: printed, ..
                },
                EffectKind::DamagePhysical { amount: live, .. },
            )
            | (EffectKind::BlockGain { amount: printed }, EffectKind::BlockGain { amount: live }) => {
                live as i32 - printed as i32
            }
            _ => 0,
        };
        if delta == 0 {
            continue;
        }
        match &mut card_upgraded.card_effects[idx].kind {
            EffectKind::DamagePhysical { amount, .. } | EffectKind::BlockGain { amount } => {
                *amount = (*amount as i32 + delta).clamp(0, u16::MAX as i32) as u16;
            }
            _ => {}
        }
    }

    // Snapshot runtime-preserved fields: cost, cost override, bottled status
    let cost = if card_upgraded.card_cost == base.cost {
        card.card_cost
    } else {
        card_upgraded.card_cost
    };

    // Overwrite non-upgraded variant with upgraded one
    state.entities[id_target] = card_upgraded;

    // Stamp preserved fields
    state.entities[id_target].card_cost = cost;
    state.entities[id_target].card_cost_override = card.card_cost_override;
    state.entities[id_target].card_bottled = card.card_bottled;
}
