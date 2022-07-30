use bevy::prelude::*;

use crate::faces;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(walk);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Walks>();
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Walks {
    pub(crate) speed: f32,
    pub(crate) walking: bool,
}

#[allow(clippy::needless_pass_by_value)]
fn walk(time: Res<Time>, mut query: Query<(&Walks, &faces::Faces, &mut Transform)>) {
    for (walks, faces, mut transform) in query.iter_mut() {
        if walks.walking {
            let distance = walks.speed * time.delta_seconds();

            match faces.direction {
                faces::Direction::Down => transform.translation.y -= distance,
                faces::Direction::Left => transform.translation.x -= distance,
                faces::Direction::Right => transform.translation.x += distance,
                faces::Direction::Up => transform.translation.y += distance,
            }
        }
    }
}
