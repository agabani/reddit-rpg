#![warn(clippy::pedantic)]

mod animation;

pub mod app;

mod camera;

mod character;

#[cfg(feature = "editor")]
mod editor;

mod faces;

mod mirror;

mod physics;

mod player;

mod runs;

mod walks;

mod window;

mod z_index;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
