use crate::utils::has_relic;
use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::Amount;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::push_entity;

pub fn process_effect_card_add_to_deck(state: &mut GameState, card_name: CardName, upgraded: bool) {
    let kind = get_card(card_name, false).card_kind;

    // Omamori: negate the next N curses outright; used up at 0 charges
    if kind == CardKind::Curse
        && let Some(id) = state.id_relics[RelicName::Omamori as usize]
        && state.entities[id].relic_counter > 0
    {
        let relic = &mut state.entities[id];
        relic.relic_counter -= 1;
        if relic.relic_counter == 0 {
            relic.relic_used_up = true;
        }
        return;
    }

    // Darkstone Periapt: obtaining a curse raises max HP by 6 and heals 6
    if kind == CardKind::Curse && has_relic(&state.id_relics, RelicName::DarkstonePeriapt) {
        let id_character = state.id_character;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(6),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(6),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    let upgraded = upgraded || egg_upgrades_kind(kind, &state.id_relics);
    let id = push_entity(&mut state.entities, get_card(card_name, upgraded));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdopt,
        id_source: None,
        target: Target::Direct(Some(id)),
    });
}
