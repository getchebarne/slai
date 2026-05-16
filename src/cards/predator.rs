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

pub static PREDATOR: Entity = make_entity_card(
    CardName::Predator,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 15 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::MonsterPicked,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DrawCardNextTurn,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static PREDATOR_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = PREDATOR.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 20 }; // +5 damage
        a
    },
    ..PREDATOR
};
