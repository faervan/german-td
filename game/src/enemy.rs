use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, enemy_ai);
}

fn enemy_ai(
    mut enemies: Query<(&mut LinearVelocity, &Transform), With<Enemy>>,
    mut right: Local<bool>,
) {
    for (mut velocity, transform) in &mut enemies {
        let dir = if *right { 1.0 } else { -1.0 };

        velocity.0.x = dir * 25.0;

        if transform.translation.x < -50.0 {
            *right = true;
        }
        if transform.translation.x > 50.0 {
            *right = false;
        }
    }
}
