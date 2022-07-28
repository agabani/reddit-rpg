use bevy::prelude::*;

use crate::animation;

const PADDING: f32 = 0.1;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(keyboard_input)
            .add_system(apply_animation);

        #[cfg(feature = "editor")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<AnimationState>();
        }
    }
}

#[derive(Default, Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
struct AnimationState {
    action: AnimationAction,
}

#[derive(Clone, Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
enum AnimationAction {
    Stand(AnimationDirection),
    Walk(AnimationDirection),
}

#[derive(Clone, PartialEq, Eq, Component)]
#[cfg_attr(feature = "editor", derive(bevy_inspector_egui::Inspectable))]
enum AnimationDirection {
    Down,
    Left,
    Right,
    Up,
}

impl std::default::Default for AnimationAction {
    fn default() -> Self {
        AnimationAction::Stand(AnimationDirection::default())
    }
}

impl std::default::Default for AnimationDirection {
    fn default() -> Self {
        AnimationDirection::Down
    }
}

impl AnimationState {
    fn new_index(&self) -> animation::AnimationIndex {
        let mut index = animation::AnimationIndex::new(0, 0);
        self.apply_index(&mut index);
        index
    }

    fn apply_index(&self, animation_index: &mut animation::AnimationIndex) {
        match &self.action {
            AnimationAction::Stand(direction) => match direction {
                AnimationDirection::Down => animation_index.change(74, 6),
                AnimationDirection::Left => animation_index.change(68, 6),
                AnimationDirection::Right => animation_index.change(56, 6),
                AnimationDirection::Up => animation_index.change(62, 6),
            },
            AnimationAction::Walk(direction) => match direction {
                AnimationDirection::Down => animation_index.change(130, 6),
                AnimationDirection::Left => animation_index.change(124, 6),
                AnimationDirection::Right => animation_index.change(112, 6),
                AnimationDirection::Up => animation_index.change(118, 6),
            },
        }
    }

    fn update_action(&mut self, action: AnimationAction) -> &Self {
        match (&self.action, &action) {
            (AnimationAction::Stand(_), AnimationAction::Stand(_)) => {}
            (AnimationAction::Walk(a), AnimationAction::Stand(b)) => {
                if a == b {
                    self.action = action;
                }
            }
            _ => self.action = action,
        };
        self
    }
}

fn apply_animation(mut query: Query<(&AnimationState, &mut animation::AnimationIndex)>) {
    for (animation_state, mut animation_index) in query.iter_mut() {
        animation_state.apply_index(&mut animation_index);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut AnimationState>) {
    for mut animation_state in query.iter_mut() {
        let mut action = None;

        if keys.just_pressed(KeyCode::W) {
            action = Some(AnimationAction::Walk(AnimationDirection::Up));
        }
        if keys.just_released(KeyCode::W) {
            action = Some(AnimationAction::Stand(AnimationDirection::Up));
        }
        if keys.just_pressed(KeyCode::A) {
            action = Some(AnimationAction::Walk(AnimationDirection::Left));
        }
        if keys.just_released(KeyCode::A) {
            action = Some(AnimationAction::Stand(AnimationDirection::Left));
        }
        if keys.just_pressed(KeyCode::S) {
            action = Some(AnimationAction::Walk(AnimationDirection::Down));
        }
        if keys.just_released(KeyCode::S) {
            action = Some(AnimationAction::Stand(AnimationDirection::Down));
        }
        if keys.just_pressed(KeyCode::D) {
            action = Some(AnimationAction::Walk(AnimationDirection::Right));
        }
        if keys.just_released(KeyCode::D) {
            action = Some(AnimationAction::Stand(AnimationDirection::Right));
        }

        if let Some(action) = action {
            animation_state.update_action(action);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_state = AnimationState::default();
    let animation_index = animation_state.new_index();
    let animation_timer = animation::AnimationTimer::new(Timer::from_seconds(0.2, true));

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
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(12.0)),
            sprite: TextureAtlasSprite {
                index: animation_index.start(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("character_04"))
        .insert(animation_index)
        .insert(animation_state)
        .insert(animation_timer);
}
