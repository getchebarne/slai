use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::get_id_actor;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::strike_dummy_bonus;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

// Physical damage on a list of targets: every target's hit, then each Thorns reflect, the last target's first, then the remaining hits; Str+Vigor+Weak/Vuln scale it unless it was scaled as its play started, x2 DoubleDmg, Intangible clamp, if_poisoned needs Poison
#[allow(clippy::too_many_arguments)]
pub fn process_effect_damage_physical(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    amount: u16,
    instances: u16,
    if_poisoned: bool,   // Bane
    lifesteal: bool,     // Life Suck
    scaled_at: ScaledAt, // Skewer: PlayStart
) {
    let id_source = id_source.expect("DamagePhysical requires id_source");
    assert!(!id_targets.is_empty(), "DamagePhysical requires id_target");
    assert!(instances > 0, "DamagePhysical requires at least one hit");
    assert!(
        instances == 1 || id_targets.len() == 1,
        "DamagePhysical lands several hits on one target only"
    );

    // Get the source _actor_ id (Character or Monster)
    let id_actor = get_id_actor(&state.entities, state.id_character, id_source);

    // A dying or half-dead attacker's remaining hits are cancelled
    let actor = &state.entities[id_actor];
    if actor.dead || actor.monster_half_dead {
        return;
    }
    let mods_source_actor = &actor.modifiers;

    // Intialize variable to accumulate base damage
    let mut base_damage = amount as i16;

    // Strike Dummy: Strike-tagged Cards get +3 base, before Strength/Weak/Vuln scaling
    let source = &state.entities[id_source];
    if source.kind == EntityKind::Card {
        base_damage += strike_dummy_bonus(source.card_name, &state.id_relics) as i16;
    }

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
    let weak_paper_krane =
        actor.kind == EntityKind::Monster && has_relic(&state.id_relics, RelicName::PaperKrane);

    // Every hit is computed as the effect resolves, before any of them lands
    state.effect_buf.clear();
    let mut reflects = Vec::new();
    let mut hits_remaining = None;
    for &id_target in id_targets {
        // A target an earlier hit killed or left half-dead takes nothing more
        let target = &state.entities[id_target];
        if target.dead || target.monster_half_dead {
            continue;
        }

        // Bane: the second hit only lands if the target is still Poisoned
        let mods_target = &target.modifiers;
        if if_poisoned && !has_modifier(mods_target, ModifierKind::Poison) {
            continue;
        }

        // Odd Mushroom: softens Vulnerable when the Character is the target
        let vuln_odd_mushroom = target.kind == EntityKind::Character
            && has_relic(&state.id_relics, RelicName::OddMushroom);

        // Calculate final base attack damage; a hit scaled as its play started keeps its amount
        let mut final_damage = if scaled_at == ScaledAt::PlayStart {
            amount
        } else {
            scale_attack_damage(
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
                has_modifier(mods_target, ModifierKind::Flight),
            )
        };

        // Intangible (target): clamps down, so a computed 0 stays 0
        if has_modifier(mods_target, ModifierKind::Intangible) && final_damage > 1 {
            final_damage = 1;
        }

        // Queue final damage effect
        if final_damage > 0 {
            state.effect_buf.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: final_damage,
                    lifesteal,
                },
                id_source: Some(id_source),
                target: Target::Direct(Some(id_target)),
            });
        }

        // Thorns: triggers per attack instance regardless of damage actually dealt
        if id_actor != id_target && has_modifier(mods_target, ModifierKind::Thorns) {
            reflects.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: modifier_stacks(mods_target, ModifierKind::Thorns) as u16,
                    lifesteal: false,
                },
                id_source: None,
                target: Target::Direct(Some(id_actor)),
            });
        }

        // The remaining hits on the one target follow its first hit's reflect
        if instances > 1 {
            hits_remaining = Some(Effect {
                kind: EffectKind::DamagePhysical {
                    amount,
                    instances: instances - 1,
                    lifesteal,
                    if_poisoned,
                    scaled_at,
                },
                id_source: Some(id_source),
                target: Target::Direct(Some(id_target)),
            });
        }
    }
    state.effect_buf.extend(reflects.into_iter().rev());
    state.effect_buf.extend(hits_remaining);
    flush_effects_from_buf_to_queue_front(state);
}
