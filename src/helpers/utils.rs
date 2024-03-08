use nannou::geom::Point2;

use super::sinus_circle::SinusCircle;

pub struct Settings {
    pub origin: Point2,
    pub radius: f32,
    pub rotation: f32,
    pub speed: f32,
}

pub fn calculate_xy(settings: &SinusCircle) -> Point2 {
    Point2::new(settings.origin.x + settings.radius * f32::cos(settings.rotation * settings.speed), settings.origin.y + settings.radius * f32::sin(settings.rotation * settings.speed))
}