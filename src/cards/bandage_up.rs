use crate::effect::Amount;
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
use crate::types::DeltaSign;

pub static BANDAGE_UP: Entity = make_entity_card(
    CardName::BandageUp,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(4),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
    "Heal 4 HP. Exhaust.",
);
// Upgraded
pub static BANDAGE_UP_PLUS: Entity = Entity {
    card_upgraded: true,
    description: "Heal 6 HP. Exhaust.",
    card_effects: {
        let mut a = BANDAGE_UP.card_effects;
        a[0].kind = EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(6),
        }; // +2 heal
        a
    },
    ..BANDAGE_UP
};
