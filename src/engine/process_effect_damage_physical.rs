use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::RelicName;
use crate::utils::scale_attack_damage;

// Source -> actor: cards delegate to character; monsters/character self
fn get_id_actor(entities: &[Entity], id_character: usize, id_source: usize) -> usize {
    if entities[id_source].kind == EntityKind::Card {
        id_character
    } else {
        id_source
    }
}

// Physical damage: if_poisoned bails unless target Poisoned; Str+Vigor+Weak/Vuln scale, ×2 DoubleDmg, Intangible clamp, Thorns reflect
pub fn process_effect_damage_physical(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
    if_poisoned: bool,
) {
    let id_source = id_source.expect("DamagePhysical requires id_source");
    let id_target = id_target.expect("DamagePhysical requires id_target");
    let target = &state.entities[id_target];
    if if_poisoned && (target.dead || !modifier_has(&target.modifiers, ModifierKind::Poison)) {
        return;
    }

    let id_actor = get_id_actor(&state.entities, state.id_character, id_source);

    // Strike Dummy: Strike-tagged cards get +3 base, before Strength/Weak/Vuln scaling
    let source = &state.entities[id_source];
    let strike_bonus = if source.kind == EntityKind::Card
        && matches!(
            source.card_name,
            CardName::Strike | CardName::SneakyStrike | CardName::SwiftStrike
        )
        && state.id_relics[RelicName::StrikeDummy as usize].is_some()
    {
        3
    } else {
        0
    };

    let mods_actor = &state.entities[id_actor].modifiers;
    let mods_target = &target.modifiers;

    // Vigor folds into the base
    let base_with_vigor = amount
        + strike_bonus
        + if modifier_has(mods_actor, ModifierKind::Vigor) {
            modifier_stacks(mods_actor, ModifierKind::Vigor).max(0) as u16
        } else {
            0
        };
    let str_stacks = if modifier_has(mods_actor, ModifierKind::Strength) {
        modifier_stacks(mods_actor, ModifierKind::Strength)
    } else {
        0
    };
    // Paper Krane sharpens Weak only on monster attackers
    let weak_paper_krane = state.entities[id_actor].kind == EntityKind::Monster
        && state.id_relics[RelicName::PaperKrane as usize].is_some();
    let mut final_damage = scale_attack_damage(
        base_with_vigor,
        str_stacks,
        modifier_has(mods_actor, ModifierKind::Weak),
        weak_paper_krane,
        modifier_has(mods_target, ModifierKind::Vulnerable),
    );
    if modifier_has(mods_actor, ModifierKind::DoubleDamage) {
        final_damage = final_damage.saturating_mul(2);
    }
    if modifier_has(mods_actor, ModifierKind::PenNib) {
        final_damage = final_damage.saturating_mul(2);
    }
    if modifier_has(mods_target, ModifierKind::Intangible) && final_damage > 1 {
        final_damage = 1;
    }

    // Thorns: triggers per attack instance regardless of damage actually dealt
    if id_actor != id_target && modifier_has(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    if final_damage > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: final_damage,
            },
            id_source: Some(id_source),
            target: Target::Direct(Some(id_target)),
        });
    }
}
