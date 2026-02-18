// Map generation: ported from map_.py.

use rand::Rng;

use crate::state::{Map, MapNode};
use crate::types::RoomType;

pub const MAP_HEIGHT: usize = 15;
pub const MAP_WIDTH: usize = 7;

const PATH_DENSITY: usize = 6;
const ANCESTOR_GAP_MIN: usize = 3;
const ANCESTOR_GAP_MAX: usize = 5;
const FACTOR_NUM_REST_SITE: f32 = 0.25;

pub fn generate_map(rng: &mut impl Rng) -> Map {
    let mut nodes = initialize_nodes();

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
            nodes[y_source][x_source] = Some(MapNode {
                y: y_source,
                x: x_source,
                room_type: RoomType::CombatMonster, // placeholder
                x_next: Vec::new(),
            });
        }

        loop {
            if y_source >= MAP_HEIGHT - 1 {
                break;
            }

            let (y_target, x_target) = create_target(y_source, x_source, &nodes, rng);

            if nodes[y_target][x_target].is_none() {
                nodes[y_target][x_target] = Some(MapNode {
                    y: y_target,
                    x: x_target,
                    room_type: RoomType::CombatMonster,
                    x_next: Vec::new(),
                });
            }

            // Add edge
            if let Some(ref mut src) = nodes[y_source][x_source] {
                if !src.x_next.contains(&x_target) {
                    src.x_next.push(x_target);
                }
            }

            y_source = y_target;
            x_source = x_target;
        }
    }

    trim_redundant_first_row_edges(&mut nodes);
    assign_room_types(&mut nodes, rng);

    Map {
        nodes,
        active_y: None,
        active_x: None,
        boss_room_y: MAP_HEIGHT,
    }
}

fn initialize_nodes() -> Vec<Vec<Option<MapNode>>> {
    let mut nodes = Vec::with_capacity(MAP_HEIGHT);
    for _ in 0..MAP_HEIGHT {
        let mut row = Vec::with_capacity(MAP_WIDTH);
        for _ in 0..MAP_WIDTH {
            row.push(None);
        }
        nodes.push(row);
    }
    nodes
}

fn create_target(
    y_source: usize,
    x_source: usize,
    nodes: &[Vec<Option<MapNode>>],
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

    // Check for common ancestors and adjust
    let target_parents = get_node_parents(y_target, x_target, nodes);
    for parent in &target_parents {
        if parent.0 == y_source && parent.1 == x_source {
            continue;
        }
        if let Some(_ancestor) = get_common_ancestor(
            (parent.0, parent.1),
            (y_source, x_source),
            nodes,
            ANCESTOR_GAP_MAX,
        ) {
            let ancestor_gap = y_target - _ancestor.0;
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
            for &x_t in &node_left.x_next {
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
            for &x_t in &node_right.x_next {
                if x_t < x_target {
                    x_target = x_t;
                }
            }
        }
    }

    (y_target, x_target)
}

fn get_node_parents(y: usize, x: usize, nodes: &[Vec<Option<MapNode>>]) -> Vec<(usize, usize)> {
    if y == 0 {
        return Vec::new();
    }
    let y_parent = y - 1;
    let mut parents = Vec::new();
    for node in &nodes[y_parent] {
        if let Some(n) = node {
            if n.x_next.contains(&x) {
                parents.push((n.y, n.x));
            }
        }
    }
    parents
}

fn get_common_ancestor(
    node1: (usize, usize),
    node2: (usize, usize),
    nodes: &[Vec<Option<MapNode>>],
    max_gap: usize,
) -> Option<(usize, usize)> {
    if node1.0 != node2.0 || node1.1 == node2.1 {
        return None;
    }

    let (left, right) = if node1.1 < node2.0 {
        (node1, node2)
    } else {
        (node2, node1)
    };

    let parents_left = get_node_parents(left.0, left.1, nodes);
    if parents_left.is_empty() {
        return None;
    }
    let parents_right = get_node_parents(right.0, right.1, nodes);
    if parents_right.is_empty() {
        return None;
    }

    let mut y_cur = node1.0;
    let y_min = node1.0.saturating_sub(max_gap);
    while y_cur >= y_min && y_cur > 0 {
        let pl = parents_left.iter().max_by_key(|p| p.1)?;
        let pr = parents_right.iter().min_by_key(|p| p.1)?;
        if pl == pr {
            return Some(*pl);
        }
        y_cur -= 1;
    }
    None
}

fn trim_redundant_first_row_edges(nodes: &mut Vec<Vec<Option<MapNode>>>) {
    let mut x_seen = Vec::new();
    let mut x_remove = Vec::new();

    for x_source in 0..MAP_WIDTH {
        if let Some(ref mut node) = nodes[0][x_source] {
            node.x_next.retain(|x| {
                if x_seen.contains(x) {
                    false
                } else {
                    x_seen.push(*x);
                    true
                }
            });
            if node.x_next.is_empty() {
                x_remove.push(x_source);
            }
        }
    }

    for x in x_remove {
        nodes[0][x] = None;
    }
}

fn assign_room_types(nodes: &mut Vec<Vec<Option<MapNode>>>, rng: &mut impl Rng) {
    // Collect all node positions
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for row in nodes.iter() {
        for node in row.iter() {
            if let Some(n) = node {
                positions.push((n.y, n.x));
            }
        }
    }

    let num_nodes = positions.len();
    let num_rest = (FACTOR_NUM_REST_SITE * num_nodes as f32) as usize;

    let mut types = vec![RoomType::CombatMonster; num_nodes];
    for t in types.iter_mut().take(num_rest) {
        *t = RoomType::RestSite;
    }

    // Shuffle
    for i in (1..types.len()).rev() {
        let j = rng.random_range(0..=i);
        types.swap(i, j);
    }

    for (i, (y, x)) in positions.iter().enumerate() {
        if let Some(node) = &mut nodes[*y][*x] {
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
