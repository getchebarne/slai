use rand::Rng;

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
use crate::entity::Entity;
use crate::entity::make_entity_room;
use crate::game::Location;
use crate::types::RoomKind;
use crate::utils::push_entity;

// Intermediate grid-cell; converted to Entity via entitize_map after finalization
#[derive(Debug, Clone, Copy)]
struct Room {
    pub y: usize,
    pub x: usize,
    pub room_kind: RoomKind,
    pub edges: u8,
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

pub fn room_at<'a>(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    entities: &'a [Entity],
    y: usize,
    x: usize,
) -> Option<&'a Entity> {
    let id_room = id_rooms[y][x]?;
    Some(&entities[id_room])
}

pub fn room_at_mut<'a>(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    entities: &'a mut [Entity],
    y: usize,
    x: usize,
) -> Option<&'a mut Entity> {
    let id_room = id_rooms[y][x]?;
    Some(&mut entities[id_room])
}

pub fn get_active_room_kind(
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    entities: &[Entity],
) -> Option<RoomKind> {
    match location {
        Location::Start => None,
        Location::BossRoom => Some(RoomKind::CombatBoss),
        Location::Overworld { y, x } => room_at(id_rooms, entities, y, x).map(|n| n.room_kind),
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

    for d in 0..PATH_DENSITY {
        let mut x_source: usize = rng.random_range(0..MAP_WIDTH);
        if d == 0 {
            x_source_first = Some(x_source);
        }
        while d == 1 && Some(x_source) == x_source_first {
            x_source = rng.random_range(0..MAP_WIDTH);
        }

        let mut y_source: usize = 0;
        if nodes[y_source][x_source].is_none() {
            nodes[y_source][x_source] = Some(Room {
                y: y_source,
                x: x_source,
                room_kind: RoomKind::CombatMonster,
                edges: 0,
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
                });
            }

            if let Some(ref mut src) = nodes[y_source][x_source] {
                src.edges |= 1 << x_target;
            }

            y_source = y_target;
            x_source = x_target;
        }
    }

    trim_redundant_first_row_edges(&mut nodes);
    assign_room_kinds(&mut nodes, rng, ascension);

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
                let new_offset = if x_target > x_source {
                    rng.random_range(-1..=0)
                } else if x_target == x_source {
                    rng.random_range(-1..=1)
                } else {
                    rng.random_range(0..=1)
                };
                x_target = ((x_source as i32) + new_offset).clamp(0, MAP_WIDTH as i32 - 1) as usize;
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
    let y_parent = y - 1;
    let mut parents = Vec::new();
    for (px, node) in nodes[y_parent].iter().enumerate() {
        if let Some(n) = node {
            if has_edge(n.edges, x) {
                parents.push((y_parent, px));
            }
        }
    }
    parents
}

fn get_common_ancestor(
    node1: (usize, usize),
    node2: (usize, usize),
    nodes: &Grid,
) -> Option<(usize, usize)> {
    if node1.0 != node2.0 || node1.1 == node2.1 {
        return None;
    }

    let parents_a = get_room_parents(node1.0, node1.1, nodes);
    let parents_b = get_room_parents(node2.0, node2.1, nodes);

    for pa in &parents_a {
        if parents_b.contains(pa) {
            return Some(*pa);
        }
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

fn assign_room_kinds(nodes: &mut Grid, rng: &mut impl Rng, ascension: u8) {
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

    for i in (1..types.len()).rev() {
        let j = rng.random_range(0..=i);
        types.swap(i, j);
    }

    for (i, &(y, x)) in positions.iter().enumerate() {
        if let Some(node) = &mut nodes[y][x] {
            node.room_kind = types[i];
        }
    }

    // Row gating
    const ELITE_MIN_Y: usize = 5;
    const REST_MIN_Y: usize = 5;
    const REST_MAX_Y_EXCL: usize = 13;

    for y in 0..MAP_HEIGHT - 1 {
        for x in 0..MAP_WIDTH {
            let kind = match &nodes[y][x] {
                Some(n) => n.room_kind,
                None => continue,
            };
            let needs_swap = match kind {
                RoomKind::CombatElite => y < ELITE_MIN_Y,
                RoomKind::RestSite => y < REST_MIN_Y || y >= REST_MAX_Y_EXCL,
                _ => false,
            };
            if !needs_swap {
                continue;
            }
            // Find a CombatMonster at a row that CAN host this kind
            let mut swapped = false;
            'swap: for y2 in 0..MAP_HEIGHT - 1 {
                // Forced rows never host a relocated kind
                if y2 == 0 || y2 == MAP_ROW_TREASURE {
                    continue;
                }
                let row_ok = match kind {
                    RoomKind::CombatElite => y2 >= ELITE_MIN_Y,
                    RoomKind::RestSite => y2 >= REST_MIN_Y && y2 < REST_MAX_Y_EXCL,
                    _ => true,
                };
                if !row_ok {
                    continue;
                }
                for x2 in 0..MAP_WIDTH {
                    if (y2, x2) == (y, x) {
                        continue;
                    }
                    if let Some(other) = &nodes[y2][x2] {
                        if matches!(other.room_kind, RoomKind::CombatMonster) {
                            if let Some(n) = &mut nodes[y][x] {
                                n.room_kind = RoomKind::CombatMonster;
                            }
                            if let Some(other) = &mut nodes[y2][x2] {
                                other.room_kind = kind;
                            }
                            swapped = true;
                            break 'swap;
                        }
                    }
                }
            }
            if !swapped {
                // No free CombatMonster host row -> downgrade this node rather than violate rule
                if let Some(n) = &mut nodes[y][x] {
                    n.room_kind = RoomKind::CombatMonster;
                }
            }
        }
    }

    for node in &mut nodes[MAP_ROW_TREASURE] {
        if let Some(n) = node {
            n.room_kind = RoomKind::Treasure;
        }
    }
    for node in &mut nodes[0] {
        if let Some(n) = node {
            n.room_kind = RoomKind::CombatMonster;
        }
    }
    for node in &mut nodes[MAP_HEIGHT - 1] {
        if let Some(n) = node {
            n.room_kind = RoomKind::RestSite;
        }
    }
}
