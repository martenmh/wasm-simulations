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

pub struct Parameters {
    // Image size
    pub width: u32,
    pub height: u32,
    
    // Fractal position
    pub x_offset: f64,
    pub y_offset: f64,
    pub scale: f64,

    // Generation parameters
    pub max_iterations: u32,
    pub resolution: f64,
}

pub fn generate_set(fractal: &dyn Fractal, palette: &dyn ColorPalette, param: Parameters, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();
    let width = (param.width as f64 * param.resolution) as u32;
    let height = (param.height as f64 * param.resolution) as u32;

    for x in 0..width {
        for y in 0..height {
            
            let z = Complex {
                real: (y as f64 / param.resolution * param.scale) - param.y_offset,
                imaginary: (x as f64 / param.resolution * param.scale) - param.x_offset,
            };
            let iter_index  = fractal.get_iterations(z, c, param.max_iterations);
            let color       = palette.color(iter_index, param.max_iterations);

            data.push(color[0]);
            data.push(color[1]);
            data.push(color[2]);
            data.push(color[3]);
        }
    }
    return data
}
