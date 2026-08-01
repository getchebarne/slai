"""Terminal UI for slai. Text-based, minimal, phase-switched.

Run with: python -m play
Global keys: Q = quit, R = new run (new seed).
"""

import curses
import os
import random
import sys

# Run-in-place dev tool: make the in-repo `slai` package importable without
# depending on an editable install in whatever venv happens to be active.
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "python"))

import slai
from slai import (
    Action,
    ActionType,
    ModeChest,
    ModeChestOpened,
    ModeCombat,
    ModeCombatEnded,
    ModeEvent,
    ModeMap,
    ModeRestSite,
    ModeReward,
    ModeShop,
    CardCostKindGrowsOnDamageInstanceTaken,
    CardCostKindMinusDiscardsThisTurn,
    CardCostKindXCost,
    CardKind,
    EffectCardDiscard,
    EffectCardDuplicate,
    EffectCardNightmarePick,
    EffectCardPurge,
    EffectCardRetain,
    EffectCardSetupPick,
    EffectCardTransform,
    EffectCardUpgrade,
    IntentKind,
    ModifierKind,
    RoomKind,
    SelectionKindInput,
)


# ---------- color pairs ----------
# Each pair encodes exactly one semantic axis; backgrounds default (-1) so hue
# always means data, never chrome.
CP_SELECTED = 1  # black on white — the single focus (reverse video, dominates)
CP_HP = 2        # red — health / attack intent / poison damage (the danger channel)
CP_BLOCK = 3     # cyan — block / defensive intent
CP_GOLD = 4      # yellow — economy: gold, energy, shop prices, cost badges
CP_GOOD = 5      # green — poison stacks, buffs, buff intent
CP_BORDER = 6    # blue — the focused panel's border + title


def init_colors() -> None:
    curses.start_color()
    curses.use_default_colors()
    curses.init_pair(CP_SELECTED, curses.COLOR_BLACK, curses.COLOR_WHITE)
    curses.init_pair(CP_HP, curses.COLOR_RED, -1)
    curses.init_pair(CP_BLOCK, curses.COLOR_CYAN, -1)
    curses.init_pair(CP_GOLD, curses.COLOR_YELLOW, -1)
    curses.init_pair(CP_GOOD, curses.COLOR_GREEN, -1)
    curses.init_pair(CP_BORDER, curses.COLOR_BLUE, -1)


# Box-drawing glyph sets. DOUBLE marks a focused/selected element; ASCII is the
# fallback when a terminal lacks the box-drawing characters.
THIN = dict(tl="┌", tr="┐", bl="└", br="┘", h="─", v="│", lt="├", rt="┤")
DOUBLE = dict(tl="╔", tr="╗", bl="╚", br="╝", h="═", v="║", lt="╠", rt="╣")
ASCII = dict(tl="+", tr="+", bl="+", br="+", h="-", v="|", lt="+", rt="+")
BAR_FULL, BAR_EMPTY = "▓", "░"
PIP_FULL, PIP_EMPTY = "●", "○"
_ACS_OK = True  # probed once at startup; falls back to ASCII glyphs/bars if False


def _probe_acs(stdscr) -> None:
    """Detect whether box-drawing glyphs render; fall back to ASCII if not."""
    global _ACS_OK, BAR_FULL, BAR_EMPTY, PIP_FULL, PIP_EMPTY
    try:
        stdscr.addstr(0, 0, "┌")
        stdscr.addstr(0, 0, " ")
    except curses.error:
        _ACS_OK = False
        BAR_FULL, BAR_EMPTY, PIP_FULL, PIP_EMPTY = "#", ".", "*", "-"


def glyphs(focus: bool) -> dict:
    if not _ACS_OK:
        return ASCII
    return DOUBLE if focus else THIN


# ---------- drawing primitives ----------
def safe_addstr(stdscr, y: int, x: int, s: str, attr: int = 0) -> None:
    try:
        stdscr.addstr(y, x, s, attr)
    except curses.error:
        pass


def write(
    stdscr,
    y: int,
    x: int,
    text: str,
    *,
    selected: bool = False,
    dim: bool = False,
    bold: bool = False,
    hp: bool = False,
    block: bool = False,
    gold: bool = False,
    good: bool = False,
    border: bool = False,
) -> None:
    """Composite writer. Color precedence: selected > hp > block > gold > good >
    border; dim/bold OR in on top."""
    attr = 0
    if selected:
        attr |= curses.color_pair(CP_SELECTED)
    elif hp:
        attr |= curses.color_pair(CP_HP)
    elif block:
        attr |= curses.color_pair(CP_BLOCK)
    elif gold:
        attr |= curses.color_pair(CP_GOLD)
    elif good:
        attr |= curses.color_pair(CP_GOOD)
    elif border:
        attr |= curses.color_pair(CP_BORDER)
    if dim:
        attr |= curses.A_DIM
    if bold:
        attr |= curses.A_BOLD
    safe_addstr(stdscr, y, x, text, attr)


def write_phase_title(stdscr, title: str) -> None:
    """Phase title as plain bold text at the top-left corner."""
    write(stdscr, 0, 2, title, bold=True)


def write_segments(stdscr, y: int, end_x: int, segments: list) -> None:
    """Right-aligns a list of (text, kwargs) segments to end at `end_x`."""
    total = sum(len(text) for text, _ in segments)
    cx = end_x - total
    for text, kwargs in segments:
        write(stdscr, y, cx, text, **(kwargs or {}))
        cx += len(text)


def sel_prefix(selected: bool) -> str:
    return "> " if selected else "  "


# ---------- panel primitives (addstr-only; no curses subwindows) ----------
def _clip(stdscr, top, left, bottom, right):
    """Clamp a panel rect into the screen; return (top,left,bottom,right) or None."""
    h, w = stdscr.getmaxyx()
    top, left = max(0, top), max(0, left)
    bottom, right = min(h - 1, bottom), min(w - 1, right)
    if bottom - top < 1 or right - left < 1:
        return None
    return top, left, bottom, right


def draw_box(stdscr, top, left, bottom, right, title="", *, focus=False, right_title=""):
    """Single-line titled border, drawn via safe_addstr. Returns the interior
    rect (iy, ix, ih, iw) for callers to clip content into, or None if too small."""
    r = _clip(stdscr, top, left, bottom, right)
    if r is None:
        return None
    top, left, bottom, right = r
    g = glyphs(focus)
    w = right - left + 1
    bkw = {"border": True} if focus else {"dim": True}
    # top + bottom edges
    write(stdscr, top, left, g["tl"] + g["h"] * (w - 2) + g["tr"], **bkw)
    write(stdscr, bottom, left, g["bl"] + g["h"] * (w - 2) + g["br"], **bkw)
    # side edges
    for y in range(top + 1, bottom):
        write(stdscr, y, left, g["v"], **bkw)
        write(stdscr, y, right, g["v"], **bkw)
    # titles inset into the top edge
    if title and w > 6:
        t = f" {title[:w - 6]} "
        write(stdscr, top, left + 2, t, selected=focus, bold=not focus)
    if right_title and w > len(right_title) + 8:
        rt = f" {right_title} "
        write(stdscr, top, right - 1 - len(rt), rt, **bkw)
    return (top + 1, left + 1, bottom - top - 1, right - left - 1)


def bar(stdscr, y, x, width, cur, mx, *, hp=False, block=False, gold=False, label="", show_num=True):
    """Proportional fill bar. Degrades to plain text below 8 cols. Never asserts
    cur<=mx (energy/block can exceed max)."""
    prefix = f"{label} " if label else ""
    suffix = f" {cur}/{mx}" if show_num else ""
    color = {"hp": hp, "block": block, "gold": gold}
    cx = x
    if prefix:
        write(stdscr, y, cx, prefix, dim=True)
        cx += len(prefix)
    field = max(0, width - len(prefix) - len(suffix))
    if field >= 4:
        filled = round(field * max(0, min(cur, mx)) / mx) if mx > 0 else 0
        write(stdscr, y, cx, BAR_FULL * filled, **color)
        write(stdscr, y, cx + filled, BAR_EMPTY * (field - filled), dim=True)
        cx += field
    if suffix:
        write(stdscr, y, cx, suffix, **color)


def energy_pips(stdscr, y, x, cur, mx) -> int:
    """Energy as pips, with overflow text when cur>mx or mx is large. Returns cols drawn."""
    if mx > 9 or mx < 0:
        s = f"E {cur}/{mx}"
        write(stdscr, y, x, s, gold=True)
        return len(s)
    pips = PIP_FULL * min(cur, mx) + PIP_EMPTY * max(mx - cur, 0)
    extra = f" {cur}/{mx}" if cur > mx else ""
    write(stdscr, y, x, pips + extra, gold=True)
    return len(pips) + len(extra)


def highlight_row(stdscr, y, x, width, text, *, selected=False, disabled=False,
                  right_text="", right_kw=None):
    """One selectable list row padded to `width`; full-width reverse when selected."""
    body = (sel_prefix(selected) + text)[:width]
    body = body + " " * (width - len(body))
    write(stdscr, y, x, body, selected=selected, dim=disabled and not selected)
    if right_text:
        write_segments(stdscr, y, x + width, [(right_text, right_kw or {"gold": True})])


def panel_list(stdscr, top, left, bottom, right, items, sel_idx, *, title="",
               focus=False, render_row=None, right_title=""):
    """Scrolling vertical list inside a draw_box. Windows items to the interior
    height (supersedes the ad-hoc WINDOW=12 scroll). Returns the scroll offset."""
    interior = draw_box(stdscr, top, left, bottom, right, title, focus=focus, right_title=right_title)
    if interior is None:
        return 0
    iy, ix, ih, iw = interior
    n = len(items)
    if n == 0:
        return 0
    sel = max(0, min(sel_idx, n - 1))
    start = 0 if n <= ih else max(0, min(sel - ih // 2, n - ih))
    end = min(n, start + ih)
    rr = render_row or (lambda s, yy, xx, ww, it, issel: highlight_row(s, yy, xx, ww, str(it), selected=issel))
    for row, i in enumerate(range(start, end)):
        rr(stdscr, iy + row, ix, iw, items[i], i == sel)
    if start > 0:
        write(stdscr, iy, right, "↑" if _ACS_OK else "^", dim=True)
    if end < n:
        write(stdscr, bottom - 1, right, "↓" if _ACS_OK else "v", dim=True)
    return start


# ---------- cursor state ----------
class Cursor:
    def __init__(self) -> None:
        # Track the (active, pending-halt Effect type) pair. Cursor state resets
        # when either changes (new screen). Halts are tracked by Effect type so
        # two Discard halts with different counts count as the same
        self.screen_key: tuple | None = None
        self.idx: int = 0
        self.target: int = 0
        self.mode: str = "primary"  # primary | target | upgrade | purge | potion | potion_target
        self.selected: set[int] = set()
        self.potion_slot: int = 0
        self.error: str | None = None


def _combat(view):
    return view.mode if isinstance(view.mode, ModeCombat) else None


class _NoEnergy:
    energy_current = 0
    energy_max = 0


def _hand(view):
    c = _combat(view)
    return c.hand if c else []


def _monsters(view):
    c = _combat(view)
    return c.monsters if c else []


def _discover(view):
    c = _combat(view)
    return c.discover if c else []


def _screen_key(view) -> tuple:
    halt_type = type(view.pending) if view.pending is not None else None
    return (view.game_over, type(view.mode).__name__, halt_type)


def reset_if_phase_changed(cursor: Cursor, view) -> None:
    key = _screen_key(view)
    if cursor.screen_key == key:
        return
    cursor.screen_key = key
    cursor.idx = 0
    cursor.target = 0
    cursor.mode = "primary"
    cursor.selected = set()


# ---------- engine-type helpers ----------
def _input_count(view):
    """Pick count for an Input-selection halt (Discard/Retain); None otherwise."""
    eff = view.pending
    tgt = getattr(eff, "target", None) if eff is not None else None
    sk = getattr(tgt, "selection_kind", None) if tgt is not None else None
    return sk.count if isinstance(sk, SelectionKindInput) else None


# Snapshot enums are native PyO3 unit enums that compare == to the IntEnum
# shims but hash differently, so dict/set lookups must normalize to int.
def legal_set(legal) -> set:
    return {(int(a.action_type), tuple(a.idxs)) for a in legal}


def is_legal(ls: set, action_type, idxs=()) -> bool:
    return (int(action_type), tuple(idxs)) in ls


def playable_hand_idxs(legal) -> set:
    cp = int(ActionType.CardPlay)
    return {a.idxs[0] for a in legal if int(a.action_type) == cp and a.idxs}


def any_potion_action(legal) -> bool:
    pu, pd = int(ActionType.PotionUse), int(ActionType.PotionDiscard)
    return any(int(a.action_type) in (pu, pd) for a in legal)


# ---------- shared row renderers ----------
CARD_ROW_W = 28  # "  (C)  Name         kind  " — fixed so selection rects align


def cost_glyph(card) -> str:
    """Cost as it should appear in the card row, annotated by cost_kind.

    X-cost cards show "X" rather than current energy (which is what `cost`
    holds at view time but is misleading — playing the card consumes all of
    it). Discounted/growing cards get an arrow so the player notices the
    drift from base_cost.
    """
    match card.cost_kind:
        case CardCostKindXCost():
            return "X"
        case CardCostKindMinusDiscardsThisTurn() if card.cost < card.cost_base:
            return f"{card.cost}↓"
        case CardCostKindGrowsOnDamageInstanceTaken() if card.cost > card.cost_base:
            return f"{card.cost}↑"
        case _:
            return f"{card.cost}"


def card_flag_tail(card) -> str:
    flags = []
    if card.cost_zero_once:
        flags.append("(free)")
    if card.ethereal:
        flags.append("[eth]")
    if card.innate:
        flags.append("[inn]")
    if card.exhaust:
        flags.append("[exh]")
    if card.retain:
        flags.append("[ret]")
    return " ".join(flags)


_INTENT_BLOCK_KINDS = frozenset(
    {int(IntentKind.Block), int(IntentKind.AttackBlock), int(IntentKind.BlockBuff)}
)
_INTENT_BUFF_KINDS = frozenset(
    {int(IntentKind.Buff), int(IntentKind.AttackBuff), int(IntentKind.BlockBuff)}
)
_INTENT_DEBUFF_KINDS = frozenset(
    {int(IntentKind.Debuff), int(IntentKind.AttackDebuff), int(IntentKind.DebuffPowerful)}
)


def intent_string(monster) -> str:
    intent = monster.intent
    parts = []
    if intent.damage is not None:
        n = intent.instances or 1
        parts.append(f"atk {intent.damage}x{n}")
    if int(intent.kind) in _INTENT_BLOCK_KINDS:
        parts.append("blk")
    if int(intent.kind) in _INTENT_BUFF_KINDS:
        parts.append("buf")
    if int(intent.kind) in _INTENT_DEBUFF_KINDS:
        # DebuffPowerful gets a marker so a "stronger" debuff (e.g. Cultist's
        # ritual ramp on stacks) reads distinctly from a normal one.
        parts.append("dbf!" if intent.kind == IntentKind.DebuffPowerful else "dbf")
    if parts:
        return "+".join(parts)
    match intent.kind:
        case IntentKind.Sleep:
            return "asleep"
        case IntentKind.Stunned:
            return "stunned"
        case IntentKind.Escape:
            return "escape"
        case _:
            return "?"


CARD_KIND_LABEL = {
    int(CardKind.Attack): "attack",
    int(CardKind.Skill): "skill",
    int(CardKind.Power): "power",
    int(CardKind.Curse): "curse",
    int(CardKind.Status): "status",
}

MOD_ABBR = {
    int(ModifierKind.Accuracy): "acc",
    int(ModifierKind.AfterImage): "aimg",
    int(ModifierKind.Angry): "ang",
    int(ModifierKind.Artifact): "art",
    int(ModifierKind.Asleep): "slp",
    int(ModifierKind.Blur): "blur",
    int(ModifierKind.Burst): "brst",
    int(ModifierKind.Choke): "chk",
    int(ModifierKind.CorpseExplosion): "cexp",
    int(ModifierKind.CurlUp): "curl",
    int(ModifierKind.Dexterity): "dex",
    int(ModifierKind.DoubleDamage): "ddmg",
    int(ModifierKind.DrawCardNextTurn): "ndraw",
    int(ModifierKind.Enrage): "enr",
    int(ModifierKind.Entangled): "entg",
    int(ModifierKind.Envenom): "env",
    int(ModifierKind.Frail): "frail",
    int(ModifierKind.InfiniteBlades): "iblad",
    int(ModifierKind.Intangible): "intg",
    int(ModifierKind.Metallicize): "metl",
    int(ModifierKind.ModeShift): "mshft",
    int(ModifierKind.NextTurnBlock): "ntblk",
    int(ModifierKind.NextTurnEnergy): "nterg",
    int(ModifierKind.NoDraw): "nodr",
    int(ModifierKind.NoxiousFumes): "nox",
    int(ModifierKind.Phantasmal): "phant",
    int(ModifierKind.PlatedArmor): "parm",
    int(ModifierKind.Poison): "poi",
    int(ModifierKind.Retain): "ret",
    int(ModifierKind.Ritual): "rit",
    int(ModifierKind.Shackled): "shk",
    int(ModifierKind.SharpHide): "shrp",
    int(ModifierKind.Splittable): "splt",
    int(ModifierKind.SporeCloud): "spore",
    int(ModifierKind.Strength): "str",
    int(ModifierKind.Thievery): "thief",
    int(ModifierKind.Thorns): "thorn",
    int(ModifierKind.ThousandCuts): "cuts",
    int(ModifierKind.ToolsOfTheTrade): "tools",
    int(ModifierKind.Vigor): "vig",
    int(ModifierKind.Vulnerable): "vuln",
    int(ModifierKind.Weak): "weak",
    int(ModifierKind.WraithForm): "wrth",
}


def mod_label(mod) -> str:
    return MOD_ABBR.get(int(mod.kind), str(mod.kind).rsplit(".", 1)[-1].lower())


def modifier_string(modifiers) -> str:
    """Compact one-line representation of a modifier list."""
    if not modifiers:
        return ""
    return "  ".join(f"{mod_label(mod)} {mod.stacks}" for mod in modifiers)


# ---------- phase: map ----------
MAP_W = 7
MAP_H = 15


def reachable_columns(view, legal=None) -> list[int]:
    """The engine's RoomSelect legality is the source of truth (it knows
    relic overrides like Wing Boots); the edge fallback covers legal=None."""
    if legal is not None:
        return sorted({a.idxs[0] for a in legal
                       if int(a.action_type) == int(ActionType.RoomSelect)})
    m = view.map
    y_current = m.y_current
    if y_current is None:
        return [c for c in range(MAP_W) if m.rooms[0][c] is not None]
    if y_current + 1 >= MAP_H:
        return []
    node = m.rooms[y_current][m.x_current]
    if node is None:
        return []
    return [c for c in node.edges if m.rooms[y_current + 1][c] is not None]


def room_glyph(rt) -> str:
    return {
        int(RoomKind.CombatMonster): "M",
        int(RoomKind.CombatElite): "E",
        int(RoomKind.CombatBoss): "B",
        int(RoomKind.RestSite): "R",
        int(RoomKind.Treasure): "T",
        int(RoomKind.Shop): "$",
        int(RoomKind.EventRoom): "!",
    }.get(int(rt), "?")


def render_map(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    H, W = stdscr.getmaxyx()
    region_bot = H - 2
    sw = draw_hud(stdscr, view, ascension, "MAP", region_bot)
    footer(stdscr, "[A/D] select  [ENTER] go  [R] new run  [Q] quit")
    box = draw_box(stdscr, 1, sw, region_bot, W - 1, "MAP", focus=True)
    if box is None:
        return
    iy, ix, ih, iw = box
    bottom = iy + ih
    grid_x = ix + 6           # room for the y label
    spacing = 3

    reachable = reachable_columns(view, legal)
    cursor_col = reachable[cursor.idx] if reachable and cursor.idx < len(reachable) else None

    m = view.map
    y_cur, x_cur = m.y_current, m.x_current

    def col_to_x(col):
        return grid_x + col * spacing

    line = iy
    # Synthetic boss row above the top of the grid; the boss isn't in id_rooms.
    grid_center_px = col_to_x(MAP_W // 2) + 1
    boss = str(m.boss).rsplit(".", 1)[-1]  # enum MonsterEncounter.X -> "X"
    label = f"[{boss}]" if y_cur == MAP_H else boss
    write(stdscr, line, max(ix, grid_center_px - len(label) // 2), label[:iw],
          **({"bold": True} if y_cur == MAP_H else {}))
    line += 1

    for y in range(MAP_H - 1, -1, -1):
        if line >= bottom:
            break
        write(stdscr, line, ix, f"{y:>2}", dim=True)
        for x in range(MAP_W):
            cell = m.rooms[y][x]
            cx = col_to_x(x)
            if cell is None:
                write(stdscr, line, cx + 1, ".", dim=True)
                continue
            glyph = room_glyph(cell.room_kind)
            if y_cur == y and x_cur == x:
                write(stdscr, line, cx, f"[{glyph}]", bold=True)
            elif cursor_col is not None and cursor_col == x and (
                (y_cur is None and y == 0) or (y_cur is not None and y == y_cur + 1)
            ):
                write(stdscr, line, cx + 1, glyph, selected=True)
            else:
                write(stdscr, line, cx + 1, glyph)
        line += 1

        if y > 0 and line < bottom:
            for x_src in range(MAP_W):
                src = m.rooms[y - 1][x_src]
                if src is None:
                    continue
                for x_tgt in src.edges:
                    if m.rooms[y][x_tgt] is None:
                        continue
                    dx = x_tgt - x_src
                    src_px = col_to_x(x_src) + 1
                    if dx == 0:
                        ch, px = "│", src_px
                    elif dx < 0:
                        ch, px = "╲", src_px - 1
                    else:
                        ch, px = "╱", src_px + 1
                    write(stdscr, line, px, ch, dim=True)
            line += 1


# ---------- phase: combat ----------
def pick_many_prompt(verb: str, picked: int, n) -> str:
    """Footer instructions for a Discard/Retain halt. A single-card pick is
    confirmed with ENTER on the highlighted card; multi-card picks use SPACE."""
    if n == 1:
        return f"{verb} a card   [W/S] choose   [ENTER] confirm"
    return f"{verb} {picked}/{n} cards   [W/S] move  [SPACE] mark   [ENTER] confirm"


def _energy_str(energy) -> str:
    if 0 <= energy.energy_max <= 9:
        return ("⚡" if _ACS_OK else "E ") + PIP_FULL * min(energy.energy_current, energy.energy_max) \
            + PIP_EMPTY * max(energy.energy_max - energy.energy_current, 0) \
            + (f" {energy.energy_current}/{energy.energy_max}" if energy.energy_current > energy.energy_max else "")
    return f"E {energy.energy_current}/{energy.energy_max}"


def _potion_summary(view, width) -> str:
    names = ["-" if p is None else str(p.name).split(".")[-1][:6] for p in view.potions]
    return ("Pot " + "/".join(names))[:width]


def _relic_token(r) -> str:
    return str(r.name).split(".")[-1] + (f" x{r.counter}" if r.counter else "")


def _relic_lines(relics, iw) -> int:
    """Rows a horizontal-wrapping relic row needs in interior width `iw`."""
    if not relics:
        return 1
    lines, x = 1, 0
    for r in relics:
        t = len(_relic_token(r))
        if x > 0 and x + t > iw:
            lines += 1
            x = 0
        x += t + 2
    return lines


def draw_sidebar(stdscr, view, ascension, top, bottom, left, width) -> int:
    """The persistent run HUD: a compact STATUS box (name/HP/block/energy/gold/
    ascension/modifiers/potions) and, directly below, a compact RELICS box whose
    relics flow horizontally and wrap (like the base game). Both boxes are only
    as tall as their content. Returns the bottom row used."""
    right = left + width - 1
    ch = view.character
    c = _combat(view)
    e = c.energy if c else _NoEnergy

    status_bot = min(top + 7, bottom)
    box = draw_box(stdscr, top, left, status_bot, right, "STATUS")
    if box is None:
        return top
    iy, ix, ih, iw = box
    write(stdscr, iy, ix, ch.name[:iw], bold=True)
    bar(stdscr, iy + 1, ix, iw, ch.health, ch.health_max, hp=True, label="HP")
    write(stdscr, iy + 2, ix, f"Blk {ch.block}", block=True)
    if e.energy_max > 0:  # energy only meaningful in combat
        write(stdscr, iy + 2, ix + min(iw - 1, 11), _energy_str(e)[: max(0, iw - 11)], gold=True)
    write(stdscr, iy + 3, ix, f"Gold {ch.gold}", gold=True)
    write(stdscr, iy + 3, ix + min(iw - 1, 13), f"Asc {ascension}", dim=True)
    mods = modifier_string(ch.modifiers)
    if mods:
        write(stdscr, iy + 4, ix, mods[:iw])
    write(stdscr, iy + 5, ix, _potion_summary(view, iw), dim=True)

    rtop = status_bot + 1
    if rtop >= bottom - 1:
        return status_bot
    relics = view.relics
    iw_r = width - 2
    nlines = min(_relic_lines(relics, iw_r), max(1, bottom - rtop - 1))
    rbot = min(rtop + nlines + 1, bottom)
    rbox = draw_box(stdscr, rtop, left, rbot, right, "RELICS")
    if rbox is None:
        return status_bot
    ry, rx, rh, rw = rbox
    if not relics:
        write(stdscr, ry, rx, "(none)", dim=True)
        return rbot
    x, y = rx, ry
    for r in relics:
        tok = _relic_token(r)
        if x > rx and x + len(tok) > rx + rw:
            y += 1
            x = rx
            if y >= ry + rh:
                write(stdscr, ry + rh - 1, rx + rw - 1, "…" if _ACS_OK else "+", dim=True)
                break
        write(stdscr, y, x, tok, dim=r.used_up)
        x += len(tok) + 2
    return rbot


def sidebar_w(W) -> int:
    return max(26, min(38, W // 3))


def draw_hud(stdscr, view, ascension, phase_title, region_bot) -> int:
    """Phase-bar title + persistent sidebar; returns the sidebar width so the
    caller can place its main panel to the right."""
    _H, W = stdscr.getmaxyx()
    write_phase_title(stdscr, phase_title)
    sw = sidebar_w(W)
    draw_sidebar(stdscr, view, ascension, 1, region_bot, 0, sw)
    return sw


def footer(stdscr, text, right_text="") -> None:
    _H, W = stdscr.getmaxyx()
    write(stdscr, _H - 1, 2, f" {text} ", dim=True)
    if right_text:
        write_segments(stdscr, _H - 1, W - 2, [(right_text + " ", {"gold": True})])


def card_line(card) -> str:
    """One-line card label for vertical card lists (reward/discover/purge/upgrade)."""
    tail = card_flag_tail(card)
    return f"({cost_glyph(card)}) {card.display_name[:16]:<16} {CARD_KIND_LABEL.get(int(card.kind), '?'):<6} {tail}".rstrip()


def render_list_screen(stdscr, view, cursor, ascension, title, rows, render_row, ctrl) -> None:
    """Phase bar + sidebar + a single scrolling main panel + footer — the shape
    shared by reward/shop/event/chest/rest/discover/deck-pick."""
    H, W = stdscr.getmaxyx()
    region_bot = H - 2
    sw = draw_hud(stdscr, view, ascension, title, region_bot)
    panel_list(stdscr, 1, sw, region_bot, W - 1, rows, cursor.idx, title=title,
               focus=True, render_row=render_row)
    footer(stdscr, ctrl)


def render_enemy_block(stdscr, y, x, w, m, selected) -> None:
    """One enemy: name / HP bar / intent + block + modifiers (3 rows)."""
    name = m.display_name[: max(1, w - 2)]
    write(stdscr, y, x, (sel_prefix(selected) + name)[:w], selected=selected, bold=not selected)
    bar(stdscr, y + 1, x + 2, min(w - 2, 26), m.health, m.health_max, hp=True, label="HP")
    is_atk = m.intent.damage is not None
    cx = x + 2
    intent = intent_string(m)
    write(stdscr, y + 2, cx, intent[: w - 2], hp=is_atk, good=not is_atk)
    cx += len(intent) + 2
    if m.block > 0:
        blk = f"blk {m.block}"
        write(stdscr, y + 2, cx, blk, block=True)
        cx += len(blk) + 2
    mods = modifier_string(m.modifiers)
    if mods and cx < x + w:
        write(stdscr, y + 2, cx, mods[: x + w - cx], dim=True)


def render_combat(stdscr, view, cursor: Cursor, *, prompt: str | None = None,
                  legal=None, ascension=0) -> None:
    H, W = stdscr.getmaxyx()
    pi = view.pending
    if isinstance(pi, EffectCardDiscard):
        title = "DISCARD"
    elif isinstance(pi, EffectCardRetain):
        title = "RETAIN"
    elif isinstance(pi, EffectCardSetupPick):
        title = "SETUP"
    elif isinstance(pi, EffectCardNightmarePick):
        title = "NIGHTMARE"
    else:
        title = "COMBAT"
    write_phase_title(stdscr, title)

    SIDEBAR_W = max(28, min(40, W // 3))
    region_bot = H - 2  # footer on H-1

    # Enemies fill the tall right panel.
    enemies_focus = cursor.mode in ("target", "potion_target")
    eb = draw_box(stdscr, 1, SIDEBAR_W, region_bot, W - 1, "ENEMIES", focus=enemies_focus)
    if eb is not None:
        iy, ix, ih, iw = eb
        monsters = _monsters(view)
        if not monsters:
            write(stdscr, iy, ix, "(no enemies)", dim=True)
        else:
            yy = iy
            for i, m in enumerate(monsters):
                if yy + 2 > iy + ih:
                    break
                render_enemy_block(stdscr, yy, ix, iw, m, enemies_focus and cursor.target == i)
                yy += 4

    # Left column: STATUS + RELICS on top, then the hand as a vertical text list.
    sb_bot = draw_sidebar(stdscr, view, ascension, 1, region_bot, 0, SIDEBAR_W)
    hand = _hand(view)
    hand_top = sb_bot + 1
    if hand_top < region_bot:
        playable = playable_hand_idxs(legal) if (legal is not None and prompt is None) else None
        picked = cursor.selected if prompt is not None else None
        rows = list(enumerate(hand))
        c = _combat(view)
        pd, px, pe = (len(c.pile_draw), len(c.pile_discard), len(c.pile_exhaust)) if c else (0, 0, 0)
        piles = f"d{pd} x{px} e{pe}"
        htitle = f"HAND {min(cursor.idx + 1, len(hand)) if hand else 0}/{len(hand)}"

        def hand_row(s, y, x, w, item, issel):
            i, card = item
            disabled = playable is not None and i not in playable
            mark = " *" if picked and i in picked else ""
            highlight_row(s, y, x, w, card_line(card) + mark, selected=issel, disabled=disabled)

        panel_list(stdscr, hand_top, 0, region_bot, SIDEBAR_W - 1, rows, cursor.idx,
                   title=htitle, focus=not enemies_focus, render_row=hand_row, right_title=piles)

    # footer
    if prompt is not None:
        ctrl = prompt
    elif enemies_focus:
        ctrl = "[W/S] target  [ENTER] confirm  [X] cancel  [Q] quit"
    else:
        ctrl = "[W/S] select  [ENTER] play  [E] end turn  [P] potion  [R] run  [Q] quit"
    write(stdscr, H - 1, 2, f" {ctrl} ", dim=True)


# ---------- phase: reward ----------
def _reward_potions(rw):
    """Reward potions, engine-shape-agnostic. The forward engine exposes a `potions`
    Vec (taken with idx [i]); the XVIII bkp build (31cbe44) exposes a single nullable
    `potion` (taken with idx []). Yields (idx_or_None, potion)."""
    potions = getattr(rw, "potions", None)
    if potions is not None:
        yield from enumerate(potions)
    elif getattr(rw, "potion", None) is not None:
        yield None, rw.potion


def reward_rows(view, ls):
    """Linearize the reward (cards + relic/potion/gold) plus a leave row.

    Each row is `(kind, label, payload)`; kind in {card, relic, potion, gold,
    leave}; payload is the card idx (or None). Non-card rows appear only when
    present and their take action is currently legal."""
    rows = []
    rw = view.mode if isinstance(view.mode, ModeReward) else None
    if rw:
        for i, c in enumerate(rw.cards):
            if ls is None or is_legal(ls, ActionType.RewardTakeCard, [i]):
                rows.append(("card", c.display_name, i))
        for i, r in enumerate(rw.relics):
            if ls is None or is_legal(ls, ActionType.RewardTakeRelic, [i]):
                rows.append(("relic", "Relic: " + str(r.name).split(".")[-1], i))
        for idx, p in _reward_potions(rw):
            take = [idx] if idx is not None else []
            if ls is None or is_legal(ls, ActionType.RewardTakePotion, take):
                rows.append(("potion", "Potion: " + str(p.name).split(".")[-1], idx))
        if rw.gold is not None and (ls is None or is_legal(ls, ActionType.RewardTakeGold)):
            rows.append(("gold", f"Gold: {rw.gold}", None))
    rows.append(("leave", "[ leave ]", None))
    return rows


def render_card_reward(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    ls = legal_set(legal) if legal is not None else None
    rows = reward_rows(view, ls)

    def row_render(s, y, x, w, row, sel):
        kind, label, payload = row
        text = card_line(view.mode.cards[payload]) if kind == "card" else label
        highlight_row(s, y, x, w, text, selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, "REWARD", rows, row_render,
                       "[W/S] select  [ENTER] take/leave  [Q] quit")


# ---------- phase: rest site ----------
def rest_options(ls):
    """Available rest-site choices, gated by the engine's legal actions. Once
    the rest action is consumed only RoomExit remains, so Leave is offered then."""
    opts = []
    if ls is None or is_legal(ls, ActionType.Rest):
        opts.append(("rest", "Rest", "heal 30% max HP"))
    if ls is None or any(k[0] == int(ActionType.CardUpgrade) for k in ls):
        opts.append(("upgrade", "Upgrade", "upgrade a card in your deck"))
    if ls is not None and is_legal(ls, ActionType.RestLift):
        opts.append(("lift", "Lift", "Girya: +1 permanent Strength"))
    if ls is not None and is_legal(ls, ActionType.RestToke):
        opts.append(("toke", "Toke", "Peace Pipe: purge a card from your deck"))
    if ls is not None and is_legal(ls, ActionType.RestDig):
        opts.append(("dig", "Dig", "Shovel: dig up a random relic"))
    if ls is None or is_legal(ls, ActionType.RoomExit):
        opts.append(("leave", "Leave", "continue on the map"))
    return opts


def render_rest_site(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    ls = legal_set(legal) if legal is not None else None
    options = rest_options(ls)

    def row_render(s, y, x, w, opt, sel):
        _kind, label, desc = opt
        highlight_row(s, y, x, w, f"{label:<9} {desc}", selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, "REST SITE", options, row_render,
                       "[W/S] select  [ENTER] confirm  [Q] quit")


# ---------- phase: rest → upgrade ----------
def render_rest_upgrade(stdscr, view, cursor: Cursor, ascension=0) -> None:
    cards = [c for c in view.deck if not c.upgraded]

    def row_render(s, y, x, w, c, sel):
        highlight_row(s, y, x, w, card_line(c), selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, "UPGRADE A CARD", cards, row_render,
                       "[W/S] select  [ENTER] upgrade  [X] back  [Q] quit")


# ---------- phase: shop ----------
def shop_rows(view):
    """Linearize shop inventory + purge + leave into a single row list.

    Each row is a tuple `(kind, label, price, payload)` where:
      kind in {"card", "relic", "potion", "purge", "leave"}
      payload is the shop-side idx (or None for purge/leave)
    """
    shop = view.mode if isinstance(view.mode, ModeShop) else None
    rows = []
    if shop:
        for i, (c, p) in enumerate(zip(shop.cards, shop.card_prices)):
            rows.append(("card", c.display_name, p, i))
        for i, (r, p) in enumerate(zip(shop.relics, shop.relic_prices)):
            rows.append(("relic", str(r.name).split(".")[-1], p, i))
        for i, (po, p) in enumerate(zip(shop.potions, shop.potion_prices)):
            rows.append(("potion", str(po.name).split(".")[-1], p, i))
        rows.append(("purge", "Remove a card from deck", shop.purge_cost, None))
    rows.append(("leave", "Leave shop", None, None))
    return rows


def _shop_row_legal(ls, kind, payload) -> bool:
    if kind == "card":
        return is_legal(ls, ActionType.ShopBuyCard, [payload])
    if kind == "relic":
        return is_legal(ls, ActionType.ShopBuyRelic, [payload])
    if kind == "potion":
        return is_legal(ls, ActionType.ShopBuyPotion, [payload])
    if kind == "purge":
        sp = int(ActionType.ShopPurge)
        return any(k[0] == sp for k in ls)
    return True


def render_shop(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    ls = legal_set(legal) if legal is not None else None
    rows = shop_rows(view)
    tags = {"card": "C", "relic": "R", "potion": "P", "purge": "X"}

    def row_render(s, y, x, w, row, sel):
        kind, label, price, payload = row
        if kind == "leave":
            highlight_row(s, y, x, w, "[ leave ]", selected=sel)
            return
        if ls is not None:
            unaff = not _shop_row_legal(ls, kind, payload)
        else:
            unaff = price is not None and view.character.gold < price
        highlight_row(s, y, x, w, f"({tags[kind]}) {label}", selected=sel, disabled=unaff,
                      right_text=(f"{price}g" if price is not None else ""))

    render_list_screen(stdscr, view, cursor, ascension, "SHOP", rows, row_render,
                       "[W/S] select  [ENTER] buy/purge/leave  [Q] quit")


def render_shop_purge_pick(stdscr, view, cursor: Cursor, ascension=0) -> None:
    def row_render(s, y, x, w, c, sel):
        highlight_row(s, y, x, w, card_line(c), selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, "REMOVE A CARD", view.deck, row_render,
                       "[W/S] select  [ENTER] purge  [X] back  [Q] quit")


# ---------- phase: game over ----------
def render_game_over(stdscr, view, _cursor: Cursor, ascension=0) -> None:
    H, W = stdscr.getmaxyx()
    write_phase_title(stdscr, "GAME OVER")
    dead = view.character.health == 0
    msg = "You died." if dead else "Victory."
    box = draw_box(stdscr, H // 2 - 2, max(0, W // 2 - 16), H // 2 + 1, W // 2 + 16,
                   "RUN OVER", focus=True)
    if box is not None:
        iy, ix, ih, iw = box
        write(stdscr, iy + ih // 2, ix + max(0, (iw - len(msg)) // 2), msg, bold=True, hp=dead, good=not dead)
    footer(stdscr, "[R] new run  [Q] quit")


# ---------- phase: discover (CardDiscover halt) ----------
def render_discover(stdscr, view, cursor: Cursor, ascension=0) -> None:
    def row_render(s, y, x, w, c, sel):
        highlight_row(s, y, x, w, card_line(c), selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, "DISCOVER", _discover(view), row_render,
                       "[W/S] select  [ENTER] pick  [Q] quit")


# ---------- phase: event ----------
def _option_label(i, effects):
    kinds = ", ".join(type(e).__name__ for e in effects)
    return f"[{i + 1}] {kinds}"


def event_actionable(view, ls):
    """Return (gated_labels, rows) where each row is `(label, (kind, payload))`,
    kind in {opt, leave}. Only currently-legal options are navigable (gating =
    absent from legal_actions); a leave row appears once RoomExit is legal."""
    ev = view.mode if isinstance(view.mode, ModeEvent) else None
    gated = []
    rows = []
    if ev is not None:
        for i, effects in enumerate(ev.options):
            label = _option_label(i, effects)
            if ls is not None and not is_legal(ls, ActionType.EventOptionSelect, [i]):
                gated.append(label)
            else:
                rows.append((label, ("opt", i)))
    if ls is None or is_legal(ls, ActionType.RoomExit):
        rows.append(("[ leave ]", ("leave", None)))
    return gated, rows


def render_event(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    ls = legal_set(legal) if legal is not None else None
    _gated, rows = event_actionable(view, ls)
    ev = view.mode if isinstance(view.mode, ModeEvent) else None
    title = (type(ev.kind).__name__ if ev is not None else "EVENT")[:24]

    def row_render(s, y, x, w, row, sel):
        highlight_row(s, y, x, w, row[0], selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, title, rows, row_render,
                       "[W/S] select  [ENTER] choose  [Q] quit")


# ---------- phase: chest ----------
def _current_room(view):
    m = view.map
    if m.y_current is None or m.x_current is None:
        return None
    try:
        return m.rooms[m.y_current][m.x_current]
    except (IndexError, TypeError):
        return None


def chest_rows(view, legal):
    ls = legal_set(legal) if legal is not None else None
    rows = []
    if ls is None or is_legal(ls, ActionType.ChestOpen):
        rows.append(("Open", ("open", None)))
    if ls is None or is_legal(ls, ActionType.RoomExit):
        rows.append(("Leave", ("leave", None)))
    if not rows:
        rows.append(("Leave", ("leave", None)))
    return rows


def render_chest(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    rows = chest_rows(view, legal)
    room = _current_room(view)
    ck = getattr(room, "chest_kind", None) if room is not None else None
    title = f"{str(ck).split('.')[-1].upper()} CHEST" if ck is not None else "CHEST"

    def row_render(s, y, x, w, row, sel):
        highlight_row(s, y, x, w, row[0], selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, title, rows, row_render,
                       "[W/S] select  [ENTER] confirm  [Q] quit")


# ---------- submode: potion belt ----------
def render_potion_picker(stdscr, view, cursor: Cursor, ascension=0) -> None:
    H, W = stdscr.getmaxyx()
    region_bot = H - 2
    sw = draw_hud(stdscr, view, ascension, "POTION", region_bot)
    if cursor.mode == "potion_target":
        eb = draw_box(stdscr, 1, sw, region_bot, W - 1, "PICK TARGET", focus=True)
        if eb is not None:
            iy, ix, ih, iw = eb
            yy = iy
            for j, m in enumerate(_monsters(view)):
                if yy + 2 > iy + ih:
                    break
                render_enemy_block(stdscr, yy, ix, iw, m, cursor.target == j)
                yy += 4
        footer(stdscr, "[W/S] target  [ENTER] use  [X] back  [Q] quit")
        return
    belt = view.potions
    rows = [(i, str(belt[i].name).split(".")[-1]) for i, p in enumerate(belt) if p is not None]

    def row_render(s, y, x, w, row, sel):
        slot, name = row
        highlight_row(s, y, x, w, f"[{slot}] {name}", selected=sel)

    panel_list(stdscr, 1, sw, region_bot, W - 1, rows, cursor.idx, title="POTIONS",
               focus=True, render_row=row_render)
    footer(stdscr, "[W/S] select  [ENTER] use  [D] discard  [X] back  [Q] quit")


# ---------- halt: deck-card pick (purge / duplicate / transform / upgrade) ----------
def _is_deck_pick(pending) -> bool:
    return isinstance(
        pending,
        (EffectCardPurge, EffectCardDuplicate, EffectCardTransform, EffectCardUpgrade),
    )


def _deck_pick_action_type(pending):
    if isinstance(pending, EffectCardPurge):
        return ActionType.CardPurge
    if isinstance(pending, EffectCardDuplicate):
        return ActionType.CardDuplicate
    if isinstance(pending, EffectCardTransform):
        return ActionType.CardTransform
    if isinstance(pending, EffectCardUpgrade):
        return ActionType.CardUpgrade
    return None


def _deck_pick_title(pending) -> str:
    if isinstance(pending, EffectCardPurge):
        return "REMOVE A CARD"
    if isinstance(pending, EffectCardDuplicate):
        return "DUPLICATE A CARD"
    if isinstance(pending, EffectCardTransform):
        return "TRANSFORM A CARD"
    return "UPGRADE A CARD"


def _deck_pick_idxs(legal, at):
    a = int(at)
    return sorted(x.idxs[0] for x in legal if int(x.action_type) == a and x.idxs)


def render_deck_pick(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    at = _deck_pick_action_type(view.pending)
    idxs = _deck_pick_idxs(legal or [], at)
    cards = [view.deck[i] for i in idxs]

    def row_render(s, y, x, w, c, sel):
        highlight_row(s, y, x, w, card_line(c), selected=sel)

    render_list_screen(stdscr, view, cursor, ascension, _deck_pick_title(view.pending),
                       cards, row_render, "[W/S] select  [ENTER] confirm  [Q] quit")


def handle_deck_pick(view, cursor: Cursor, key: int, legal=None):
    at = _deck_pick_action_type(view.pending)
    if at is None:
        return None
    idxs = _deck_pick_idxs(legal or [], at)
    if not idxs:
        return None
    if cursor.idx >= len(idxs):
        cursor.idx = len(idxs) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(idxs) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        return Action(at, [idxs[cursor.idx]])
    return None


# ---------- top-level render dispatch ----------
def render(stdscr, view, cursor: Cursor, legal=None, ascension=0) -> None:
    if view.game_over:
        render_game_over(stdscr, view, cursor)
    elif cursor.mode in ("potion", "potion_target"):
        render_potion_picker(stdscr, view, cursor, ascension)
    elif _discover(view):
        render_discover(stdscr, view, cursor, ascension)
    elif isinstance(view.pending, EffectCardDiscard):
        render_combat(stdscr, view, cursor, legal=legal, ascension=ascension,
                      prompt=pick_many_prompt("DISCARD", len(cursor.selected), _input_count(view)))
    elif isinstance(view.pending, EffectCardRetain):
        render_combat(stdscr, view, cursor, legal=legal, ascension=ascension,
                      prompt=pick_many_prompt("RETAIN", len(cursor.selected), _input_count(view)))
    elif isinstance(view.pending, EffectCardSetupPick):
        render_combat(
            stdscr, view, cursor, legal=legal, ascension=ascension,
            prompt="PICK A CARD TO SETUP (top of draw, free next play)",
        )
    elif isinstance(view.pending, EffectCardNightmarePick):
        render_combat(
            stdscr, view, cursor, legal=legal, ascension=ascension,
            prompt="PICK A CARD TO NIGHTMARE (3 copies into hand next turn)",
        )
    elif _is_deck_pick(view.pending):
        render_deck_pick(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, ModeCombat):
        render_combat(stdscr, view, cursor, legal=legal, ascension=ascension)
    elif isinstance(view.mode, (ModeReward, ModeChestOpened)):
        render_card_reward(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, ModeRestSite):
        if cursor.mode == "upgrade":
            render_rest_upgrade(stdscr, view, cursor, ascension)
        else:
            render_rest_site(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, (ModeMap, ModeCombatEnded)):
        render_map(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, ModeShop):
        if cursor.mode == "purge":
            render_shop_purge_pick(stdscr, view, cursor, ascension)
        else:
            render_shop(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, ModeEvent):
        render_event(stdscr, view, cursor, legal, ascension)
    elif isinstance(view.mode, ModeChest):
        render_chest(stdscr, view, cursor, legal, ascension)
    else:
        write(
            stdscr,
            2,
            2,
            f"Unknown mode: active={type(view.mode).__name__}, pending={type(view.pending).__name__ if view.pending else None}",
            hp=True,
        )

    # Error toast + seed on the same bottom line.
    if cursor.error:
        maxy, maxx = stdscr.getmaxyx()
        text = f" ! {cursor.error} "
        write(stdscr, maxy - 1, max(0, (maxx - len(text)) // 2), text, hp=True)


# ---------- input handling ----------
ENTER_KEYS = {10, 13, curses.KEY_ENTER}


def is_left(key: int) -> bool:  return key in (ord("a"), ord("A"))
def is_right(key: int) -> bool: return key in (ord("d"), ord("D"))
def is_up(key: int) -> bool:    return key in (ord("w"), ord("W"))
def is_down(key: int) -> bool:  return key in (ord("s"), ord("S"))
def is_back(key: int) -> bool:  return key in (ord("x"), ord("X"), curses.KEY_BACKSPACE, 127)


def handle_map(view, cursor: Cursor, key: int, legal=None):
    reachable = reachable_columns(view, legal)
    if not reachable:
        return None
    if is_left(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_right(key):
        cursor.idx = min(len(reachable) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        return Action(ActionType.RoomSelect, [reachable[cursor.idx]])
    return None


def handle_combat(view, cursor: Cursor, key: int, legal=None):
    hand = _hand(view)
    monsters = _monsters(view)
    ls = legal_set(legal) if legal is not None else None
    cp = int(ActionType.CardPlay)

    if cursor.mode == "target":
        if not monsters:
            cursor.mode = "primary"
            return None
        if is_up(key):
            cursor.target = max(0, cursor.target - 1)
        elif is_down(key):
            cursor.target = min(len(monsters) - 1, cursor.target + 1)
        elif is_back(key):
            cursor.mode = "primary"
        elif key in ENTER_KEYS:
            if ls is not None and not is_legal(ls, ActionType.CardPlay, [cursor.idx, cursor.target]):
                cursor.error = "Invalid target"
                return None
            cursor.mode = "primary"
            return Action(ActionType.CardPlay, [cursor.idx, cursor.target])
        return None

    if key in (ord("e"), ord("E")):
        return Action(ActionType.TurnEnd, [])

    if not hand:
        return None

    # Horizontal hand fan: A/D (or W/S) move between tiles.
    if is_up(key) or is_left(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key) or is_right(key):
        cursor.idx = min(len(hand) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        card = hand[cursor.idx]
        card_label = card.display_name
        if ls is not None and is_legal(ls, ActionType.CardPlay, [cursor.idx]):
            return Action(ActionType.CardPlay, [cursor.idx])
        needs_target = ls is None or any(
            k[0] == cp and len(k[1]) == 2 and k[1][0] == cursor.idx for k in ls
        )
        if needs_target:
            if not monsters:
                cursor.error = "No target available"
                return None
            cursor.mode = "target"
            cursor.target = 0
            return None
        cursor.error = f"{card_label} cannot be played right now"
        return None
    return None


def handle_combat_await_pick(view, cursor: Cursor, key: int):
    """A/D (or W/S) scroll the hand fan, ENTER picks. Used by Setup / Nightmare —
    both halts just need a single hand index, no skip/cancel."""
    hand = _hand(view)
    if not hand:
        return None
    if is_up(key) or is_left(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key) or is_right(key):
        cursor.idx = min(len(hand) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        if isinstance(view.pending, EffectCardNightmarePick):
            return Action(ActionType.CardNightmare, [cursor.idx])
        return Action(ActionType.CardSetup, [cursor.idx])
    return None


def handle_combat_await_pick_many(view, cursor: Cursor, key: int):
    """SPACE to toggle a card, W/S to scroll, ENTER to confirm. Used by Discard
    and Retain — both halt with an Input-selection count of cards to pick."""
    hand = _hand(view)
    if not hand:
        return None
    num = _input_count(view) or 0
    if is_up(key) or is_left(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key) or is_right(key):
        cursor.idx = min(len(hand) - 1, cursor.idx + 1)
    elif key == ord(" "):
        if cursor.idx in cursor.selected:
            cursor.selected.discard(cursor.idx)
        elif len(cursor.selected) < num:
            cursor.selected.add(cursor.idx)
        else:
            cursor.error = f"Already picked {num} cards"
    elif key in ENTER_KEYS:
        # Single-card pick: ENTER confirms the highlighted card (no SPACE needed).
        if num == 1 and not cursor.selected:
            cursor.selected = {cursor.idx}
        if len(cursor.selected) != num:
            cursor.error = f"Pick {num} cards (have {len(cursor.selected)})"
            return None
        indices = sorted(cursor.selected)
        cursor.selected = set()
        if isinstance(view.pending, EffectCardRetain):
            return Action(ActionType.CardRetain, indices)
        return Action(ActionType.CardDiscard, indices)
    return None


def handle_card_reward(view, cursor: Cursor, key: int, legal=None):
    ls = legal_set(legal) if legal is not None else None
    rows = reward_rows(view, ls)
    n = len(rows)
    if cursor.idx >= n:
        cursor.idx = n - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(n - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        kind, _label, payload = rows[cursor.idx]
        if kind == "card":
            return Action(ActionType.RewardTakeCard, [payload])
        if kind == "relic":
            return Action(ActionType.RewardTakeRelic, [payload])
        if kind == "potion":
            return Action(ActionType.RewardTakePotion, [payload] if payload is not None else [])
        if kind == "gold":
            return Action(ActionType.RewardTakeGold, [])
        return Action(ActionType.RoomExit, [])
    return None


def handle_rest_site(view, cursor: Cursor, key: int, legal=None):
    ls = legal_set(legal) if legal is not None else None
    if cursor.mode == "upgrade":
        deck = view.deck
        non_upgraded = [(i, c) for i, c in enumerate(deck) if not c.upgraded]
        if is_back(key):
            cursor.mode = "primary"
            cursor.idx = 0
            return None
        if not non_upgraded:
            return None
        if cursor.idx >= len(non_upgraded):
            cursor.idx = len(non_upgraded) - 1
        if is_up(key):
            cursor.idx = max(0, cursor.idx - 1)
        elif is_down(key):
            cursor.idx = min(len(non_upgraded) - 1, cursor.idx + 1)
        elif key in ENTER_KEYS:
            deck_idx, _ = non_upgraded[cursor.idx]
            if ls is not None and not is_legal(ls, ActionType.CardUpgrade, [deck_idx]):
                cursor.error = "Can't upgrade that card"
                return None
            cursor.mode = "primary"
            cursor.idx = 0
            return Action(ActionType.CardUpgrade, [deck_idx])
        return None

    options = rest_options(ls)
    if not options:
        return None
    if cursor.idx >= len(options):
        cursor.idx = len(options) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(options) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        kind = options[cursor.idx][0]
        if kind == "rest":
            return Action(ActionType.Rest, [])
        if kind == "lift":
            return Action(ActionType.RestLift, [])
        if kind == "toke":
            return Action(ActionType.RestToke, [])
        if kind == "dig":
            return Action(ActionType.RestDig, [])
        if kind == "leave":
            return Action(ActionType.RoomExit, [])
        cursor.mode = "upgrade"
        cursor.idx = 0
        return None
    return None


def handle_discover(view, cursor: Cursor, key: int):
    cards = _discover(view)
    if not cards:
        return None
    if cursor.idx >= len(cards):
        cursor.idx = len(cards) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(cards) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        return Action(ActionType.CardDiscover, [cursor.idx])
    return None


def handle_event(view, cursor: Cursor, key: int, legal=None):
    ls = legal_set(legal) if legal is not None else None
    _gated, rows = event_actionable(view, ls)
    if not rows:
        return None
    if cursor.idx >= len(rows):
        cursor.idx = len(rows) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(rows) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        _label, (kind, payload) = rows[cursor.idx]
        if kind == "opt":
            return Action(ActionType.EventOptionSelect, [payload])
        return Action(ActionType.RoomExit, [])
    return None


def handle_chest(view, cursor: Cursor, key: int, legal=None):
    rows = chest_rows(view, legal)
    if cursor.idx >= len(rows):
        cursor.idx = len(rows) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(rows) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        _text, (kind, _p) = rows[cursor.idx]
        if kind == "open":
            return Action(ActionType.ChestOpen, [])
        return Action(ActionType.RoomExit, [])
    return None


def handle_potion(view, cursor: Cursor, key: int, legal=None):
    ls = legal_set(legal) if legal is not None else None
    belt = view.potions
    pu = int(ActionType.PotionUse)

    if cursor.mode == "potion_target":
        monsters = _monsters(view)
        if is_back(key) or not monsters:
            cursor.mode = "potion"
            return None
        if is_up(key):
            cursor.target = max(0, cursor.target - 1)
        elif is_down(key):
            cursor.target = min(len(monsters) - 1, cursor.target + 1)
        elif key in ENTER_KEYS:
            slot = cursor.potion_slot
            if ls is not None and not is_legal(ls, ActionType.PotionUse, [slot, cursor.target]):
                cursor.error = "Invalid target"
                return None
            cursor.mode = "primary"
            return Action(ActionType.PotionUse, [slot, cursor.target])
        return None

    slots = [i for i, p in enumerate(belt) if p is not None]
    if is_back(key) or not slots:
        cursor.mode = "primary"
        return None
    if cursor.idx >= len(slots):
        cursor.idx = len(slots) - 1
    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(slots) - 1, cursor.idx + 1)
    elif key in (ord("d"), ord("D")):
        slot = slots[cursor.idx]
        if ls is None or is_legal(ls, ActionType.PotionDiscard, [slot]):
            cursor.mode = "primary"
            return Action(ActionType.PotionDiscard, [slot])
        cursor.error = "Can't discard that potion"
        return None
    elif key in ENTER_KEYS:
        slot = slots[cursor.idx]
        if ls is None or is_legal(ls, ActionType.PotionUse, [slot]):
            cursor.mode = "primary"
            return Action(ActionType.PotionUse, [slot])
        if any(k[0] == pu and len(k[1]) == 2 and k[1][0] == slot for k in ls):
            cursor.potion_slot = slot
            cursor.target = 0
            cursor.mode = "potion_target"
            return None
        cursor.error = "Can't use that potion now"
        return None
    return None


def handle_key(view, cursor: Cursor, key: int, legal=None):
    if view.game_over:
        return None
    if cursor.mode in ("potion", "potion_target"):
        return handle_potion(view, cursor, key, legal)
    if key in (ord("p"), ord("P")) and any(p is not None for p in view.potions) \
            and any_potion_action(legal or []):
        cursor.mode = "potion"
        cursor.idx = 0
        return None
    if _discover(view):
        return handle_discover(view, cursor, key)
    pending = view.pending
    if isinstance(pending, (EffectCardDiscard, EffectCardRetain)):
        return handle_combat_await_pick_many(view, cursor, key)
    if isinstance(pending, (EffectCardSetupPick, EffectCardNightmarePick)):
        return handle_combat_await_pick(view, cursor, key)
    if _is_deck_pick(pending):
        return handle_deck_pick(view, cursor, key, legal)
    if isinstance(view.mode, ModeCombat):
        return handle_combat(view, cursor, key, legal)
    if isinstance(view.mode, (ModeReward, ModeChestOpened)):
        return handle_card_reward(view, cursor, key, legal)
    if isinstance(view.mode, ModeRestSite):
        return handle_rest_site(view, cursor, key, legal)
    if isinstance(view.mode, (ModeMap, ModeCombatEnded)):
        return handle_map(view, cursor, key, legal)
    if isinstance(view.mode, ModeShop):
        return handle_shop(view, cursor, key, legal)
    if isinstance(view.mode, ModeEvent):
        return handle_event(view, cursor, key, legal)
    if isinstance(view.mode, ModeChest):
        return handle_chest(view, cursor, key, legal)
    return None


def handle_shop(view, cursor: Cursor, key: int, legal=None):
    ls = legal_set(legal) if legal is not None else None
    if cursor.mode == "purge":
        deck = view.deck
        if is_back(key):
            cursor.mode = "primary"
            cursor.idx = 0
            return None
        if not deck:
            return None
        if cursor.idx >= len(deck):
            cursor.idx = len(deck) - 1
        if is_up(key):
            cursor.idx = max(0, cursor.idx - 1)
        elif is_down(key):
            cursor.idx = min(len(deck) - 1, cursor.idx + 1)
        elif key in ENTER_KEYS:
            idx = cursor.idx
            if ls is not None and not is_legal(ls, ActionType.ShopPurge, [idx]):
                cursor.error = "Can't purge that card"
                return None
            cursor.mode = "primary"
            cursor.idx = 0
            return Action(ActionType.ShopPurge, [idx])
        return None

    rows = shop_rows(view)
    if not rows:
        return None
    if cursor.idx >= len(rows):
        cursor.idx = len(rows) - 1

    if is_up(key):
        cursor.idx = max(0, cursor.idx - 1)
    elif is_down(key):
        cursor.idx = min(len(rows) - 1, cursor.idx + 1)
    elif key in ENTER_KEYS:
        kind, label, _price, payload = rows[cursor.idx]
        if kind == "leave":
            return Action(ActionType.RoomExit, [])
        if kind == "purge":
            if ls is not None and not _shop_row_legal(ls, "purge", None):
                cursor.error = "Can't afford to purge"
                return None
            cursor.mode = "purge"
            cursor.idx = 0
            return None
        if ls is not None and not _shop_row_legal(ls, kind, payload):
            cursor.error = f"Can't buy {label}"
            return None
        at = {
            "card": ActionType.ShopBuyCard,
            "relic": ActionType.ShopBuyRelic,
            "potion": ActionType.ShopBuyPotion,
        }[kind]
        return Action(at, [payload])
    return None


# ---------- main loop ----------
def main(stdscr) -> None:
    curses.curs_set(0)
    init_colors()
    _probe_acs(stdscr)
    stdscr.keypad(True)

    ascension = 0
    seed = random.randint(0, 2**31 - 1)
    env = slai.GameEnv(ascension)
    view = env.reset(seed=seed)
    cursor = Cursor()

    while True:
        reset_if_phase_changed(cursor, view)
        legal = env.get_legal_actions()

        stdscr.erase()
        render(stdscr, view, cursor, legal, ascension)
        _maxy, maxx = stdscr.getmaxyx()
        write_segments(stdscr, 0, maxx - 1, [(f"seed {seed} ", {"dim": True})])
        stdscr.refresh()

        key = stdscr.getch()

        if key in (ord("q"), ord("Q")):
            return
        if key in (ord("r"), ord("R")):
            seed = random.randint(0, 2**31 - 1)
            view = env.reset(seed=seed)
            cursor = Cursor()
            continue

        action = handle_key(view, cursor, key, legal)
        if action is None:
            continue

        try:
            view, _terminated = env.step(action)
            cursor.error = None
        except Exception as e:
            cursor.error = str(e)


if __name__ == "__main__":
    try:
        curses.wrapper(main)
    except KeyboardInterrupt:
        sys.exit(0)
