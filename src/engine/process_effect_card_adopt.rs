use crate::effect::Amount;
use crate::effect::EFFECT_DU_VU_DOLL_RECOUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::types::CardKind;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::increase_max_hp;

// A Card is adopted in two passes: Omamori's check now, the landing after the queued work
pub fn process_effect_card_adopt(id_target: Option<usize>, state: &mut GameState, landing: bool) {
    let id_card = id_target.expect("CardAdopt requires id_target");
    let card = state.entities[id_card];

    if !landing {
        // Omamori: a charge negates the Curse outright; no other hook triggers
        if card.card_kind == CardKind::Curse
            && let Some(id_relic) = state.id_relics[RelicName::Omamori as usize]
            && state.entities[id_relic].relic_counter > 0
        {
            let relic = &mut state.entities[id_relic];
            relic.relic_counter -= 1;
            relic.relic_used_up = relic.relic_counter == 0;
            return;
        }

        // The Card lands once the queued work settles, so a Relic granted beside it sees it land
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAdopt { landing: true },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        return;
    }

    // The Card joins the deck; its obtain hooks see every Relic held by now
    state.id_card_deck.push(id_card);

    // Du-Vu Doll: a Card landing in the deck recounts its Curses
    if has_relic(&state.id_relics, RelicName::DuVuDoll) {
        state.effect_queue.push_front(EFFECT_DU_VU_DOLL_RECOUNT);
    }

    // Frozen / Molten / Toxic Egg: matching kinds are upgraded in place, keeping what the Card grew
    if !card.card_upgraded && egg_upgrades_kind(card.card_kind, &state.id_relics) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    // Ceramic Fish: 9 gold per Card that actually joins the deck
    if has_relic(&state.id_relics, RelicName::CeramicFish) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(9),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Darkstone Periapt: obtaining a Curse raises max HP by 6 (healed)
    if card.card_kind == CardKind::Curse && has_relic(&state.id_relics, RelicName::DarkstonePeriapt)
    {
        increase_max_hp(state, state.id_character, 6);
    }
}
