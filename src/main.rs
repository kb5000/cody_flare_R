#![windows_subsystem = "windows"]
pub mod cody_flare;

use crate::cody_flare::graphics::window;


#[cfg(windows)]
fn main() {
    let mut window = window::init_display("CodyFlare", "CodyX").unwrap();
    loop {
        if !window::message_loop(&mut window) {
            break;
        }
    }
}
