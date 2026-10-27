use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::first_target;
use crate::game::GameState;
use crate::potions::remove_potion;
use crate::types::DeltaSign;
use crate::types::PotionName;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::monsters_alive;

// id_source is the used Potion, id_target its Monster if it needs one
pub fn process_effect_potion_use(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    at_selection: bool,
) {
    let id_target = first_target(
        state,
        EffectKind::PotionUse { at_selection },
        id_source,
        id_targets,
    );
    let id_potion = id_source.expect("PotionUse requires id_source");

    // The Potion's selected-Monster effects resolve against this use's target
    if id_target.is_some() {
        assert!(
            state.combat.active,
            "Targeted Potion use outside the Combat frame"
        );
        state.combat.id_monster_selected = id_target;
    }

    // Consume the Potion from its belt slot before its effects run
    remove_potion(&mut state.id_potions, id_potion);
    let potion = &state.entities[id_potion];

    // Smoke Bomb drops at once the selection parked behind it, every queued effect, and the waiting Card plays and turn phases
    if potion.potion_name == PotionName::SmokeBomb {
        state.effect_queue.clear();
        state.id_card_pile_selected.clear();
        state.card_play_queue.clear();
        state.phase_queue.clear();
    }

    // In combat a drink waits behind the work in progress, such as a selection's chain; out of combat, and Fruit Juice anywhere, it lands at once
    let at_once = !state.combat.active || potion.potion_name == PotionName::FruitJuice;
    for effect in potion.potion_effects.iter() {
        let mut effect = Effect {
            id_source: Some(id_potion), // Stamp the Potion's ID
            ..*effect
        };

        // Distilled Chaos rolls its targets in play order, all before any Card resolves, among the living Monsters; with none, the play goes ahead targetless
        if matches!(effect.kind, EffectKind::CardPlayFromCardPileDrawTop) {
            let id_alive: Vec<usize> =
                monsters_alive(&state.entities, &state.combat.id_monsters).collect();
            let id_monster =
                (!id_alive.is_empty()).then(|| id_alive[state.rng.random_range(0..id_alive.len())]);
            effect.target = Target::Direct(id_monster);
        }

        // Gambler's Brew reads the hand as it is drunk; Cards selected at the open selection have left it
        if matches!(effect.kind, EffectKind::Gamble { .. })
            && state
                .combat
                .id_card_pile_hand
                .iter()
                .all(|id| state.id_card_pile_selected.contains(id))
        {
            continue;
        }
        if at_once {
            state.effect_buf.push(effect);
        } else if matches!(effect.kind, EffectKind::CombatEnd { .. }) {
            // Smoke Bomb's escape waits as a phase, so the work the drink sets off resolves first
            state.phase_queue.push_back(effect);
        } else {
            state.effect_queue.push_back(effect);
        }
    }

    // Toy Ornithopter: any Potion use heals 5; in combat behind everything queued, out of combat at once
    // At a selection Smoke Bomb's escape preempts everything queued, Toy Ornithopter's heal included
    if has_relic(&state.id_relics, RelicName::ToyOrnithopter)
        && !(at_selection && potion.potion_name == PotionName::SmokeBomb)
    {
        let heal = Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        };
        if state.combat.active {
            state.effect_queue.push_back(heal);
        } else {
            state.effect_buf.push(heal);
        }
    }
    flush_effects_from_buf_to_queue_front(state);
}
