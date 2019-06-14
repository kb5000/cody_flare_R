#![windows_subsystem = "windows"]
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate user32;
#[cfg(windows)] extern crate kernel32;

use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;
use std::io::Error;
use std::mem;
use super::super::utility::pos::Pos;

static mut WINDOW_SIZE: [i32; 2] = [0, 0];
static mut WINDOW_DSIZE: Pos = Pos {x: 10.0, y: 7.0};


#[cfg(windows)]
fn win_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(windows)]
pub struct Window {
    handle: winapi::shared::windef::HWND,
}

#[cfg(windows)]
pub fn init_display(name: &str, title: &str) -> Result<Window, Error> {
    let name = win_str(name);
    let title = win_str(title);
    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
        let wcs = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hInstance: hinstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: name.as_ptr(),
            lpszMenuName: null_mut(),
            hbrBackground: null_mut(),
        };
        RegisterClassW(&wcs);

        let handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut()
        );
        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Window {handle})
        }
    }
}

#[cfg(windows)]
pub fn message_loop(window: &mut Window) -> bool {
    unsafe {
        let mut msg: MSG = mem::uninitialized();
        if GetMessageW(&mut msg as *mut MSG, window.handle, 0, 0) < 0 {
            return false;
        }
        match msg {

            _ => {
                TranslateMessage(&msg as *const MSG);
                DispatchMessageW(&msg as *const MSG);
            }
        }
        true
    }
}

#[cfg(windows)]
pub fn clear_display() {
    //let rect: windef::RECT = windef::RECT
}