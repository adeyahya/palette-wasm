mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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

#[wasm_bindgen]
pub async fn get_from_js(url: String) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let url = "https://images.unsplash.com/photo-1595446472774-37c5fc18ce46?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1350&q=80";
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.array_buffer()?).await?;
    let image = js_sys::Uint8Array::new(&text);
    let image = image.to_vec();
    let colors = dominant_color::get_colors(&image, false);
    let mut rgb_colors: Vec<RgbColor<u8>> = Vec::new();
    for n in (2..colors.len()).step_by(3) {
        rgb_colors.push(RgbColor {
            r: colors[n - 2],
            g: colors[n - 1],
            b: colors[n],
        })
    }
    let rgb_colors: Vec<String> = rgb_colors
        .into_iter()
        .map(|color| color.to_string())
        .collect();
    Ok(JsValue::from_serde(&rgb_colors).unwrap())
}
