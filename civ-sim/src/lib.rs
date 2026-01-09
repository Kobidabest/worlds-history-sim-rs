use wasm_bindgen::prelude::*;

pub mod world;
pub mod civilization;
pub mod culture;
pub mod religion;
pub mod technology;
pub mod politics;
pub mod military;
pub mod economy;
pub mod simulation;
pub mod history;
pub mod names;
pub mod ui;
pub mod utils;

use simulation::Simulation;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
pub(crate) use console_log;

thread_local! {
    static SIMULATION: RefCell<Option<Simulation>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn init_simulation(width: u32, height: u32, seed: u32) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();

    console_log!("Initializing civilization simulation...");
    console_log!("World size: {}x{}, Seed: {}", width, height, seed);

    let sim = Simulation::new(width, height, seed);

    SIMULATION.with(|s| {
        *s.borrow_mut() = Some(sim);
    });

    console_log!("Simulation initialized successfully!");
    Ok(())
}

#[wasm_bindgen]
pub fn advance_simulation(years: u32) -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let mut sim_ref = s.borrow_mut();
        if let Some(ref mut sim) = *sim_ref {
            for _ in 0..years {
                sim.tick();
            }
            Ok(sim.get_state_json())
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_world_data() -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.get_world_json())
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_civilizations_data() -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.get_civilizations_json())
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_history_data() -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.get_history_json())
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_tile_info(x: u32, y: u32) -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.get_tile_info_json(x, y))
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_current_year() -> Result<i32, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.current_year)
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}

#[wasm_bindgen]
pub fn get_statistics() -> Result<String, JsValue> {
    SIMULATION.with(|s| {
        let sim_ref = s.borrow();
        if let Some(ref sim) = *sim_ref {
            Ok(sim.get_statistics_json())
        } else {
            Err(JsValue::from_str("Simulation not initialized"))
        }
    })
}
