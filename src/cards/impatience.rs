use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static IMPATIENCE: Entity = make_entity_card(
    CardName::Impatience,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardDrawIfNoAttacks { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static IMPATIENCE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = IMPATIENCE.card_effects;
        effects[0].kind = EffectKind::CardDrawIfNoAttacks { count: 3 }; // +1 draw
        effects
    },
    ..IMPATIENCE
};
