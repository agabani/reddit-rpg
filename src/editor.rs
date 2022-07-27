use bevy_editor_pls::{controls::EditorControls, EditorPlugin};

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        fn editor_controls() -> EditorControls {
            EditorControls::default_bindings()
        }

        app.add_plugin(EditorPlugin)
            .insert_resource(editor_controls());
    }
}
