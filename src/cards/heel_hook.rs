use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static HEEL_HOOK: Entity = make_entity_card(
    CardName::HeelHook,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::HeelHookProc,
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static HEEL_HOOK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = HEEL_HOOK.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 8,
            lifesteal: false,
        }; // +3 damage
        a
    },
    ..HEEL_HOOK
};
