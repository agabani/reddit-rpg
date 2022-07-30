use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(48.0));

        #[cfg(feature = "editor")]
        {
            app.add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}
