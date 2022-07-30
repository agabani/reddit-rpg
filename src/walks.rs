use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    pub(crate) strength: f32,
    pub(crate) walking: bool,
}

#[allow(clippy::needless_pass_by_value)]
fn walk(mut query: Query<(&Walks, &faces::Faces, &mut ExternalImpulse)>) {
    for (walks, faces, mut impulse) in query.iter_mut() {
        if walks.walking {
            match faces.direction {
                faces::Direction::Down => impulse.impulse = Vec2::new(0.0, -walks.strength),
                faces::Direction::Left => impulse.impulse = Vec2::new(-walks.strength, 0.0),
                faces::Direction::Right => impulse.impulse = Vec2::new(walks.strength, 0.0),
                faces::Direction::Up => impulse.impulse = Vec2::new(0.0, walks.strength),
            }
        }
    }
}
