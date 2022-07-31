use bevy::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<AnimationIndex>()
                .register_type::<AnimationTimer>();
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct AnimationIndex {
    range: std::ops::Range<usize>,
}

#[derive(Component, Deref, DerefMut)]
#[cfg_attr(feature = "editor", derive(Default, Reflect), reflect(Component))]
pub(crate) struct AnimationTimer(Timer);

impl AnimationIndex {
    pub(crate) fn new(index: usize, length: usize) -> AnimationIndex {
        AnimationIndex {
            range: index..(index + length),
        }
    }

    pub(crate) fn change(&mut self, index: usize, length: usize) {
        self.range = index..(index + length);
    }

    fn next(&self, index: usize) -> usize {
        let next = index + 1;

        if self.range.contains(&next) {
            next
        } else {
            self.range.start
        }
    }

    pub(crate) fn start(&self) -> usize {
        self.range.start
    }
}

impl AnimationTimer {
    pub(crate) fn new(timer: Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }

    pub(crate) fn duration(&self) -> f32 {
        self.0.duration().as_secs_f32()
    }
}

#[allow(clippy::needless_pass_by_value)]
fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationIndex,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, index, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = index.next(sprite.index);
        }
    }
}
