use prelude::*;
use std::f32;

pub trait Clamp {
    fn clamp(&self, min: Self, max: Self) -> Self;
    fn saturate(&self) -> Self;
    fn fix_nan(&self) -> Self;
}

impl Clamp for f32 {
    fn clamp(&self, min: f32, max: f32) -> f32 {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }

    fn saturate(&self) -> f32 {
        self.clamp(0.0, 1.0)
    }

    fn fix_nan(&self) -> f32 {
        if *self == f32::NAN { 0.0 } else { *self } 
    }
}

impl Clamp for Rgb {
    fn clamp(&self, min: Rgb, max: Rgb) -> Rgb {
        Rgb::new(self.red.clamp(min.red, max.red),
                 self.green.clamp(min.green, max.green),
                 self.blue.clamp(min.blue, max.blue))
    }

    fn saturate(&self) -> Rgb {
        self.clamp(Rgb::new(0.0, 0.0, 0.0), Rgb::new(1.0, 1.0, 1.0))
    }

    fn fix_nan(&self) -> Rgb {
        Rgb::new(self.red.fix_nan(), self.green.fix_nan(), self.blue.fix_nan())
    }
}
