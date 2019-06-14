#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate user32;
#[cfg(windows)] extern crate kernel32;

use winapi::um::winuser::*;
use winapi::shared::windef::*;
use std::num::Wrapping;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

static COLOR_ENTRIES: [(&'static str, Color); 14] = [
    ("Black", Color {r: 0, g: 0, b: 0}),
    ("DarkGrey", Color {r: 90, g: 90, b: 90}),
    ("Grey", Color {r: 154, g: 154, b: 154}),
    ("LightGrey", Color {r: 192, g: 192, b: 192}),
    ("White", Color {r: 255, g: 255, b: 255}),
    ("Brown", Color {r: 90, g: 51, b: 13}),
    ("Red", Color {r: 255, g: 0, b: 0}),
    ("Orange", Color {r: 255, g: 102, b: 255}),
    ("Yellow", Color {r: 255, g: 255, b: 0}),
    ("Green", Color {r: 0, g: 255, b: 0}),
    ("Blue", Color {r: 0, g: 0, b: 255}),
    ("Violet", Color {r: 238, g: 128, b: 238}),
    ("Magenta", Color {r: 255, g: 0, b: 255}),
    ("Cyan", Color {r: 0, g: 255, b: 255}),
];

impl Color {
    pub fn color_by_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {r, g, b}
    }
    pub fn color_by_real(mut r: f64, mut g: f64, mut b: f64) -> Self {
        if r > 1.0 {r = 0.999;}
        else if r < 0.0 {r = 0.0;}
        if g > 1.0 {g = 0.999;}
        else if g < 0.0 {g = 0.0;}
        if b > 1.0 {b = 0.999;}
        else if b < 0.0 {b = 0.0;}
        Color {r: (r * 256.0) as u8,
               g: (g * 256.0) as u8,
               b: (b * 256.0) as u8,}
    }
    pub fn color_by_name(name: &str) -> Self {
        for i in COLOR_ENTRIES.iter() {
            if ((*i).0) == name {
                return i.1;
            }
        }
        Self::color_by_rgb(0, 0, 0)
    }
    pub fn color_by_hsl(h: i32, s: i32, l: i32) -> Self {
        let (mh, ms, ml) = (h as f64 / 360.0, s as f64 / 256.0, l as f64 / 256.0);
        let q = if ml < 0.5 {
            ml * (1.0 + ms)
        } else {
            ml + ms - ml * ms
        };
        let p = 2.0 * ml - q;
        let mut tc = [mh + 1.0 / 3.0, mh, mh - 1.0 / 3.0];
        let mut res: [f64; 3] = [0.0, 0.0, 0.0];
        for i in 0..3 {
            if tc[i] < 0.0 {tc[i] += 1.0}
            if tc[i] > 1.0 {tc[i] -= 1.0}
            res[i] = match tc[i] {
                n if n < 1.0 / 6.0 => p + ((q - p) * 6.0 * n),
                n if n < 1.0 / 2.0 => q,
                n if n < 2.0 / 3.0 => p + ((q - p) * 6.0 * (2.0 / 3.0 - n)),
                _ => p
            };
        }
        Color::color_by_real(res[0], res[1], res[2])
    }
    pub fn color_by_yuv(y: u32, mut u: u32, mut v: u32) -> Self {
        u -= 128;
        v -= 128;
        let mut res = [(y as f64 + 1.403 * v as f64) as u32,
            (y as f64 - 0.343 * u as f64 - 0.714 * v as f64) as u32,
            (y as f64 + 1.77 * u as f64) as u32];
        for i in res.iter_mut() {
            if *i > 255 {*i = 255;}
        }
        Color::color_by_rgb(res[0] as u8, res[1] as u8, res[2] as u8)
    }
}



