use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

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
    if_poisoned: bool, // Bane
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

    // Strike Dummy: Strike-tagged cards get +3 base, before Strength/Weak/Vuln scaling
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

    // Paper Krane: Boosts Weak on monster attackers
    let weak_paper_krane = state.entities[id_actor].kind == EntityKind::Monster
        && has_relic(&state.id_relics, RelicName::PaperKrane);

    // Odd Mushroom: softens Vulnerable when the character is the target
    let vuln_odd_mushroom = state.entities[id_target].kind == EntityKind::Character
        && has_relic(&state.id_relics, RelicName::OddMushroom);

    // Calculate final base attack damage
    let mut final_damage = scale_attack_damage(
        base_damage.max(0) as u16,
        source_str_stacks,
        weak_factor(
            has_modifier(mods_source_actor, ModifierKind::Weak),
            weak_paper_krane,
        ),
        vuln_factor(
            has_modifier(mods_target, ModifierKind::Vulnerable),
            vuln_odd_mushroom,
        ),
    );

    // Double damage
    if has_modifier(mods_source_actor, ModifierKind::DoubleDamage) {
        final_damage = final_damage.saturating_mul(2);
    }

    // Pen Nib modifier (double damage)
    if has_modifier(mods_source_actor, ModifierKind::PenNib) {
        final_damage = final_damage.saturating_mul(2);
    }

    // Intangible (target)
    if has_modifier(mods_target, ModifierKind::Intangible) {
        final_damage = 1;
    }

    // Thorns: triggers per attack instance regardless of damage actually dealt
    if id_actor != id_target && has_modifier(mods_target, ModifierKind::Thorns) {
        let stacks = modifier_stacks(mods_target, ModifierKind::Thorns);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamageDeal {
                amount: stacks as u16,
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
            },
            id_source: Some(id_source),
            target: Target::Direct(Some(id_target)),
        });
    }
}
