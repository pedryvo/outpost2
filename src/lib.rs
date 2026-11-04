use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent, Window};

mod archive;
mod audio;
mod i18n;
pub mod net;
mod render;
pub mod sim;

use audio::AudioManager;
use render::Viewport;
use sim::{ColonyState, Faction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EngineeringMode {
    Wall,
    Tube,
    Clear,
    Bulldoze,
}

struct GameApp {
    colony: ColonyState,
    viewport: Viewport,
    audio: AudioManager,
    hud: render::HudRenderer,
    menu: render::MenuManager,
    is_dragging: bool,
    last_mouse_x: f64,
    last_mouse_y: f64,
    last_frame_time: f64,
    pending_placement: Option<sim::structures::StructureType>,
    pending_engineering: Option<EngineeringMode>,
    pending_convec_deploy: bool,
    pending_patrol: bool,
    hover_tile: (i32, i32),
    drag_select_start: Option<(f64, f64)>,
    drag_select_current: Option<(f64, f64)>,
    pan_candidate: bool,
    net_session: Option<net::WebRtcSession>,
    last_autosave_sol: u32,
    playtime_seconds: u32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("global window does not exist");
    let document = window.document().expect("document should exist on window");

    let canvas = document
        .get_element_by_id("op2-canvas")
        .expect("canvas #op2-canvas not found")
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .expect("2d context not available")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let minimap_canvas = document
        .get_element_by_id("minimap-canvas")
        .expect("minimap-canvas not found")
        .dyn_into::<HtmlCanvasElement>()?;

    let minimap_ctx = minimap_canvas
        .get_context("2d")?
        .expect("minimap 2d context not available")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let app = Rc::new(RefCell::new(GameApp {
        colony: ColonyState::new(Faction::Eden),
        viewport: Viewport::new(),
        audio: AudioManager::new(),
        hud: render::HudRenderer::new(64, 64, 180.0, 180.0),
        menu: render::MenuManager::new(),
        is_dragging: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        last_frame_time: 0.0,
        pending_placement: None,
        pending_engineering: None,
        pending_convec_deploy: false,
        pending_patrol: false,
        hover_tile: (20, 20),
        drag_select_start: None,
        drag_select_current: None,
        pan_candidate: false,
        net_session: None,
        last_autosave_sol: 0,
        playtime_seconds: 0,
    }));

    {
        let app_for_art = app.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let art = async {
                let prt = fetch_art_bytes("assets/op2_art.prt").await?;
                let bitmap = fetch_art_bytes("assets/OP2_ART.BMP").await?;
                render::sprites::SpriteAtlas::load_from_prt_and_bmp(&prt, &bitmap)
                    .map_err(|error| JsValue::from_str(&error))
            }.await;
            match art {
                Ok(atlas) => {
                    let game = app_for_art.borrow();
                    game.viewport.sprite_mgr.borrow_mut().atlas = atlas;
                }
                Err(error) => web_sys::console::error_1(&error),
            }
        });
    }

    // Calibrate initial Canvas & Minimap Dimensions immediately on Boot
    {
        let dpr = window.device_pixel_ratio().max(1.0);
        let win_w = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
        let win_h = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);
        let client_w = if canvas.client_width() > 0 { canvas.client_width() as f64 } else { win_w };
        let client_h = if canvas.client_height() > 0 { canvas.client_height() as f64 } else { win_h };
        let dims = render::touch::compute_high_dpi_canvas_dimensions(client_w, client_h, dpr);
        canvas.set_width(dims.physical_width);
        canvas.set_height(dims.physical_height);

        let mini_w = if minimap_canvas.client_width() > 0 { minimap_canvas.client_width() as f64 } else { 180.0 };
        let mini_h = if minimap_canvas.client_height() > 0 { minimap_canvas.client_height() as f64 } else { 180.0 };
        let mini_dims = render::touch::compute_high_dpi_canvas_dimensions(mini_w, mini_h, dpr);
        minimap_canvas.set_width(mini_dims.physical_width);
        minimap_canvas.set_height(mini_dims.physical_height);
        {
            let mut g = app.borrow_mut();
            g.hud.state.radar.radar_width = mini_w;
            g.hud.state.radar.radar_height = mini_h;
            center_on_colony(&mut g, &canvas);
        }
    }

    // Responsive Window Resize & High-DPI Viewport Recalibration Handler
    {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let minimap_canvas_clone = minimap_canvas.clone();
        let window_clone = window.clone();
        let on_resize = Closure::wrap(Box::new(move |_: web_sys::UiEvent| {
            let mut g = app_clone.borrow_mut();
            let dpr = window_clone.device_pixel_ratio().max(1.0);
            let win_w = window_clone.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            let win_h = window_clone.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);

            let client_w = if canvas_clone.client_width() > 0 { canvas_clone.client_width() as f64 } else { win_w };
            let client_h = if canvas_clone.client_height() > 0 { canvas_clone.client_height() as f64 } else { win_h };
            if client_w > 0.0 && client_h > 0.0 {
                let dims = render::touch::compute_high_dpi_canvas_dimensions(client_w, client_h, dpr);
                if canvas_clone.width() != dims.physical_width || canvas_clone.height() != dims.physical_height {
                    canvas_clone.set_width(dims.physical_width);
                    canvas_clone.set_height(dims.physical_height);
                }
            }

            let mini_w = if minimap_canvas_clone.client_width() > 0 { minimap_canvas_clone.client_width() as f64 } else { 180.0 };
            let mini_h = if minimap_canvas_clone.client_height() > 0 { minimap_canvas_clone.client_height() as f64 } else { 180.0 };
            if mini_w > 0.0 && mini_h > 0.0 {
                let mini_dims = render::touch::compute_high_dpi_canvas_dimensions(mini_w, mini_h, dpr);
                if minimap_canvas_clone.width() != mini_dims.physical_width || minimap_canvas_clone.height() != mini_dims.physical_height {
                    minimap_canvas_clone.set_width(mini_dims.physical_width);
                    minimap_canvas_clone.set_height(mini_dims.physical_height);
                }
                g.hud.state.radar.radar_width = mini_w;
                g.hud.state.radar.radar_height = mini_h;
            }
        }) as Box<dyn FnMut(web_sys::UiEvent)>);
        window.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())?;
        on_resize.forget();
    }

    // Mouse Controls (Pan, Box Select & Ghost Construction Placement)
    {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let canvas_clone = canvas.clone();
        let on_mouse_down = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();

            let (click_x, click_y) = canvas_position(&canvas_clone, e.client_x(), e.client_y());

            if !g.menu.is_in_game() {
                if e.button() == 0 {
                    let canvas_w = canvas_clone.client_width() as f64;
                    let canvas_h = canvas_clone.client_height() as f64;
                    let action = g.menu.handle_mouse_down(click_x, click_y, canvas_w, canvas_h, true);
                    match action {
                        render::MenuAction::StartScenario(sc_id) => {
                            g.colony = ColonyState::new_scenario(sc_id);
                            g.menu.switch_screen(render::MenuScreen::None);
                            center_on_colony(&mut g, &canvas_clone);
                            g.audio.play_build_confirm();
                        }
                        render::MenuAction::StartSkirmish(cfg) => {
                            let mut new_colony = ColonyState::new_scenario(cfg.map_index);
                            let opps = cfg.build_opponents();
                            new_colony.setup_skirmish(cfg.human_faction, &opps);
                            let (c_ore, r_ore) = cfg.resources.starting_ore();
                            new_colony.common_ore = c_ore;
                            new_colony.rare_ore = r_ore;
                            g.colony = new_colony;
                            g.menu.switch_screen(render::MenuScreen::None);
                            center_on_colony(&mut g, &canvas_clone);
                            g.audio.play_build_confirm();
                        }
                        render::MenuAction::ResumeGame => {
                            g.menu.switch_screen(render::MenuScreen::None);
                        }
                        render::MenuAction::QuickSave => {
                            let _ = sim::save::execute_quicksave(&g.colony, g.playtime_seconds);
                            g.menu.set_status("QuickSave Stored (Slot 0)", 3.0);
                            g.audio.play_build_confirm();
                        }
                        render::MenuAction::QuickLoad => {
                            if let Ok(meta) = sim::save::execute_quickload(&mut g.colony) {
                                g.menu.set_status(format!("QuickLoad: Sol {:.2}", meta.sol), 3.0);
                                g.menu.switch_screen(render::MenuScreen::None);
                                g.audio.play_build_confirm();
                            } else {
                                g.menu.set_status("QuickLoad Failed: No QuickSave", 3.0);
                                g.audio.play_build_error();
                            }
                        }
                        render::MenuAction::LoadSlot(slot) => {
                            if let Ok(st) = sim::save::LocalStorageBackend::load(slot) {
                                st.apply_to_colony(&mut g.colony);
                                g.menu.set_status(format!("Loaded Slot #{}", slot), 3.0);
                                g.menu.switch_screen(render::MenuScreen::None);
                                g.audio.play_build_confirm();
                            } else {
                                g.menu.set_status(format!("Load failed for Slot #{}", slot), 3.0);
                                g.audio.play_build_error();
                            }
                        }
                        render::MenuAction::SaveSlot(slot) => {
                            let st = sim::save::SaveGameState::from_colony(&g.colony, slot, format!("Colony Save #{}", slot), g.playtime_seconds);
                            if sim::save::LocalStorageBackend::save(slot, &st).is_ok() {
                                g.menu.refresh_save_slots();
                                g.menu.set_status(format!("Saved to Slot #{}", slot), 3.0);
                                g.audio.play_build_confirm();
                            }
                        }
                        render::MenuAction::DeleteSlot(slot) => {
                            let _ = sim::save::LocalStorageBackend::delete_slot(slot);
                            g.menu.refresh_save_slots();
                            g.menu.set_status(format!("Deleted Slot #{}", slot), 3.0);
                        }
                        render::MenuAction::ExportOp2Save => {
                            let st = sim::save::SaveGameState::from_colony(&g.colony, 1, "Outpost 2 Archive", g.playtime_seconds);
                            if let Ok(json) = sim::save::FileExportBackend::export_to_op2save(&st) {
                                let filename = sim::save::FileExportBackend::generate_download_filename(&st.metadata);
                                let _ = sim::save::FileExportBackend::trigger_download(&filename, &json);
                                g.menu.set_status("Downloaded .op2save file", 3.0);
                            }
                        }
                        render::MenuAction::RestartScenario => {
                            g.colony = ColonyState::new_scenario(g.colony.mission.scenario_id);
                            g.menu.switch_screen(render::MenuScreen::None);
                            center_on_colony(&mut g, &canvas_clone);
                            g.audio.play_build_confirm();
                        }
                        render::MenuAction::SurrenderToMainMenu => {
                            g.menu.switch_screen(render::MenuScreen::MainMenu);
                            g.audio.play_defeat_alert();
                        }
                        render::MenuAction::CopyOfferToClipboard => {
                            if let Some(offer) = &g.menu.mp_room.host_offer_sdp {
                                if let Some(win) = web_sys::window() {
                                    let _ = win.navigator().clipboard().write_text(offer);
                                }
                            }
                            g.audio.play_ui_beep();
                        }
                        render::MenuAction::CopyAnswerToClipboard => {
                            if let Some(ans) = &g.menu.mp_room.guest_answer_sdp {
                                if let Some(win) = web_sys::window() {
                                    let _ = win.navigator().clipboard().write_text(ans);
                                }
                            }
                            g.audio.play_ui_beep();
                        }
                        render::MenuAction::LaunchMultiplayerMatch => {
                            let map_name = g.menu.mp_room.map_name.clone();
                            let map_idx = if map_name.contains("Eden 1") {
                                1
                            } else if map_name.contains("Eden 2") {
                                2
                            } else if map_name.contains("Eden 3") {
                                3
                            } else if map_name.contains("Plymouth 1") {
                                4
                            } else if map_name.contains("Plymouth 2") {
                                5
                            } else if map_name.contains("Plymouth 3") {
                                6
                            } else {
                                1
                            };
                            g.colony = ColonyState::new_scenario(map_idx);
                            g.hud.state.telemetry.is_multiplayer = true;
                            g.hud.state.telemetry.is_connected = true;
                            g.hud.state.telemetry.host_faction = g.menu.mp_room.faction;
                            g.hud.state.telemetry.guest_faction = g.menu.mp_room.guest_faction.or(Some(Faction::Plymouth));
                            g.menu.switch_screen(render::MenuScreen::None);
                            center_on_colony(&mut g, &canvas_clone);
                            g.audio.play_build_confirm();
                        }
                        render::MenuAction::ApplyOptions(opts) => {
                            g.menu.options = opts.clone();
                            g.hud.state.language = opts.language.clone();
                            i18n::apply_language(&opts.language, &doc_clone);
                            g.audio.apply_options(opts.master_volume, opts.sfx_volume, opts.music_volume);
                            g.audio.play_ui_beep();
                        }
                        render::MenuAction::PlayBeep => {
                            g.audio.play_ui_click();
                        }
                        _ => {}
                    }
                }
                return;
            }

            if e.button() == 0 {
                // If in structure placement mode: deploy structure
                if let Some(st) = g.pending_placement {
                    let (gx, gy) = g.hover_tile;
                    match g.colony.order_structure_construction(st, gx, gy) {
                        Ok(msg) => {
                            g.audio.play_build_confirm();
                            g.pending_placement = None;
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                toast.set_text_content(Some(&msg));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                            }
                        }
                        Err(err) => {
                            g.audio.play_build_error();
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                toast.set_text_content(Some(&err));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                            }
                        }
                    }
                    return;
                }

                // If in engineering order mode (Earthworker wall/tube/hazard):
                if let Some(eng_mode) = g.pending_engineering {
                    let (tx, ty) = g.viewport.screen_to_tile(click_x, click_y);
                    let mut ordered = false;
                    for u in &mut g.colony.units {
                        if u.is_selected && u.name.contains("Earthworker") {
                            u.current_order = match eng_mode {
                                EngineeringMode::Wall => sim::pathfinding::UnitOrder::BuildWall { target_x: tx, target_y: ty },
                                EngineeringMode::Tube => sim::pathfinding::UnitOrder::BuildTube { target_x: tx, target_y: ty },
                                EngineeringMode::Clear => sim::pathfinding::UnitOrder::ClearHazard { target_x: tx, target_y: ty },
                                EngineeringMode::Bulldoze => sim::pathfinding::UnitOrder::Bulldoze { target_x: tx, target_y: ty },
                            };
                            ordered = true;
                        }
                    }
                    g.pending_engineering = None;
                    if ordered {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some("Engineering order dispatched to Earthworker."));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    } else {
                        g.audio.play_build_error();
                    }
                    return;
                }

                // If in ConVec deployment mode:
                if g.pending_convec_deploy {
                    let (tx, ty) = g.viewport.screen_to_tile(click_x, click_y);
                    let mut deployed = false;
                    for u in &mut g.colony.units {
                        if u.is_selected && u.name.contains("ConVec") && u.cargo_kit.is_some() {
                            let kit = u.cargo_kit.as_ref().unwrap().clone();
                            u.current_order = sim::pathfinding::UnitOrder::DeployStructure {
                                kit_name: kit,
                                tile_x: tx,
                                tile_y: ty,
                            };
                            deployed = true;
                        }
                    }
                    g.pending_convec_deploy = false;
                    if deployed {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some("Deployment site designated for ConVec."));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    } else {
                        g.audio.play_build_error();
                    }
                    return;
                }

                // If in tactical patrol mode:
                if g.pending_patrol {
                    let (tx, ty) = g.viewport.screen_to_tile(click_x, click_y);
                    let mut patrolled = false;
                    let player_faction = g.colony.faction;
                    for u in &mut g.colony.units {
                        if u.is_selected && u.faction == player_faction {
                            u.current_order = sim::pathfinding::UnitOrder::Patrol {
                                start_x: u.tile_x,
                                start_y: u.tile_y,
                                end_x: tx,
                                end_y: ty,
                                heading_to_end: true,
                            };
                            patrolled = true;
                        }
                    }
                    g.pending_patrol = false;
                    if patrolled {
                        g.audio.play_unit_ack();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some("Patrol route assigned."));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    } else {
                        g.audio.play_build_error();
                    }
                    return;
                }

                g.drag_select_start = Some((click_x, click_y));
                g.drag_select_current = Some((click_x, click_y));
                g.last_mouse_x = click_x;
                g.last_mouse_y = click_y;
                g.is_dragging = false;
                g.pan_candidate = !e.shift_key();
            } else if e.button() == 2 {
                // Right Click = Cancel Placement/Engineering or issue Tactical Command
                if g.pending_placement.is_some() || g.pending_engineering.is_some() || g.pending_convec_deploy || g.pending_patrol {
                    g.pending_placement = None;
                    g.pending_engineering = None;
                    g.pending_convec_deploy = false;
                    g.pending_patrol = false;
                    g.audio.play_ui_beep();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        let _ = toast.set_attribute("style", "display: none;");
                    }
                    return;
                }

                let zoom = g.viewport.zoom;
                let mut clicked_enemy = None;
                let mut clicked_friendly_damaged = None;
                for u in &g.colony.units {
                    if g.colony.fog.get(u.tile_x as usize, u.tile_y as usize) != sim::fog::FogState::Visible {
                        continue;
                    }
                    let (tile_sx, tile_sy) = g.viewport.tile_to_screen(u.tile_x as f64, u.tile_y as f64);
                    let (sx, sy) = (tile_sx + 16.0 * zoom, tile_sy + 16.0 * zoom);
                    let dist = ((click_x - sx).powi(2) + (click_y - sy).powi(2)).sqrt();
                    if dist < 24.0 * zoom {
                        if u.faction != g.colony.faction {
                            clicked_enemy = Some(u.id);
                            break;
                        } else if u.hp < u.max_hp {
                            clicked_friendly_damaged = Some(u.id);
                            break;
                        }
                    }
                }

                let player_faction = g.colony.faction;
                let (target_tx, target_ty) = g.viewport.screen_to_tile(click_x, click_y);
                let clicked_enemy_building = g.colony.buildings.iter()
                    .find(|b| b.faction != Some(player_faction)
                        && target_tx >= b.tile_x && target_tx < b.tile_x + b.width
                        && target_ty >= b.tile_y && target_ty < b.tile_y + b.height
                        && g.colony.fog.get(b.tile_x as usize, b.tile_y as usize) == sim::fog::FogState::Visible)
                    .map(|b| b.id);
                if let Some(ally_id) = clicked_friendly_damaged {
                    let mut repaired_any = false;
                    for u in &mut g.colony.units {
                        if u.is_selected && u.faction == player_faction && u.name.contains("Repair") {
                            u.current_order = sim::pathfinding::UnitOrder::Repair { target_unit_id: ally_id };
                            repaired_any = true;
                        }
                    }
                    if repaired_any {
                        g.audio.play_build_confirm();
                    } else {
                        let (target_tx, target_ty) = g.viewport.screen_to_tile(click_x, click_y);
                        g.colony.order_selected_move(target_tx, target_ty);
                        g.audio.play_unit_ack();
                    }
                } else if let Some(enemy_id) = clicked_enemy {
                    for u in &mut g.colony.units {
                        if u.is_selected && u.faction == player_faction {
                            u.path.clear();
                            u.current_order = sim::pathfinding::UnitOrder::Attack { target_unit_id: enemy_id };
                        }
                    }
                    g.audio.play_build_confirm();
                } else if let Some(building_id) = clicked_enemy_building {
                    for u in &mut g.colony.units {
                        if u.is_selected && u.faction == player_faction && u.weapon.is_some() {
                            u.path.clear();
                            u.current_order = sim::pathfinding::UnitOrder::AttackBuilding { target_building_id: building_id };
                        }
                    }
                    g.audio.play_unit_ack();
                } else {
                    let (target_tx, target_ty) = g.viewport.screen_to_tile(click_x, click_y);
                    g.colony.order_selected_move(target_tx, target_ty);
                    g.audio.play_unit_ack();
                }
            } else if e.button() == 1 {
                g.is_dragging = true;
                g.last_mouse_x = click_x;
                g.last_mouse_y = click_y;
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref())?;
        on_mouse_down.forget();
    }

    {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let on_mouse_move = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            let (cur_x, cur_y) = canvas_position(&canvas_clone, e.client_x(), e.client_y());

            if !g.menu.is_in_game() {
                if g.menu.handle_mouse_move(cur_x, cur_y) {
                    g.audio.play_ui_hover();
                }
                return;
            }

            let (tx, ty) = g.viewport.screen_to_tile(cur_x, cur_y);
            g.hover_tile = (tx, ty);

            if g.drag_select_start.is_some() {
                if g.pan_candidate {
                    let (start_x, start_y) = g.drag_select_start.unwrap();
                    if (cur_x - start_x).hypot(cur_y - start_y) > 8.0 {
                        g.drag_select_start = None;
                        g.drag_select_current = None;
                        g.pan_candidate = false;
                        g.is_dragging = true;
                        let dx = cur_x - g.last_mouse_x;
                        let dy = cur_y - g.last_mouse_y;
                        g.viewport.pan_by(
                            dx, dy,
                            canvas_clone.client_width() as f64, canvas_clone.client_height() as f64,
                        );
                        g.last_mouse_x = cur_x;
                        g.last_mouse_y = cur_y;
                    }
                } else {
                    g.drag_select_current = Some((cur_x, cur_y));
                }
            } else if g.is_dragging {
                let dx = cur_x - g.last_mouse_x;
                let dy = cur_y - g.last_mouse_y;
                g.viewport.pan_by(dx, dy, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64);
                g.last_mouse_x = cur_x;
                g.last_mouse_y = cur_y;
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        window.add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())?;
        on_mouse_move.forget();
    }

    {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let on_mouse_up = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            if !g.menu.is_in_game() {
                g.drag_select_start = None;
                g.drag_select_current = None;
                g.is_dragging = false;
                g.pan_candidate = false;
                return;
            }

            let (click_x, click_y) = canvas_position(&canvas_clone, e.client_x(), e.client_y());

            if let (Some((x1, y1)), Some((x2, y2))) = (g.drag_select_start, g.drag_select_current) {
                let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                if dist > 12.0 {
                    // Multi-Unit Box Selection
                    let min_x = x1.min(x2);
                    let max_x = x1.max(x2);
                    let min_y = y1.min(y2);
                    let max_y = y1.max(y2);

                    let player_faction = g.colony.faction;
                    let mut selected_any = false;
                    let mut first_selected_name = None;
                    let unit_count = g.colony.units.len();
                    for i in 0..unit_count {
                        if g.colony.units[i].faction == player_faction {
                            let (tile_sx, tile_sy) = g.viewport.tile_to_screen(
                                g.colony.units[i].tile_x as f64,
                                g.colony.units[i].tile_y as f64,
                            );
                            let (sx, sy) = (tile_sx + 16.0 * g.viewport.zoom, tile_sy + 16.0 * g.viewport.zoom);
                            if sx >= min_x && sx <= max_x && sy >= min_y && sy <= max_y
                                && g.colony.fog.get(g.colony.units[i].tile_x as usize, g.colony.units[i].tile_y as usize) == sim::fog::FogState::Visible {
                                g.colony.units[i].is_selected = true;
                                if first_selected_name.is_none() {
                                    first_selected_name = Some(g.colony.units[i].name.clone());
                                }
                                selected_any = true;
                            } else if !e.shift_key() {
                                g.colony.units[i].is_selected = false;
                            }
                        } else if !e.shift_key() {
                            g.colony.units[i].is_selected = false;
                        }
                    }

                    if selected_any || !e.shift_key() {
                        for b in &mut g.colony.buildings {
                            b.is_selected = false;
                        }
                    }
                    if selected_any {
                        if let Some(name) = first_selected_name {
                            g.audio.play_vehicle_selection(&name);
                        } else {
                            g.audio.play_unit_ack();
                        }
                    }
                } else {
                    // Single Unit or Building Click Selection
                    let zoom = g.viewport.zoom;
                    let unit_count = g.colony.units.len();
                    let clicked_unit_index = (0..unit_count)
                        .filter_map(|i| {
                            let u = &g.colony.units[i];
                            if g.colony.fog.get(u.tile_x as usize, u.tile_y as usize) != sim::fog::FogState::Visible {
                                return None;
                            }
                            let (tile_sx, tile_sy) = g.viewport.tile_to_screen(u.tile_x as f64, u.tile_y as f64);
                            let d = ((click_x - tile_sx - 16.0 * zoom).powi(2) + (click_y - tile_sy - 16.0 * zoom).powi(2)).sqrt();
                            (d < 20.0 * zoom).then_some((i, d))
                        })
                        .min_by(|a, b| a.1.total_cmp(&b.1))
                        .map(|(i, _)| i);
                    for i in 0..unit_count {
                        if Some(i) == clicked_unit_index {
                            g.colony.units[i].is_selected = true;
                        } else if !e.shift_key() {
                            g.colony.units[i].is_selected = false;
                        }
                    }

                    let clicked_unit_name = clicked_unit_index.map(|i| g.colony.units[i].name.clone());
                    if let Some(name) = clicked_unit_name {
                        for b in &mut g.colony.buildings {
                            b.is_selected = false;
                        }
                        g.audio.play_vehicle_selection(&name);
                    } else {
                        let (tx, ty) = g.viewport.screen_to_tile(click_x, click_y);
                        let mut clicked_building = false;
                        let clicked_building_id = g.colony.buildings.iter()
                            .find(|b| tx >= b.tile_x && tx < b.tile_x + b.width && ty >= b.tile_y && ty < b.tile_y + b.height
                                && g.colony.fog.get(b.tile_x as usize, b.tile_y as usize) == sim::fog::FogState::Visible)
                            .map(|b| b.id);
                        for b in &mut g.colony.buildings {
                            if Some(b.id) == clicked_building_id {
                                b.is_selected = true;
                                clicked_building = true;
                            } else if !e.shift_key() {
                                b.is_selected = false;
                            }
                        }

                        if clicked_building {
                            g.audio.play_build_confirm();
                        }
                    }
                }
            }

            g.drag_select_start = None;
            g.drag_select_current = None;
            g.is_dragging = false;
            g.pan_candidate = false;
        }) as Box<dyn FnMut(MouseEvent)>);
        window.add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())?;
        on_mouse_up.forget();
    }

    // Zoom toward the mouse pointer without changing the tile beneath it.
    {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let on_wheel = Closure::wrap(Box::new(move |e: WheelEvent| {
            let mut g = app_clone.borrow_mut();
            if !g.menu.is_in_game() { return; }
            e.prevent_default();
            let (x, y) = canvas_position(&canvas_clone, e.client_x(), e.client_y());
            let delta = match e.delta_mode() {
                1 => e.delta_y() * 16.0,
                2 => e.delta_y() * canvas_clone.client_height() as f64,
                _ => e.delta_y(),
            };
            let factor = 1.15_f64.powf((-delta / 100.0).clamp(-2.0, 2.0));
            g.viewport.zoom_at(x, y, factor, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64);
        }) as Box<dyn FnMut(WheelEvent)>);
        canvas.add_event_listener_with_callback("wheel", on_wheel.as_ref().unchecked_ref())?;
        on_wheel.forget();
    }

    // Disable context menu on canvas so right-click is exclusive to game commands
    {
        let on_contextmenu = Closure::wrap(Box::new(move |e: MouseEvent| {
            e.prevent_default();
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.add_event_listener_with_callback("contextmenu", on_contextmenu.as_ref().unchecked_ref())?;
        on_contextmenu.forget();
    }

    // Keyboard Navigation (WASD / Arrows) & Tactical HUD Hotkey Dispatcher
    {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let canvas_clone = canvas.clone();
        let on_key_down = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let mut g = app_clone.borrow_mut();
            let step = 32.0;
            let key = e.key();
            let ctrl = e.ctrl_key() || e.meta_key();
            let shift = e.shift_key();

            if !g.menu.is_in_game() && g.menu.current_screen != render::MenuScreen::PauseMenu {
                let action = g.menu.handle_key_down(&key, false);
                match action {
                    render::MenuAction::StartScenario(sc_id) => {
                        g.colony = ColonyState::new_scenario(sc_id);
                        g.menu.current_screen = render::MenuScreen::None;
                        center_on_colony(&mut g, &canvas_clone);
                        g.audio.play_build_confirm();
                    }
                    _ => {}
                }
                return;
            }

            // Tactical Chat in-game prompt input interception
            if g.hud.state.chat.is_open {
                match key.as_str() {
                    "Enter" => {
                        let cur_sol = g.colony.sol;
                        let my_faction = g.colony.faction;
                        let my_sender = format!("{:?} Commander", my_faction);
                        if let Some(msg) = g.hud.state.chat.commit_message(cur_sol, &my_sender, my_faction) {
                            if let Some(session) = &g.net_session {
                                let _ = session.send_chat(&msg.sender, &msg.text);
                            }
                            g.audio.play_ui_beep();
                            if let Some(log_el) = doc_clone.get_element_by_id("net-chat-log") {
                                if let Ok(chat_div) = doc_clone.create_element("div") {
                                    chat_div.set_text_content(Some(&msg.full_line()));
                                    let _ = chat_div.set_attribute("style", "color: #38bdf8; margin-bottom: 2px;");
                                    let _ = log_el.append_child(&chat_div);
                                }
                            }
                        }
                    }
                    "Escape" => {
                        g.hud.state.chat.close();
                    }
                    "Backspace" => {
                        g.hud.state.chat.backspace();
                    }
                    s if s.chars().count() == 1 && !ctrl && !e.alt_key() => {
                        if let Some(c) = s.chars().next() {
                            g.hud.state.chat.type_char(c);
                        }
                    }
                    _ => {}
                }
                return;
            }

            // Focus in-game tactical chat on Enter keypress
            if key == "Enter" {
                g.hud.state.chat.open();
                return;
            }

            match key.as_str() {
                "ArrowLeft" | "a" | "A" => g.viewport.pan_by(step, 0.0, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "ArrowRight" | "d" | "D" => g.viewport.pan_by(-step, 0.0, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "ArrowUp" | "w" | "W" => g.viewport.pan_by(0.0, step, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "ArrowDown" | "s" | "S" => g.viewport.pan_by(0.0, -step, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "+" | "=" => g.viewport.zoom_at(canvas_clone.client_width() as f64 * 0.5, canvas_clone.client_height() as f64 * 0.5, 1.15, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "-" | "_" => g.viewport.zoom_at(canvas_clone.client_width() as f64 * 0.5, canvas_clone.client_height() as f64 * 0.5, 1.0 / 1.15, canvas_clone.client_width() as f64, canvas_clone.client_height() as f64),
                "Escape" => {
                    if g.pending_placement.is_some()
                        || g.pending_engineering.is_some()
                        || g.pending_convec_deploy
                        || g.pending_patrol
                        || g.hud.state.factory.is_open
                        || g.hud.state.research.is_open
                    {
                        g.pending_placement = None;
                        g.pending_engineering = None;
                        g.pending_convec_deploy = false;
                        g.pending_patrol = false;
                        g.hud.state.factory.is_open = false;
                        g.hud.state.research.is_open = false;
                    } else if g.menu.is_in_game() {
                        g.menu.switch_screen(render::MenuScreen::PauseMenu);
                    } else if g.menu.current_screen == render::MenuScreen::PauseMenu {
                        g.menu.switch_screen(render::MenuScreen::None);
                    }
                }
                "F5" => {
                    let _ = sim::save::execute_quicksave(&g.colony, g.playtime_seconds);
                    g.audio.play_build_confirm();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some("QuickSave Stored (Slot 0)"));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                    }
                }
                "F9" => {
                    if let Ok(meta) = sim::save::execute_quickload(&mut g.colony) {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&format!("QuickLoad: Sol {:.2}", meta.sol)));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    } else {
                        g.audio.play_build_error();
                    }
                }
                _ => {}
            }

            // Dispatch responsive HUD shortcuts: Space, P, 1-9, F, R, B, M
            match g.hud.state.hotkeys.handle_key(&key, ctrl, shift) {
                render::hud::HotkeyAction::JumpToLastAlert => {
                    if let Some((ax, ay)) = g.hud.state.notifications.jump_to_last_alert() {
                        let screen_w = canvas_clone.client_width() as f64;
                        let screen_h = canvas_clone.client_height() as f64;
                        g.viewport.camera_x = screen_w * 0.5 - ax as f64 * g.viewport.tile_width * g.viewport.zoom;
                        g.viewport.camera_y = screen_h * 0.5 - ay as f64 * g.viewport.tile_height * g.viewport.zoom;
                        g.audio.play_ui_beep();
                    }
                }
                render::hud::HotkeyAction::TogglePause => {
                    g.audio.play_ui_beep();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        let msg = if g.hud.state.hotkeys.is_paused {
                            "SIMULATION PAUSED (Press 'P' to resume)"
                        } else {
                            "SIMULATION RESUMED"
                        };
                        toast.set_text_content(Some(msg));
                        let _ = toast.set_attribute(
                            "style",
                            "font-size: 11px; font-weight: 500; color: #facc15; display: block;",
                        );
                    }
                }
                render::hud::HotkeyAction::AssignControlGroup(group) => {
                    let selected_ids: Vec<u32> = g
                        .colony
                        .units
                        .iter()
                        .filter(|u| u.is_selected && u.faction == g.colony.faction)
                        .map(|u| u.id)
                        .collect();
                    if !selected_ids.is_empty() {
                        g.hud.state.hotkeys.assign_group(group, selected_ids.clone());
                        g.audio.play_ui_beep();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            let msg = format!("Squad Group {} assigned ({} units)", group, selected_ids.len());
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute(
                                "style",
                                "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;",
                            );
                        }
                    }
                }
                render::hud::HotkeyAction::RecallControlGroup(group) => {
                    let ids = g.hud.state.hotkeys.get_group(group).to_vec();
                    if !ids.is_empty() {
                        let faction = g.colony.faction;
                        for u in &mut g.colony.units {
                            u.is_selected = u.faction == faction && ids.contains(&u.id);
                        }
                        g.audio.play_ui_beep();
                    }
                }
                render::hud::HotkeyAction::ToggleFactoryView => {
                    g.hud.state.factory.is_open = !g.hud.state.factory.is_open;
                    g.audio.play_ui_beep();
                }
                render::hud::HotkeyAction::ToggleResearchView => {
                    g.hud.state.research.is_open = !g.hud.state.research.is_open;
                    g.audio.play_ui_beep();
                }
                render::hud::HotkeyAction::JumpToBaseOverview => {
                    let cc_pos = g
                        .colony
                        .buildings
                        .iter()
                        .find(|b| b.name.contains("Command Center"))
                        .map(|b| (b.tile_x, b.tile_y));
                    if let Some((cc_x, cc_y)) = cc_pos {
                        let screen_w = canvas_clone.client_width() as f64;
                        let screen_h = canvas_clone.client_height() as f64;
                        g.viewport.camera_x = screen_w * 0.5 - cc_x as f64 * g.viewport.tile_width * g.viewport.zoom;
                        g.viewport.camera_y = screen_h * 0.5 - cc_y as f64 * g.viewport.tile_height * g.viewport.zoom;
                        g.audio.play_ui_beep();
                    }
                }
                render::hud::HotkeyAction::ToggleMinimapDetail => {
                    let d = g.hud.state.radar.toggle_detail();
                    g.audio.play_ui_beep();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        let msg = if d {
                            "TACTICAL RADAR DETAIL: ON"
                        } else {
                            "TACTICAL RADAR DETAIL: OFF"
                        };
                        toast.set_text_content(Some(msg));
                        let _ = toast.set_attribute(
                            "style",
                            "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;",
                        );
                    }
                }
                render::hud::HotkeyAction::QuickSave => {
                    let play_secs = g.playtime_seconds;
                    match sim::execute_quicksave(&g.colony, play_secs) {
                        Ok(_) => {
                            g.audio.play_build_confirm();
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                toast.set_text_content(Some("QUICKSAVE (F5): Colony progress saved to storage!"));
                                let _ = toast.set_attribute(
                                    "style",
                                    "font-size: 11px; font-weight: 500; color: #4ade80; display: block;",
                                );
                            }
                        }
                        Err(e) => {
                            g.audio.play_ui_beep();
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                let msg = format!("Quicksave failed: {}", e);
                                toast.set_text_content(Some(&msg));
                                let _ = toast.set_attribute(
                                    "style",
                                    "font-size: 11px; font-weight: 500; color: #ef4444; display: block;",
                                );
                            }
                        }
                    }
                }
                render::hud::HotkeyAction::QuickLoad => {
                    match sim::execute_quickload(&mut g.colony) {
                        Ok(meta) => {
                            g.audio.play_unit_ack();
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                let msg = format!("QUICKLOAD (F9): Restored '{}' (Sol {})", meta.name, meta.sol);
                                toast.set_text_content(Some(&msg));
                                let _ = toast.set_attribute(
                                    "style",
                                    "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;",
                                );
                            }
                        }
                        Err(e) => {
                            g.audio.play_ui_beep();
                            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                                let msg = format!("Quickload failed: {}", e);
                                toast.set_text_content(Some(&msg));
                                let _ = toast.set_attribute(
                                    "style",
                                    "font-size: 11px; font-weight: 500; color: #ef4444; display: block;",
                                );
                            }
                        }
                    }
                }
                render::hud::HotkeyAction::ToggleChat => {
                    g.hud.state.chat.toggle();
                }
                render::hud::HotkeyAction::CloseChat => {
                    g.hud.state.chat.close();
                }
                render::hud::HotkeyAction::None => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref())?;
        on_key_down.forget();
    }

    // Tactical Radar Interactive Click-to-Pan
    {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let mini_clone = minimap_canvas.clone();
        let on_mini_click = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            let click_x = e.offset_x() as f64;
            let click_y = e.offset_y() as f64;

            let rect_w = (mini_clone.client_width() as f64).max(1.0);
            let rect_h = (mini_clone.client_height() as f64).max(1.0);
            let screen_w = canvas_clone.client_width() as f64;
            let screen_h = canvas_clone.client_height() as f64;

            let (cam_x, cam_y) = render::hud::RadarProjection::calculate_camera_pan(
                click_x,
                click_y,
                rect_w,
                rect_h,
                screen_w,
                screen_h,
                g.viewport.tile_width,
                g.viewport.tile_height,
                g.viewport.zoom,
                g.viewport.map_cols,
                g.viewport.map_rows,
            );

            g.viewport.camera_x = cam_x;
            g.viewport.camera_y = cam_y;
            g.audio.play_ui_beep();
        }) as Box<dyn FnMut(MouseEvent)>);
        minimap_canvas.add_event_listener_with_callback("mousedown", on_mini_click.as_ref().unchecked_ref())?;
        on_mini_click.forget();

        let app_drag = app.clone();
        let canvas_drag = canvas.clone();
        let mini_drag = minimap_canvas.clone();
        let on_mini_drag = Closure::wrap(Box::new(move |e: MouseEvent| {
            if e.buttons() == 1 {
                let mut g = app_drag.borrow_mut();
                let click_x = e.offset_x() as f64;
                let click_y = e.offset_y() as f64;

                let rect_w = (mini_drag.client_width() as f64).max(1.0);
                let rect_h = (mini_drag.client_height() as f64).max(1.0);
                let screen_w = canvas_drag.client_width() as f64;
                let screen_h = canvas_drag.client_height() as f64;

                let (cam_x, cam_y) = render::hud::RadarProjection::calculate_camera_pan(
                    click_x,
                    click_y,
                    rect_w,
                    rect_h,
                    screen_w,
                    screen_h,
                    g.viewport.tile_width,
                    g.viewport.tile_height,
                    g.viewport.zoom,
                    g.viewport.map_cols,
                    g.viewport.map_rows,
                );

                g.viewport.camera_x = cam_x;
                g.viewport.camera_y = cam_y;
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        minimap_canvas.add_event_listener_with_callback("mousemove", on_mini_drag.as_ref().unchecked_ref())?;
        on_mini_drag.forget();
    }

    // Zoom Buttons
    if let Some(btn_in) = document.get_element_by_id("btn-zoom-in") {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            let w = canvas_clone.client_width() as f64;
            let h = canvas_clone.client_height() as f64;
            g.viewport.zoom_at(w * 0.5, h * 0.5, 1.25, w, h);
        }) as Box<dyn FnMut(_)>);
        btn_in.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_out) = document.get_element_by_id("btn-zoom-out") {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            let w = canvas_clone.client_width() as f64;
            let h = canvas_clone.client_height() as f64;
            g.viewport.zoom_at(w * 0.5, h * 0.5, 1.0 / 1.25, w, h);
        }) as Box<dyn FnMut(_)>);
        btn_out.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Tactical Command Buttons (Build Base Drawer & Construction Menu)
    if let Some(btn_build) = document.get_element_by_id("btn-build") {
        let doc_clone = document.clone();
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            if let Some(drawer) = doc_clone.get_element_by_id("build-drawer") {
                let is_open = drawer.get_attribute("style").map_or(false, |s| s.contains("display: flex"));
                if is_open {
                    let _ = drawer.set_attribute("style", "display: none;");
                } else {
                    let _ = drawer.set_attribute("style", "position: absolute; bottom: 76px; left: 16px; width: 330px; max-height: calc(100vh - 160px); background: rgba(12, 14, 18, 0.95); border: 2px solid var(--op2-accent-eden); border-radius: 8px; backdrop-filter: blur(12px); box-shadow: 0 12px 32px rgba(0,0,0,0.8); z-index: 20; display: flex; flex-direction: column; overflow: hidden;");
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_build.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_close_build) = document.get_element_by_id("btn-close-build") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(drawer) = doc_clone.get_element_by_id("build-drawer") {
                let _ = drawer.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn_close_build.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Ok(structure_buttons) = document.query_selector_all(".btn-structure-choice") {
        for i in 0..structure_buttons.length() {
            if let Some(node) = structure_buttons.item(i) {
                if let Ok(btn) = node.dyn_into::<web_sys::HtmlElement>() {
                    let st_type = btn.get_attribute("data-type").unwrap_or_default();
                    let structure_type = match st_type.as_str() {
                        "SolarArray" => sim::structures::StructureType::SolarArray,
                        "TokamakReactor" => sim::structures::StructureType::TokamakReactor,
                        "CommonOreSmelter" => sim::structures::StructureType::CommonOreSmelter,
                        "RareOreSmelter" => sim::structures::StructureType::RareOreSmelter,
                        "DirtLab" => sim::structures::StructureType::DirtLab,
                        "StandardLab" => sim::structures::StructureType::StandardLab,
                        "Agrodome" => sim::structures::StructureType::Agrodome,
                        "Residence" => sim::structures::StructureType::Residence,
                        "VehicleFactory" => sim::structures::StructureType::VehicleFactory,
                        "RepairFacility" => sim::structures::StructureType::RepairFacility,
                        "GuardPost" => sim::structures::StructureType::GuardPost,
                        "GeothermalPlant" => sim::structures::StructureType::GeothermalPlant,
                        "Spaceport" => sim::structures::StructureType::Spaceport,
                        _ => sim::structures::StructureType::SolarArray,
                    };
                    let app_c = app.clone();
                    let doc_c = document.clone();
                    let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        let mut g = app_c.borrow_mut();
                        let _ = g.audio.init();

                        // Tech prerequisite verification
                        if structure_type == sim::structures::StructureType::GuardPost && !g.colony.tech_tree.can_build_guard_posts() {
                            g.audio.play_build_error();
                            audio::radio::RadioDispatcher::announce("Research required: Automated Point Defense Turrets.");
                            if let Some(toast) = doc_c.get_element_by_id("research-toast") {
                                toast.set_text_content(Some("Requires Research: Automated Point Defense Turrets (600 C)"));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                            }
                            return;
                        }
                        if structure_type == sim::structures::StructureType::GeothermalPlant && !g.colony.tech_tree.can_build_geothermal() {
                            g.audio.play_build_error();
                            audio::radio::RadioDispatcher::announce("Research required: Vulcanology and Hydrothermal Extraction.");
                            if let Some(toast) = doc_c.get_element_by_id("research-toast") {
                                toast.set_text_content(Some("Requires Research: Vulcanology & Hydrothermal Extraction (750 C)"));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                            }
                            return;
                        }

                        g.audio.play_build_confirm();
                        g.pending_placement = Some(structure_type);
                        if let Some(drawer) = doc_c.get_element_by_id("build-drawer") {
                            let _ = drawer.set_attribute("style", "display: none;");
                        }
                        if let Some(toast) = doc_c.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&format!("Deploying {}: Click ground to place. Right-click or ESC to cancel.", structure_type.name())));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
                        }
                    }) as Box<dyn FnMut(_)>);
                    let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                    cb.forget();
                }
            }
        }
    }

    if let Some(btn_units) = document.get_element_by_id("btn-units") {
        let app_clone = app.clone();
        let canvas_clone = canvas.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let faction = g.colony.faction;
            let count = g.colony.units.len();
            if count == 0 { return; }
            let current = g.colony.units.iter()
                .position(|unit| unit.is_selected && unit.faction == faction)
                .map(|index| index + 1).unwrap_or(0);
            if let Some(next) = (0..count).map(|offset| (current + offset) % count)
                .find(|&index| g.colony.units[index].faction == faction) {
                for unit in &mut g.colony.units { unit.is_selected = false; }
                for building in &mut g.colony.buildings { building.is_selected = false; }
                g.colony.units[next].is_selected = true;
                let (x, y) = (g.colony.units[next].tile_x as f64 + 0.5,
                    g.colony.units[next].tile_y as f64 + 0.5);
                g.viewport.camera_x = canvas_clone.client_width() as f64 * 0.5
                    - x * g.viewport.tile_width * g.viewport.zoom;
                g.viewport.camera_y = canvas_clone.client_height() as f64 * 0.5
                    - y * g.viewport.tile_height * g.viewport.zoom;
                g.audio.play_unit_ack();
            }
        }) as Box<dyn FnMut(_)>);
        btn_units.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    let render_tech_list = {
        let doc_c = document.clone();
        let app_c = app.clone();
        Rc::new(move || {
            if let Some(list_el) = doc_c.get_element_by_id("tech-list") {
                list_el.set_inner_html("");
                let g = app_c.borrow();
                let ore = g.colony.common_ore;

                for tech in &g.colony.tech_tree.techs {
                    if let Ok(card) = doc_c.create_element("div") {
                        let bg = if tech.is_researched { "rgba(16, 185, 129, 0.12)" } else { "#1a202c" };
                        let border = if tech.is_researched { "#10b981" } else { "#3b475e" };
                        let _ = card.set_attribute("style", &format!("background: {}; border: 1px solid {}; border-radius: 6px; padding: 10px; display: flex; justify-content: space-between; align-items: center; gap: 12px;", bg, border));

                        let cat_name = sim::tech::TechTree::category_name(tech.category);
                        let html = format!(
                            "<div style='display: flex; flex-direction: column; gap: 3px;'><div style='display: flex; align-items: center; gap: 8px;'><span style='font-size: 12px; font-weight: bold; color: #f8fafc;'>{}</span><span style='font-size: 9px; padding: 1px 5px; border-radius: 3px; background: #334155; color: #94a3b8;'>{}</span><span style='font-size: 11px; font-weight: bold; color: #facc15;'>{} C</span></div><span style='font-size: 11px; color: #94a3b8;'>{}</span></div>",
                            tech.name, cat_name, tech.cost, tech.description
                        );
                        card.set_inner_html(&html);

                        if let Ok(btn) = doc_c.create_element("button") {
                            let btn_c = btn.clone();
                            if tech.is_researched {
                                btn.set_text_content(Some("✅ Active"));
                                let _ = btn.set_attribute("class", "action-btn");
                                let _ = btn.set_attribute("style", "background: #065f46; color: #6ee7b7; border-color: #10b981; cursor: default; white-space: nowrap;");
                            } else if ore >= tech.cost {
                                btn.set_text_content(Some("🔬 Research"));
                                let _ = btn.set_attribute("class", "action-btn");
                                let _ = btn.set_attribute("style", "background: #0369a1; color: #e0f2fe; border-color: #38bdf8; cursor: pointer; white-space: nowrap;");
                                let app_inner = app_c.clone();
                                let doc_inner = doc_c.clone();
                                let tid = tech.id;
                                let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                                    let mut g_mut = app_inner.borrow_mut();
                                    let mut ore_mut = g_mut.colony.common_ore;
                                    if let Ok(msg) = g_mut.colony.tech_tree.research(tid, &mut ore_mut) {
                                        g_mut.colony.common_ore = ore_mut;
                                        g_mut.audio.play_research_completed();
                                        if !g_mut.audio.is_muted() {
                                            audio::radio::RadioDispatcher::announce(&format!("Scientific advancement: {}", msg));
                                        }
                                        if let Some(toast) = doc_inner.get_element_by_id("research-toast") {
                                            toast.set_text_content(Some(&msg));
                                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                                        }
                                        btn_c.set_text_content(Some("✅ Active"));
                                        let _ = btn_c.set_attribute("style", "background: #065f46; color: #6ee7b7; border-color: #10b981; cursor: default; white-space: nowrap;");
                                    }
                                }) as Box<dyn FnMut(_)>);
                                let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
                                cb.forget();
                            } else {
                                btn.set_text_content(Some("Low Ore"));
                                let _ = btn.set_attribute("class", "action-btn");
                                let _ = btn.set_attribute("style", "background: #1e293b; color: #64748b; border-color: #334155; cursor: not-allowed; white-space: nowrap;");
                            }
                            let _ = card.append_child(&btn);
                        }
                        let _ = list_el.append_child(&card);
                    }
                }
            }
        })
    };

    if let Some(btn_research) = document.get_element_by_id("btn-research") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let render_tech_clone = render_tech_list.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            render_tech_clone();
            if let Some(modal) = doc_clone.get_element_by_id("tech-modal") {
                let _ = modal.set_attribute("style", "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.75); backdrop-filter: blur(8px); display: flex; align-items: center; justify-content: center; z-index: 50;");
            }
        }) as Box<dyn FnMut(_)>);
        btn_research.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-close-tech") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(modal) = doc_clone.get_element_by_id("tech-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Multiplayer Lobby Modal Controls
    if let Some(btn_mp) = document.get_element_by_id("btn-multiplayer") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            if let Some(modal) = doc_clone.get_element_by_id("multiplayer-modal") {
                let _ = modal.set_attribute("style", "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.85); backdrop-filter: blur(10px); display: flex; align-items: center; justify-content: center; z-index: 55;");
            }
        }) as Box<dyn FnMut(_)>);
        btn_mp.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-close-multiplayer") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(modal) = doc_clone.get_element_by_id("multiplayer-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Host Action: Create Offer
    if let Some(btn_host) = document.get_element_by_id("btn-net-host") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            match net::WebRtcSession::new_host() {
                Ok(session) => {
                    g.net_session = Some(session.clone());
                    let doc_c = doc_clone.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Some(badge) = doc_c.get_element_by_id("net-status-badge") {
                            badge.set_text_content(Some("GATHERING ICE (HOST)"));
                            let _ = badge.set_attribute("style", "font-size: 11px; font-weight: bold; color: #facc15; background: rgba(250, 204, 21, 0.15); border: 1px solid #facc15; border-radius: 4px; padding: 3px 8px;");
                        }
                        match session.create_offer().await {
                            Ok(offer_sdp) => {
                                if let Some(textarea) = doc_c.get_element_by_id("net-offer-out") {
                                    if let Ok(ta) = textarea.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                        ta.set_value(&offer_sdp);
                                    }
                                }
                                if let Some(badge) = doc_c.get_element_by_id("net-status-badge") {
                                    badge.set_text_content(Some("OFFER READY (SHARE)"));
                                }
                            }
                            Err(e) => {
                                web_sys::console::error_1(&e);
                            }
                        }
                    });
                }
                Err(e) => {
                    web_sys::console::error_1(&e);
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_host.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Host Action: Accept Answer
    if let Some(btn_acc) = document.get_element_by_id("btn-accept-answer") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            if let Some(session) = g.net_session.clone() {
                if let Some(ta_el) = doc_clone.get_element_by_id("net-answer-in") {
                    if let Ok(ta) = ta_el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                        let answer_sdp = ta.value();
                        if !answer_sdp.is_empty() {
                            let doc_c = doc_clone.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                match session.accept_answer(&answer_sdp).await {
                                    Ok(_) => {
                                        if let Some(badge) = doc_c.get_element_by_id("net-status-badge") {
                                            badge.set_text_content(Some("P2P LOCKED (ONLINE)"));
                                            let _ = badge.set_attribute("style", "font-size: 11px; font-weight: bold; color: #4ade80; background: rgba(74, 222, 128, 0.15); border: 1px solid #4ade80; border-radius: 4px; padding: 3px 8px;");
                                        }
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(&e);
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_acc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Join Action: Accept Offer & Generate Answer
    if let Some(btn_join) = document.get_element_by_id("btn-net-join") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            if let Some(ta_el) = doc_clone.get_element_by_id("net-offer-in") {
                if let Ok(ta) = ta_el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    let offer_sdp = ta.value();
                    if !offer_sdp.is_empty() {
                        match net::WebRtcSession::new_guest() {
                            Ok(session) => {
                                g.net_session = Some(session.clone());
                                let doc_c = doc_clone.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Some(badge) = doc_c.get_element_by_id("net-status-badge") {
                                        badge.set_text_content(Some("GATHERING ICE (GUEST)"));
                                        let _ = badge.set_attribute("style", "font-size: 11px; font-weight: bold; color: #facc15; background: rgba(250, 204, 21, 0.15); border: 1px solid #facc15; border-radius: 4px; padding: 3px 8px;");
                                    }
                                    match session.accept_offer_and_create_answer(&offer_sdp).await {
                                        Ok(answer_sdp) => {
                                            if let Some(out_el) = doc_c.get_element_by_id("net-answer-out") {
                                                if let Ok(out_ta) = out_el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                                                    out_ta.set_value(&answer_sdp);
                                                }
                                            }
                                            if let Some(badge) = doc_c.get_element_by_id("net-status-badge") {
                                                badge.set_text_content(Some("ANSWER READY (SEND TO HOST)"));
                                            }
                                        }
                                        Err(e) => {
                                            web_sys::console::error_1(&e);
                                        }
                                    }
                                });
                            }
                            Err(e) => {
                                web_sys::console::error_1(&e);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_join.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Copy Offer to Clipboard
    if let Some(btn_copy) = document.get_element_by_id("btn-copy-offer") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(ta_el) = doc_clone.get_element_by_id("net-offer-out") {
                if let Ok(ta) = ta_el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    let _ = ta.select();
                    let val = ta.value();
                    if let Some(win) = web_sys::window() {
                        let _ = win.navigator().clipboard().write_text(&val);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_copy.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Copy Answer to Clipboard
    if let Some(btn_copy) = document.get_element_by_id("btn-copy-answer") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(ta_el) = doc_clone.get_element_by_id("net-answer-out") {
                if let Ok(ta) = ta_el.dyn_into::<web_sys::HtmlTextAreaElement>() {
                    let _ = ta.select();
                    let val = ta.value();
                    if let Some(win) = web_sys::window() {
                        let _ = win.navigator().clipboard().write_text(&val);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_copy.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Send Tactical Chat
    if let Some(btn_chat) = document.get_element_by_id("btn-net-send-chat") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let g = app_clone.borrow();
            if let Some(input_el) = doc_clone.get_element_by_id("net-chat-input") {
                if let Ok(inp) = input_el.dyn_into::<web_sys::HtmlInputElement>() {
                    let text = inp.value().trim().to_string();
                    if !text.is_empty() {
                        inp.set_value("");
                        let sender_name = format!("{:?} Commander", g.colony.faction);
                        let msg = net::NetMessage::ChatMessage {
                            sender: sender_name.clone(),
                            text: text.clone(),
                        };
                        if let Some(session) = &g.net_session {
                            let _ = session.send_message(&msg);
                        }
                        if let Some(log_el) = doc_clone.get_element_by_id("net-chat-log") {
                            if let Ok(chat_div) = doc_clone.create_element("div") {
                                chat_div.set_text_content(Some(&format!("[{}] {}", sender_name, text)));
                                let _ = chat_div.set_attribute("style", "color: #38bdf8; margin-bottom: 2px;");
                                let _ = log_el.append_child(&chat_div);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_chat.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_save) = document.get_element_by_id("btn-save") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let window_clone = window.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let play_secs = g.playtime_seconds;
            let save_state = sim::SaveGameState::from_colony(&g.colony, 1, "Slot 1 Colony Save", play_secs);
            if let Ok(Some(storage)) = window_clone.local_storage() {
                if let Ok(json) = save_state.to_json() {
                    let _ = storage.set_item("op2_save_slot_1", &json);
                    let _ = storage.set_item("op2_colony_save", &json);
                    if let Ok(meta_json) = serde_json::to_string(&save_state.metadata) {
                        let _ = storage.set_item("op2_save_slot_1_meta", &meta_json);
                    }
                    g.audio.play_build_confirm();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some("Colony progress saved to browser storage (Slot 1)!"));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                    }
                    return;
                }
            }
            g.audio.play_ui_beep();
        }) as Box<dyn FnMut(_)>);
        btn_save.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_load) = document.get_element_by_id("btn-load") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let window_clone = window.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            if let Ok(Some(storage)) = window_clone.local_storage() {
                let save_str = storage.get_item("op2_save_slot_1").ok().flatten()
                    .or_else(|| storage.get_item("op2_colony_save").ok().flatten());
                if let Some(json) = save_str {
                    if let Ok(save_obj) = sim::SaveGameState::from_json(&json) {
                        save_obj.apply_to_colony(&mut g.colony);
                        g.audio.play_unit_ack();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some("Colony save successfully restored!"));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
                        }
                        return;
                    } else if let Ok(legacy_obj) = serde_json::from_str::<sim::SaveGameData>(&json) {
                        g.colony.apply_save_data(legacy_obj);
                        g.audio.play_unit_ack();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some("Colony save successfully restored!"));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
                        }
                        return;
                    }
                }
            }
            g.audio.play_ui_beep();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("No saved colony data found in storage."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn_load.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Mobile Touch Event Handling (Pan and Pinch-to-zoom)
    {
        let app_clone = app.clone();
        let touch_start = Closure::wrap(Box::new(move |e: web_sys::TouchEvent| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            if let Some(touch) = e.touches().item(0) {
                g.is_dragging = true;
                g.last_mouse_x = touch.client_x() as f64;
                g.last_mouse_y = touch.client_y() as f64;
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("touchstart", touch_start.as_ref().unchecked_ref())?;
        touch_start.forget();
    }

    {
        let app_clone = app.clone();
        let touch_move = Closure::wrap(Box::new(move |e: web_sys::TouchEvent| {
            let mut g = app_clone.borrow_mut();
            if e.touches().length() == 1 && g.is_dragging {
                if let Some(touch) = e.touches().item(0) {
                    let cur_x = touch.client_x() as f64;
                    let cur_y = touch.client_y() as f64;
                    let dx = cur_x - g.last_mouse_x;
                    let dy = cur_y - g.last_mouse_y;
                    g.viewport.camera_x += dx;
                    g.viewport.camera_y += dy;
                    g.last_mouse_x = cur_x;
                    g.last_mouse_y = cur_y;
                }
            } else if e.touches().length() == 2 {
                // Pinch to zoom
                if let (Some(t1), Some(t2)) = (e.touches().item(0), e.touches().item(1)) {
                    let dist = ((t1.client_x() - t2.client_x()).pow(2) as f64 + (t1.client_y() - t2.client_y()).pow(2) as f64).sqrt();
                    if dist > 50.0 {
                        let target_zoom = (dist / 200.0).clamp(0.5, 2.5);
                        g.viewport.zoom = g.viewport.zoom * 0.95 + target_zoom * 0.05;
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())?;
        touch_move.forget();
    }

    {
        let app_clone = app.clone();
        let touch_end = Closure::wrap(Box::new(move |_: web_sys::TouchEvent| {
            let mut g = app_clone.borrow_mut();
            g.is_dragging = false;
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())?;
        touch_end.forget();
    }

    // In-game Language Switcher
    if let Some(lang_select) = document.get_element_by_id("lang-select") {
        let doc_clone = document.clone();
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    let val = select.value();
                    i18n::apply_language(&val, &doc_clone);
                    let mut g = app_clone.borrow_mut();
                    g.menu.options.language = val.clone();
                    g.hud.state.language = val;
                }
            }
        }) as Box<dyn FnMut(_)>);
        lang_select.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Campaign Scenario Selector
    if let Some(camp_select) = document.get_element_by_id("campaign-select") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    let val = select.value();
                    let scenario_id = val.parse::<usize>().unwrap_or(0);
                    let mut g = app_clone.borrow_mut();
                    let _ = g.audio.init();
                    g.colony = sim::ColonyState::new_scenario(scenario_id);
                    g.audio.play_build_confirm();

                    // Update faction label
                    if let Some(badge) = doc_clone.get_element_by_id("colony-faction") {
                        match g.colony.faction {
                            sim::Faction::Eden => {
                                badge.set_text_content(Some("EDEN"));
                                let _ = badge.set_attribute("class", "telemetry-val val-eden");
                            }
                            sim::Faction::Plymouth => {
                                badge.set_text_content(Some("PLYMOUTH"));
                                let _ = badge.set_attribute("class", "telemetry-val val-plymouth");
                            }
                        }
                    }

                    // Update modal briefing
                    if let Some(title) = doc_clone.get_element_by_id("briefing-title") {
                        title.set_text_content(Some(&g.colony.mission.codename));
                    }
                    if let Some(text) = doc_clone.get_element_by_id("briefing-text") {
                        text.set_text_content(Some(&g.colony.mission.briefing));
                    }
                    if let Some(ul) = doc_clone.get_element_by_id("briefing-objectives") {
                        ul.set_inner_html("");
                        for obj in &g.colony.mission.objectives {
                            if let Ok(li) = doc_clone.create_element("li") {
                                li.set_text_content(Some(&obj.description));
                                let _ = ul.append_child(&li);
                            }
                        }
                    }

                    if !g.audio.is_muted() {
                        audio::radio::RadioDispatcher::announce(&format!("Tactical computer initialized: {}", g.colony.mission.codename));
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        camp_select.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Structure Inspector Controls (Queue Units / Starship Modules)
    let setup_queue_btn = |btn_id: &'static str, item_name: &'static str, app_rc: Rc<RefCell<GameApp>>, doc_rc: web_sys::Document| {
        if let Some(btn) = doc_rc.get_element_by_id(btn_id) {
            let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let mut g = app_rc.borrow_mut();
                let _ = g.audio.init();
                let selected_b_id = g.colony.buildings.iter().find(|b| b.is_selected).map(|b| b.id);
                if let Some(b_id) = selected_b_id {
                    match g.colony.queue_production(b_id, item_name.to_string()) {
                        Ok(msg) => {
                            g.audio.play_build_confirm();
                            if let Some(toast) = doc_rc.get_element_by_id("research-toast") {
                                toast.set_text_content(Some(&msg));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                            }
                        }
                        Err(err) => {
                            g.audio.play_build_error();
                            if err.contains("Requires Research") && !g.audio.is_muted() {
                                audio::radio::RadioDispatcher::announce(&err);
                            }
                            if let Some(toast) = doc_rc.get_element_by_id("research-toast") {
                                toast.set_text_content(Some(&err));
                                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                            }
                        }
                    }
                } else {
                    g.audio.play_ui_beep();
                }
            }) as Box<dyn FnMut(_)>);
            let _ = btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    };

    setup_queue_btn("btn-queue-tank", "Panther Battle Tank", app.clone(), document.clone());
    setup_queue_btn("btn-queue-tiger", "Tiger Heavy Tank", app.clone(), document.clone());
    setup_queue_btn("btn-queue-scout", "Lynx Scout", app.clone(), document.clone());
    setup_queue_btn("btn-queue-truck", "Cargo Truck", app.clone(), document.clone());
    setup_queue_btn("btn-queue-miner", "Robo-Miner", app.clone(), document.clone());
    setup_queue_btn("btn-queue-repair", "Repair Vehicle", app.clone(), document.clone());
    setup_queue_btn("btn-queue-earthworker", "Earthworker", app.clone(), document.clone());
    setup_queue_btn("btn-queue-convec", "ConVec", app.clone(), document.clone());
    setup_queue_btn("btn-queue-convec-smelter", "ConVec (Smelter)", app.clone(), document.clone());
    setup_queue_btn("btn-queue-convec-solar", "ConVec (Solar)", app.clone(), document.clone());
    setup_queue_btn("btn-queue-convec-guard", "ConVec (Guard)", app.clone(), document.clone());
    setup_queue_btn("btn-queue-convec-geothermal", "ConVec (Geothermal)", app.clone(), document.clone());
    setup_queue_btn("btn-queue-starship", "Starship Component", app.clone(), document.clone());

    if let Some(btn) = document.get_element_by_id("btn-toggle-power") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let mut changed = false;
            let faction = g.colony.faction;
            for b in &mut g.colony.buildings {
                if b.is_selected && b.faction == Some(faction) && b.construction_progress >= 100.0 {
                    b.is_active = !b.is_active;
                    changed = true;
                }
            }
            if changed {
                g.audio.play_ui_beep();
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-recycle-building") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let selected_id = g.colony.buildings.iter().find(|b| b.is_selected).map(|b| b.id);
            if let Some(b_id) = selected_id {
                match g.colony.recycle_structure(b_id) {
                    Ok(msg) => {
                        g.audio.play_build_confirm();
                        if let Some(card) = doc_clone.get_element_by_id("inspector-card") {
                            let _ = card.set_attribute("style", "display: none;");
                        }
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    }
                    Err(err) => {
                        g.audio.play_build_error();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&err));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                        }
                    }
                }
            } else {
                g.audio.play_build_error();
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-launch-starship") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            match g.colony.launch_starship() {
                Ok(msg) => {
                    g.audio.play_victory_fanfare();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some(&msg));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: bold; color: #a855f7; display: block;");
                    }
                }
                Err(err) => {
                    g.audio.play_build_error();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some(&err));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Debriefing Modal Controls
    if let Some(btn) = document.get_element_by_id("btn-close-debriefing") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(modal) = doc_clone.get_element_by_id("debriefing-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-debriefing-continue") {
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Some(modal) = doc_clone.get_element_by_id("debriefing-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-debriefing-restart") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let curr_scen = g.colony.scenario_id;
            g.colony = sim::ColonyState::new_scenario(curr_scen);
            g.audio.play_ui_beep();
            if let Some(modal) = doc_clone.get_element_by_id("debriefing-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-debriefing-next") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let next_scen = (g.colony.scenario_id + 1) % 3;
            g.colony = sim::ColonyState::new_scenario(next_scen);
            g.audio.play_ui_beep();
            if let Some(modal) = doc_clone.get_element_by_id("debriefing-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Earthworker Contextual Engineering Orders
    if let Some(btn) = document.get_element_by_id("btn-order-wall") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();

            if !g.colony.tech_tree.can_build_walls() {
                g.audio.play_build_error();
                if !g.audio.is_muted() {
                    audio::radio::RadioDispatcher::announce("Research required: Lava Containment Walls.");
                }
                if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                    toast.set_text_content(Some("Requires Research: Lava Containment Walls (1100 C)"));
                    let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                }
                return;
            }

            g.pending_engineering = Some(EngineeringMode::Wall);
            g.audio.play_build_confirm();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("Engineering Wall: Click target terrain to erect fortified ceramic barrier (15 C)."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #f59e0b; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-order-tube") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.pending_engineering = Some(EngineeringMode::Tube);
            g.audio.play_build_confirm();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("Engineering Tube: Click target terrain to lay pressurized transport conduit (5 C)."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-order-clear") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.pending_engineering = Some(EngineeringMode::Clear);
            g.audio.play_build_confirm();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("Engineering Hazard Purge: Click target terrain to neutralize Blight spores."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #a855f7; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-order-bulldoze") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.pending_engineering = Some(EngineeringMode::Bulldoze);
            g.audio.play_build_confirm();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("Engineering Bulldoze: Click wall or conduit to demolish obstruction."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // ConVec Kit Deployment & Loading Orders
    if let Some(btn) = document.get_element_by_id("btn-convec-deploy") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.pending_convec_deploy = true;
            g.audio.play_build_confirm();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("ConVec Deployment: Click target map tile to unfold and deploy structure kit."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #facc15; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-convec-load-smelter") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let selected_u_id = g.colony.units.iter().find(|u| u.is_selected && u.faction == g.colony.faction).map(|u| u.id);
            if let Some(uid) = selected_u_id {
                match g.colony.load_convec_kit(uid, "Common Ore Smelter".to_string()) {
                    Ok(msg) => {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    }
                    Err(err) => {
                        g.audio.play_build_error();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&err));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-convec-load-solar") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let selected_u_id = g.colony.units.iter().find(|u| u.is_selected && u.faction == g.colony.faction).map(|u| u.id);
            if let Some(uid) = selected_u_id {
                match g.colony.load_convec_kit(uid, "Solar Power Array".to_string()) {
                    Ok(msg) => {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    }
                    Err(err) => {
                        g.audio.play_build_error();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&err));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Tactical Stance Orders (Guard, Patrol, Halt)
    if let Some(btn) = document.get_element_by_id("btn-order-guard") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let mut ordered = false;
            let player_faction = g.colony.faction;
            for u in &mut g.colony.units {
                if u.is_selected && u.faction == player_faction {
                    u.current_order = sim::pathfinding::UnitOrder::Guard { anchor_x: u.tile_x, anchor_y: u.tile_y };
                    ordered = true;
                }
            }
            if ordered {
                g.audio.play_unit_ack();
                if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                    toast.set_text_content(Some("Guard anchor established at current sector."));
                    let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #60a5fa; display: block;");
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-order-patrol") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.pending_patrol = true;
            g.audio.play_ui_beep();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("Patrol Route: Click target destination on map to designate patrol lane."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn) = document.get_element_by_id("btn-order-halt") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let player_faction = g.colony.faction;
            for u in &mut g.colony.units {
                if u.is_selected && u.faction == player_faction {
                    u.current_order = sim::pathfinding::UnitOrder::Idle;
                    u.path.clear();
                }
            }
            g.audio.play_ui_beep();
            if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                toast.set_text_content(Some("All orders halted. Units holding position."));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Robo-Miner Subterranean Mine Head Construction Order
    if let Some(btn) = document.get_element_by_id("btn-deploy-mine") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let selected_u_id = g.colony.units.iter().find(|u| u.is_selected && u.faction == g.colony.faction).map(|u| u.id);
            if let Some(uid) = selected_u_id {
                match g.colony.deploy_mine_head(uid) {
                    Ok(msg) => {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    }
                    Err(err) => {
                        g.audio.play_build_error();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&err));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Campaign Briefing Modal Controls & Menu Integration
    if let Some(btn) = document.get_element_by_id("btn-briefing") {
        let doc_clone = document.clone();
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            g.menu.selected_scenario_id = g.colony.mission.scenario_id;
            g.menu.switch_screen(render::MenuScreen::CampaignBriefing);
            if let Some(modal) = doc_clone.get_element_by_id("briefing-modal") {
                let _ = modal.set_attribute("style", "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.75); backdrop-filter: blur(8px); display: flex; align-items: center; justify-content: center; z-index: 50;");
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // In-game Pause Button
    if let Some(btn_pause) = document.get_element_by_id("btn-pause") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            if g.menu.is_in_game() {
                g.menu.switch_screen(render::MenuScreen::PauseMenu);
            } else if g.menu.current_screen == render::MenuScreen::PauseMenu {
                g.menu.switch_screen(render::MenuScreen::None);
            }
        }) as Box<dyn FnMut(_)>);
        btn_pause.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    let close_modal = {
        let doc_clone = document.clone();
        let app_clone = app.clone();
        Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_unit_ack();
            if let Some(modal) = doc_clone.get_element_by_id("briefing-modal") {
                let _ = modal.set_attribute("style", "display: none;");
            }
        }) as Box<dyn FnMut(_)>)
    };

    if let Some(btn) = document.get_element_by_id("btn-close-briefing") {
        btn.add_event_listener_with_callback("click", close_modal.as_ref().unchecked_ref())?;
    }
    if let Some(btn) = document.get_element_by_id("btn-ack-briefing") {
        btn.add_event_listener_with_callback("click", close_modal.as_ref().unchecked_ref())?;
    }
    close_modal.forget();

    // Background Music Controls
    if let Some(btn) = document.get_element_by_id("btn-music") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let playing = g.audio.music.toggle();
            if let Some(target) = e.target() {
                if let Ok(btn_el) = target.dyn_into::<web_sys::HtmlElement>() {
                    let label = if playing { "🎵 Music: ON" } else { "🎵 Music: OFF" };
                    btn_el.set_text_content(Some(label));
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Sound Effects Mute Toggle
    if let Some(btn) = document.get_element_by_id("btn-sfx") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            let muted = g.audio.toggle_mute();
            if muted {
                audio::radio::RadioDispatcher::silence();
            }
            if let Some(target) = e.target() {
                if let Ok(btn_el) = target.dyn_into::<web_sys::HtmlElement>() {
                    let label = if muted { "🔇 SFX: OFF" } else { "🔊 SFX: ON" };
                    btn_el.set_text_content(Some(label));
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Main Animation Loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let window_clone = window.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
        let mut app_borrow = app.borrow_mut();

        // Calculate delta time
        if app_borrow.last_frame_time == 0.0 {
            app_borrow.last_frame_time = now;
        }
        let dt = (now - app_borrow.last_frame_time) / 1000.0;
        app_borrow.last_frame_time = now;

        // Resize canvas to match display window and high-DPI scaling
        let dpr = window_clone.device_pixel_ratio().max(1.0);
        if let Some(container) = document.get_element_by_id("game-container") {
            let playing = app_borrow.menu.is_in_game()
                || app_borrow.menu.current_screen == render::MenuScreen::PauseMenu;
            let current = container.get_attribute("class").as_deref() == Some("playing");
            if playing != current {
                if playing { let _ = container.set_attribute("class", "playing"); }
                else { let _ = container.remove_attribute("class"); }
            }
        }
        let win_w = window_clone.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
        let win_h = window_clone.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);
        let client_w = if canvas.client_width() > 0 { canvas.client_width() as f64 } else { win_w };
        let client_h = if canvas.client_height() > 0 { canvas.client_height() as f64 } else { win_h };
        if client_w > 0.0 && client_h > 0.0 {
            let dims = render::touch::compute_high_dpi_canvas_dimensions(client_w, client_h, dpr);
            if canvas.width() != dims.physical_width || canvas.height() != dims.physical_height {
                canvas.set_width(dims.physical_width);
                canvas.set_height(dims.physical_height);
            }
        }

        let mini_w = if minimap_canvas.client_width() > 0 { minimap_canvas.client_width() as f64 } else { 180.0 };
        let mini_h = if minimap_canvas.client_height() > 0 { minimap_canvas.client_height() as f64 } else { 180.0 };
        if mini_w > 0.0 && mini_h > 0.0 {
            let mini_dims = render::touch::compute_high_dpi_canvas_dimensions(mini_w, mini_h, dpr);
            if minimap_canvas.width() != mini_dims.physical_width || minimap_canvas.height() != mini_dims.physical_height {
                minimap_canvas.set_width(mini_dims.physical_width);
                minimap_canvas.set_height(mini_dims.physical_height);
                app_borrow.hud.state.radar.radar_width = mini_w;
                app_borrow.hud.state.radar.radar_height = mini_h;
            }
        }

        // WebRTC P2P Multiplayer Packet Dispatch & Telemetry
        let net_session_opt = app_borrow.net_session.clone();
        if let Some(session) = net_session_opt {
            let connected = session.is_connected();
            let ping = session.ping_ms();
            let turn = (app_borrow.colony.sol * 100.0) as u64;
            app_borrow.hud.state.telemetry.is_multiplayer = true;
            app_borrow.hud.state.telemetry.is_connected = connected;
            app_borrow.hud.state.telemetry.update_telemetry(ping, turn, connected);

            if connected {
                if let Some(badge) = document.get_element_by_id("net-status-badge") {
                    badge.set_text_content(Some("ONLINE (P2P LOCKED)"));
                    let _ = badge.set_attribute("style", "font-size: 11px; font-weight: bold; color: #4ade80; background: rgba(74, 222, 128, 0.15); border: 1px solid #4ade80; border-radius: 4px; padding: 3px 8px;");
                }
            }
            let net_messages = session.poll_messages();
            for msg in net_messages {
                match msg {
                    net::NetMessage::UnitOrder { unit_ids, order } => {
                        let sim_order = order.to_unit_order();
                        for u in &mut app_borrow.colony.units {
                            if unit_ids.contains(&u.id) {
                                u.current_order = sim_order.clone();
                            }
                        }
                    }
                    net::NetMessage::QueueProduction { building_id, item } => {
                        let _ = app_borrow.colony.queue_production(building_id, item);
                    }
                    net::NetMessage::RecycleStructure { building_id } => {
                        let _ = app_borrow.colony.recycle_structure(building_id);
                    }
                    net::NetMessage::ChatMessage { sender, text } => {
                        let cur_sol = app_borrow.colony.sol;
                        let faction = if sender.contains("Eden") {
                            Some(Faction::Eden)
                        } else if sender.contains("Plymouth") {
                            Some(Faction::Plymouth)
                        } else {
                            match app_borrow.colony.faction {
                                Faction::Eden => Some(Faction::Plymouth),
                                Faction::Plymouth => Some(Faction::Eden),
                            }
                        };
                        app_borrow.hud.state.chat.push_incoming(&sender, &text, faction, cur_sol);
                        app_borrow.hud.state.notifications.push_chat(
                            &sender,
                            "[Radio]",
                            &text,
                            cur_sol,
                        );
                        if let Some(log_el) = document.get_element_by_id("net-chat-log") {
                            if let Ok(chat_div) = document.create_element("div") {
                                chat_div.set_text_content(Some(&format!("[{}] {}", sender, text)));
                                let _ = chat_div.set_attribute("style", "color: #facc15; margin-bottom: 2px;");
                                let _ = log_el.append_child(&chat_div);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Synchronize DOM interface visibility with menu state
        let in_game = app_borrow.menu.is_in_game();
        if let Some(container) = document.get_element_by_id("game-container") {
            let _ = container.set_attribute("class", if in_game { "playing" } else { "" });
        }
        if let Some(tb) = document.get_element_by_id("top-bar") {
            let _ = tb.set_attribute("style", if in_game { "" } else { "display: none;" });
        }
        if let Some(bb) = document.get_element_by_id("bottom-bar") {
            let _ = bb.set_attribute("style", if in_game { "" } else { "display: none;" });
        }
        if let Some(mc) = document.get_element_by_id("minimap-card") {
            let _ = mc.set_attribute("style", if in_game { "" } else { "display: none;" });
        }
        if in_game {
            if let Some(status) = document.get_element_by_id("classic-status") {
                let morale = if app_borrow.colony.morale_pct >= 80 { "EXCELLENT" }
                    else if app_borrow.colony.morale_pct >= 50 { "FAIR" } else { "POOR" };
                status.set_text_content(Some(&format!(
                    "SOL {:.2}  MORALE {}  POP {}  C {}  R {}  FOOD {}  PWR {}/{}",
                    app_borrow.colony.sol, morale, app_borrow.colony.population,
                    app_borrow.colony.common_ore, app_borrow.colony.rare_ore,
                    app_borrow.colony.food_units, app_borrow.colony.power_produced,
                    app_borrow.colony.power_consumed,
                )));
            }
            if let Some(label) = document.get_element_by_id("sidebar-mission") {
                label.set_text_content(Some(&app_borrow.colony.mission.codename));
            }
            if let Some(list) = document.get_element_by_id("sidebar-objectives") {
                let objectives = app_borrow.colony.mission.objectives.iter()
                    .filter(|objective| objective.is_primary)
                    .map(|objective| format!("{} {}", if objective.is_completed { "✓" } else { "□" }, objective.description))
                    .collect::<Vec<_>>().join("\n\n");
                if list.text_content().as_deref() != Some(&objectives) {
                    list.set_text_content(Some(&objectives));
                }
            }
            if let Some(selection) = document.get_element_by_id("sidebar-selection") {
                let has_selection = app_borrow.colony.units.iter().any(|unit| unit.is_selected)
                    || app_borrow.colony.buildings.iter().any(|building| building.is_selected);
                if let Some(detail) = document.get_element_by_id("classic-detail") {
                    let _ = detail.set_attribute("class", if has_selection { "has-selection" } else { "" });
                }
                let selected = app_borrow.colony.units.iter().find(|unit| unit.is_selected)
                    .map(|unit| format!("{} — {}/{} HP", unit.name, unit.hp, unit.max_hp))
                    .or_else(|| app_borrow.colony.buildings.iter().find(|building| building.is_selected)
                        .map(|building| {
                            if building.construction_progress < 100.0 {
                                format!("{} — Under Construction ({:.0}%)", building.name, building.construction_progress)
                            } else {
                                format!("{} — {}/{} HP", building.name, building.hp, building.max_hp)
                            }
                        }))
                    .unwrap_or_else(|| "Select a unit or building".into());
                if selection.text_content().as_deref() != Some(&selected) {
                    selection.set_text_content(Some(&selected));
                }
            }
        }

        // Update simulation (respecting pause state from 'P' hotkey or Menu)
        let effective_dt = if app_borrow.hud.state.hotkeys.is_paused || !app_borrow.menu.is_in_game() {
            0.0
        } else {
            dt.min(0.1) * app_borrow.menu.options.game_speed
        };
        app_borrow.menu.update(dt);
        let sim_result = app_borrow.colony.update(effective_dt);
        let mut hud_tech_completed = None;
        if app_borrow.menu.is_in_game() {
            let g = &mut *app_borrow;
            hud_tech_completed = g.hud.update(effective_dt, &mut g.colony);
        }
        if hud_tech_completed.is_some() {
            app_borrow.audio.play_research_completed();
        }

        let cur_sol = app_borrow.colony.sol;
        if let Some(warning) = sim_result.notice {
            app_borrow.hud.state.notifications.push_alert(
                "TACTICAL TRANSMISSION",
                warning,
                render::hud::AlertPriority::High,
                None,
                cur_sol,
            );
            if !app_borrow.audio.is_muted() {
                audio::radio::RadioDispatcher::announce(warning);
            }
            if let Some(toast) = document.get_element_by_id("research-toast") {
                toast.set_text_content(Some(warning));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: bold; color: #f97316; display: block;");
            }
        }
        if sim_result.vortex_active {
            if let Some(v) = app_borrow.colony.disasters.active_vortices.first() {
                app_borrow.hud.state.notifications.last_alert_location = Some((v.x as i32, v.y as i32));
            }
        }
        if sim_result.meteor_hit {
            if let Some(m) = app_borrow.colony.disasters.pending_meteors.first() {
                app_borrow.hud.state.notifications.last_alert_location = Some((m.x as i32, m.y as i32));
            }
        }
        if sim_result.laser_fired {
            app_borrow.audio.play_laser();
        }
        if sim_result.railgun_fired {
            app_borrow.audio.play_railgun();
        }
        if sim_result.microwave_fired {
            app_borrow.audio.play_microwave();
        }
        if sim_result.acid_fired {
            app_borrow.audio.play_acid_cloud();
        }
        if sim_result.unit_destroyed || sim_result.meteor_hit {
            app_borrow.audio.play_explosion();
        }
        if sim_result.under_attack {
            app_borrow.audio.play_under_attack();
        }
        if sim_result.structure_destroyed {
            app_borrow.audio.play_structure_destroyed();
        }
        if sim_result.friendly_vehicle_destroyed {
            app_borrow.audio.play_vehicle_destroyed();
        }
        if sim_result.volcano_erupted {
            app_borrow.audio.play_volcanic_eruption();
        }
        if sim_result.vortex_spawned {
            app_borrow.audio.play_vortex_detected();
        }
        if sim_result.disaster_siren {
            app_borrow.audio.play_disaster_siren();
        }
        if sim_result.building_deployed {
            app_borrow.audio.play_construction_deploy();
        }
        if sim_result.vortex_active && sim_result.notice.map(|n| n.contains("Vortex")).unwrap_or(false) {
            app_borrow.audio.play_vortex_howl();
        }

        // Dynamic soundtrack update
        let is_combat = sim_result.laser_fired || sim_result.railgun_fired || sim_result.microwave_fired || sim_result.acid_fired;
        let is_alert = sim_result.disaster_siren;
        app_borrow.audio.update_music(dt.min(0.1), is_combat, is_alert);

        // Milestone autosave execution (every 100 Sols)
        let cur_sol_int = cur_sol.max(0.0) as u32;
        if sim::is_autosave_due(cur_sol_int, app_borrow.last_autosave_sol) {
            app_borrow.last_autosave_sol = cur_sol_int;
            let play_secs = app_borrow.playtime_seconds;
            if let Ok(state) = sim::execute_autosave(&app_borrow.colony, play_secs) {
                app_borrow.hud.state.notifications.push_alert(
                    "AUTOMATED ARCHIVE SNAPSHOT",
                    &format!("Milestone Autosave: Colony state captured at Sol {}!", state.metadata.sol),
                    render::hud::AlertPriority::Medium,
                    None,
                    cur_sol,
                );
                if let Some(toast) = document.get_element_by_id("research-toast") {
                    let msg = format!("💾 AUTOSAVE: Milestone recorded at Sol {}", state.metadata.sol);
                    toast.set_text_content(Some(&msg));
                    let _ = toast.set_attribute("style", "font-size: 11px; font-weight: bold; color: #38bdf8; display: block;");
                }
            }
        }

        if sim_result.starship_launched && app_borrow.colony.launch_anim_time < 0.5 {
            if let Some(toast) = document.get_element_by_id("research-toast") {
                toast.set_text_content(Some("🚀 ORBITAL LAUNCH IN PROGRESS: Evacuation Starship ascending to escape velocity!"));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: bold; color: #c084fc; display: block;");
            }
        }

        // Mission Debriefing & Victory/Defeat Trigger
        if let Some(status) = sim_result.mission_ended {
            let is_victory = status == sim::mission::MissionStatus::Victory;
            if is_victory {
                app_borrow.audio.play_victory_fanfare();
            } else {
                app_borrow.audio.play_defeat_alert();
            }

            if let Some(modal) = document.get_element_by_id("debriefing-modal") {
                let _ = modal.set_attribute("style", "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.85); backdrop-filter: blur(10px); display: flex; align-items: center; justify-content: center; z-index: 60;");

                let active_lang = crate::i18n::Language::from_code(&app_borrow.menu.options.language);
                let debrief_dict = crate::i18n::get_dictionary(active_lang).debriefing;

                if let Some(title_el) = document.get_element_by_id("debriefing-title") {
                    let title_text = if is_victory { debrief_dict.victory_title } else { debrief_dict.defeat_title };
                    let title_col = if is_victory { "#4ade80" } else { "#ef4444" };
                    title_el.set_text_content(Some(title_text));
                    let _ = title_el.set_attribute("style", &format!("font-size: 16px; color: {}; font-weight: bold; margin: 0;", title_col));
                }

                if let Some(icon_el) = document.get_element_by_id("debriefing-icon") {
                    icon_el.set_text_content(Some(if is_victory { "🏆" } else { "⚠️" }));
                }

                if let Some(sub_el) = document.get_element_by_id("debriefing-subtitle") {
                    let template = if is_victory {
                        debrief_dict.victory_subtitle_template
                    } else {
                        debrief_dict.defeat_subtitle_template
                    };
                    let desc = template.replace("{}", &app_borrow.colony.mission.codename);
                    sub_el.set_text_content(Some(&desc));
                }

                if let Some(sol_el) = document.get_element_by_id("debriefing-sol") {
                    sol_el.set_text_content(Some(&format!("{:.2} Sols", app_borrow.colony.sol)));
                }

                if let Some(ship_el) = document.get_element_by_id("debriefing-starship") {
                    let ship_text = if app_borrow.colony.starship_launched {
                        "🚀 Escaped into Orbit!".to_string()
                    } else {
                        format!("{:.0}% Assembled", app_borrow.colony.starship_progress)
                    };
                    ship_el.set_text_content(Some(&ship_text));
                }

                if let Some(pop_el) = document.get_element_by_id("debriefing-pop") {
                    pop_el.set_text_content(Some(&format!("{} Pop | {}% Morale", app_borrow.colony.population, app_borrow.colony.morale_pct)));
                }

                if let Some(ore_el) = document.get_element_by_id("debriefing-ore") {
                    ore_el.set_text_content(Some(&format!("{} C | {} R", app_borrow.colony.common_ore, app_borrow.colony.rare_ore)));
                }

                if let Some(ul) = document.get_element_by_id("debriefing-checklist") {
                    ul.set_inner_html("");
                    for obj in &app_borrow.colony.mission.objectives {
                        if let Ok(li) = document.create_element("li") {
                            let icon = if obj.is_completed { "✅" } else { "❌" };
                            let col = if obj.is_completed { "#4ade80" } else { "#ef4444" };
                            li.set_text_content(Some(&format!("{} {}", icon, obj.description)));
                            let _ = li.set_attribute("style", &format!("color: {};", col));
                            let _ = ul.append_child(&li);
                        }
                    }
                }
            }
        }

        // Update HUD DOM telemetry values
        if let Some(el) = document.get_element_by_id("colony-sol") {
            el.set_text_content(Some(&format!("{:.2}", app_borrow.colony.sol)));
        }
        if let Some(el) = document.get_element_by_id("colony-sun") {
            let pct = (app_borrow.colony.sunlight * 100.0) as u32;
            let phase_desc = if app_borrow.colony.sunlight > 0.75 {
                "Zenith"
            } else if app_borrow.colony.sunlight > 0.40 {
                "Twilight"
            } else {
                "Night"
            };
            el.set_text_content(Some(&format!("{}% ({})", pct, phase_desc)));
        }
        if let Some(el) = document.get_element_by_id("colony-pop") {
            el.set_text_content(Some(&format!("{} ({}% Morale)", app_borrow.colony.population, app_borrow.colony.morale_pct)));
        }
        if let Some(el) = document.get_element_by_id("colony-power") {
            el.set_text_content(Some(&format!("{} / {} kW", app_borrow.colony.power_produced, app_borrow.colony.power_consumed)));
        }
        if let Some(el) = document.get_element_by_id("colony-ore") {
            el.set_text_content(Some(&format!("{} t", app_borrow.colony.common_ore)));
        }
        if let Some(el) = document.get_element_by_id("colony-rare") {
            el.set_text_content(Some(&format!("{} t", app_borrow.colony.rare_ore)));
        }
        if let Some(el) = document.get_element_by_id("colony-food") {
            el.set_text_content(Some(&format!("{}", app_borrow.colony.food_units)));
        }
        if let Some(el) = document.get_element_by_id("colony-starship") {
            el.set_text_content(Some(&format!("{:.0}%", app_borrow.colony.starship_progress)));
        }

        // Update Building Inspector Card
        let selected_b = app_borrow.colony.buildings.iter().find(|b| b.is_selected);
        if let Some(card) = document.get_element_by_id("inspector-card") {
            if let Some(b) = selected_b {
                let _ = card.set_attribute("style", "position: absolute; bottom: 76px; left: 16px; width: 230px; background: rgba(12, 14, 18, 0.92); border: 2px solid var(--op2-border); border-radius: 6px; backdrop-filter: blur(8px); box-shadow: 0 8px 24px rgba(0,0,0,0.6); z-index: 15; display: flex; flex-direction: column; overflow: hidden; padding: 12px; font-size: 12px;");
                if let Some(name_el) = document.get_element_by_id("inspect-name") {
                    name_el.set_text_content(Some(&b.name));
                }
                if let Some(status_el) = document.get_element_by_id("inspect-status") {
                    let status_text = if b.construction_progress < 100.0 { "UNDER CONSTRUCTION" } else if b.is_active { "ONLINE" } else { "OFFLINE" };
                    let status_col = if b.construction_progress < 100.0 { "#e2d39a" } else if b.is_active { "#4ade80" } else { "#ef4444" };
                    status_el.set_text_content(Some(status_text));
                    let _ = status_el.set_attribute("style", &format!("color: {}; font-size: 10px; font-weight: bold;", status_col));
                }
                if let Some(power_el) = document.get_element_by_id("inspect-power") {
                    power_el.set_text_content(Some(&format!("{} kW", b.power_cost)));
                }
                let own = b.faction == Some(app_borrow.colony.faction);
                if let Some(actions) = document.get_element_by_id("factory-actions") {
                    let show = own && b.is_active && (b.name.contains("Factory") || b.name.contains("Spaceport"));
                    let _ = actions.set_attribute("style", if show { "display: flex; flex-direction: column; gap: 6px; margin-top: 6px;" } else { "display: none;" });
                }
                if let Some(button) = document.get_element_by_id("btn-toggle-power") {
                    let _ = button.set_attribute("style", if own { "margin-top: 8px; justify-content: center; background: #334155; display: flex;" } else { "display: none;" });
                }
                if let Some(button) = document.get_element_by_id("btn-recycle-building") {
                    let _ = button.set_attribute("style", if own { "margin-top: 6px; justify-content: center; background: #7f1d1d; border-color: #ef4444; display: flex;" } else { "display: none;" });
                }
                if let Some(queue_el) = document.get_element_by_id("inspect-queue") {
                    if let Some(item) = b.production_queue.first() {
                        queue_el.set_text_content(Some(&format!("{} ({:.0}%)", item, b.production_progress)));
                        if let Some(box_el) = document.get_element_by_id("inspect-queue-box") {
                            let _ = box_el.set_attribute("style", "margin-bottom: 6px; font-size: 11px; color: #38bdf8; display: block;");
                        }
                    } else {
                        queue_el.set_text_content(Some("Idle"));
                        if let Some(box_el) = document.get_element_by_id("inspect-queue-box") {
                            let _ = box_el.set_attribute("style", "display: none;");
                        }
                    }
                }

                if let Some(launch_btn) = document.get_element_by_id("btn-launch-starship") {
                    if own && app_borrow.colony.starship_progress >= 100.0 && !app_borrow.colony.starship_launched {
                        let _ = launch_btn.set_attribute("style", "margin-top: 8px; justify-content: center; background: #581c87; border-color: #c084fc; font-weight: bold; color: #fff; display: flex;");
                    } else {
                        let _ = launch_btn.set_attribute("style", "display: none;");
                    }
                }
            } else {
                let _ = card.set_attribute("style", "display: none;");
            }
        }

        // Update Unit Command & Engineering / ConVec / Combat Inspector Card
        let selected_u = app_borrow.colony.units.iter().find(|u| u.is_selected && u.faction == app_borrow.colony.faction);
        if let Some(card) = document.get_element_by_id("unit-card") {
            if let Some(u) = selected_u {
                let border_color = if u.name.contains("Earthworker") {
                    "#f59e0b"
                } else if u.name.contains("ConVec") {
                    "#eab308"
                } else if u.weapon.is_some() {
                    "#38bdf8"
                } else {
                    "#10b981"
                };
                let style = format!("position: absolute; bottom: 76px; left: 16px; width: 240px; background: rgba(12, 14, 18, 0.94); border: 2px solid {}; border-radius: 6px; backdrop-filter: blur(8px); box-shadow: 0 8px 24px rgba(0,0,0,0.6); z-index: 15; display: flex; flex-direction: column; overflow: hidden; padding: 12px; font-size: 12px;", border_color);
                let _ = card.set_attribute("style", &style);

                if let Some(name_el) = document.get_element_by_id("unit-inspect-name") {
                    name_el.set_text_content(Some(&u.name));
                }
                if let Some(hp_el) = document.get_element_by_id("unit-inspect-hp") {
                    hp_el.set_text_content(Some(&format!("{}/{} HP", u.hp, u.max_hp)));
                }

                if let Some(cargo_el) = document.get_element_by_id("unit-inspect-cargo") {
                    if let Some(kit) = &u.cargo_kit {
                        cargo_el.set_text_content(Some(&format!("📦 Loaded Kit: {}", kit)));
                        let _ = cargo_el.set_attribute("style", "font-size: 11px; color: #facc15; font-weight: bold; margin-bottom: 4px; display: block;");
                    } else if let Some((ore_type, amt)) = &u.cargo {
                        cargo_el.set_text_content(Some(&format!("⛏️ Cargo: {} units of {:?}", amt, ore_type)));
                        let _ = cargo_el.set_attribute("style", "font-size: 11px; color: #38bdf8; margin-bottom: 4px; display: block;");
                    } else {
                        let _ = cargo_el.set_attribute("style", "display: none;");
                    }
                }

                if let Some(order_el) = document.get_element_by_id("unit-inspect-order") {
                    let order_str = match &u.current_order {
                        sim::pathfinding::UnitOrder::Idle => "Status: Idle (Awaiting orders)".to_string(),
                        sim::pathfinding::UnitOrder::MoveTo { target_x, target_y } => format!("Status: En Route to ({}, {})", target_x, target_y),
                        sim::pathfinding::UnitOrder::Attack { target_unit_id } => format!("Status: Engaging Target #{}", target_unit_id),
                        sim::pathfinding::UnitOrder::AttackBuilding { target_building_id } => format!("Status: Attacking Structure #{}", target_building_id),
                        sim::pathfinding::UnitOrder::Repair { target_unit_id } => format!("Status: Repairing Ally #{}", target_unit_id),
                        sim::pathfinding::UnitOrder::MineOre { deposit_x, deposit_y } => format!("Status: Mining Ore at ({}, {})", deposit_x, deposit_y),
                        sim::pathfinding::UnitOrder::DeployStructure { kit_name, tile_x, tile_y } => format!("Status: Deploying {} at ({}, {})", kit_name, tile_x, tile_y),
                        sim::pathfinding::UnitOrder::BuildWall { target_x, target_y } => format!("Status: Building Wall at ({}, {})", target_x, target_y),
                        sim::pathfinding::UnitOrder::BuildTube { target_x, target_y } => format!("Status: Laying Conduit at ({}, {})", target_x, target_y),
                        sim::pathfinding::UnitOrder::ClearHazard { target_x, target_y } => format!("Status: Purging Hazard at ({}, {})", target_x, target_y),
                        sim::pathfinding::UnitOrder::Bulldoze { target_x, target_y } => format!("Status: Bulldozing Obstruction at ({}, {})", target_x, target_y),
                        sim::pathfinding::UnitOrder::Guard { anchor_x, anchor_y } => format!("Status: Guarding Sector ({}, {})", anchor_x, anchor_y),
                        sim::pathfinding::UnitOrder::Patrol { start_x, start_y, end_x, end_y, .. } => format!("Status: Patrol ({},{}) ↔ ({},{})", start_x, start_y, end_x, end_y),
                    };
                    order_el.set_text_content(Some(&order_str));
                }

                if let Some(actions) = document.get_element_by_id("earthworker-actions") {
                    if u.name.contains("Earthworker") {
                        let _ = actions.set_attribute("style", "display: flex; flex-direction: column; gap: 6px; margin-top: 4px;");
                    } else {
                        let _ = actions.set_attribute("style", "display: none;");
                    }
                }

                if let Some(actions) = document.get_element_by_id("convec-actions") {
                    if u.name.contains("ConVec") {
                        let _ = actions.set_attribute("style", "display: flex; flex-direction: column; gap: 6px; margin-top: 4px;");
                    } else {
                        let _ = actions.set_attribute("style", "display: none;");
                    }
                }

                if let Some(actions) = document.get_element_by_id("combat-actions") {
                    if u.weapon.is_some() {
                        let _ = actions.set_attribute("style", "display: flex; flex-direction: column; gap: 6px; margin-top: 4px;");
                    } else {
                        let _ = actions.set_attribute("style", "display: none;");
                    }
                }

                if let Some(actions) = document.get_element_by_id("miner-actions") {
                    if u.name.contains("Robo-Miner") {
                        let _ = actions.set_attribute("style", "display: flex; flex-direction: column; gap: 6px; margin-top: 4px;");
                    } else {
                        let _ = actions.set_attribute("style", "display: none;");
                    }
                }
            } else {
                let _ = card.set_attribute("style", "display: none;");
            }
        }

        let ghost_data = if let Some(st) = app_borrow.pending_placement {
            let (gx, gy) = app_borrow.hover_tile;
            let (w, h) = st.dimensions();
            let (c_cost, r_cost) = st.ore_cost();
            let loaded_convec = app_borrow.colony.units.iter().any(|u|
                u.faction == app_borrow.colony.faction && u.name.contains("ConVec") && u.cargo_kit.as_deref() == Some(st.name()));
            let empty_convec = app_borrow.colony.units.iter().any(|u|
                u.faction == app_borrow.colony.faction && u.name.contains("ConVec") && u.cargo_kit.is_none());
            let has_resources = app_borrow.colony.common_ore >= c_cost && app_borrow.colony.rare_ore >= r_cost;
            let is_valid = (loaded_convec || empty_convec && has_resources)
                && app_borrow.colony.fog.get(gx as usize, gy as usize) == sim::fog::FogState::Visible
                && app_borrow.colony.validate_structure_site(gx, gy, w, h).is_ok();
            Some((st, (gx, gy), is_valid))
        } else {
            None
        };

        let drag_box = if let (Some(start), Some(curr)) = (app_borrow.drag_select_start, app_borrow.drag_select_current) {
            let dist = ((curr.0 - start.0).powi(2) + (curr.1 - start.1).powi(2)).sqrt();
            if dist > 10.0 {
                Some((start, curr))
            } else {
                None
            }
        } else {
            None
        };

        let dpr = window_clone.device_pixel_ratio().max(1.0);
        let win_w = window_clone.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
        let win_h = window_clone.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0);
        let canvas_w = if canvas.client_width() > 0 { canvas.client_width() as f64 } else { win_w };
        let canvas_h = if canvas.client_height() > 0 { canvas.client_height() as f64 } else { win_h };
        let mini_w = if minimap_canvas.client_width() > 0 { minimap_canvas.client_width() as f64 } else { 180.0 };
        let mini_h = if minimap_canvas.client_height() > 0 { minimap_canvas.client_height() as f64 } else { 180.0 };

        app_borrow.viewport.map_cols = app_borrow.colony.game_map.width as i32;
        app_borrow.viewport.map_rows = app_borrow.colony.game_map.height as i32;
        app_borrow.hud.state.radar.map_cols = app_borrow.colony.game_map.width as i32;
        app_borrow.hud.state.radar.map_rows = app_borrow.colony.game_map.height as i32;
        app_borrow.viewport.clamp_camera(canvas_w, canvas_h);

        let _ = ctx.save();
        let _ = ctx.scale(dpr, dpr);
        ctx.set_image_smoothing_enabled(false);

        if !app_borrow.menu.is_in_game() {
            let has_active_game = app_borrow.playtime_seconds > 0
                || app_borrow.menu.previous_screen != render::MenuScreen::None;
            let summary = if has_active_game {
                Some(render::ColonySummary::from_colony(&app_borrow.colony))
            } else {
                None
            };
            if app_borrow.menu.current_screen == render::MenuScreen::PauseMenu {
                // Render game viewport in background
                app_borrow.viewport.render(
                    &ctx,
                    &app_borrow.colony,
                    0.0,
                    canvas_w,
                    canvas_h,
                    None,
                    None,
                );
                // Render Pause Menu overlay
                app_borrow.menu.render(&ctx, canvas_w, canvas_h, summary);
            } else {
                // Render full Outpost 2 menu screen
                app_borrow.menu.render(&ctx, canvas_w, canvas_h, summary);
            }
        } else {
            // Render game viewport
            app_borrow.viewport.render(
                &ctx,
                &app_borrow.colony,
                effective_dt,
                canvas_w,
                canvas_h,
                ghost_data,
                drag_box,
            );

            // Render modern tactical HUD overlays (Command bar, alerts/chat, factory, research)
            app_borrow.hud.render_alerts_and_chat(&ctx, canvas_w, canvas_h);
            app_borrow.hud.render_factory_panel(&ctx, &app_borrow.colony, canvas_w, canvas_h);
            app_borrow.hud.render_research_browser(&ctx, &app_borrow.colony, canvas_w, canvas_h);

            // Render tactical radar
            let _ = minimap_ctx.save();
            let _ = minimap_ctx.scale(dpr, dpr);
            minimap_ctx.set_image_smoothing_enabled(false);
            app_borrow.hud.render_radar(
                &minimap_ctx,
                &app_borrow.colony,
                app_borrow.viewport.camera_x,
                app_borrow.viewport.camera_y,
                canvas_w,
                canvas_h,
                app_borrow.viewport.tile_width,
                app_borrow.viewport.tile_height,
                app_borrow.viewport.zoom,
                mini_w,
                mini_h,
            );
            let _ = minimap_ctx.restore();
        }

        let _ = ctx.restore();

        request_animation_frame(&window_clone, f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(&window, g.borrow().as_ref().unwrap());

    web_sys::console::log_1(&"Outpost 2 WebAssembly engine initialized successfully!".into());
    Ok(())
}

fn request_animation_frame(window: &Window, f: &Closure<dyn FnMut(f64)>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

async fn fetch_art_bytes(path: &str) -> Result<Vec<u8>, JsValue> {
    use wasm_bindgen_futures::JsFuture;
    let response = JsFuture::from(web_sys::window().unwrap().fetch_with_str(path))
        .await?.dyn_into::<web_sys::Response>()?;
    if !response.ok() {
        return Err(JsValue::from_str(&format!("Failed to load {path}: {}", response.status())));
    }
    let buffer = JsFuture::from(response.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}

fn center_on_colony(app: &mut GameApp, canvas: &HtmlCanvasElement) {
    app.viewport.map_cols = app.colony.game_map.width as i32;
    app.viewport.map_rows = app.colony.game_map.height as i32;
    app.hud.state.radar.map_cols = app.colony.game_map.width as i32;
    app.hud.state.radar.map_rows = app.colony.game_map.height as i32;
    if app.menu.is_in_game() {
        if let Some(container) = web_sys::window().unwrap().document().unwrap().get_element_by_id("game-container") {
            let _ = container.set_attribute("class", "playing");
        }
    }
    let base = app.colony.buildings.iter()
        .find(|b| b.faction == Some(app.colony.faction) && b.name.contains("Command Center"))
        .or_else(|| app.colony.buildings.iter().find(|b| b.faction == Some(app.colony.faction)));
    if let Some(base) = base {
        let x = base.tile_x as f64 + base.width as f64 * 0.5;
        let y = base.tile_y as f64 + base.height as f64 * 0.5;
        app.viewport.camera_x = canvas.client_width() as f64 * 0.5 - x * app.viewport.tile_width * app.viewport.zoom;
        app.viewport.camera_y = canvas.client_height() as f64 * 0.5 - y * app.viewport.tile_height * app.viewport.zoom;
    }
    app.viewport.clamp_camera(canvas.client_width() as f64, canvas.client_height() as f64);
}

fn canvas_position(canvas: &HtmlCanvasElement, client_x: i32, client_y: i32) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    logical_canvas_position(
        client_x as f64,
        client_y as f64,
        rect.x(),
        rect.y(),
        rect.width(),
        rect.height(),
        canvas.client_width() as f64,
        canvas.client_height() as f64,
    )
}

fn logical_canvas_position(
    client_x: f64,
    client_y: f64,
    left: f64,
    top: f64,
    displayed_width: f64,
    displayed_height: f64,
    logical_width: f64,
    logical_height: f64,
) -> (f64, f64) {
    (
        (client_x - left) * logical_width / displayed_width,
        (client_y - top) * logical_height / displayed_height,
    )
}

#[cfg(test)]
mod input_tests {
    use super::logical_canvas_position;

    #[test]
    fn pointer_uses_canvas_origin_and_css_size() {
        assert_eq!(
            logical_canvas_position(400.0, 378.0, 0.0, 48.0, 800.0, 552.0, 800.0, 552.0),
            (400.0, 330.0)
        );
        assert_eq!(
            logical_canvas_position(400.0, 378.0, 0.0, 48.0, 400.0, 276.0, 800.0, 552.0),
            (800.0, 660.0)
        );
    }
}
