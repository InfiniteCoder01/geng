#![recursion_limit = "256"]

pub use geng_derive::*;

pub mod prelude {
    pub use crate::{draw_2d, Geng};
    pub use ::batbox::*;
    pub use ugli::{self, Ugli};
}

use crate::prelude::*;
#[allow(unused_imports)]
use log::{trace, warn};

mod asset;
mod context;
pub mod draw_2d;
mod font;
mod loading_screen;
pub mod obj;
mod shader_lib;
mod sound;
mod state;
mod texture_atlas;
mod window;

pub use asset::*;
pub use context::*;
pub use draw_2d::Draw2D;
pub use font::*;
pub use loading_screen::*;
pub use shader_lib::*;
pub use sound::*;
pub use state::*;
pub use texture_atlas::*;
pub use window::*;
