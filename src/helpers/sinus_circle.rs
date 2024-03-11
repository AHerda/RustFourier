use nannou::{
    draw::{
        primitive::{Ellipse, Line},
        Drawing,
    },
    geom::Point2,
};

use super::utils::calculate_xy;

pub struct SinusCircle {
    pub origin: Point2,
    pub end: Point2,
    pub radius: f32,
    pub rotation: f32,
    pub speed: f32,
}

impl SinusCircle {
    pub fn new(
        draw: &nannou::Draw,
        x: f32,
        y: f32,
        radius: f32,
        rotation: f32,
        speed: f32,
    ) -> Self {
        let mut new = Self {
            origin: Point2::new(x, y),
            end: Point2::new(0., 0.),
            radius,
            rotation,
            speed,
        };

        let elipse = draw
            .ellipse()
            .no_fill()
            .stroke_weight(3_f32)
            .stroke_color(nannou::color::BEIGE)
            .radius(radius)
            .x_y(x, y);

        let arrow = draw
            .line()
            .stroke_weight(3_f32)
            .color(nannou::color::BEIGE)
            .start(new.origin)
            .end(calculate_xy(&new));

        new.end = calculate_xy(&new);
        new
    }
}
