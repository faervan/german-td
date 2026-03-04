use crate::{
    assets::{AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_asset::<EnemyDefinition>();
        app.register_asset_loader(RonAssetLoader::<EnemyAsset>::new());
        app.load_folder("enemies");

        app.init_library::<EnemyDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct EnemyAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
    pub attack_duration_ms: u64,
    pub walk_speed: f32,
    pub health: f32,
    pub drop: f32,
    // animations
    pub idle_animation: Option<String>,
    pub walk_animation: Option<String>,
    pub attack_animation: Option<String>,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct EnemyDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub damage: f32,
    pub attack_duration: Duration,
    pub walk_speed: f32,
    pub health: f32,
    pub drop: f32,
    // animations
    pub idle_animation: Option<Result<AnimationNodeIndex, String>>,
    pub walk_animation: Option<Result<AnimationNodeIndex, String>>,
    pub attack_animation: Option<Result<AnimationNodeIndex, String>>,
    pub graph: Option<Handle<AnimationGraph>>,
}

impl RonAsset for EnemyAsset {
    type Asset = EnemyDefinition;
    const EXTENSION: &str = "enemy";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        EnemyDefinition {
            name: self.name,
            gltf: context.load(self.gltf),
            scene: Default::default(),
            icon: context.load(self.icon),
            damage: self.damage,
            attack_duration: Duration::from_millis(self.attack_duration_ms),
            walk_speed: self.walk_speed,
            health: self.health,
            drop: self.drop,
            // animations
            idle_animation: self.idle_animation.map(Err),
            walk_animation: self.walk_animation.map(Err),
            attack_animation: self.attack_animation.map(Err),
            graph: None,
        }
    }
}

impl AssetNameExt for EnemyDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl AssetLoadedHook for EnemyDefinition {
    fn on_loaded_hook(&mut self, world: &mut World) {
        let gltf = world.resource::<Assets<Gltf>>().get(&self.gltf).unwrap();
        self.scene = gltf.default_scene.clone().expect("Missing default scene");

        let mut named_clips = HashMap::new();
        for animation in [
            &self.idle_animation,
            &self.walk_animation,
            &self.attack_animation,
        ] {
            if let Some(Err(name)) = animation {
                named_clips.insert(name.clone(), gltf.named_animations[name.as_str()].clone());
            }
        }

        if !named_clips.is_empty() {
            let (names, clips): (Vec<_>, Vec<_>) = named_clips.into_iter().unzip();
            let (graph, indices) = AnimationGraph::from_clips(clips);

            let asset_server = world.resource::<AssetServer>();
            self.graph = Some(asset_server.add(graph));

            for (name, index) in names.into_iter().zip(indices) {
                match name.as_str() {
                    "Idle" => self.idle_animation = Some(Ok(index)),
                    "Walk" => self.walk_animation = Some(Ok(index)),
                    "Attack" => self.attack_animation = Some(Ok(index)),
                    _ => warn!("Unknown animation name: {name}"),
                }
            }
        }

        if let Some(scene_world) = world
            .resource_mut::<Assets<Scene>>()
            .get_mut(&self.scene)
            .map(|scene| &mut scene.world)
            && let Some(graph) = &self.graph
        {
            let mut query = scene_world.query_filtered::<Entity, With<AnimationPlayer>>();
            let animation_players = query.iter(scene_world).collect::<Vec<_>>();

            #[cfg(feature = "dev_native")]
            if animation_players.len() != 1 {
                warn!(
                    "Gltf scene of {} has {} AnimationPlayers, but exactly 1 was expected",
                    self.name,
                    animation_players.len()
                );
            }

            for animation_player in animation_players {
                scene_world.commands().entity(animation_player).insert((
                    AnimationGraphHandle(graph.clone()),
                    AnimationTransitions::new(),
                ));
            }

            scene_world.flush();
        }
    }
}
