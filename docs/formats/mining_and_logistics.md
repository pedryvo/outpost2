# Outpost 2: Divided Destiny — Mining & Resource Logistics Specification

## 1. Overview
In *Outpost 2: Divided Destiny*, metal extraction is the lifeblood of colony infrastructure, vehicle manufacturing, and scientific research. The planetary surface of New Terra contains underground mineral veins of two distinct grades:
- **Common Ore (Hematite / Basalt):** High-density iron, nickel, and copper ores required for structural frames, colonial tubes, and baseline vehicle hulls.
- **Rare Ore (Titanium / Platinum):** Strategic radioactive isotopes, precious metals, and semiconductors required for high-energy pulse lasers, railgun assemblies, and advanced research lab equipment.

---

## 2. Extraction & Logistics Transport Cycle

```
┌──────────────────────┐         Autonomous Haul         ┌────────────────────────┐
│  Robo-Miner Platform │ ──────────────────────────────> │   Common/Rare Smelter  │
│  (Anchored to Vein)  │ <────────────────────────────── │   (Refining to Ingot)  │
└──────────────────────┘         Return for Load         └───────────┬────────────┘
                                                                     │
                                                                     ▼
                                                         ┌────────────────────────┐
                                                         │ Colony Ore Stockpiles  │
                                                         │ (Common & Rare Tonnes) │
                                                         └────────────────────────┘
```

### 2.1 Logistics Finite State Machine (Cargo Truck)
Each Cargo Truck autonomously runs a state machine:
1. **`Idle`**: Scans the colony sensor grid for operational mineral deposits with an attached Robo-Miner (`has_miner == true` and `remaining_amount > 0`).
2. **`DrivingToMine { deposit_id }`**: Computes an 8-way isometric A* path to the mining deposit.
3. **`Loading { deposit_id, progress_pct }`**: Bores drill chutes to receive 50 tonnes of raw mineral chunks (~1.5 seconds loading cycle).
4. **`DrivingToSmelter { smelter_id, ore_type, amount }`**: Pathfinds to the nearest functional ore smelter facility.
5. **`Unloading { smelter_id, ore_type, amount, progress_pct }`**: Dumps payload into the smelter intake hopper (~1.5 seconds), crediting `colony.common_ore` or `colony.rare_ore` directly.
6. **Cycle Repeats**: Vehicle resets to `Idle` and returns to the active mine.

---

## 3. Visual Representation
- **Mineral Deposits:** Rendered on the isometric grid as metallic nodules with ore markers (Silver for Common Ore, Cyan for Rare Ore).
- **Cargo Payloads:** Cargo Trucks physically carry a metallic ore container on their rear chassis during the haul phase.
