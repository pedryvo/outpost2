//! Unit pathfinding, orders, and eight-way grid navigation.

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::sim::map::GameMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitOrder {
    Idle,
    MoveTo { target_x: i32, target_y: i32 },
    Attack { target_unit_id: u32 },
    AttackBuilding { target_building_id: u32 },
    Repair { target_unit_id: u32 },
    MineOre { deposit_x: i32, deposit_y: i32 },
    DeployStructure { kit_name: String, tile_x: i32, tile_y: i32 },
    BuildWall { target_x: i32, target_y: i32 },
    BuildTube { target_x: i32, target_y: i32 },
    ClearHazard { target_x: i32, target_y: i32 },
    Bulldoze { target_x: i32, target_y: i32 },
    Guard { anchor_x: i32, anchor_y: i32 },
    Patrol { start_x: i32, start_y: i32, end_x: i32, end_y: i32, heading_to_end: bool },
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    pos: (i32, i32),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathFinder;

impl PathFinder {
    /// Reach a passable tile beside a solid structure, choosing the shortest route.
    pub fn find_path_to_building(
        map: &GameMap,
        start: (i32, i32),
        building: (i32, i32, i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        let (x, y, w, h) = building;
        let mut best: Option<Vec<(i32, i32)>> = None;
        for ty in y - 1..=y + h {
            for tx in x - 1..=x + w {
                if tx >= x && tx < x + w && ty >= y && ty < y + h {
                    continue;
                }
                if let Some(path) = Self::find_path(map, start, (tx, ty)) {
                    if best.as_ref().is_none_or(|old| path.len() < old.len()) {
                        best = Some(path);
                    }
                }
            }
        }
        best
    }

    pub fn can_step(map: &GameMap, from: (i32, i32), to: (i32, i32)) -> bool {
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        if dx.abs() > 1 || dy.abs() > 1 || (dx == 0 && dy == 0) {
            return false;
        }
        let passable = |x: i32, y: i32| {
            x >= 0 && y >= 0 && (x as u32) < map.width && (y as u32) < map.height
                && map.get_tile(x as u32, y as u32)
                    .is_some_and(|tile| !tile.is_lava && !tile.is_wall_or_building)
        };
        passable(to.0, to.1)
            && (dx == 0 || dy == 0
                || (passable(from.0 + dx, from.1) && passable(from.0, from.1 + dy)))
    }

    /// Compute an 8-directional path from (start_x, start_y) to (goal_x, goal_y)
    pub fn find_path(
        map: &GameMap,
        start: (i32, i32),
        goal: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        if start == goal {
            return Some(Vec::new());
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut g_score: HashMap<(i32, i32), u32> = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(Node {
            cost: Self::heuristic(start, goal),
            pos: start,
        });

        // 8-directional neighbor offsets (horizontal, vertical, diagonal)
        let directions = [
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, 1), (-1, 1), (1, -1), (-1, -1),
        ];

        let mut closed_set = HashSet::new();
        let max_iterations = (map.width * map.height).max(3000);
        let mut iterations = 0;
        while let Some(Node { pos: current, .. }) = open_set.pop() {
            if !closed_set.insert(current) {
                continue;
            }

            iterations += 1;
            if iterations > max_iterations {
                // Safeguard budget to prevent frame stalls in WebAssembly
                break;
            }

            if current == goal {
                // Reconstruct path
                let mut path = Vec::new();
                let mut curr = goal;
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(curr);
                    curr = prev;
                }
                path.reverse();
                return Some(path);
            }

            let current_g = *g_score.get(&current).unwrap_or(&u32::MAX);

            for (dx, dy) in &directions {
                let neighbor = (current.0 + dx, current.1 + dy);

                if !Self::can_step(map, current, neighbor) {
                    continue;
                }

                // Step cost: diagonal = 14, cardinal = 10
                let step_cost = if *dx != 0 && *dy != 0 { 14 } else { 10 };
                let tentative_g = current_g + step_cost;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g);
                    let f_score = tentative_g + Self::heuristic(neighbor, goal);
                    open_set.push(Node {
                        cost: f_score,
                        pos: neighbor,
                    });
                }
            }
        }

        None
    }

    /// Octile distance heuristic for 8-way movement
    fn heuristic(a: (i32, i32), b: (i32, i32)) -> u32 {
        let dx = (a.0 - b.0).abs() as u32;
        let dy = (a.1 - b.1).abs() as u32;
        10 * (dx + dy).saturating_sub(6 * dx.min(dy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_never_cuts_a_building_corner() {
        let mut map = GameMap::new_blank(5, 5);
        map.get_tile_mut(2, 1).unwrap().is_wall_or_building = true;
        map.get_tile_mut(1, 2).unwrap().is_wall_or_building = true;
        assert!(!PathFinder::can_step(&map, (1, 1), (2, 2)));
        let path = PathFinder::find_path(&map, (1, 1), (3, 3)).unwrap();
        let mut previous = (1, 1);
        for step in path {
            assert!(PathFinder::can_step(&map, previous, step));
            previous = step;
        }
    }
}
