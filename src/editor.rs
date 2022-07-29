use bevy::prelude::*;

use bevy_editor_pls::{
    controls::{Action, Binding, BindingCondition, Button, EditorControls, UserInput},
    EditorPlugin,
};

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EditorPlugin)
            .insert_resource(editor_controls());
    }
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();

    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::F12)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
