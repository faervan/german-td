use crate::prelude::*;

pub trait LinearlyInterpolatable {
    fn interpolate(&mut self, progress: f32, init: &Self, end: &Self);
}

impl LinearlyInterpolatable for Transform {
    fn interpolate(&mut self, progress: f32, init: &Self, end: &Self) {
        self.rotation = init.rotation.slerp(end.rotation, progress);
    }
}
