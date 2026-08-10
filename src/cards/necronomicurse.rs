use crate::cards::make_entity_card;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

// Necronomicon's companion curse; unremovable (the source game respawns it on removal)
pub static NECRONOMICURSE: Entity = make_entity_card(
    CardName::Necronomicurse,
    CardKind::Curse,
    CardColor::Curse,
    CardRarity::Curse,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[],
    &[],
    &[],
    PlayRestriction::Never,
);
