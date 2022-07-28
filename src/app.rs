use bevy::prelude::*;

use crate::{camera, sprite, window};

pub fn run() {
    let mut app = App::new();

    app.insert_resource(window::descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::Plugin)
        .add_plugin(sprite::Plugin);

    #[cfg(feature = "editor")]
    {
        use crate::editor;
        app.add_plugin(editor::Plugin);
    }

    app.run();
}
