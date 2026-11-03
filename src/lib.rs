use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, MouseEvent, Window};

mod archive;
mod audio;
mod render;
mod sim;

use audio::AudioManager;
use render::Viewport;
use sim::{ColonyState, Faction};

struct GameApp {
    colony: ColonyState,
    viewport: Viewport,
    audio: AudioManager,
    is_dragging: bool,
    last_mouse_x: f64,
    last_mouse_y: f64,
    last_frame_time: f64,
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
        is_dragging: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        last_frame_time: 0.0,
    }));

    // Mouse Controls (Pan & Select Unit)
    {
        let app_clone = app.clone();
        let on_mouse_down = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();

            let click_x = e.client_x() as f64;
            let click_y = e.client_y() as f64;

            if e.button() == 0 {
                // Check if clicked on a unit
                let zoom = g.viewport.zoom;
                let mut clicked_unit = false;
                let unit_count = g.colony.units.len();
                for i in 0..unit_count {
                    let tx = g.colony.units[i].tile_x as f64;
                    let ty = g.colony.units[i].tile_y as f64;
                    let (sx, sy) = g.viewport.tile_to_screen(tx, ty);
                    let dist = ((click_x - sx).powi(2) + (click_y - sy).powi(2)).sqrt();
                    if dist < 20.0 * zoom {
                        g.colony.units[i].is_selected = true;
                        clicked_unit = true;
                    } else if !e.shift_key() {
                        g.colony.units[i].is_selected = false;
                    }
                }

                if clicked_unit {
                    g.audio.play_unit_ack();
                } else {
                    g.is_dragging = true;
                    g.last_mouse_x = click_x;
                    g.last_mouse_y = click_y;
                }
            } else if e.button() == 2 {
                // Right Click = Tactical Command (Attack if enemy clicked, otherwise Move)
                let zoom = g.viewport.zoom;
                let mut clicked_enemy = None;
                for u in &g.colony.units {
                    if u.faction != sim::Faction::Eden {
                        let (sx, sy) = g.viewport.tile_to_screen(u.tile_x as f64, u.tile_y as f64);
                        let dist = ((click_x - sx).powi(2) + (click_y - sy).powi(2)).sqrt();
                        if dist < 24.0 * zoom {
                            clicked_enemy = Some(u.id);
                            break;
                        }
                    }
                }

                if let Some(enemy_id) = clicked_enemy {
                    for u in &mut g.colony.units {
                        if u.is_selected && u.faction == sim::Faction::Eden {
                            u.current_order = sim::pathfinding::UnitOrder::Attack { target_unit_id: enemy_id };
                        }
                    }
                    g.audio.play_build_confirm();
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
        let on_mouse_move = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut g = app_clone.borrow_mut();
            if g.is_dragging {
                let cur_x = e.client_x() as f64;
                let cur_y = e.client_y() as f64;
                let dx = cur_x - g.last_mouse_x;
                let dy = cur_y - g.last_mouse_y;
                g.viewport.camera_x += dx;
                g.viewport.camera_y += dy;
                g.last_mouse_x = cur_x;
                g.last_mouse_y = cur_y;
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        window.add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())?;
        on_mouse_move.forget();
    }

    {
        let app_clone = app.clone();
        let on_mouse_up = Closure::wrap(Box::new(move |_: MouseEvent| {
            app_clone.borrow_mut().is_dragging = false;
        }) as Box<dyn FnMut(MouseEvent)>);
        window.add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())?;
        on_mouse_up.forget();
    }

    // Disable context menu on canvas so right-click is exclusive to game commands
    {
        let on_contextmenu = Closure::wrap(Box::new(move |e: MouseEvent| {
            e.prevent_default();
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.add_event_listener_with_callback("contextmenu", on_contextmenu.as_ref().unchecked_ref())?;
        on_contextmenu.forget();
    }

    // Keyboard Navigation (WASD / Arrows)
    {
        let app_clone = app.clone();
        let on_key_down = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let mut g = app_clone.borrow_mut();
            let step = 32.0;
            match e.key().as_str() {
                "ArrowLeft" | "a" | "A" => g.viewport.camera_x += step,
                "ArrowRight" | "d" | "D" => g.viewport.camera_x -= step,
                "ArrowUp" | "w" | "W" => g.viewport.camera_y += step,
                "ArrowDown" | "s" | "S" => g.viewport.camera_y -= step,
                "+" | "=" => g.viewport.zoom = (g.viewport.zoom * 1.15).min(2.5),
                "-" | "_" => g.viewport.zoom = (g.viewport.zoom / 1.15).max(0.4),
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.add_event_listener_with_callback("keydown", on_key_down.as_ref().unchecked_ref())?;
        on_key_down.forget();
    }

    // Zoom Buttons
    if let Some(btn_in) = document.get_element_by_id("btn-zoom-in") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            g.viewport.zoom = (g.viewport.zoom * 1.25).min(2.5);
        }) as Box<dyn FnMut(_)>);
        btn_in.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_out) = document.get_element_by_id("btn-zoom-out") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_ui_beep();
            g.viewport.zoom = (g.viewport.zoom / 1.25).max(0.4);
        }) as Box<dyn FnMut(_)>);
        btn_out.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Tactical Command Buttons
    if let Some(btn_build) = document.get_element_by_id("btn-build") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();

            let count = g.colony.buildings.len();
            let structure_types = [
                sim::structures::StructureType::CommonOreSmelter,
                sim::structures::StructureType::DirtLab,
                sim::structures::StructureType::Residence,
                sim::structures::StructureType::Agrodome,
                sim::structures::StructureType::TokamakReactor,
                sim::structures::StructureType::VehicleFactory,
            ];
            let st = structure_types[count % structure_types.len()];

            // Compute coordinate adjacent to base
            let base_x = 20 + ((count as i32 * 3) % 15);
            let base_y = 25 + ((count as i32 * 3) / 15 * 3);

            match g.colony.deploy_structure(st, base_x, base_y) {
                Ok(msg) => {
                    g.audio.play_build_confirm();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some(&msg));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                    }
                }
                Err(err) => {
                    g.audio.play_ui_beep();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some(&err));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_build.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_units) = document.get_element_by_id("btn-units") {
        let app_clone = app.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            g.audio.play_unit_ack();
            let next_id = g.colony.units.len() as u32 + 1;
            let (name, hp, weapon) = if next_id % 2 == 0 {
                ("Panther Battle Tank (Railgun)", 240, Some(sim::combat::WeaponType::Railgun))
            } else {
                ("Lynx Combat Scout (Laser)", 140, Some(sim::combat::WeaponType::Laser))
            };

            g.colony.units.push(sim::Unit {
                id: next_id,
                name: format!("{} #{}", name, next_id),
                faction: sim::Faction::Eden,
                tile_x: 21 + (next_id as i32) % 6,
                tile_y: 25 + (next_id as i32) % 6,
                hp,
                max_hp: hp,
                is_selected: false,
                current_order: sim::pathfinding::UnitOrder::Idle,
                path: Vec::new(),
                move_cooldown: 0.0,
                weapon,
                attack_cooldown: 0.0,
            });
        }) as Box<dyn FnMut(_)>);
        btn_units.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_research) = document.get_element_by_id("btn-research") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            
            // Find next unresearched tech
            let next_tech_id = g.colony.tech_tree.techs.iter()
                .find(|t| !t.is_researched)
                .map(|t| t.id);

            if let Some(id) = next_tech_id {
                let mut ore = g.colony.common_ore;
                let res = g.colony.tech_tree.research(id, &mut ore);
                g.colony.common_ore = ore;
                match res {
                    Ok(msg) => {
                        g.audio.play_build_confirm();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&msg));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                        }
                    }
                    Err(err) => {
                        g.audio.play_ui_beep();
                        if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                            toast.set_text_content(Some(&err));
                            let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #ef4444; display: block;");
                        }
                    }
                }
            } else {
                g.audio.play_ui_beep();
                if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                    toast.set_text_content(Some("All primary technologies researched!"));
                    let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #38bdf8; display: block;");
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn_research.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    if let Some(btn_save) = document.get_element_by_id("btn-save") {
        let app_clone = app.clone();
        let doc_clone = document.clone();
        let window_clone = window.clone();
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let mut g = app_clone.borrow_mut();
            let _ = g.audio.init();
            if let Ok(Some(storage)) = window_clone.local_storage() {
                let save_obj = g.colony.to_save_data();
                if let Ok(json) = serde_json::to_string(&save_obj) {
                    let _ = storage.set_item("op2_colony_save", &json);
                    g.audio.play_build_confirm();
                    if let Some(toast) = doc_clone.get_element_by_id("research-toast") {
                        toast.set_text_content(Some("Colony progress saved to browser storage!"));
                        let _ = toast.set_attribute("style", "font-size: 11px; font-weight: 500; color: #4ade80; display: block;");
                    }
                }
            }
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
                if let Ok(Some(json)) = storage.get_item("op2_colony_save") {
                    if let Ok(save_obj) = serde_json::from_str::<sim::SaveGameData>(&json) {
                        g.colony.apply_save_data(save_obj);
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

        // Resize canvas to match display window
        let client_w = canvas.client_width() as u32;
        let client_h = canvas.client_height() as u32;
        if canvas.width() != client_w || canvas.height() != client_h {
            canvas.set_width(client_w);
            canvas.set_height(client_h);
        }

        let mini_w = minimap_canvas.client_width() as u32;
        let mini_h = minimap_canvas.client_height() as u32;
        if minimap_canvas.width() != mini_w || minimap_canvas.height() != mini_h {
            minimap_canvas.set_width(mini_w);
            minimap_canvas.set_height(mini_h);
        }

        // Update simulation
        let disaster_warning = app_borrow.colony.update(dt.min(0.1));

        if let Some(warning) = disaster_warning {
            if let Some(toast) = document.get_element_by_id("research-toast") {
                toast.set_text_content(Some(warning));
                let _ = toast.set_attribute("style", "font-size: 11px; font-weight: bold; color: #f97316; display: block;");
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

        // Render game viewport
        app_borrow.viewport.render(
            &ctx,
            &app_borrow.colony,
            canvas.width() as f64,
            canvas.height() as f64,
        );

        // Render tactical radar
        app_borrow.viewport.render_minimap(
            &minimap_ctx,
            &app_borrow.colony,
            minimap_canvas.width() as f64,
            minimap_canvas.height() as f64,
        );

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
