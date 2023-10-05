//a position component

use glam::Vec2;
use raylib::color::Color;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

pub struct CColor {
    pub color: Color,
}
