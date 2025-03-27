// rust
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::gamepad_values::GamepadValues1;
use crate::hardware::{read_button, read_slider, read_slider_middle};
use crate::hidimpl::gatt_application::GattApplication;

pub struct GamepadUpdater {
    gamepad_def: Arc<Mutex<GamepadValues1>>,
    app: Arc<Mutex<GattApplication>>,
    poll_interval: Duration,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl GamepadUpdater {
    /// Creates a new updater.
    pub fn new(
        gamepad_def: Arc<Mutex<GamepadValues1>>,
        app: Arc<Mutex<GattApplication>>,
        poll_interval: Duration,
    ) -> Self {
        Self {
            gamepad_def,
            app,
            poll_interval,
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    /// Starts the background polling thread.
    pub fn start(&mut self) {
        println!("GamepadUpdater start...");
        
        if !self.running.load(Ordering::SeqCst) {
            self.running.store(true, Ordering::SeqCst);
            let poll_interval = self.poll_interval;
            let running = self.running.clone();
            let gamepad_def = self.gamepad_def.clone();
            let app = self.app.clone();
            self.thread = Some(thread::spawn(move || {
                while running.load(Ordering::SeqCst) {
                    if update_gamepad_controls(&gamepad_def) {
                        app.lock().unwrap().notify_hid_report();
                    }
                    thread::sleep(poll_interval);
            }
            }));
            
            println!("GamepadUpdater started");
        } else {
            println!("GamepadUpdater already running");
        }
    }

    /// Stops the background polling thread.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread.take() {
            handle.join().expect("Failed to join thread");
        }
    }
}

/// Polls hardware and updates GamepadValues1.
/// Returns true if any control has changed.
fn update_gamepad_controls(gamepad_def: &Arc<Mutex<GamepadValues1>>) -> bool {
    let mut changed = false;
    let mut gp = gamepad_def.lock().unwrap();

    let new_slider0 = read_slider(0).unwrap();
    if gp.slider0 != (new_slider0 & 0xFF) {
        gp.set_slider0(new_slider0);
        changed = true;
    }
    let new_axis_x0 = read_slider_middle(1).unwrap();
    if gp.axis_x0 != (new_axis_x0 & 0xFF) {
        gp.set_axis_x0(new_axis_x0);
        changed = true;
    }
    let new_axis_y0 = read_slider(2).unwrap();
    if gp.axis_y0 != (new_axis_y0 & 0xFF) {
        gp.set_axis_y0(new_axis_y0);
        changed = true;
    }
    let new_axis_z0 = read_slider(3).unwrap();
    if gp.axis_z0 != (new_axis_z0 & 0xFF) {
        gp.set_axis_z0(new_axis_z0);
        changed = true;
    }
    let new_axis_rx0 = read_slider(4).unwrap();
    if gp.axis_rx0 != (new_axis_rx0 & 0xFF) {
        gp.set_axis_rx0(new_axis_rx0);
        changed = true;
    }
    let new_axis_ry0 = read_slider(5).unwrap();
    if gp.axis_ry0 != (new_axis_ry0 & 0xFF) {
        gp.set_axis_ry0(new_axis_ry0);
        changed = true;
    }
    let new_axis_rz0 = read_slider(6).unwrap();
    if gp.axis_rz0 != (new_axis_rz0 & 0xFF) {
        gp.set_axis_rz0(new_axis_rz0);
        changed = true;
    }
    let new_axis_vx0 = read_slider(7).unwrap();
    if gp.axis_vx0 != (new_axis_vx0 & 0xFF) {
        gp.set_axis_vx0(new_axis_vx0);
        changed = true;
    }

    let new_btn10 = read_button(0).unwrap();
    if gp.btn10 != (new_btn10) {
        gp.set_btn10(new_btn10);
        changed = true;
    }
    let new_btn11 = read_button(1).unwrap();
    if gp.btn11 != (new_btn11) {
        gp.set_btn11(new_btn11);
        changed = true;
    }
    let new_btn12 = read_button(2).unwrap();
    if gp.btn12 != (new_btn12) {
        gp.set_btn12(new_btn12);
        changed = true;
    }
    let new_btn13 = read_button(3).unwrap();
    if gp.btn13 != (new_btn13) {
        gp.set_btn13(new_btn13);
        changed = true;
    }
    let new_btn14 = read_button(4).unwrap();
    if gp.btn14 != (new_btn14) {
        gp.set_btn14(new_btn14);
        changed = true;
    }
    let new_btn15 = read_button(5).unwrap();
    if gp.btn15 != (new_btn15) {
        gp.set_btn15(new_btn15);
        changed = true;
    }
    let new_btn16 = read_button(6).unwrap();
    if gp.btn16 != (new_btn16) {
        gp.set_btn16(new_btn16);
        changed = true;
    }
    let new_btn17 = read_button(7).unwrap();
    if gp.btn17 != (new_btn17) {
        gp.set_btn17(new_btn17);
        changed = true;
    }

    changed
}