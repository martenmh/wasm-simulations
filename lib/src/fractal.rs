use crate::complex::Complex; 

pub trait Fractal {
    fn get_iterations(&self, z: Complex, c: Complex, max_iterations: u32) -> u32;
}

type Color = [u8; 4];

pub trait ColorPalette {
    fn color(&self, iterations: u32, max_iterations: u32) -> Color;
}

pub struct GrayScale;
//pub struct SmoothGradient;

impl ColorPalette for GrayScale {
    fn color(&self, iterations: u32, max_iterations: u32) -> Color {
        let intensity = ((1.0 - (iterations as f64 / max_iterations as f64)) * 255.0) as u8;
        return [intensity, intensity , intensity, 255];
    }
}


pub fn generate_set(fractal: &dyn Fractal, palette: &dyn ColorPalette, width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    // parameters
    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.002;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = fractal.get_iterations(z, c, 900);
            let color = palette.color(iter_index, 900);

            data.push(color[0]);
            data.push(color[1]);
            data.push(color[2]);
            data.push(color[3]);
        }
    }
    return data
}
