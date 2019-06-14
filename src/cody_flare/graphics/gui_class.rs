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
//use std::intrinsics::unchecked_shl;
use self::winapi::shared::minwindef::DWORD;
use self::winapi::um::wingdi::GetDeviceCaps;
use self::winapi::um::wingdi::LOGPIXELSX;
use self::winapi::um::wingdi::LOGPIXELSY;
use std::ptr::null;
use std::ffi;
use self::winapi::um::wingdi::GetStockObject;
use self::winapi::um::wingdi::WHITE_BRUSH;
use self::winapi::shared::basetsd::UINT32;
use self::winapi::shared::minwindef::WPARAM;
use self::winapi::shared::minwindef::LPARAM;
use self::winapi::um::winnt::LONG;
use self::winapi::um::wingdi::CreateCompatibleDC;
use self::winapi::um::wingdi::CreateCompatibleBitmap;
use self::winapi::um::wingdi::SelectObject;
use winapi::ctypes::c_void;

const LEFT_MARGIN: i32 = 10;
const TOP_MARGIN: i32 = 0;

#[cfg(windows)]
pub struct Window {
    title: String,      //window title
    size: [i32; 2],     //window size
    resolution: Option<[i32; 2]>,    //screen size
    dsize: Pos,         //window size using double
    window: HWND,       //window handle
    osdc: HDC,          //buffer
    gdc: HDC,           //buffer
    os_bits: HBITMAP,   //bit map
}

#[cfg(windows)]
impl Window {
    pub fn init() -> Result<Self, &'static str> {
        unsafe {
            let mut res = Window {
                title: "Default Window".to_string(),
                size: [0, 0],
                resolution: None,
                dsize: Pos::new(0.0, 0.0),
                window: null_mut(),
                osdc: null_mut(),
                gdc: null_mut(),
                os_bits: null_mut(),
            };
            let mut wndcls: WNDCLASSW;
            let (mut bounds, graphics_rect): (RECT, RECT);
            let screen_size: Pos;
            let screen_space: Pos;
            let style: DWORD;
            let (top, mut dx, mut dy, c_width): (i32, i32, i32, i32);
            res.get_resolution();
            res.dsize = res.get_screen_size();
            graphics_rect = rect_from_size([LEFT_MARGIN, TOP_MARGIN],
                                           res.get_resolution());
            style = (WS_OVERLAPPEDWINDOW ^ WS_THICKFRAME) & !(WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
            let name = win_str("sd");
            let title = win_str("s");
            let hinstance = GetModuleHandleW(null_mut());
            wndcls = WNDCLASSW {
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
            RegisterClassW(&mut wndcls);
            res.window = CreateWindowExW(
                0,
                name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                graphics_rect.left,
                graphics_rect.top,
                rect_size(&graphics_rect)[0],
                rect_size(&graphics_rect)[1],
                null_mut(),
                null_mut(),
                hinstance,
                null_mut()
            );
            //panic!("yes");
            if res.window.is_null() {return Err("res.window is null");}
            bounds = RECT {left: 0, right: 0, top: 0, bottom: 0};
            GetClientRect(res.window, &mut bounds);
            dx = rect_size(&graphics_rect)[0] - rect_size(&bounds)[0];
            dy = rect_size(&graphics_rect)[1] - rect_size(&bounds)[1];
            SetWindowPos(res.window, HWND_TOP, graphics_rect.left,
                graphics_rect.top, rect_size(&graphics_rect)[0] + dx,
                         rect_size(&graphics_rect)[1] + dy, 0);
            res.gdc = GetDC(res.window);
            GetClientRect(res.window, &mut bounds);

            ShowWindow(res.window, SW_SHOWNORMAL);
            UpdateWindow(res.window);
            res.osdc = CreateCompatibleDC(res.gdc);
            if res.osdc.is_null() {return Err("osdc is null");}
            res.os_bits = CreateCompatibleBitmap(res.gdc, rect_size(&bounds)[0],
                    rect_size(&bounds)[1]);
            if res.os_bits.is_null() {return Err("os_bits is null");}
            SelectObject(res.osdc, res.os_bits as *mut c_void);
            Ok(res)
        }
        //unimplemented!()
    }
    pub fn get_resolution(&mut self) -> [i32; 2] {
        if let Some(res) = self.resolution {
            res
        } else {
            unsafe {
                let desktop: HWND = GetDesktopWindow();
                let dc: HDC = GetDC(desktop);
                let (xdpi, ydpi) = (GetDeviceCaps(dc, LOGPIXELSX),
                                              GetDeviceCaps(dc, LOGPIXELSY));
                ReleaseDC(desktop, dc);
                self.resolution = Some([xdpi, ydpi]);
                self.resolution.unwrap()
            }
        }
    }
    pub fn get_screen_size(&mut self) -> Pos {
        unsafe {
            let desktop: HWND = GetDesktopWindow();
            let mut bounds: RECT = RECT {left: 0, right: 0, top: 0, bottom: 0};
            GetWindowRect(desktop, &mut bounds);
            let bound_size = rect_size(&bounds);
            let resolution = self.get_resolution();
            Pos {
                x: bound_size[0] as f64 / resolution[0] as f64,
                y: bound_size[1] as f64 / resolution[1] as f64
            }
        }

    }
    pub fn set_windows_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    pub fn get_windows_title(&self) -> String {
        self.title.clone()
    }
}

#[cfg(windows)]
fn win_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub fn rect_size(rect: &RECT) -> [i32; 2] {
    [rect.right - rect.left, rect.bottom - rect.top]
}

pub fn rect_from_size(position: [i32; 2], size: [i32; 2]) -> RECT {
    let mut rect = RECT {left: 0, right: 0, top: 0, bottom: 0};
    unsafe {
        SetRect(&mut rect, position[0], position[1], size[0], size[1]);
    }
    rect
}


#[cfg(windows)]
#[no_mangle]
pub fn event_loop(window: &mut Window) -> bool {
    unsafe {
        let mut msg: MSG = mem::uninitialized();
        if GetMessageW(&mut msg as *mut MSG, window.window, 0, 0) < 0 {
            return false;
        }
        match msg.message {
            WM_ERASEBKGND => {  }
            WM_PAINT => {  }
            _ => { DefWindowProcW(msg.hwnd, msg.message, msg.wParam, msg.lParam); }
        };
        true
    }
}
