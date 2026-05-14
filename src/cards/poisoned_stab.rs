use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static POISONED_STAB: Entity = make_entity_card(
    CardName::PoisonedStab,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Poison,
                stacks: 3,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::CardTarget,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded: +2 damage, +1 poison
pub static POISONED_STAB_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = POISONED_STAB.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 8 }; // +2 damage
        a[1].kind = EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 4, // +1 poison
        };
        a
    },
    ..POISONED_STAB
};
