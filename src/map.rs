// Map generation and queries.
// TODO: check if this is the exact same logic from the de-compiled original Java code

use rand::Rng;

use crate::consts::*;
use crate::state::{Entity, EntityKind, Map, Room, Position};
use crate::types::RoomType;

type Grid = [[Option<Room>; MAP_WIDTH]; MAP_HEIGHT];

// ───────── Queries ─────────

/// True if `node` has an edge to column `x` in the next row.
pub fn has_edge(node: &Room, x: usize) -> bool {
    node.edges & (1 << x) != 0
}

/// Every next-row column reachable from `node`.
pub fn edge_indices(node: &Room) -> impl Iterator<Item = usize> {
    let edges = node.edges;
    (0..MAP_WIDTH).filter(move |&x| edges & (1 << x) != 0)
}

/// Look up the node at `(y, x)` via the entity array.
pub fn node_at<'a>(map: &Map, entities: &'a [Entity], y: usize, x: usize) -> Option<&'a Room> {
    let id = map.nodes[y][x]?;
    let EntityKind::Room(node) = &entities[id].kind else {
        unreachable!()
    };
    Some(node)
}

/// Look up the node at the player's current position, if any. Returns
/// `None` at `Start` (no node picked yet) and at `BossRoom` (off the grid).
pub fn active_node<'a>(map: &Map, entities: &'a [Entity]) -> Option<&'a Room> {
    match map.position {
        Position::Overworld { y, x } => node_at(map, entities, y, x),
        Position::Start | Position::BossRoom => None,
    }
}

/// The room type of the player's current position. Returns `None` at
/// `Start`, `Some(CombatBoss)` in the boss room.
pub fn active_room_type(map: &Map, entities: &[Entity]) -> Option<RoomType> {
    match map.position {
        Position::Start => None,
        Position::BossRoom => Some(RoomType::CombatBoss),
        Position::Overworld { y, x } => node_at(map, entities, y, x).map(|n| n.room_type),
    }
}

// ───────── Generation ─────────

/// Generates a map as an intermediate grid of inline `Room`s. Callers
/// are expected to entitize the grid via `entitize_map` before storing it
/// in `GameState`.
pub fn generate_map(rng: &mut impl Rng) -> Grid {
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
                room_type: RoomType::CombatMonster,
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
                    room_type: RoomType::CombatMonster,
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
    assign_room_types(&mut nodes, rng);

    nodes
}

/// Entitizes a generated grid: each `Some(node)` is pushed into `entities`
/// as an `EntityKind::Room`, and the returned `Map` stores the entity ids.
pub fn entitize_map(grid: Grid, entities: &mut Vec<Entity>) -> Map {
    let mut nodes: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT] = [[None; MAP_WIDTH]; MAP_HEIGHT];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(node) = cell {
                let id = entities.len();
                entities.push(Entity {
                    kind: EntityKind::Room(*node),
                });
                nodes[y][x] = Some(id);
            }
        }
    }
    Map {
        nodes,
        position: Position::Start,
    }
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

    let target_parents = get_node_parents(y_target, x_target, nodes);
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
        if let Some(ref node_left) = nodes[y_source][x_left] {
            for x_t in edge_indices(node_left) {
                if x_t > x_target {
                    x_target = x_t;
                }
            }
        }
    }

    // Right to left
    if x_source < MAP_WIDTH - 1 {
        let x_right = x_source + 1;
        if let Some(ref node_right) = nodes[y_source][x_right] {
            for x_t in edge_indices(node_right) {
                if x_t < x_target {
                    x_target = x_t;
                }
            }
        }
    }

    (y_target, x_target)
}

fn get_node_parents(y: usize, x: usize, nodes: &Grid) -> Vec<(usize, usize)> {
    if y == 0 {
        return Vec::new();
    }
    let y_parent = y - 1;
    let mut parents = Vec::new();
    for (px, node) in nodes[y_parent].iter().enumerate() {
        if let Some(n) = node {
            if has_edge(n, x) {
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

    let parents_a = get_node_parents(node1.0, node1.1, nodes);
    let parents_b = get_node_parents(node2.0, node2.1, nodes);

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

fn assign_room_types(nodes: &mut Grid, rng: &mut impl Rng) {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (y, row) in nodes.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if node.is_some() {
                positions.push((y, x));
            }
        }
    }

    let num_nodes = positions.len();
    let num_rest = (FACTOR_NUM_REST_SITE * num_nodes as f32) as usize;

    let mut types = vec![RoomType::CombatMonster; num_nodes];
    for t in types.iter_mut().take(num_rest) {
        *t = RoomType::RestSite;
    }

    for i in (1..types.len()).rev() {
        let j = rng.random_range(0..=i);
        types.swap(i, j);
    }

    for (i, &(y, x)) in positions.iter().enumerate() {
        if let Some(node) = &mut nodes[y][x] {
            node.room_type = types[i];
        }
    }

    // Last floor is all rest sites
    for node in &mut nodes[MAP_HEIGHT - 1] {
        if let Some(n) = node {
            n.room_type = RoomType::RestSite;
        }
    }
}
