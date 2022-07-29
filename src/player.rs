use bevy::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Player>();
        }
    }
}

#[derive(Component, Default)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Player {
    #[cfg_attr(feature = "editor", inspectable(ignore))]
    keys: Input<KeyCode>,
}

impl Player {
    pub(crate) fn keys(&self) -> &Input<KeyCode> {
        &self.keys
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    let keys = keys.into_inner();

    for mut player in query.iter_mut() {
        player.keys = keys.clone();
    }
}
