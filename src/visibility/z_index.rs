use bevy::prelude::*;

pub(crate) struct Plugin;

const FRUSTUM_SCALING: f32 = 0.000_000_01;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<ZIndex>();
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct ZIndex(f32);

impl ZIndex {
    pub(crate) fn new(z_index: f32) -> ZIndex {
        ZIndex(z_index)
    }
}

fn layer(mut query: Query<(&ZIndex, &mut Transform)>) {
    for (z_index, mut transform) in query.iter_mut() {
        transform.translation.z = -FRUSTUM_SCALING * transform.translation.y + z_index.0;
    }
}
