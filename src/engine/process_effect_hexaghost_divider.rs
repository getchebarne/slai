use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Initial Hexaghost hit; damage was snapshotted when the move was selected
pub fn process_effect_hexaghost_divider(id_source: Option<usize>, state: &mut GameState) {
    let id_character = state.id_character;
    let id_monster = id_source.expect("HexaghostDivider requires id_source");
    let dmg = state.entities[id_monster].monster_divider_damage;

    for _ in 0..HEXAGHOST_DIVIDER_HITS {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: dmg },
            id_source,
            target: Target::Direct(Some(id_character)),
        });
    }
}
