use bevy::prelude::*;

use crate::{animation, z_index};

const PADDING: f32 = 0.1;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_index = animation::AnimationIndex::new(0, 12);
    let animation_timer = animation::AnimationTimer::new(Timer::from_seconds(0.2, true));

    let texture_handle = asset_server.load("mirror/01_48x48.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(48.0 - PADDING, 96.0 - PADDING),
        12,
        1,
        Vec2::new(PADDING, PADDING),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(48.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("mirror"))
        .insert(animation_index)
        .insert(animation_timer)
        .insert(z_index::ZIndex);
}
