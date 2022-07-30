use bevy::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Faces>()
                .register_inspectable::<Direction>();
        }
    }
}

#[derive(Copy, Clone, Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Faces {
    pub(crate) direction: Direction,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) enum Direction {
    Down,
    Left,
    Right,
    Up,
}
