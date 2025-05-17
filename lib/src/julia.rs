use crate::complex::Complex;
use crate::fractal::Fractal; 

pub struct Julia;

impl Fractal for Julia {
    fn get_iterations(&self, z: Complex, c: Complex, max_iterations: u32) -> u32 {
        let mut iter_index: u32 = 0;
        let mut z = z;
        while iter_index < max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            // z = z^2 + c
            z = z.square() + c;
            iter_index += 1;
        }
        return iter_index;
    }

    // get the set of complex numbers for a given point z
    // used for showing the points of iterations until it either hits max_iterations or is outside
    // of the set
    /*
    fn get_iteration_set(&self, z: Complex, c: Complex, max_iterations: u32) -> vec<Complex> {
        unimplemented!();
    }
    */
}


