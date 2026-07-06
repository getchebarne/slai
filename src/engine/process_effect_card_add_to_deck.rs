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
    if kind == CardKind::Curse && state.id_relics[RelicName::DarkstonePeriapt as usize].is_some() {
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

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::GameState;
    use crate::game::create_game_state;
    use crate::types::CardKind;
    use crate::types::CardName;
    use crate::types::RelicName;
    use crate::types::RewardKind;
    use crate::engine::test_support::grant_relic;
    use crate::utils::push_entity;
    use crate::utils::roll_card_rewards;

    fn game_with_relic(relic: RelicName) -> GameState {
        let mut state = create_game_state(0, 42, false);
        grant_relic(relic, &mut state.id_relics, &mut state.entities);
        state
    }

    fn add(state: &mut GameState, name: CardName) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAddToDeck {
                card_name: name,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(state);
    }

    fn gold(state: &GameState) -> u16 {
        state.entities[state.id_character].character_gold
    }

    #[test]
    fn omamori_negates_two_curses_then_wears_out() {
        let mut state = game_with_relic(RelicName::Omamori);
        let id = state.id_relics[RelicName::Omamori as usize].unwrap();
        let deck_before = state.id_deck.len();
        add(&mut state, CardName::Regret);
        add(&mut state, CardName::Regret);
        assert_eq!(state.id_deck.len(), deck_before);
        assert_eq!(state.entities[id].relic_counter, 0);
        assert!(state.entities[id].relic_used_up);
        // Third curse goes through
        add(&mut state, CardName::Regret);
        assert_eq!(state.id_deck.len(), deck_before + 1);
    }

    #[test]
    fn darkstone_periapt_pays_for_curses() {
        let mut state = game_with_relic(RelicName::DarkstonePeriapt);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 20;
        let hp_before = state.entities[id_character].vitals.health;
        let max_before = state.entities[id_character].vitals.health_max;
        add(&mut state, CardName::Regret);
        let vitals = &state.entities[id_character].vitals;
        assert_eq!(vitals.health_max, max_before + 6);
        assert_eq!(vitals.health, hp_before + 6);
        // Non-curses pay nothing
        add(&mut state, CardName::Strike);
        assert_eq!(state.entities[id_character].vitals.health_max, max_before + 6);
    }

    #[test]
    fn ceramic_fish_pays_on_both_deck_add_paths() {
        let mut state = game_with_relic(RelicName::CeramicFish);
        let gold0 = gold(&state);
        add(&mut state, CardName::Strike);
        assert_eq!(gold(&state), gold0 + 9);
        // Reward claim pushes into the deck directly, bypassing CardAddToDeck
        let id_card = push_entity(
            &mut state.entities,
            crate::cards::get_card(CardName::Backflip, false),
        );
        state.screen = crate::types::Screen::Reward;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RewardTake {
                kind: RewardKind::Card,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        process_effect_queue(&mut state);
        assert_eq!(gold(&state), gold0 + 18);
    }

    #[test]
    fn eggs_upgrade_only_their_matching_kind() {
        let mut state = game_with_relic(RelicName::MoltenEgg);
        add(&mut state, CardName::Strike);
        assert!(state.entities[*state.id_deck.last().unwrap()].card_upgraded);
        add(&mut state, CardName::Footwork);
        assert!(!state.entities[*state.id_deck.last().unwrap()].card_upgraded);

        let mut state = game_with_relic(RelicName::ToxicEgg);
        add(&mut state, CardName::Backflip);
        assert!(state.entities[*state.id_deck.last().unwrap()].card_upgraded);

        let mut state = game_with_relic(RelicName::FrozenEgg);
        add(&mut state, CardName::Footwork);
        assert!(state.entities[*state.id_deck.last().unwrap()].card_upgraded);
    }

    #[test]
    fn molten_egg_upgrades_rolled_rewards_at_roll_time() {
        let mut state = game_with_relic(RelicName::MoltenEgg);
        let mut out = Vec::new();
        roll_card_rewards(
            state.id_character,
            &mut state.entities,
            &mut state.rng,
            &mut out,
            &state.id_relics,
        );
        assert_eq!(out.len(), 3);
        for &id in &out {
            let card = &state.entities[id];
            assert_eq!(card.card_upgraded, card.card_kind == CardKind::Attack);
        }
    }
}
