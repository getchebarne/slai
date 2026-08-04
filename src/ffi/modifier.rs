use pyo3::prelude::*;

use super::macros::mirror_enum;

use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::active_modifier_kinds;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::stacks_max_for;

mirror_enum!(PyModifierKind from ModifierKind, "ModifierKind", from_py_object, {
    Accuracy, AfterImage, Angry, Artifact, Asleep, Blur, Burst, Choke, CorpseExplosion, CurlUp,
    Dexterity, DoubleDamage, DrawCardNextTurn, Enrage, Entangled, Envenom, Frail,
    InfiniteBlades, Intangible, Metallicize, ModeShift, NextTurnBlock, NextTurnEnergy, NoDraw,
    NoxiousFumes, Phantasmal, PlatedArmor, Poison, Retain, Ritual, Shackled, SharpHide,
    Splittable, SporeCloud, Strength, Thievery, Thorns, ThousandCuts, ToolsOfTheTrade, Vigor,
    Vulnerable, Weak, WraithForm, Buffer, PenNib, Magnetism, NoBlock, Panache, SadisticNature,
    Mayhem, TheBomb, Regeneration, LoseStrength, LoseDexterity, DuplicateNextCardPlay,
});

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

#[pyclass(
    skip_from_py_object,
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

pub(crate) fn snapshot_modifiers(mods: &Modifiers) -> Vec<PyModifier> {
    active_modifier_kinds(mods.active)
        .map(|kind| PyModifier {
            kind: kind.into(),
            stacks: mods.stacks[kind as usize],
            stacks_max: stacks_max_for(kind),
        })
        .collect()
}
