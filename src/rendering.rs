use glam::UVec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

pub type RenderCommandBuffer = Vec<DrawCommand>;

#[derive(Clone)]
pub enum DrawCommand {
    ClearScreen,
    ColoredSquare { pos: UVec2, color: Color },
}

// defualt entity size
const SIZE: i32 = 1;

pub fn execute_render_command_buffer(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    render_command_buffer: &mut RenderCommandBuffer,
) {
    for command in render_command_buffer.iter() {
        match command {
            DrawCommand::ClearScreen => {
                d.clear_background(Color::WHITE);
            }
            DrawCommand::ColoredSquare { pos, color } => {
                d.draw_rectangle(pos.x as i32, pos.y as i32, SIZE, SIZE, *color);
            }
        }
    }
}
