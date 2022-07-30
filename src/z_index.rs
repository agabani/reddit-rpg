use bevy::prelude::*;

pub(crate) struct Plugin;

const FRUSTUM_OFFSET: f32 = 500.0;
const FRUSTUM_SCALING: f32 = 0.0001;

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
pub(crate) struct ZIndex;

fn layer(mut query: Query<(&ZIndex, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.z = -FRUSTUM_SCALING * transform.translation.y + FRUSTUM_OFFSET;
    }
}
