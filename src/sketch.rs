use glam::Vec2;
pub use legion::*;
use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;

use crate::{
    components::{Body, CColor},
    rendering::{execute_render_command_buffer, RenderCommandBuffer},
    systems::{gravity_system, physics_system, render_entities_system},
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub ecs: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut rng: ThreadRng = rand::thread_rng();

        let center: Vec2 = Vec2 { x: 120.0, y: 80.0 };
        let offset = 20.0;

        for _ in 0..1000 {
            ecs.push((
                Body {
                    pos: Vec2::new(
                        rng.gen_range(center.x - offset..center.x + offset),
                        rng.gen_range(center.y - offset..center.y + offset),
                    ),
                    vel: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                    acc: Vec2::ZERO,
                },
                CColor {
                    color: Color {
                        r: rng.gen_range(0..255),
                        g: rng.gen_range(0..255),
                        b: rng.gen_range(0..255),
                        a: 255,
                    },
                },
            ));
        }

        let resources = Resources::default();
        let schedule = Schedule::builder()
            .add_system(gravity_system())
            .add_system(physics_system())
            .add_system(render_entities_system())
            .build();

        Self {
            running: true,
            time_since_last_update: 0.0,
            ecs,
            resources,
            schedule,
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

pub fn step(_rl: &mut RaylibHandle, _rlt: &mut RaylibThread, state: &mut State) {
    state.schedule.execute(&mut state.ecs, &mut state.resources);
}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Low Res Sketch!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    if let Some(mut render_command_buffer) = state.resources.get_mut::<RenderCommandBuffer>() {
        execute_render_command_buffer(d, render_command_buffer.as_mut());
    }
}
