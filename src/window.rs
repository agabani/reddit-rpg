use bevy::prelude::*;

pub(crate) fn descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: "Reddit RPG".into(),
        ..Default::default()
    }
}
