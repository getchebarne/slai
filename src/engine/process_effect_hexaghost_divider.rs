use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Initial Hexaghost hit
pub fn process_effect_hexaghost_divider(id_source: Option<usize>, state: &mut GameState) {
    let id_character = state.id_character;
    let health = state.entities[id_character].vitals.health;
    let dmg: u16 = health / 12 + 1;

    for _ in 0..HEXAGHOST_DIVIDER_HITS {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: dmg },
            id_source,
            target: Target::Direct(Some(id_character)),
        });
    }
}
