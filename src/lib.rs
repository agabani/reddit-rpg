#![warn(clippy::pedantic)]

mod animation;
pub mod app;
mod camera;
#[cfg(feature = "editor")]
mod editor;
mod movement;
mod object;
mod physics;
mod player;
mod visibility;
mod window;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
