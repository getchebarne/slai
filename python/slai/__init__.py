"""slai — Slay-the-Spire-like engine with a Python binding."""

from .slai import (
    GameEnv,
    # Views (unprefixed, pyo3 sets their Python names via `#[pyclass(name = ...)]`):
    Card,
    Character,
    Energy,
    GameState,
    Intent,
    Map,
    Room,
    Modifier,
    Monster,
    # Native unit enums (PyO3 mirrors of internal Rust enums):
    CardKind,
    CardColor,
    CardRarity,
    CardCostKind,
    ModifierKind,
    CandidatePool,
    RoomKind,
    # Complex enum mirrors (parent classes; variants reachable as Phase.Map etc.):
    Phase,
    Selection,
    Target,
    Effect,
    Action,
)


__all__ = [
    "GameEnv",
    # Views:
    "Card",
    "Character",
    "Energy",
    "GameState",
    "Intent",
    "Map",
    "Room",
    "Modifier",
    "Monster",
    # Unit enums:
    "CardKind",
    "CardColor",
    "CardRarity",
    "CardCostKind",
    "ModifierKind",
    "CandidatePool",
    "RoomKind",
    # Complex enums:
    "Phase",
    "Selection",
    "Target",
    "Effect",
    "Action",
]
