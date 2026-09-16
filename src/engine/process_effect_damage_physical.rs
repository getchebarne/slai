use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::RelicName;
use crate::utils::get_id_actor;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

// Physical damage: if_poisoned bails unless target Poisoned; Str+Vigor+Weak/Vuln scale, x2 DoubleDmg, Intangible clamp, Thorns reflect
pub fn process_effect_damage_physical(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u16,
    if_poisoned: bool, // Bane
    lifesteal: bool,   // Life Suck
) {
    let id_source = id_source.expect("DamagePhysical requires id_source");
    let id_target = id_target.expect("DamagePhysical requires id_target");

    // Check if the target is poisoned if `if_poisoned`. If not, early return
    let target = &state.entities[id_target];
    if if_poisoned && (target.dead || !has_modifier(&target.modifiers, ModifierKind::Poison)) {
        return;
    }

    // Intialize variable to accumulate base damage
    let mut base_damage = amount as i16;

    // Flight is read from the snapshot the play took, not from the live stacks
    let flight = match state
        .combat
        .id_monsters
        .iter()
        .position(|&slot| slot == Some(id_target))
    {
        Some(slot) => state.combat.flight_baked[slot],
        None => has_modifier(&state.entities[id_target].modifiers, ModifierKind::Flight),
    };

    // Strike Dummy: Strike-tagged Cards get +3 base, before Strength/Weak/Vuln scaling
    let source = &state.entities[id_source];
    if source.kind == EntityKind::Card
        && matches!(
            source.card_name,
            CardName::Strike | CardName::SneakyStrike | CardName::SwiftStrike
        )
        && has_relic(&state.id_relics, RelicName::StrikeDummy)
    {
        base_damage += 3;
    }

    // Get the source _actor_ id (Character or Monster)
    let id_actor = get_id_actor(&state.entities, state.id_character, id_source);

    // A dying attacker's remaining hits are cancelled
    if state.entities[id_actor].dead {
        return;
    }

    // Get the source and target actor's modifiers
    let mods_source_actor = &state.entities[id_actor].modifiers;
    let mods_target = &target.modifiers;

    // Vigor
    if has_modifier(mods_source_actor, ModifierKind::Vigor) {
        base_damage += modifier_stacks(mods_source_actor, ModifierKind::Vigor);
    }

    // Strength
    let source_str_stacks = if has_modifier(mods_source_actor, ModifierKind::Strength) {
        modifier_stacks(mods_source_actor, ModifierKind::Strength)
    } else {
        0
    };

    // Paper Krane: Boosts Weak on Monster attackers
    let weak_paper_krane = state.entities[id_actor].kind == EntityKind::Monster
        && has_relic(&state.id_relics, RelicName::PaperKrane);

    // Odd Mushroom: softens Vulnerable when the Character is the target
    let vuln_odd_mushroom = state.entities[id_target].kind == EntityKind::Character
        && has_relic(&state.id_relics, RelicName::OddMushroom);

    // Calculate final base attack damage
    let mut final_damage = scale_attack_damage(
        base_damage.max(0) as u16,
        source_str_stacks,
        has_modifier(mods_source_actor, ModifierKind::DoubleDamage),
        has_modifier(mods_source_actor, ModifierKind::PenNib),
        weak_factor(
            has_modifier(mods_source_actor, ModifierKind::Weak),
            weak_paper_krane,
        ),
        vuln_factor(
            has_modifier(mods_target, ModifierKind::Vulnerable),
            vuln_odd_mushroom,
        ),
        flight,
    );

    // Intangible (target): clamps down, so a computed 0 stays 0
    if has_modifier(mods_target, ModifierKind::Intangible) && final_damage > 1 {
        final_damage = 1;
    }

    // Executes in reverse:
    //     1. DamageDeal (attack)
    //     2. DamageDeal (Thorns reflect)
    // Thorns: triggers per attack instance regardless of damage actually dealt
    if id_actor != id_target && has_modifier(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
                lifesteal: false,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    // Queue final damage effect
    if final_damage > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: final_damage,
                lifesteal,
            },
            id_source: Some(id_source),
            target: Target::Direct(Some(id_target)),
        });
    }
}
