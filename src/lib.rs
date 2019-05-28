use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod draw_utils;
mod renderers;
mod termion;

fn get_frame(width: u32, height: u32) -> image::RgbaImage {
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

    image::RgbaImage::from_raw(width, height, data.to_vec()).unwrap()
}

#[wasm_bindgen]
pub fn render_blocks(width: u32, height: u32, ansi: bool, blend: bool, style: u32) -> String {
    let frame = get_frame(width, height);

    renderers::block::still(image::DynamicImage::ImageRgba8(frame), ansi, blend, style)
}

#[wasm_bindgen]
pub fn render_braille(width: u32, height: u32, ansi: bool) -> String {
    let frame = get_frame(width, height);

    renderers::braille::still(image::DynamicImage::ImageRgba8(frame), ansi)
}
