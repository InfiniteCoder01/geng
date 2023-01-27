use super::*;

#[derive(ugli::Vertex)]
pub struct Vertex {
    a_pos: vec2<f32>,
}

pub struct EmptyLoadingScreen;

impl ProgressScreen for EmptyLoadingScreen {}

impl State for EmptyLoadingScreen {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        #![allow(unused_variables)]
    }
}
