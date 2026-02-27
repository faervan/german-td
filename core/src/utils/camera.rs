use bevy::window::{CursorGrabMode, CursorOptions};

use crate::prelude::*;

pub fn grab_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn ungrab_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
