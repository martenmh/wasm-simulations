mod utils;
mod complex;
mod mandelbrot;

use wasm_bindgen::prelude::*;
use complex::Complex;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn get_set(width: u32, height: u32, real: f64, imaginary: f64) -> Vec<u8> {
    let c = Complex { real, imaginary };
    return mandelbrot::get_julia_set(width, height, c);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, lib!");
    console_log!("Hello, console!");
}


