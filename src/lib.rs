#[macro_use]
pub mod log;
mod utils;

use color_convert::color::Color;
use image::{ColorType, DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct RgbColor<T> {
    r: T,
    g: T,
    b: T,
}

impl RgbColor<u8> {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&format!("rgb({},{},{})", self.r, self.g, self.b));
        buffer
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let img = match image::load_from_memory(_array) {
        Ok(img) => img,
        Err(error) => {
            log!("There was a problem opening the file: {:?}", error);
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    return img;
}

fn _get_image_as_array(_img: DynamicImage) -> Vec<u8> {
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());

    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            log!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            );
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    log!("Sends array back");
    return out;
}

// color_type: hex, cmyk, rgb, hsl
// default color_type: hex
#[wasm_bindgen]
pub fn get_color_palette(_array: &[u8], color_type: Option<String>) -> JsValue {
    utils::set_panic_hook();
    let img = load_image_from_array(_array);
    let has_alpha = match img.color() {
        ColorType::Rgba8 => true,
        ColorType::Bgra8 => true,
        _ => false,
    };

    // throw if detect image with alpha
    if has_alpha {
        log!("doesn't support image with alpha yet!");
        panic!("doesn't support image with alpha yet!")
    }
    let color_type = match color_type {
        Some(color_type) => color_type,
        _ => "hex".to_string(),
    };

    let colors = dominant_color::get_colors_with_config(&img.to_bytes(), false, 244.0 * 244.0, 0.0);
    let mut color_strings: Vec<String> = Vec::new();
    for n in (2..colors.len()).step_by(3) {
        let rgb = RgbColor {
            r: colors[n - 2],
            g: colors[n - 1],
            b: colors[n],
        };
        let rgb_string = rgb.to_string().to_owned();
        let color = Color::new(rgb_string.as_str());

        match color_type.to_lowercase().as_str() {
            "hex" => {
                color_strings.push(color.to_hex().unwrap());
            }
            "cmyk" => {
                color_strings.push(color.to_cmyk().unwrap());
            }
            "rgb" => {
                color_strings.push(color.to_rgb().unwrap());
            }
            "hsl" => {
                color_strings.push(color.to_hsl().unwrap());
            }
            _ => {
                color_strings.push(color.to_hex().unwrap());
            }
        };
    }
    JsValue::from_serde(&color_strings).unwrap()
}
