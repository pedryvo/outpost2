# Outpost 2 Pathfinding & Unit Movement Specification

Outpost 2 utilizes an 8-directional isometric pathfinding system that balances realistic collision against the hostile, hazardous terrain of planet New Terra.

## Grid Topology & Coordinate Space

- **Map Projection:** 2D Isometric Diamond Grid (`Width` $\times$ `Height` tiles).
- **Adjacency:** 8-way movement:
  - Cardinal moves (North, South, East, West): Step cost $10$.
  - Diagonal moves (NE, NW, SE, SW): Step cost $14$ ($\approx 10 \times \sqrt{2}$).
- **Distance Metric:** Octile heuristic:
  $$h(a, b) = 10 \cdot (\Delta x + \Delta y) - 6 \cdot \min(\Delta x, \Delta y)$$

## Terrain Traversability Rules

A tile $(x, y)$ is evaluated against the `MapTile` bitfield:

| Condition | Traversable? | Notes |
|---|---|---|
| `is_lava == true` | ❌ Blocked | Vehicles burn and explode upon contact with molten magma |
| `is_wall_or_building == true` | ❌ Blocked | Solid walls and active colony buildings obstruct passage |
| `is_blight == true` | ⚠️ Slowed / Hazard | The microbial blight corrodes vehicles and slows chassis speed |
| `cell_type == Bulldozed / Road` | ⚡ Speed Bonus | Units travel 30% faster along paved colonial pads |
| `cell_type == Rock / Rough` | 🐢 Speed Penalty | Rough volcanic terrain slows down heavy wheeled vehicles |

## WebAssembly Performance Safeguards

To prevent frame drops and keep the browser UI responsive at 60+ FPS:
1. **Search Budget:** A* search iterations are capped at 1,500 expansions per path request.
2. **Path Smoothing:** Units follow waypoints with sub-pixel interpolation on the canvas viewport.
3. **Collision Avoidance:** Moving units occupy their destination tile index to prevent overlap.
