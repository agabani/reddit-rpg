use bevy::prelude::*;

use crate::{animation, camera, character, faces, mirror, player, walks, window};

pub fn run() {
    let mut app = App::new();

    app.insert_resource(window::descriptor())
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "editor")]
    {
        use crate::editor;
        app.add_plugin(editor::Plugin);
    }

    app.add_plugin(animation::Plugin)
        .add_plugin(camera::Plugin)
        .add_plugin(character::Plugin)
        .add_plugin(faces::Plugin)
        .add_plugin(mirror::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(walks::Plugin);

    app.run();
}
