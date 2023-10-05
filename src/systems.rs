use glam::Vec2;
use legion::world::SubWorld;
pub use legion::*;
use raylib::prelude::*;

use crate::components::{Body, CColor};
use crate::rendering::{DrawCommand, RenderCommandBuffer};
const ORBIT_CENTER: Vec2 = Vec2 { x: 120.0, y: 80.0 };

#[system]
#[write_component(Body)]
pub fn gravity(ecs: &mut SubWorld) {
    let mut query = <&mut Body>::query();
    for body in query.iter_mut(ecs) {
        let dir = (ORBIT_CENTER - body.pos).normalize();
        let f = dir * 0.1;
        body.acc += f;
    }
}

#[system]
#[write_component(Body)]
pub fn physics(ecs: &mut SubWorld) {
    let mut query = <&mut Body>::query();
    for body in query.iter_mut(ecs) {
        body.vel += body.acc;
        body.pos += body.vel;
        body.acc = Vec2::ZERO;
    }
}

#[system]
#[read_component(Body)]
#[read_component(CColor)]
pub fn render_entities(
    ecs: &SubWorld,
    #[resource] render_command_buffer: &mut RenderCommandBuffer,
) {
    render_command_buffer.clear();
    let mut query = <(&Body, &CColor)>::query();
    for (body, ccolor) in query.iter(ecs) {
        render_command_buffer.push(DrawCommand::ColoredSquare {
            pos: body.pos.as_uvec2(),
            color: ccolor.color,
        })
    }
}
