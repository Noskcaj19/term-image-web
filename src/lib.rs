use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod block;
mod draw_utils;
mod termion;

#[wasm_bindgen]
pub fn render_blocks(width: u32, height: u32, ansi: bool, blend: bool, extended: bool) -> String {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let data = context
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap()
        .data();

    let source = image::RgbaImage::from_raw(width, height, data.to_vec()).unwrap();

    block::still(
        image::DynamicImage::ImageRgba8(source),
        ansi,
        blend,
        extended,
    )
}
