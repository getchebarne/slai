use pyo3::prelude::*;

use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::active_modifier_kinds;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::stacks_max_for;

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "ModifierKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyModifierKind {
    Accuracy,
    AfterImage,
    Angry,
    Artifact,
    Asleep,
    Blur,
    Burst,
    Choke,
    CorpseExplosion,
    CurlUp,
    Dexterity,
    DoubleDamage,
    DrawCardNextTurn,
    Enrage,
    Entangled,
    Envenom,
    Frail,
    InfiniteBlades,
    Intangible,
    Metallicize,
    ModeShift,
    NextTurnBlock,
    NextTurnEnergy,
    NoDraw,
    NoxiousFumes,
    Phantasmal,
    PlatedArmor,
    Poison,
    Retain,
    Ritual,
    Shackled,
    SharpHide,
    Splittable,
    SporeCloud,
    Strength,
    Thievery,
    Thorns,
    ThousandCuts,
    ToolsOfTheTrade,
    Vigor,
    Vulnerable,
    Weak,
    WraithForm,
    Buffer,
    PenNib,
    Magnetism,
    NoBlock,
    Panache,
    SadisticNature,
    Mayhem,
    TheBomb,
    Regen,
    LoseStrength,
    LoseDexterity,
    Duplication,
}

#[pymethods]
impl PyModifierKind {
    #[getter]
    fn is_buff(&self) -> bool {
        modifier_is_buff(modifier_kind_from_u8(*self as u8))
    }

    // hash by discriminant (other unit enums get this via impl_discriminant_hash)
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

impl From<ModifierKind> for PyModifierKind {
    fn from(kind: ModifierKind) -> Self {
        match kind {
            ModifierKind::Accuracy => Self::Accuracy,
            ModifierKind::AfterImage => Self::AfterImage,
            ModifierKind::Angry => Self::Angry,
            ModifierKind::Artifact => Self::Artifact,
            ModifierKind::Asleep => Self::Asleep,
            ModifierKind::Blur => Self::Blur,
            ModifierKind::Burst => Self::Burst,
            ModifierKind::Choke => Self::Choke,
            ModifierKind::CorpseExplosion => Self::CorpseExplosion,
            ModifierKind::CurlUp => Self::CurlUp,
            ModifierKind::Dexterity => Self::Dexterity,
            ModifierKind::DoubleDamage => Self::DoubleDamage,
            ModifierKind::DrawCardNextTurn => Self::DrawCardNextTurn,
            ModifierKind::Enrage => Self::Enrage,
            ModifierKind::Entangled => Self::Entangled,
            ModifierKind::Envenom => Self::Envenom,
            ModifierKind::Frail => Self::Frail,
            ModifierKind::InfiniteBlades => Self::InfiniteBlades,
            ModifierKind::Intangible => Self::Intangible,
            ModifierKind::Metallicize => Self::Metallicize,
            ModifierKind::ModeShift => Self::ModeShift,
            ModifierKind::NextTurnBlock => Self::NextTurnBlock,
            ModifierKind::NextTurnEnergy => Self::NextTurnEnergy,
            ModifierKind::NoDraw => Self::NoDraw,
            ModifierKind::NoxiousFumes => Self::NoxiousFumes,
            ModifierKind::Phantasmal => Self::Phantasmal,
            ModifierKind::PlatedArmor => Self::PlatedArmor,
            ModifierKind::Poison => Self::Poison,
            ModifierKind::Retain => Self::Retain,
            ModifierKind::Ritual => Self::Ritual,
            ModifierKind::Shackled => Self::Shackled,
            ModifierKind::SharpHide => Self::SharpHide,
            ModifierKind::Splittable => Self::Splittable,
            ModifierKind::SporeCloud => Self::SporeCloud,
            ModifierKind::Strength => Self::Strength,
            ModifierKind::Thievery => Self::Thievery,
            ModifierKind::Thorns => Self::Thorns,
            ModifierKind::ThousandCuts => Self::ThousandCuts,
            ModifierKind::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            ModifierKind::Vigor => Self::Vigor,
            ModifierKind::Vulnerable => Self::Vulnerable,
            ModifierKind::Weak => Self::Weak,
            ModifierKind::WraithForm => Self::WraithForm,
            ModifierKind::Buffer => Self::Buffer,
            ModifierKind::PenNib => Self::PenNib,
            ModifierKind::Magnetism => Self::Magnetism,
            ModifierKind::NoBlock => Self::NoBlock,
            ModifierKind::Panache => Self::Panache,
            ModifierKind::SadisticNature => Self::SadisticNature,
            ModifierKind::Mayhem => Self::Mayhem,
            ModifierKind::TheBomb => Self::TheBomb,
            ModifierKind::Regen => Self::Regen,
            ModifierKind::LoseStrength => Self::LoseStrength,
            ModifierKind::LoseDexterity => Self::LoseDexterity,
            ModifierKind::Duplication => Self::Duplication,
        }
    }
}

#[pyclass(
    from_py_object,
    frozen,
    get_all,
    name = "Modifier",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModifier {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub stacks_max: i16,
}

#[pymethods]
impl PyModifier {
    #[new]
    fn new(kind: PyModifierKind, stacks: i16, stacks_max: i16) -> Self {
        Self {
            kind,
            stacks,
            stacks_max,
        }
    }
}

pub(crate) fn snapshot_modifiers(mods: &Modifiers) -> Vec<PyModifier> {
    active_modifier_kinds(mods.active)
        .map(|kind| PyModifier {
            kind: kind.into(),
            stacks: mods.stacks[kind as usize],
            stacks_max: stacks_max_for(kind),
        })
        .collect()
}
