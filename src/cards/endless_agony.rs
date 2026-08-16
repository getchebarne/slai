use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
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
    &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 4,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    &[],
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::EndlessAgony,
            pile: CardPile::Hand,
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
        let mut effects = ENDLESS_AGONY.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 6,
            lifesteal: false,
        }; // +2 damage
        effects
    },
    card_effects_on_draw: &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::EndlessAgony,
            pile: CardPile::Hand,
            count: 1,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    ..ENDLESS_AGONY
};
