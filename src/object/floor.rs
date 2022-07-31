use bevy::prelude::*;

use crate::visibility::z_index;

const DIMENSION: f32 = 48.0;
const PADDING: f32 = 0.1;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_startup_system(setup)
            .add_system(spawn);
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Floor;

struct SpawnEvent {
    x: f32,
    y: f32,
}

#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
fn spawn(
    mut event_reader: EventReader<SpawnEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for event in event_reader.iter() {
        let texture_handle = asset_server.load("room/builder_48x48.png");
        let texture_atlas = TextureAtlas::from_grid_with_padding(
            texture_handle,
            Vec2::new(DIMENSION - PADDING, DIMENSION - PADDING),
            76,
            109,
            Vec2::new(PADDING, PADDING),
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut entity = commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 2619,
                ..Default::default()
            },
            transform: Transform::from_xyz(event.x, event.y, 0.0),
            ..Default::default()
        });

        // identity
        entity.insert(Name::new("floor")).insert(Floor);

        // visibility
        entity.insert(z_index::ZIndex::new(4.0));
    }
}

#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
fn setup(mut event_writer: EventWriter<SpawnEvent>) {
    for y in -4..6 {
        for x in -6..12 {
            event_writer.send(SpawnEvent {
                x: x as f32 * DIMENSION,
                y: y as f32 * DIMENSION,
            });
        }
    }
}
