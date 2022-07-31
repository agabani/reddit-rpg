use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation, faces, player, runs, walks, z_index};

const DIMENSION: f32 = 48.0;
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
        &runs::Runs,
        &mut animation::AnimationIndex,
        &mut animation::AnimationTimer,
    )>,
) {
    for (_, faces, walks, runs, mut animation_index, mut animation_timer) in query.iter_mut() {
        match (faces.direction, walks.walking || runs.running) {
            (faces::Direction::Down, true) => animation_index.change(130, 6),
            (faces::Direction::Down, false) => animation_index.change(74, 6),
            (faces::Direction::Left, true) => animation_index.change(124, 6),
            (faces::Direction::Left, false) => animation_index.change(68, 6),
            (faces::Direction::Right, true) => animation_index.change(112, 6),
            (faces::Direction::Right, false) => animation_index.change(56, 6),
            (faces::Direction::Up, true) => animation_index.change(118, 6),
            (faces::Direction::Up, false) => animation_index.change(62, 6),
        }

        if runs.running && (animation_timer.duration() - 0.1).abs() > f32::EPSILON {
            *animation_timer = animation::AnimationTimer::new(Timer::from_seconds(0.1, true));
        }
        if !runs.running && (animation_timer.duration() - 0.2).abs() > f32::EPSILON {
            *animation_timer = animation::AnimationTimer::new(Timer::from_seconds(0.2, true));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard_input(
    mut query: Query<(
        &player::Player,
        &mut faces::Faces,
        &mut walks::Walks,
        &mut runs::Runs,
    )>,
) {
    for (player, mut faces, mut walks, mut runs) in query.iter_mut() {
        let keys = player.keys();

        // pressed directional
        if keys.just_pressed(KeyCode::W) {
            faces.direction = faces::Direction::Up;
        }
        if keys.just_pressed(KeyCode::A) {
            faces.direction = faces::Direction::Left;
        }
        if keys.just_pressed(KeyCode::S) {
            faces.direction = faces::Direction::Down;
        }
        if keys.just_pressed(KeyCode::D) {
            faces.direction = faces::Direction::Right;
        }

        // pressed directional
        if keys.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            if keys.pressed(KeyCode::LShift) {
                runs.running = true;
                walks.walking = false;
            } else {
                runs.running = false;
                walks.walking = true;
            }
        }

        // released directional
        if keys.just_released(KeyCode::W) && faces.direction == faces::Direction::Up {
            runs.running = false;
            walks.walking = false;
        }
        if keys.just_released(KeyCode::A) && faces.direction == faces::Direction::Left {
            runs.running = false;
            walks.walking = false;
        }
        if keys.just_released(KeyCode::S) && faces.direction == faces::Direction::Down {
            runs.running = false;
            walks.walking = false;
        }
        if keys.just_released(KeyCode::D) && faces.direction == faces::Direction::Right {
            runs.running = false;
            walks.walking = false;
        }

        // pressed LShift
        if keys.just_pressed(KeyCode::LShift)
            && keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        {
            runs.running = true;
            walks.walking = false;
        }

        // released LShift
        if keys.just_released(KeyCode::LShift)
            && keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        {
            runs.running = false;
            walks.walking = true;
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
        Vec2::new(DIMENSION - PADDING, DIMENSION * 2.0 - PADDING),
        56,
        20,
        Vec2::new(PADDING, PADDING),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut entity = commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite {
            index: animation_index.start(),
            ..Default::default()
        },
        ..Default::default()
    });

    // identity
    entity.insert(Name::new("character_04")).insert(Character);

    // animation
    entity
        .insert(animation_index)
        .insert(animation_timer)
        .insert(faces::Faces {
            direction: faces::Direction::Down,
        })
        .insert(runs::Runs {
            strength: 48.0,
            running: false,
        })
        .insert(walks::Walks {
            strength: 24.0,
            walking: false,
        });

    // visibility
    entity.insert(z_index::ZIndex);

    // control
    entity.insert(player);

    // physics
    entity
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        })
        .insert(Damping {
            linear_damping: 18.0,
            angular_damping: 1.0,
        })
        .with_children(|children| {
            children
                .spawn()
                .insert(Name::new("solid collider"))
                .insert(Collider::ball(24.0))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -24.0, 0.0)));
        });
}
