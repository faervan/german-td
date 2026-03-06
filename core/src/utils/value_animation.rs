use bevy::ecs::component::Mutable;

use crate::{prelude::*, utils::LinearlyInterpolatable};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, run_value_animations::<Transform>);
}

#[derive(Component)]
/// Gradually transform the value of a [`Component`] `T` over a timespan
pub struct AnimateValue<T>
where
    T: Component<Mutability = Mutable>,
{
    duration: Duration,
    /// Starts at 0., ends at 1.
    progress: f32,
    init: T,
    end: T,
    /// Takes progress, init and end values, as well as a mutable reference to the value that
    /// should be changed
    f: Box<dyn Fn(f32, &T, &T, &mut T) + Send + Sync>,
}

fn run_value_animations<T: Component<Mutability = Mutable>>(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<(Entity, &mut AnimateValue<T>, Option<&mut T>)>,
) {
    for (entity, mut animation, value_maybe) in query {
        let Some(mut value) = value_maybe else {
            warn!("Entity {entity} has an AnimateValue<T> component, but no matching component T");
            commands.entity(entity).remove::<AnimateValue<T>>();
            continue;
        };
        animation.progress += time.delta_secs() / animation.duration.as_secs_f32();
        if animation.progress > 1. {
            animation.progress = 1.;
            commands.entity(entity).remove::<AnimateValue<T>>();
        }
        (animation.f)(
            animation.progress,
            &animation.init,
            &animation.end,
            &mut value,
        );
    }
}

pub trait AnimateValueExt {
    /// Linearly interpolate the value of `T` towards `end` over the provided `duration`
    fn animate_towards<T: Component<Mutability = Mutable> + LinearlyInterpolatable + Clone>(
        &mut self,
        end: T,
        duration: Duration,
    ) -> &mut Self;
    /// Modify the value of `T` over the provided duration using the provided function `f`. The
    /// last call of `f` will be with a progress value of `1_f32`
    fn animate_value_with<
        T: Component<Mutability = Mutable> + Default,
        F: Fn(f32, &mut T) + Send + Sync + 'static,
    >(
        &mut self,
        f: F,
        duration: Duration,
    ) -> &mut Self;
}

impl AnimateValueExt for EntityCommands<'_> {
    fn animate_towards<T: Component<Mutability = Mutable> + LinearlyInterpolatable + Clone>(
        &mut self,
        end: T,
        duration: Duration,
    ) -> &mut Self {
        self.insert(InsertLinearAnimateValue { duration, end })
    }
    fn animate_value_with<
        T: Component<Mutability = Mutable> + Default,
        F: Fn(f32, &mut T) + Send + Sync + 'static,
    >(
        &mut self,
        f: F,
        duration: Duration,
    ) -> &mut Self {
        self.insert(AnimateValue {
            duration,
            progress: 0.,
            init: T::default(),
            end: T::default(),
            f: Box::new(move |progress, _init, _end, value| f(progress, value)),
        })
    }
}

#[derive(Component)]
#[component(on_add)]
/// Is inserted to trigger its `on_add` hook, removing this component and querying the current
/// value of `T` to insert the actual [`AnimateValue`] component
struct InsertLinearAnimateValue<T>
where
    T: Component<Mutability = Mutable> + LinearlyInterpolatable + Clone,
{
    duration: Duration,
    end: T,
}

impl<T: Component<Mutability = Mutable> + LinearlyInterpolatable + Clone>
    InsertLinearAnimateValue<T>
{
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let this = world.get::<Self>(hook.entity).unwrap();
        let Some(t) = world.get::<T>(hook.entity) else {
            warn!(
                "Tried to add AnimateValue for a value T that is not available on entity {}",
                hook.entity
            );
            return;
        };
        let animation = AnimateValue {
            duration: this.duration,
            progress: 0.,
            init: t.clone(),
            end: this.end.clone(),
            f: Box::new(|progress, init, end, value| {
                value.interpolate(progress, init, end);
            }),
        };
        world
            .commands()
            .entity(hook.entity)
            .remove::<Self>()
            .insert(animation);
    }
}
