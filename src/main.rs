#![windows_subsystem = "windows"]
pub mod cody_flare;

use crate::cody_flare::graphics::window;
use crate::cody_flare::graphics::gui_class;

#[cfg(windows)]
fn main() {
//    let mut window = window::init_display("CodyFlare", "CodyX").unwrap();
    let mut window = gui_class::Window::init().unwrap();
    loop {
        if !gui_class::event_loop(&mut window) {
            break;
        }
    }
//    loop {
//        if !window::message_loop(&mut window) {
//            break;
//        }
//    }
}
