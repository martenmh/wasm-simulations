mod utils;
mod complex;
mod fractal;
mod julia;
mod mandelbrot;

use wasm_bindgen::prelude::*;
use complex::Complex;
use crate::julia::Julia; 
use crate::fractal::GrayScale; 
use crate::fractal::Parameters; 
use crate::fractal::generate_set;

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
pub fn get_set(height: u32, width: u32, x_offset: f64, y_offset: f64, scale: f64, max_iterations: u32, resolution: f64, real: f64, imaginary: f64) -> Vec<u8> {
    let c = Complex { real, imaginary };
    let julia_set = Julia {};
    let grayscale = GrayScale {};

    let parameters = Parameters {
        height, width, x_offset, y_offset, scale, max_iterations, resolution
    };
    return generate_set(&julia_set, &grayscale, parameters, c);
}

#[wasm_bindgen]
pub fn greet() {
    //alert("Hello, lib!");
    //console_log!("Hello, console!");
}


