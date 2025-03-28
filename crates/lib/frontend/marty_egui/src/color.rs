/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2025 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

    --------------------------------------------------------------------------

    egui::color.rs

    Miscellaneous color manipulation routines.
*/

use egui::Color32;

pub const STATUS_UPDATE_COLOR: Color32 = Color32::from_rgb(0, 255, 255);

pub fn darken_c32(color: Color32, percent: f32) -> Color32 {
    let (r, g, b, _) = color.to_tuple();

    let dr = ((r as f32) * (1.0 - percent)) as u8;
    let dg = ((g as f32) * (1.0 - percent)) as u8;
    let db = ((b as f32) * (1.0 - percent)) as u8;

    egui::Color32::from_rgb(dr, dg, db)
}

pub fn lighten_c32(color: Color32, percent: f32) -> Color32 {
    let (r, g, b, _) = color.to_tuple();

    let dr = r.saturating_add(((r as f32) * percent) as u8);
    let dg = g.saturating_add(((g as f32) * percent) as u8);
    let db = b.saturating_add(((b as f32) * percent) as u8);

    egui::Color32::from_rgb(dr, dg, db)
}

pub fn add_c32(color: Color32, amount: u8) -> Color32 {
    let (r, g, b, _) = color.to_tuple();

    let dr = r.saturating_add(amount);
    let dg = g.saturating_add(amount);
    let db = b.saturating_add(amount);

    egui::Color32::from_rgb(dr, dg, db)
}

pub fn fade_c32(color1: Color32, color2: Color32, amount: u8) -> Color32 {
    let c1r: f32 = (color1.r() as f32) / 255.0;
    let c1g: f32 = (color1.g() as f32) / 255.0;
    let c1b: f32 = (color1.b() as f32) / 255.0;

    let c2r: f32 = (color2.r() as f32) / 255.0;
    let c2g: f32 = (color2.g() as f32) / 255.0;
    let c2b: f32 = (color2.b() as f32) / 255.0;

    let percent: f32 = (amount as f32) / 255.0;

    let result_r = c1r + (percent * (c2r - c1r));
    let result_g = c1g + (percent * (c2g - c1g));
    let result_b = c1b + (percent * (c2b - c1b));

    Color32::from_rgb(
        (result_r * 255.0) as u8,
        (result_g * 255.0) as u8,
        (result_b * 255.0) as u8,
    )
}

#[allow(dead_code)]
pub fn hex_to_rgb(hex: u32) -> (u8, u8, u8) {
    (
        ((hex >> 16) & 0xFF) as u8,
        ((hex >> 8) & 0xFF) as u8,
        (hex & 0xFF) as u8,
    )
}

pub fn hex_to_c32(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xFF) as u8,
        ((hex >> 8) & 0xFF) as u8,
        (hex & 0xFF) as u8,
    )
}
