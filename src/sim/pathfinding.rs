//! Unit Pathfinding, Orders, and Grid Navigation (A* on Isometric Grid)

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::sim::map::GameMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitOrder {
    Idle,
    MoveTo { target_x: i32, target_y: i32 },
    Attack { target_unit_id: u32 },
    Repair { target_unit_id: u32 },
    MineOre { deposit_x: i32, deposit_y: i32 },
    DeployStructure { kit_name: String, tile_x: i32, tile_y: i32 },
    BuildWall { target_x: i32, target_y: i32 },
    BuildTube { target_x: i32, target_y: i32 },
    ClearHazard { target_x: i32, target_y: i32 },
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
    /// Compute an 8-directional path from (start_x, start_y) to (goal_x, goal_y)
    pub fn find_path(
        map: &GameMap,
        start: (i32, i32),
        goal: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        if start == goal {
            return Some(vec![goal]);
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

        let mut iterations = 0;
        while let Some(Node { pos: current, .. }) = open_set.pop() {
            iterations += 1;
            if iterations > 1500 {
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

                // Boundary check
                if neighbor.0 < 0 || neighbor.0 >= map.width as i32 || neighbor.1 < 0 || neighbor.1 >= map.height as i32 {
                    continue;
                }

                // Terrain passability check: cannot cross active lava or solid buildings
                if let Some(t) = map.get_tile(neighbor.0 as u32, neighbor.1 as u32) {
                    if t.is_lava || t.is_wall_or_building {
                        continue;
                    }
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
