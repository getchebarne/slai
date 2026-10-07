use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::remove_potion;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

// id_source is the used Potion, id_target its Monster if it needs one
pub fn process_effect_potion_use(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
) {
    let id_potion = id_source.expect("PotionUse requires id_source");

    // The Potion's picked-Monster effects resolve against this use's target
    if id_target.is_some() {
        assert!(
            state.combat.active,
            "Targeted Potion use outside the Combat frame"
        );
        state.combat.id_monster_picked = id_target;
    }

    // Consume the Potion from its belt slot before its effects run
    remove_potion(&mut state.id_potions, id_potion);
    let potion = &state.entities[id_potion];

    // Push the Potions's on-use effects
    for effect in potion.potion_effects.iter() {
        let mut effect = Effect {
            id_source: Some(id_potion), // Stamp the Potion's ID
            ..*effect
        };

        // Distilled Chaos rolls its targets in play order, all before any Card resolves
        if matches!(effect.kind, EffectKind::CardPlayFromDrawTop) {
            let alive: Vec<usize> = state.combat.id_monsters.iter().flatten().copied().collect();
            let id_monster = alive[state.rng.random_range(0..alive.len())];
            effect.target = Target::Direct(Some(id_monster));
        }
        state.effect_buf.push(effect);
    }
    flush_effects_from_buf_to_queue_front(state);

    // Toy Ornithopter: any Potion use heals 5, in or out of combat
    if has_relic(&state.id_relics, RelicName::ToyOrnithopter) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
}
