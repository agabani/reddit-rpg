#![warn(clippy::pedantic)]

pub mod app;

mod camera;

#[cfg(feature = "editor")]
mod editor;

mod sprite;

mod window;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
