use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerHealth { health: 20 });

    app.add_systems(Update, decrease_health.run_if(in_state(AppState::Game)));
}

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
struct PlayerHealth {
    health: usize,
}

fn decrease_health(
    mut goal_reached: MessageReader<EnemyReachedGoal>,
    mut health: ResMut<PlayerHealth>,
    mut commands: Commands,
) {
    for enemy in goal_reached.read() {
        if **health > 0 {
            **health -= 1;
        } else {
            **health = 10;
            info!(
                "All health lost! Well, you got another chance with {} new health points!",
                **health
            );
        }
        debug!(
            "Enemy {} reached the goal! New health: {}",
            enemy.entity, **health
        );
        commands
            .entity(enemy.entity)
            .insert(DespawnAfter::new(Duration::from_secs(5)));
    }
}
