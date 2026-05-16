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

pub static ENDLESS_AGONY: Entity = make_entity_card(
    CardName::EndlessAgony,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::MonsterPicked,
            selection: SelectionKind::Single,
        },
    }],
    &[],
    &[Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::EndlessAgony,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    PlayRestriction::Always,
);
// Upgraded
pub static ENDLESS_AGONY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ENDLESS_AGONY.card_effects;
        a[0].kind = EffectKind::DamagePhysical { amount: 6 }; // +2 damage
        a
    },
    card_on_draw_effects: &[Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::EndlessAgony,
            count: 1,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    ..ENDLESS_AGONY
};
