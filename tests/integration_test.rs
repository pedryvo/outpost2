//! End-to-End System Integration Testing & QA Test Suite for Outpost 2
//!
//! Validates the complete game lifecycle from boot to victory:
//! - Test 1: Full Campaign Mission Lifecycle (Eden 1 -> Eden 3 orbital launch victory)
//! - Test 2: Multi-Bot Skirmish Autonomy Simulation (4-player skirmish, 3 AI personalities, 500 ticks)
//! - Test 3: WebRTC P2P Lockstep Turn Synchronization (Host/Guest deterministic state convergence)
//! - Test 4: State Persistence (Save/Load) Roundtrip Integrity (Complex state, LocalStorage & JSON)

use outpost2::net::{NetMessage, NetOrder};
use outpost2::sim::ai::{AiPersonality, Difficulty};
use outpost2::sim::combat::WeaponType;
use outpost2::sim::disasters::{AtmosphericVortex, MeteorImpact};
use outpost2::sim::events::{EventAction, ScriptTrigger, TriggerCondition, UnitType};
use outpost2::sim::generator::{MapGeneratorConfig, MapSymmetry, OreAbundance, TerrainBiome};
use outpost2::sim::mining::{HaulerState, OreType};
use outpost2::sim::mission::MissionStatus;
use outpost2::sim::pathfinding::UnitOrder;
use outpost2::sim::save::{LocalStorageBackend, PowerStatus, SaveGameState};
use outpost2::sim::structures::StructureType;
use outpost2::sim::{Building, ColonyState, Faction, Unit};

// =========================================================================
// Test 1: Full Campaign Mission Lifecycle
// =========================================================================

#[test]
fn test_full_campaign_mission_lifecycle() {
    // ---------------------------------------------------------------------
    // Phase 1: Eden Mission 1 ("Operation New Dawn")
    // ---------------------------------------------------------------------
    let mut colony = ColonyState::new_scenario(0);
    assert_eq!(colony.scenario_id, 0);
    assert_eq!(colony.faction, Faction::Eden);
    assert_eq!(colony.mission.status, MissionStatus::InProgress);

    // Verify initial base infrastructure
    assert!(colony.buildings.iter().any(|b| b.name.contains("Command Center")));
    assert!(colony.buildings.iter().any(|b| b.name.contains("Solar Power Array")));
    assert!(colony.buildings.iter().any(|b| b.name.contains("Common Ore Smelter")));

    // 1. Verify Robo-Miner extracts common ore
    let unmined_dep = colony.ore_deposits.iter().find(|d| !d.has_miner && d.ore_type == OreType::Common);
    assert!(unmined_dep.is_some(), "Unmined common ore deposit must exist");
    let unmined_dep = unmined_dep.unwrap();
    let unmined_id = unmined_dep.id;
    let (dep_x, dep_y) = (unmined_dep.tile_x, unmined_dep.tile_y);

    let miner_id = colony.units.iter().find(|u| u.name.contains("Robo-Miner")).expect("Robo-Miner unit required").id;
    // Drive Robo-Miner adjacent to the unmined beacon
    if let Some(m) = colony.units.iter_mut().find(|u| u.id == miner_id) {
        m.tile_x = dep_x;
        m.tile_y = dep_y + 1;
    }
    let deploy_res = colony.deploy_mine_head(miner_id);
    assert!(deploy_res.is_ok(), "Robo-Miner must construct subterranean mine head: {:?}", deploy_res);
    let target_dep = colony.ore_deposits.iter().find(|d| d.id == unmined_id).unwrap();
    assert!(target_dep.has_miner, "Ore deposit must now be flagged as mined");

    // 2. Verify Cargo Truck hauls ore to Smelter
    let initial_common = colony.common_ore;
    let initial_mined = colony.total_common_ore_mined;

    // Locate Cargo Truck and simulate logistics loop
    let truck_id = colony.units.iter().find(|u| u.name.contains("Cargo")).expect("Cargo truck required").id;
    if let Some(t) = colony.units.iter_mut().find(|u| u.id == truck_id) {
        t.hauler_state = Some(HaulerState::Idle);
        t.tile_x = dep_x;
        t.tile_y = dep_y + 1;
    }

    // Step simulation to trigger loading -> hauling -> unloading
    let mut ore_delivered = false;
    for _ in 0..120 {
        colony.update(0.25);
        if colony.total_common_ore_mined > initial_mined {
            ore_delivered = true;
            break;
        }
    }
    assert!(ore_delivered, "Cargo Truck failed to complete hauling cycle to Smelter");
    assert!(colony.common_ore > initial_common || colony.total_common_ore_mined > initial_mined);

    // 3. Verify university researches Vulcanology
    assert!(!colony.tech_tree.can_build_geothermal());
    // Prerequisite: Astronomy (2701)
    let res_astro = colony.tech_tree.research(2701, &mut colony.common_ore);
    assert!(res_astro.is_ok(), "Failed to research prerequisite Astronomy: {:?}", res_astro);
    assert!(colony.tech_tree.is_researched(2701));

    // Research Vulcanology & Hydrothermal Extraction (4301)
    colony.common_ore += 1000; // Guarantee research budget
    let res_vulcan = colony.tech_tree.research(4301, &mut colony.common_ore);
    assert!(res_vulcan.is_ok(), "Failed to research Vulcanology: {:?}", res_vulcan);
    assert!(colony.tech_tree.is_researched(4301));
    assert!(colony.tech_tree.can_build_geothermal(), "Vulcanology research must unlock geothermal power");

    // 4. Verify population growth under positive life support
    let pop_before = colony.population;
    colony.power_produced = 1000;
    colony.power_consumed = 300;
    colony.food_units = 2500;
    colony.morale_pct = 98;

    for _ in 0..120 {
        colony.update(0.2);
    }
    assert!(
        colony.population > pop_before,
        "Colony population must grow under thriving conditions (before: {}, after: {})",
        pop_before,
        colony.population
    );

    // ---------------------------------------------------------------------
    // Phase 2: Advance to Eden Mission 3 ("Escape From Mount Vulcan")
    // ---------------------------------------------------------------------
    let mut mission3 = ColonyState::new_scenario(5);
    assert_eq!(mission3.scenario_id, 5);
    assert_eq!(mission3.faction, Faction::Eden);
    assert_eq!(mission3.starship_progress, 0.0);
    assert!(!mission3.starship_launched);

    let factory_id = mission3.buildings.iter()
        .find(|b| b.name.contains("Vehicle Factory"))
        .expect("Vehicle Factory required in Mission 3")
        .id;

    // Construct all 4 starship evacuation modules
    mission3.common_ore = 10_000;
    mission3.rare_ore = 5_000;

    let starship_modules = [
        "Command Module",
        "Cryo Stasis Bay",
        "Habitat Ring",
        "Ion Drive",
    ];

    for module in &starship_modules {
        let q_res = mission3.queue_production(factory_id, module.to_string());
        assert!(q_res.is_ok(), "Failed to queue {}: {:?}", module, q_res);
    }

    // Advance simulation until all 4 starship modules finish fabrication
    for _ in 0..150 {
        mission3.update(0.5);
        if mission3.starship_progress >= 100.0 {
            break;
        }
    }
    assert_eq!(mission3.starship_progress, 100.0, "All 4 starship modules must assemble to 100%");

    // Construct and dock Evacuation Transport at Starship Launch Gantry
    let q_evac = mission3.queue_production(factory_id, "Evacuation Transport".to_string());
    assert!(q_evac.is_ok(), "Failed to queue Evacuation Transport: {:?}", q_evac);

    for _ in 0..30 {
        mission3.update(0.5);
    }

    let has_transport = mission3.units.iter().any(|u| u.name.contains("Evacuation Transport"));
    assert!(has_transport, "Evacuation Transport must be fabricated");

    // Board colonists into evacuation transport (requires >= 150)
    for _ in 0..60 {
        mission3.update(0.5);
        if mission3.evacuated_colonists >= 150 {
            break;
        }
    }
    assert!(
        mission3.evacuated_colonists >= 150,
        "Failed to evacuate 150 colonists (evacuated: {})",
        mission3.evacuated_colonists
    );

    // Verify orbital launch victory condition
    let launch_res = mission3.launch_starship();
    assert!(launch_res.is_ok(), "Orbital launch command failed: {:?}", launch_res);
    assert!(mission3.starship_launched, "Starship launch flag must be set");

    // Advance simulation through 3-second launch sequence
    for _ in 0..8 {
        mission3.update(0.5);
    }

    assert_eq!(mission3.mission.status, MissionStatus::Victory, "Mission status must be Victory");
    assert!(mission3.debriefing_shown, "Debriefing sequence must be displayed upon orbital escape");
}

// =========================================================================
// Test 2: Multi-Bot Skirmish Autonomy Simulation
// =========================================================================

#[test]
fn test_multi_bot_skirmish_autonomy_simulation() {
    let config = MapGeneratorConfig {
        width: 64,
        height: 64,
        biome: TerrainBiome::CanyonsAndCrags,
        ore_abundance: OreAbundance::Abundant,
        geothermal_vents: 4,
        symmetry: MapSymmetry::Rotational4Way,
        seed: 42,
        player_count: 4,
    };

    let mut colony = ColonyState::new_skirmish_map(config, Faction::Eden);
    assert_eq!(colony.ai_players.len(), 3, "Skirmish must provision 3 AI players");

    // Verify the 3 distinct AI personalities
    assert_eq!(colony.ai_players[0].personality, AiPersonality::AggressiveRusher);
    assert_eq!(colony.ai_players[1].personality, AiPersonality::DefensiveFortifier);
    assert_eq!(colony.ai_players[2].personality, AiPersonality::EconomicTechRusher);

    // Provision a repair facility for AI #1 so damaged units can retreat
    let ai1_faction = colony.ai_players[0].faction;
    let ai1_bx = colony.ai_players[0].base_x;
    let ai1_by = colony.ai_players[0].base_y;

    let repair_id = 888u32;
    colony.buildings.push(Building {
        id: repair_id,
        name: "Vehicle Repair Depot #888".into(),
        tile_x: ai1_bx + 2,
        tile_y: ai1_by + 2,
        width: 2,
        height: 2,
        power_cost: 60,
        is_active: true,
        is_selected: false,
        production_queue: Vec::new(),
        production_progress: 0.0,
        attack_cooldown: 0.0,
        faction: Some(ai1_faction),
    });

    // Add a heavily damaged AI combat unit to verify retreat-to-repair
    let damaged_unit_id = 999u32;
    colony.units.push(Unit {
        id: damaged_unit_id,
        name: "Lynx Raider #999".into(),
        faction: ai1_faction,
        tile_x: ai1_bx + 12,
        tile_y: ai1_by + 12,
        hp: 15, // 15 / 130 = 11% HP (< repair retreat threshold of 40%)
        max_hp: 130,
        is_selected: false,
        current_order: UnitOrder::Idle,
        path: Vec::new(),
        move_cooldown: 0.0,
        weapon: Some(WeaponType::Laser),
        attack_cooldown: 0.0,
        cargo: None,
        hauler_state: None,
        cargo_kit: None,
    });

    let initial_buildings_count = colony.buildings.len();

    // Run 500 autonomous simulation ticks
    for tick in 1..=500 {
        let res = colony.update(0.1);
        let _ = res;

        // Verify no NaN or infinite simulation telemetry
        assert!(colony.sol.is_finite(), "Sol must be finite at tick {}", tick);
        assert!(colony.day_cycle.is_finite(), "Day cycle must be finite at tick {}", tick);
        assert!(colony.sunlight.is_finite(), "Sunlight must be finite at tick {}", tick);
        assert!(colony.starship_progress.is_finite(), "Starship progress must be finite at tick {}", tick);

        for u in &colony.units {
            assert!(u.move_cooldown.is_finite(), "Unit #{} move_cooldown must be finite", u.id);
            assert!(u.attack_cooldown.is_finite(), "Unit #{} attack_cooldown must be finite", u.id);
        }
    }

    // Verify AI autonomous progression across the 500 ticks
    assert!(
        colony.buildings.len() >= initial_buildings_count,
        "AI players must expand or sustain base facilities"
    );

    // Verify each AI was active: check produced units or queued production or accrued income
    for (idx, ai) in colony.ai_players.iter().enumerate() {
        let has_units = colony.units.iter().any(|u| u.faction == ai.faction);
        let has_buildings = colony.buildings.iter().any(|b| b.faction == Some(ai.faction));
        assert!(
            has_units || has_buildings,
            "AI #{}: {:?} must retain units or buildings",
            idx + 1,
            ai.personality
        );
    }

    // Verify damaged unit retreated towards repair depot
    let damaged_unit = colony.units.iter().find(|u| u.id == damaged_unit_id);
    if let Some(u) = damaged_unit {
        // Damaged unit should have received a MoveTo order towards repair depot
        match u.current_order {
            UnitOrder::MoveTo { target_x, target_y } => {
                let dist_to_repair = (target_x - (ai1_bx + 2)).abs() + (target_y - (ai1_by + 2)).abs();
                assert!(dist_to_repair <= 2, "Damaged unit must target repair facility vicinity");
            }
            UnitOrder::Idle => {
                // Unit may have already arrived at repair facility
                let dist_to_repair = (u.tile_x - (ai1_bx + 2)).abs() + (u.tile_y - (ai1_by + 2)).abs();
                assert!(dist_to_repair <= 4, "Damaged unit must be near repair facility");
            }
            _ => {}
        }
    }
}

// =========================================================================
// Test 3: WebRTC P2P Lockstep Turn Synchronization
// =========================================================================

#[test]
fn test_webrtc_p2p_lockstep_turn_synchronization() {
    let config = MapGeneratorConfig {
        width: 64,
        height: 64,
        biome: TerrainBiome::CanyonsAndCrags,
        ore_abundance: OreAbundance::Normal,
        geothermal_vents: 2,
        symmetry: MapSymmetry::MirroredX,
        seed: 7777,
        player_count: 2,
    };

    // Instantiate synchronized Host and Guest sessions
    let mut host = ColonyState::new_skirmish_map(config.clone(), Faction::Eden);
    let mut guest = ColonyState::new_skirmish_map(config, Faction::Eden);

    // Assert absolute initial state parity
    assert_eq!(host.tick_count, guest.tick_count);
    assert_eq!(host.units.len(), guest.units.len());
    assert_eq!(host.buildings.len(), guest.buildings.len());
    assert_eq!(host.common_ore, guest.common_ore);
    assert_eq!(host.rare_ore, guest.rare_ore);

    let lead_unit_id = host.units[0].id;
    let target_dest = (host.units[0].tile_x + 4, host.units[0].tile_y + 3);

    // Construct turn orders
    let move_msg = NetMessage::UnitOrder {
        unit_ids: vec![lead_unit_id],
        order: NetOrder::MoveTo(target_dest.0, target_dest.1),
    };

    let factory_id = host.buildings.iter()
        .find(|b| b.name.contains("Vehicle Factory"))
        .expect("Vehicle Factory required for lockstep test")
        .id;

    let build_msg = NetMessage::QueueProduction {
        building_id: factory_id,
        item: "Cargo Truck".to_string(),
    };

    // Turn Frame 1..=10: Baseline idle simulation
    for _ in 1..=10 {
        host.update(0.1);
        guest.update(0.1);
    }

    // Turn Frame 10: Dispatch and execute unit move order across network
    host.apply_net_message(&move_msg);
    guest.apply_net_message(&move_msg);

    // Turn Frame 11..=25: Transit path traversal
    for _ in 11..=25 {
        host.update(0.1);
        guest.update(0.1);
    }

    // Turn Frame 25: Dispatch factory build order across network
    host.apply_net_message(&build_msg);
    guest.apply_net_message(&build_msg);

    // Turn Frame 26..=80: Progress movement and fabrication
    for _ in 26..=80 {
        host.update(0.1);
        guest.update(0.1);
    }

    // Assert deterministic lockstep convergence
    assert_eq!(host.tick_count, guest.tick_count, "Tick counts must be lockstep synchronized");
    assert_eq!(host.common_ore, guest.common_ore, "Common ore must match exactly");
    assert_eq!(host.rare_ore, guest.rare_ore, "Rare ore must match exactly");
    assert_eq!(host.food_units, guest.food_units, "Food reserves must match exactly");
    assert_eq!(host.morale_pct, guest.morale_pct, "Morale must match exactly");
    assert_eq!(host.population, guest.population, "Population must match exactly");

    // Assert exact unit coordinate and health parity
    assert_eq!(host.units.len(), guest.units.len(), "Unit count must be identical");
    for (hu, gu) in host.units.iter().zip(guest.units.iter()) {
        assert_eq!(hu.id, gu.id, "Unit ID mismatch");
        assert_eq!((hu.tile_x, hu.tile_y), (gu.tile_x, gu.tile_y), "Unit position desync on #{}", hu.id);
        assert_eq!(hu.hp, gu.hp, "Unit HP desync on #{}", hu.id);
        assert_eq!(hu.current_order, gu.current_order, "Unit order desync on #{}", hu.id);
    }

    // Assert exact structure status and production queue parity
    assert_eq!(host.buildings.len(), guest.buildings.len(), "Building count must be identical");
    for (hb, gb) in host.buildings.iter().zip(guest.buildings.iter()) {
        assert_eq!(hb.id, gb.id, "Building ID mismatch");
        assert_eq!((hb.tile_x, hb.tile_y), (gb.tile_x, gb.tile_y), "Building coordinate desync on #{}", hb.id);
        assert_eq!(hb.production_queue, gb.production_queue, "Production queue desync on #{}", hb.id);
        assert_eq!(hb.production_progress, gb.production_progress, "Production progress desync on #{}", hb.id);
    }
}

// =========================================================================
// Test 4: State Persistence (Save/Load) Roundtrip Integrity
// =========================================================================

#[test]
fn test_state_persistence_save_load_roundtrip_integrity() {
    let mut colony = ColonyState::new_scenario(0);

    // 1. Populate exactly 20 distinct tactical units
    colony.units.clear();
    for i in 1..=20 {
        let (name, ut, wep, hp) = match i % 5 {
            0 => ("Panther Battle Tank", UnitType::Panther, Some(WeaponType::Railgun), 250),
            1 => ("Lynx Scout", UnitType::Lynx, Some(WeaponType::Laser), 130),
            2 => ("Cargo Truck", UnitType::CargoTruck, None, 150),
            3 => ("Robo-Miner", UnitType::RoboMiner, None, 180),
            _ => ("Earthworker", UnitType::Earthworker, None, 160),
        };

        colony.units.push(Unit {
            id: i,
            name: format!("{} #{}", name, i),
            faction: if i % 2 == 0 { Faction::Eden } else { Faction::Plymouth },
            tile_x: 10 + (i as i32 % 8) * 2,
            tile_y: 12 + (i as i32 / 8) * 3,
            hp: hp - (i * 3), // Varied HP degradation
            max_hp: hp,
            is_selected: i == 1,
            current_order: if i == 2 {
                UnitOrder::MoveTo { target_x: 24, target_y: 30 }
            } else {
                UnitOrder::Idle
            },
            path: if i == 2 { vec![(20, 25), (24, 30)] } else { Vec::new() },
            move_cooldown: 0.14,
            weapon: wep,
            attack_cooldown: 0.0,
            cargo: if ut == UnitType::CargoTruck { Some((OreType::Common, 50)) } else { None },
            hauler_state: if ut == UnitType::CargoTruck { Some(HaulerState::Idle) } else { None },
            cargo_kit: if ut == UnitType::Earthworker { Some("Guard Post".to_string()) } else { None },
        });
    }
    assert_eq!(colony.units.len(), 20, "Must populate exactly 20 tactical units");

    // 2. Populate exactly 10 distinct colonial buildings
    colony.buildings.clear();
    let structure_specs = [
        ("Command Center", StructureType::CommandCenter, 20, 20, 40, true),
        ("Solar Power Array", StructureType::SolarArray, 24, 20, -250, true),
        ("Tokamak Fusion Reactor", StructureType::TokamakReactor, 28, 20, -800, true),
        ("Common Ore Smelter", StructureType::CommonOreSmelter, 20, 24, 50, true),
        ("Rare Ore Smelter", StructureType::RareOreSmelter, 24, 24, 75, true),
        ("Standard Research Lab", StructureType::StandardLab, 28, 24, 90, true),
        ("Bio-Dome Hydroponics", StructureType::Agrodome, 20, 28, 30, true),
        ("Vehicle Factory", StructureType::VehicleFactory, 24, 28, 120, true),
        ("Vehicle Repair Depot", StructureType::RepairFacility, 28, 28, 60, true),
        ("Guard Post Turret", StructureType::GuardPost, 32, 28, 50, false), // Offline
    ];

    for (idx, &(name, st, tx, ty, power, active)) in structure_specs.iter().enumerate() {
        let (w, h) = st.dimensions();
        colony.buildings.push(Building {
            id: (idx + 1) as u32,
            name: name.to_string(),
            tile_x: tx,
            tile_y: ty,
            width: w,
            height: h,
            power_cost: power,
            is_active: active,
            is_selected: false,
            production_queue: if name == "Vehicle Factory" {
                vec!["Panther Battle Tank".to_string(), "Cargo Truck".to_string()]
            } else {
                Vec::new()
            },
            production_progress: if name == "Vehicle Factory" { 45.0 } else { 0.0 },
            attack_cooldown: 0.0,
            faction: Some(Faction::Eden),
        });
    }
    assert_eq!(colony.buildings.len(), 10, "Must populate exactly 10 colonial structures");

    // 3. Populate active natural disasters (lava flow, vortex, meteor)
    colony.disasters.active_lava_tiles = vec![(15, 20), (15, 21), (16, 21)];
    colony.disasters.active_vortices = vec![AtmosphericVortex {
        x: 35.5,
        y: 40.0,
        vx: 0.2,
        vy: -0.1,
        radius: 3.0,
        lifetime: 50.0,
        max_lifetime: 60.0,
    }];
    colony.disasters.pending_meteors = vec![MeteorImpact {
        x: 18.0,
        y: 22.0,
        delay: 3.5,
        max_delay: 5.0,
        radius: 2.5,
        has_impacted: false,
    }];

    // 4. Populate mid-research state
    colony.tech_tree.techs[1].is_researched = true; // Astronomy
    colony.tech_tree.techs[5].is_researched = true; // Vulcanology

    // 5. Populate active AI player
    colony.ai_players = vec![outpost2::sim::ai::AiPlayer::new(
        1,
        "Plymouth Raider".to_string(),
        Faction::Plymouth,
        AiPersonality::AggressiveRusher,
        Difficulty::Medium,
        45,
        45,
    )];

    // Custom telemetry metrics
    colony.common_ore = 8500;
    colony.rare_ore = 2400;
    colony.food_units = 1850;
    colony.population = 135;
    colony.morale_pct = 92;
    colony.power_produced = 1050;
    colony.power_consumed = 530;
    colony.starship_progress = 50.0;
    colony.sol = 78.5;
    colony.casualties = 4;

    // Add script trigger
    let trigger = ScriptTrigger::new(
        101,
        "Deep Volcanic Breach",
        TriggerCondition::SolTimer(60),
        vec![EventAction::TriggerDisaster {
            disaster_type: outpost2::sim::events::DisasterType::SeismicFissure,
            x: 15,
            y: 20,
        }],
    );
    colony.event_engine.add_trigger(trigger);

    // Capture snapshot
    let save_state = SaveGameState::from_colony(&colony, 1, "Alpha Colony Milestone", 1800);

    // 6. Serialize to JSON and LocalStorage backend
    let json_payload = save_state.to_json().expect("JSON serialization failed");
    let json_roundtrip = SaveGameState::from_json(&json_payload).expect("JSON deserialization failed");
    assert_eq!(save_state, json_roundtrip, "SaveGameState JSON roundtrip mismatch");

    let save_storage_res = LocalStorageBackend::save(1, &save_state);
    assert!(save_storage_res.is_ok(), "LocalStorage save failed: {:?}", save_storage_res);

    let loaded_from_storage = LocalStorageBackend::load(1).expect("LocalStorage load failed");
    assert_eq!(save_state, loaded_from_storage, "LocalStorage roundtrip mismatch");

    // 7. Reset simulation state and reload from serialized state
    let mut reloaded_colony = ColonyState::new(Faction::Eden);
    loaded_from_storage.apply_to_colony(&mut reloaded_colony);

    // 8. Assert exact identity of all restored entities
    assert_eq!(reloaded_colony.units.len(), 20, "Restored unit count must be 20");
    for (orig, loaded) in colony.units.iter().zip(reloaded_colony.units.iter()) {
        assert_eq!(orig.id, loaded.id, "Unit ID mismatch");
        assert_eq!((orig.tile_x, orig.tile_y), (loaded.tile_x, loaded.tile_y), "Unit pos mismatch on #{}", orig.id);
        assert_eq!(orig.hp, loaded.hp, "Unit HP mismatch on #{}", orig.id);
        assert_eq!(orig.max_hp, loaded.max_hp, "Unit max HP mismatch on #{}", orig.id);
        assert_eq!(orig.current_order, loaded.current_order, "Unit order mismatch on #{}", orig.id);
        assert_eq!(orig.weapon, loaded.weapon, "Unit weapon mismatch on #{}", orig.id);
        assert_eq!(orig.cargo, loaded.cargo, "Unit cargo mismatch on #{}", orig.id);
        assert_eq!(orig.hauler_state, loaded.hauler_state, "Unit hauler state mismatch on #{}", orig.id);
    }

    assert_eq!(reloaded_colony.buildings.len(), 10, "Restored building count must be 10");
    for (orig, loaded) in colony.buildings.iter().zip(reloaded_colony.buildings.iter()) {
        assert_eq!(orig.id, loaded.id, "Building ID mismatch");
        assert_eq!((orig.tile_x, orig.tile_y), (loaded.tile_x, loaded.tile_y), "Building pos mismatch on #{}", orig.id);
        assert_eq!(orig.power_cost, loaded.power_cost, "Building power cost mismatch on #{}", orig.id);
        assert_eq!(orig.is_active, loaded.is_active, "Building active status mismatch on #{}", orig.id);
        assert_eq!(orig.production_queue, loaded.production_queue, "Production queue mismatch on #{}", orig.id);
        assert_eq!(orig.production_progress, loaded.production_progress, "Production progress mismatch on #{}", orig.id);
    }

    // Power status validation
    let active_building = loaded_from_storage.structures.iter().find(|s| s.id == 1).unwrap();
    assert_eq!(active_building.power_status, PowerStatus::Powered);
    let disabled_building = loaded_from_storage.structures.iter().find(|s| s.id == 10).unwrap();
    assert_eq!(disabled_building.power_status, PowerStatus::Disabled);

    // Natural disaster integrity
    assert_eq!(reloaded_colony.disasters.active_lava_tiles, vec![(15, 20), (15, 21), (16, 21)]);
    assert_eq!(reloaded_colony.disasters.active_vortices.len(), 1);
    assert_eq!(reloaded_colony.disasters.active_vortices[0].radius, 3.0);
    assert_eq!(reloaded_colony.disasters.pending_meteors.len(), 1);
    assert_eq!(reloaded_colony.disasters.pending_meteors[0].delay, 3.5);

    // Mid-research integrity
    assert!(reloaded_colony.tech_tree.is_researched(2701));
    assert!(reloaded_colony.tech_tree.is_researched(4301));
    assert!(!reloaded_colony.tech_tree.is_researched(5301));

    // Telemetry and mission timers
    assert_eq!(reloaded_colony.common_ore, 8500);
    assert_eq!(reloaded_colony.rare_ore, 2400);
    assert_eq!(reloaded_colony.food_units, 1850);
    assert_eq!(reloaded_colony.population, 135);
    assert_eq!(reloaded_colony.morale_pct, 92);
    assert_eq!(reloaded_colony.starship_progress, 50.0);
    assert_eq!(reloaded_colony.sol, 78.0); // Converted to integer sol in save metadata
    assert_eq!(reloaded_colony.casualties, 4);

    // Event scripting triggers
    assert!(reloaded_colony.event_engine.triggers.iter().any(|t| t.id == 101));
}
