use std::ops::Add;
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    // Also known as the absolute value/magnitude
    // Return the square root
    pub fn norm(&self) -> f64 {
        return (self.real * self.real) + (self.imaginary * self.imaginary);
    }

    pub fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        return Complex { real, imaginary };
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
