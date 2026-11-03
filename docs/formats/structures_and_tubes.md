# Outpost 2: Divided Destiny — Colonial Structures & Tube Network Specification

## 1. Overview
In *Outpost 2: Divided Destiny*, the survival of both Eden and Plymouth relies on continuous expansion of modular underground and surface facilities. Every deployed structure must be integrated into the colony's arterial life-support and electrical grid via pressurized transport tubes.

---

## 2. Colony Structure Classes & Kit Metrics

| Structure | Footprint ($W \times H$) | Common Ore | Rare Ore | Power Δ | Purpose |
|:---|:---:|:---:|:---:|:---:|:---|
| **Command Center** | $3 \times 3$ | 500 t | 100 t | -40 kW | Colony primary nexus, life support core, radar transmitter. |
| **Solar Power Array** | $2 \times 2$ | 200 t | 0 t | +250 kW* | Clean diurnal power generation (*modulated by solar phase). |
| **Tokamak Fusion Reactor**| $3 \times 3$ | 600 t | 250 t | +800 kW | Baseload thermonuclear energy unaffected by night cycles. |
| **Common Ore Smelter** | $2 \times 2$ | 350 t | 50 t | -50 kW | Refines raw basalt/hematite into common building metal. |
| **Rare Ore Smelter** | $2 \times 2$ | 400 t | 150 t | -75 kW | Purifies titanium, platinum, and radioactive isotopes. |
| **D.I.R.T. Laboratory** | $2 \times 2$ | 300 t | 100 t | -60 kW | Planetary biology, vulcanology, and disaster mitigation research. |
| **Standard Research Lab**| $2 \times 2$ | 500 t | 200 t | -90 kW | Advanced physics, laser weaponry, spacecraft engineering. |
| **Colonial Residence** | $2 \times 2$ | 150 t | 0 t | -20 kW | Housing units providing life support for 100 colonists. |
| **Bio-Dome Hydroponics**| $2 \times 2$ | 200 t | 50 t | -30 kW | Agricultural sustenance keeping food rations above starvation thresholds. |
| **Vehicle Factory** | $3 \times 3$ | 500 t | 150 t | -120 kW | Chassis fabrication for scouts, cargo transports, and combat tanks. |

---

## 3. Pressurized Transport Tube Network
1. **Connectivity Rule:**
   - Any building erected outside of direct contact with the Command Center is piped via automatic arterial conduits.
   - The pathfinder computes an orthogonal corridor $(x_1, y_1) \to (x_2, y_2)$ connecting the structural airlocks.
2. **Visual & Sensor Representation:**
   - Tubes feature pressurized titanium exterior bulkheads (`#475569`) with an interior glowing cyan life-support core (`#38bdf8`).
   - Tubes are rendered directly onto the terrain grid and highlighted as connected links on the tactical radar minimap.
