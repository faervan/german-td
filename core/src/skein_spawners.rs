use bevy::light::NotShadowCaster;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Transform>();
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component, Default)]
#[component(on_add)]
struct InsertLight {
    intensity: f32,
    color: Color,
    range: f32,
}

impl InsertLight {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let light = world.get::<InsertLight>(hook.entity).unwrap().clone();

        world.commands().entity(hook.entity).insert((
            PointLight {
                intensity: light.intensity,
                color: light.color,
                range: light.range,
                shadows_enabled: true,
                ..Default::default()
            },
            DelayedChildrenDespawn,
        ));
    }
}

#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component, Default)]
#[component(on_add)]
struct SetChildNonShadowCaster;

impl SetChildNonShadowCaster {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let children = world.get::<Children>(hook.entity).unwrap().to_vec();

        for child in children {
            world.commands().entity(child).insert(NotShadowCaster);
        }
    }
}
