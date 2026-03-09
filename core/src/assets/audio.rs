use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_assets::<GameSoundHandles>();
}

#[derive(Resource, Asset, Reflect)]
#[reflect(Resource)]
pub struct GameSoundHandles {
    #[dependency]
    pub enemy_death: Vec<Handle<AudioSource>>,
}

impl GameSoundHandles {
    /// Will panic if `enemy_death` is empty
    pub fn enemy_death_from_time(&self, time: &Time) -> Handle<AudioSource> {
        let n = self.enemy_death.len();
        let sin = time.elapsed_secs().sin();
        let index = (sin * n as f32) as usize;
        self.enemy_death[index].clone()
    }
}

impl FromWorld for GameSoundHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            enemy_death: vec![
                asset_server.load("sounds/enemies/death1.ogg"),
                asset_server.load("sounds/enemies/death2.ogg"),
            ],
        }
    }
}
