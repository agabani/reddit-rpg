use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::visibility::z_index;

const DIMENSION: f32 = 48.0;
const PADDING: f32 = 0.1;
const MAX_REFLECTION_STRENGTH: usize = 7;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateRequestEvent>()
            .add_startup_system(setup)
            .add_system(animate)
            .add_system(sensor)
            .add_system(spawn);
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct Mirror;

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct ReflectionStrengthIndex(usize);

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
pub(crate) struct ReflectionStrengths([bool; MAX_REFLECTION_STRENGTH]);

fn animate(mut query: Query<(&Mirror, &ReflectionStrengths, &mut TextureAtlasSprite)>) {
    for (_, reflection_strength, mut sprite) in query.iter_mut() {
        let reflection = reflection_strength
            .0
            .iter()
            .enumerate()
            .rev()
            .find(|(_, reflecting)| **reflecting)
            .map_or(0, |(index, _)| index);

        sprite.index = reflection;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sensor(
    mut collision_events: EventReader<CollisionEvent>,
    query_parent: Query<(&Parent, &ReflectionStrengthIndex)>,
    mut query_mirror: Query<(&Mirror, &mut ReflectionStrengths)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                let parents = [query_parent.get(*a).ok(), query_parent.get(*b).ok()]
                    .into_iter()
                    .flatten();

                for (parent, reflection_strength_index) in parents {
                    if let Ok((_, mut reflection_strengths)) = query_mirror.get_mut(parent.0) {
                        reflection_strengths.0[reflection_strength_index.0] = true;
                    }
                }
            }
            CollisionEvent::Stopped(a, b, _) => {
                let parents = [query_parent.get(*a).ok(), query_parent.get(*b).ok()]
                    .into_iter()
                    .flatten();

                for (parent, reflection_strength_index) in parents {
                    if let Ok((_, mut reflection_strengths)) = query_mirror.get_mut(parent.0) {
                        reflection_strengths.0[reflection_strength_index.0] = false;
                    }
                }
            }
        }
    }
}

struct CreateRequestEvent {
    x: f32,
    y: f32,
}

#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
fn spawn(
    mut event_reader: EventReader<CreateRequestEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for event in event_reader.iter() {
        let texture_handle = asset_server.load("mirror/01_48x48.png");
        let texture_atlas = TextureAtlas::from_grid_with_padding(
            texture_handle,
            Vec2::new(DIMENSION - PADDING, DIMENSION * 2.0 - PADDING),
            12,
            1,
            Vec2::new(PADDING, PADDING),
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut entity = commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(event.x, event.y, 0.0),
            ..Default::default()
        });

        // identity
        entity.insert(Name::new("mirror")).insert(Mirror);

        // animation
        entity.insert(ReflectionStrengths([false; MAX_REFLECTION_STRENGTH]));

        // visibility
        entity.insert(z_index::ZIndex);

        // physics
        entity.insert(RigidBody::Fixed).with_children(|children| {
            children
                .spawn()
                .insert(Name::new("solid collider"))
                .insert(Collider::cuboid(DIMENSION / 2.0, DIMENSION / 2.0))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -DIMENSION / 2.0,
                    0.0,
                )));

            for i in 1..MAX_REFLECTION_STRENGTH {
                let collider_height = DIMENSION / 4.0;

                children
                    .spawn()
                    .insert(Name::new(format!("sensor collider ({})", i)))
                    .insert(ReflectionStrengthIndex(MAX_REFLECTION_STRENGTH - i))
                    .insert(Collider::cuboid(DIMENSION / 4.0, collider_height / 2.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(
                        0.0,
                        -DIMENSION + collider_height / 2.0 - i as f32 * collider_height,
                        0.0,
                    )));
            }
        });
    }
}

#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
fn setup(mut event_writer: EventWriter<CreateRequestEvent>) {
    for y in 0..2 {
        for x in 0..8 {
            event_writer.send(CreateRequestEvent {
                x: x as f32 * DIMENSION,
                y: y as f32 * DIMENSION * 2.0,
            });
        }
    }
}
