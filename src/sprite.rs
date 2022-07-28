use bevy::prelude::*;

const PADDING: f32 = 0.1;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(animate);

        #[cfg(feature = "editor")]
        {
            app.register_type::<AnimationIndexRange>()
                .register_type::<AnimationTimer>();
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "editor", derive(Default, Reflect), reflect(Component))]
struct AnimationIndexRange {
    start: usize,
    end: usize,
}

impl AnimationIndexRange {
    fn next(&self, index: usize) -> usize {
        if index < self.end - 1 {
            index + 1
        } else {
            self.start
        }
    }
}

#[derive(Component, Deref, DerefMut)]
#[cfg_attr(feature = "editor", derive(Default, Reflect), reflect(Component))]
struct AnimationTimer(Timer);

#[allow(clippy::needless_pass_by_value)]
fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationIndexRange,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, animation_index_range, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = animation_index_range.next(sprite.index);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_range = AnimationIndexRange {
        start: 56,
        end: 56 + 6,
    };
    let texture_handle = asset_server.load("characters/04_48x48.png");
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
            sprite: TextureAtlasSprite {
                index: animation_range.start,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(12.0)),
            ..Default::default()
        })
        .insert(Name::new("character_04"))
        .insert(animation_range)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}
