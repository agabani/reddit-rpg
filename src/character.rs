use bevy::prelude::*;

use crate::{animation, faces, player, walks};

const PADDING: f32 = 0.1;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(keyboard_input)
            .add_system(animate);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<Character>();
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Character;

#[allow(clippy::needless_pass_by_value)]
fn animate(
    mut query: Query<(
        &Character,
        &faces::Faces,
        &walks::Walks,
        &mut animation::AnimationIndex,
    )>,
) {
    for (_, faces, walks, mut animation_index) in query.iter_mut() {
        match (faces.direction, walks.walking) {
            (faces::Direction::Down, true) => animation_index.change(130, 6),
            (faces::Direction::Down, false) => animation_index.change(74, 6),
            (faces::Direction::Left, true) => animation_index.change(124, 6),
            (faces::Direction::Left, false) => animation_index.change(68, 6),
            (faces::Direction::Right, true) => animation_index.change(112, 6),
            (faces::Direction::Right, false) => animation_index.change(56, 6),
            (faces::Direction::Up, true) => animation_index.change(118, 6),
            (faces::Direction::Up, false) => animation_index.change(62, 6),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard_input(mut query: Query<(&player::Player, &mut faces::Faces, &mut walks::Walks)>) {
    for (player, mut faces, mut walks) in query.iter_mut() {
        let keys = player.keys();

        if keys.just_pressed(KeyCode::W) {
            faces.direction = faces::Direction::Up;
            walks.walking = true;
        }
        if keys.just_released(KeyCode::W) && faces.direction == faces::Direction::Up {
            walks.walking = false;
        }
        if keys.just_pressed(KeyCode::A) {
            faces.direction = faces::Direction::Left;
            walks.walking = true;
        }
        if keys.just_released(KeyCode::A) && faces.direction == faces::Direction::Left {
            walks.walking = false;
        }
        if keys.just_pressed(KeyCode::S) {
            faces.direction = faces::Direction::Down;
            walks.walking = true;
        }
        if keys.just_released(KeyCode::S) && faces.direction == faces::Direction::Down {
            walks.walking = false;
        }
        if keys.just_pressed(KeyCode::D) {
            faces.direction = faces::Direction::Right;
            walks.walking = true;
        }
        if keys.just_released(KeyCode::D) && faces.direction == faces::Direction::Right {
            walks.walking = false;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_index = animation::AnimationIndex::new(74, 6);
    let animation_timer = animation::AnimationTimer::new(Timer::from_seconds(0.2, true));

    let player = player::Player::default();

    let texture_handle = asset_server.load("character/04_48x48.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(48.0 - PADDING, 96.0 - PADDING),
        56,
        20,
        Vec2::new(PADDING, PADDING),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: animation_index.start(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("character_04"))
        .insert(Character)
        .insert(animation_index)
        .insert(animation_timer)
        .insert(player)
        .insert(faces::Faces {
            direction: faces::Direction::Down,
        })
        .insert(walks::Walks {
            speed: 150.0,
            walking: false,
        });
}
