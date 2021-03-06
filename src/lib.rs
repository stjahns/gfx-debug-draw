#![feature(plugin)]
#![feature(old_io)]
#![feature(old_path)]
#![plugin(gfx_macros)]

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate image;
extern crate xml;

mod debug_renderer;
mod line_renderer;
mod text_renderer;
mod bitmap_font;
mod utils;

pub use debug_renderer::{ DebugRenderer, DebugRendererBuilder };
pub use bitmap_font::{ BitmapFont };
