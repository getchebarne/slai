use crate::cards::make_entity_card;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static ALL_OUT_ATTACK: Entity = make_entity_card(
    CardName::AllOutAttack,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 10,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        },
        Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ALL_OUT_ATTACK_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = ALL_OUT_ATTACK.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 14,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..ALL_OUT_ATTACK
};
