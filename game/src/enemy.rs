use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, enemy_ai);
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

fn enemy_ai(
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<Enemy>>,
    mut right: Local<bool>,
) {
    for mut transform in &mut transforms {
        let dir = if *right { 1.0 } else { -1.0 };

        transform.translation.x += dir * 25.0 * time.delta_secs();

        if transform.translation.x < -50.0 {
            *right = true;
        }
        if transform.translation.x > 50.0 {
            *right = false;
        }
    }
}
