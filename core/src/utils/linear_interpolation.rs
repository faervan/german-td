use crate::prelude::*;

pub trait LinearlyInterpolatable {
    fn interpolate(&mut self, progress: f32, init: &Self, end: &Self);
}

impl LinearlyInterpolatable for Transform {
    fn interpolate(&mut self, progress: f32, init: &Self, end: &Self) {
        if init.translation != end.translation {
            let diff = end.translation - init.translation;
            self.translation = init.translation + diff * progress;
        }
        if init.rotation != end.rotation {
            self.rotation = init.rotation.slerp(end.rotation, progress);
        }
        if init.scale != end.scale {
            let diff = end.scale - init.scale;
            self.scale = init.scale + diff * progress;
        }
    }
}
