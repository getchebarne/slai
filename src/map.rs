use rand::Rng;

use crate::consts::ANCESTOR_GAP_MAX;
use crate::consts::ANCESTOR_GAP_MIN;
use crate::consts::FACTOR_NUM_ELITE;
use crate::consts::FACTOR_NUM_ELITE_A1_MULT;
use crate::consts::FACTOR_NUM_EVENT;
use crate::consts::FACTOR_NUM_REST_SITE;
use crate::consts::FACTOR_NUM_SHOP;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_ROW_TREASURE;
use crate::consts::MAP_WIDTH;
use crate::consts::PATH_DENSITY;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::Location;
use crate::types::RoomKind;
use crate::utils::push_entity;
use crate::utils::shuffle;

// Intermediate grid-cell; converted to Entity via entitize_map after finalization
#[derive(Debug, Clone, Copy)]
struct Room {
    pub y: usize,
    pub x: usize,
    pub room_kind: RoomKind,
    pub edges: u8,
    // addParent appends without a uniqueness test, so one parent can appear twice
    pub parents: [u8; PATH_DENSITY],
    pub parents_len: u8,
}

type Grid = [[Option<Room>; MAP_WIDTH]; MAP_HEIGHT];

/// True if the edge bitmap has an edge to column `x` in the next row.
pub fn has_edge(edges: u8, x: usize) -> bool {
    edges & (1 << x) != 0
}

/// Every next-row column reachable from an edge bitmap.
pub fn edge_indices(edges: u8) -> impl Iterator<Item = usize> {
    (0..MAP_WIDTH).filter(move |&x| edges & (1 << x) != 0)
}

fn room_at<'a>(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    entities: &'a [Entity],
    y: usize,
    x: usize,
) -> Option<&'a Entity> {
    let id_room = id_rooms[y][x]?;
    Some(&entities[id_room])
}

pub fn get_active_room_kind(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
) -> Option<RoomKind> {
    match location {
        Location::Start => None,
        Location::BossRoom => Some(RoomKind::CombatBoss),
        Location::Overworld { y, x } => {
            room_at(id_rooms, entities, y, x).map(|room| room.room_kind)
        }
    }
}

type IdRooms = [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT];

pub fn generate_map(
    rng: &mut impl Rng,
    entities: &mut Vec<Entity>,
    ascension: u8,
) -> (IdRooms, Location) {
    let grid = generate_grid(rng, ascension);
    entitize_grid(grid, entities)
}

fn generate_grid(rng: &mut impl Rng, ascension: u8) -> Grid {
    let mut nodes: Grid = [[None; MAP_WIDTH]; MAP_HEIGHT];

    let mut x_source_first: Option<usize> = None;

    for idx_path in 0..PATH_DENSITY {
        let mut x_source: usize = rng.random_range(0..MAP_WIDTH);
        if idx_path == 0 {
            x_source_first = Some(x_source);
        }
        while idx_path == 1 && Some(x_source) == x_source_first {
            x_source = rng.random_range(0..MAP_WIDTH);
        }

        let mut y_source: usize = 0;
        if nodes[y_source][x_source].is_none() {
            nodes[y_source][x_source] = Some(Room {
                y: y_source,
                x: x_source,
                room_kind: RoomKind::CombatMonster,
                edges: 0,
                parents: [0; PATH_DENSITY],
                parents_len: 0,
            });
        }

        loop {
            if y_source >= MAP_HEIGHT - 1 {
                break;
            }

            let (y_target, x_target) = create_target(y_source, x_source, &nodes, rng);

            if nodes[y_target][x_target].is_none() {
                nodes[y_target][x_target] = Some(Room {
                    y: y_target,
                    x: x_target,
                    room_kind: RoomKind::CombatMonster,
                    edges: 0,
                    parents: [0; PATH_DENSITY],
                    parents_len: 0,
                });
            }

            if let Some(ref mut src) = nodes[y_source][x_source] {
                src.edges |= 1 << x_target;
            }
            if let Some(ref mut dst) = nodes[y_target][x_target] {
                let len = dst.parents_len as usize;
                if len < dst.parents.len() {
                    dst.parents[len] = x_source as u8;
                    dst.parents_len += 1;
                }
            }

            y_source = y_target;
            x_source = x_target;
        }
    }

    // Pre-trim row-0 edges define parenthood for row 1: the source keeps stale
    // parent links after redundant-edge removal, and their surviving children
    // still count as siblings during Room-kind assignment
    let mut edges_row0_pretrim = [0u8; MAP_WIDTH];
    for (x, node) in nodes[0].iter().enumerate() {
        if let Some(node) = node {
            edges_row0_pretrim[x] = node.edges;
        }
    }

    trim_redundant_first_row_edges(&mut nodes);
    assign_room_kinds(&mut nodes, rng, ascension, &edges_row0_pretrim);

    nodes
}

fn entitize_grid(grid: Grid, entities: &mut Vec<Entity>) -> (IdRooms, Location) {
    let mut id_rooms: IdRooms = [[None; MAP_WIDTH]; MAP_HEIGHT];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(room) = cell {
                let id_room = push_entity(
                    entities,
                    make_entity_room(room.y, room.x, room.room_kind, room.edges),
                );
                id_rooms[y][x] = Some(id_room);
            }
        }
    }
    (id_rooms, Location::Start)
}

fn create_target(
    y_source: usize,
    x_source: usize,
    nodes: &Grid,
    rng: &mut impl Rng,
) -> (usize, usize) {
    let y_target = y_source + 1;

    let offset_x: i32 = if x_source == 0 {
        rng.random_range(0..=1)
    } else if x_source == MAP_WIDTH - 1 {
        rng.random_range(-1..=0)
    } else {
        rng.random_range(-1..=1)
    };

    let mut x_target = (x_source as i32 + offset_x).clamp(0, MAP_WIDTH as i32 - 1) as usize;

    let target_parents = get_room_parents(y_target, x_target, nodes);
    for &(py, px) in &target_parents {
        if py == y_source && px == x_source {
            continue;
        }
        if let Some(ancestor) = get_common_ancestor((py, px), (y_source, x_source), nodes) {
            let ancestor_gap = y_target - ancestor.0;
            if ancestor_gap < ANCESTOR_GAP_MIN {
                let x_end = MAP_WIDTH as i32 - 1;
                let base = x_source as i32;

                // Each arm resolves its own out-of-range case: two bounce, one falls back
                let x_new = if x_target > x_source {
                    let rolled = base + rng.random_range(-1..=0);
                    if rolled < 0 { base } else { rolled }
                } else if x_target == x_source {
                    let rolled = base + rng.random_range(-1..=1);
                    if rolled > x_end {
                        base - 1
                    } else if rolled < 0 {
                        base + 1
                    } else {
                        rolled
                    }
                } else {
                    let rolled = base + rng.random_range(0..=1);
                    if rolled > x_end { base } else { rolled }
                };
                x_target = x_new as usize;
            }
        }
    }

    // Trim to prevent path overlap (left to right)
    if x_source > 0 {
        let x_left = x_source - 1;
        if let Some(ref room_left) = nodes[y_source][x_left] {
            for x_t in edge_indices(room_left.edges) {
                if x_t > x_target {
                    x_target = x_t;
                }
            }
        }
    }

    // Right to left
    if x_source < MAP_WIDTH - 1 {
        let x_right = x_source + 1;
        if let Some(ref room_right) = nodes[y_source][x_right] {
            for x_t in edge_indices(room_right.edges) {
                if x_t < x_target {
                    x_target = x_t;
                }
            }
        }
    }

    (y_target, x_target)
}

fn get_room_parents(y: usize, x: usize, nodes: &Grid) -> Vec<(usize, usize)> {
    if y == 0 {
        return Vec::new();
    }
    match &nodes[y][x] {
        Some(room) => room.parents[..room.parents_len as usize]
            .iter()
            .map(|&px| (y - 1, px as usize))
            .collect(),
        None => Vec::new(),
    }
}

// Climbs both sides row by row — max X on the left, min X on the right — until they
// meet. The left/right split keeps the source's `node1.x < node2.y` typo
fn get_common_ancestor(
    node1: (usize, usize),
    node2: (usize, usize),
    nodes: &Grid,
) -> Option<(usize, usize)> {
    if node1.0 != node2.0 || node1.1 == node2.1 {
        return None;
    }

    let (mut l_node, mut r_node) = if node1.1 < node2.0 {
        (node1, node2)
    } else {
        (node2, node1)
    };

    let y_start = node1.0 as i32;
    let mut current_y = y_start;
    while current_y >= 0 && current_y >= y_start - ANCESTOR_GAP_MAX as i32 {
        let parents_l = get_room_parents(l_node.0, l_node.1, nodes);
        let parents_r = get_room_parents(r_node.0, r_node.1, nodes);
        if parents_l.is_empty() || parents_r.is_empty() {
            return None;
        }
        l_node = *parents_l
            .iter()
            .max_by_key(|parent| parent.1)
            .expect("non-empty");
        r_node = *parents_r
            .iter()
            .min_by_key(|parent| parent.1)
            .expect("non-empty");
        if l_node == r_node {
            return Some(l_node);
        }
        current_y -= 1;
    }
    None
}

fn trim_redundant_first_row_edges(nodes: &mut Grid) {
    let mut x_seen: u8 = 0;
    let mut x_remove: Vec<usize> = Vec::new();

    for x_source in 0..MAP_WIDTH {
        if let Some(ref mut node) = nodes[0][x_source] {
            // Remove edges that point to already-seen targets
            node.edges &= !x_seen;
            x_seen |= node.edges;

            if node.edges == 0 {
                x_remove.push(x_source);
            }
        }
    }

    for x in x_remove {
        nodes[0][x] = None;
    }
}

const ELITE_MIN_Y: usize = 5;
const REST_MIN_Y: usize = 5;
const REST_MAX_Y_EXCL: usize = 13;

fn rule_row_ok(kind: RoomKind, y: usize) -> bool {
    match kind {
        RoomKind::CombatElite => y >= ELITE_MIN_Y,
        RoomKind::RestSite => y >= REST_MIN_Y && y < REST_MAX_Y_EXCL,
        _ => true,
    }
}

// Vanilla parent-rule kinds (Treasure is listed too but never drawable)
fn rule_parent_applies(kind: RoomKind) -> bool {
    matches!(
        kind,
        RoomKind::RestSite | RoomKind::Shop | RoomKind::CombatElite
    )
}

fn rule_sibling_applies(kind: RoomKind) -> bool {
    matches!(
        kind,
        RoomKind::RestSite
            | RoomKind::CombatMonster
            | RoomKind::Unknown
            | RoomKind::CombatElite
            | RoomKind::Shop
    )
}

// Forced rows stamped first, then each
// node takes the first drawn kind passing the row/parent/sibling rules; nodes
// with no passing kind stay CombatMonster and never block later siblings
fn assign_room_kinds(
    nodes: &mut Grid,
    rng: &mut impl Rng,
    ascension: u8,
    edges_row0_pretrim: &[u8; MAP_WIDTH],
) {
    // Ratio denominator counts every node except row 13; forced rows
    // (0=Monster, 8=Treasure, 14=Rest) count but never receive a drawn kind
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let mut num_rooms: usize = 0;
    for (y, row) in nodes.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if node.is_none() {
                continue;
            }
            if y != MAP_HEIGHT - 2 {
                num_rooms += 1;
            }
            if y != 0 && y != MAP_ROW_TREASURE && y != MAP_HEIGHT - 1 {
                positions.push((y, x));
            }
        }
    }

    // Counts round to nearest; A1+ spawns ~60% more elites
    let num_rest = (num_rooms as f32 * FACTOR_NUM_REST_SITE).round() as usize;
    let num_elite = if ascension >= 1 {
        (num_rooms as f32 * FACTOR_NUM_ELITE * FACTOR_NUM_ELITE_A1_MULT).round() as usize
    } else {
        (num_rooms as f32 * FACTOR_NUM_ELITE).round() as usize
    };
    let num_event = (num_rooms as f32 * FACTOR_NUM_EVENT).round() as usize;
    let num_shop = (num_rooms as f32 * FACTOR_NUM_SHOP).round() as usize;

    let mut types = vec![RoomKind::CombatMonster; positions.len()];
    let mut offset = 0;
    for (count, kind) in [
        (num_rest, RoomKind::RestSite),
        (num_elite, RoomKind::CombatElite),
        (num_event, RoomKind::Unknown),
        (num_shop, RoomKind::Shop),
    ] {
        types[offset..offset + count].fill(kind);
        offset += count;
    }
    shuffle(&mut types, rng);

    // Forced rows are assigned up front so the rules see them
    for node in &mut nodes[MAP_HEIGHT - 1] {
        if let Some(n) = node {
            n.room_kind = RoomKind::RestSite;
        }
    }
    for node in &mut nodes[0] {
        if let Some(n) = node {
            n.room_kind = RoomKind::CombatMonster;
        }
    }
    for node in &mut nodes[MAP_ROW_TREASURE] {
        if let Some(n) = node {
            n.room_kind = RoomKind::Treasure;
        }
    }
    let mut assigned = [[false; MAP_WIDTH]; MAP_HEIGHT];
    for y in [0, MAP_ROW_TREASURE, MAP_HEIGHT - 1] {
        for x in 0..MAP_WIDTH {
            assigned[y][x] = nodes[y][x].is_some();
        }
    }

    for &(y, x) in &positions {
        // Row 1 derives parenthood from pre-trim edges; deeper rows are untrimmed
        let parents: Vec<(usize, usize)> = if y == 1 {
            (0..MAP_WIDTH)
                .filter(|&px| has_edge(edges_row0_pretrim[px], x))
                .map(|px| (0, px))
                .collect()
        } else {
            get_room_parents(y, x, nodes)
        };

        // Kind bitmasks of assigned parents and assigned same-parent siblings
        let mut mask_parents: u8 = 0;
        let mut mask_siblings: u8 = 0;
        for &(py, px) in &parents {
            let Some(parent) = &nodes[py][px] else {
                continue;
            };
            if assigned[py][px] {
                mask_parents |= 1 << parent.room_kind as u8;
            }
            for cx in edge_indices(parent.edges) {
                if cx == x {
                    continue;
                }
                if nodes[y][cx].is_some() && assigned[y][cx] {
                    mask_siblings |= 1 << nodes[y][cx].unwrap().room_kind as u8;
                }
            }
        }

        let pick = types.iter().position(|&kind| {
            rule_row_ok(kind, y)
                && !(rule_parent_applies(kind) && mask_parents & (1 << kind as u8) != 0)
                && !(rule_sibling_applies(kind) && mask_siblings & (1 << kind as u8) != 0)
        });
        if let Some(idx) = pick {
            let kind = types.remove(idx);
            if let Some(n) = &mut nodes[y][x] {
                n.room_kind = kind;
            }
            assigned[y][x] = true;
        }
    }
}

pub const fn make_entity_room(y: usize, x: usize, room_kind: RoomKind, room_edges: u8) -> Entity {
    Entity {
        kind: EntityKind::Room,
        room_y: y,
        room_x: x,
        room_kind,
        room_edges,
        ..ENTITY_ZERO
    }
}
