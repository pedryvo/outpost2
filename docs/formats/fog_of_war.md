# Outpost 2: Divided Destiny — Fog of War and Sensor Grid Specification

## 1. Overview
In *Outpost 2: Divided Destiny*, information warfare and situational awareness are fundamental to surviving New Terra's hostile environment and rival faction engagements. The terrain map is governed by a dynamic visibility grid consisting of three distinct visibility tiers:

| Fog State | Value | Description | Visual Representation |
|:---|:---:|:---|:---|
| **Unexplored** | `0` | Completely uncharted terrain. Topography, units, disasters, and ore deposits are obscured. | Pitch Black shroud (`#030712`) |
| **Explored** | `1` | Previously mapped terrain. Geographic contours, rock basalt, and roads remain visible, but dynamic enemy units and shifting hazards are hidden. | Darkened ambient tone (0.35× luminosity) |
| **Visible** | `2` | Active sensor coverage by friendly vehicles, watchtowers, or colony structures. Live unit movements and disaster updates are real-time. | Full fidelity scaled by planetary solar flux (0.2× – 1.0×) |

---

## 2. Sensor Projection Mechanics

### 2.1 Sight Radii
Each entity radiates a sensory field calculated in tile coordinate space using Euclidean distance:

$$\Delta x^2 + \Delta y^2 \le R^2$$

- **Colony Structures** (Command Center, Solar Power Array, DIRT lab): Radius $R = 8$ tiles.
- **Scout Vehicles** (Lynx, Robo-Surveyor): Radius $R = 8$ tiles.
- **Heavy & Utility Vehicles** (ConVec, Cargo Truck, Robo-Miner): Radius $R = 6$ tiles.

### 2.2 Dynamic Cycle Refresh
1. At the beginning of each simulation tick:
   - All tiles currently flagged as `Visible (2)` are demoted to `Explored (1)`.
2. For each active friendly unit and operational building:
   - Compute bounding box $(x_c \pm R, y_c \pm R)$ clipped to map dimensions $[0, W-1] \times [0, H-1]$.
   - For all cells meeting the radial constraint, promote cell to `Visible (2)`.
3. The minimap radar updates its pixel mask to reflect both explored sectors and real-time sensor coverage.

---

## 3. Diurnal Day/Night Ambient Light Modulation

The planetary rotation of New Terra creates variable solar illumination:
- **Solar Phase:** $\theta = (\text{phase} \times 2\pi)$.
- **Raw Sunlight:** $S = \sin(\theta) \times 0.45 + 0.55$, clamped to $[0.20, 1.00]$.
- **Solar Power Output:**
  $$P_{\text{solar}} = 250 \times S \text{ kW}$$
  At night ($S \approx 0.20$), solar arrays produce minimal power, necessitating auxiliary power systems (geothermal plants, tokamak reactors) or energy conservation protocols.
