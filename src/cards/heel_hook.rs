use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
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
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::MonsterPicked,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::HeelHookProc,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::MonsterPicked,
                selection_kind: SelectionKind::Single,
            },
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
        a[0].kind = EffectKind::DamagePhysical { amount: 8 }; // +3 damage
        a
    },
    ..HEEL_HOOK
};
