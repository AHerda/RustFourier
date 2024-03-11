use nannou::prelude::*;
use Fourier::helpers::{sinus_circle::SinusCircle, utils::calculate_xy};

static mut TEMP: Vec<Point2> = Vec::new();

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    let t = app.time;
    draw.background().color(rgba8(0, 0, 0, 0));

    let mut scs: Vec<SinusCircle> = Vec::new();

    scs.push(SinusCircle::new(&draw, 0., 0., 40., t, 4.));
    scs.push(SinusCircle::new(
        &draw,
        scs[0].end.x,
        scs[0].end.y,
        20.,
        t,
        8.,
    ));

    let points = 75;
    let temp: Vec<Point2>;

    unsafe {
        TEMP.push(scs[1].end);
        if TEMP.len() == points {
            TEMP.remove(0);
        }
        temp = TEMP.clone();
    }

    let vertecies = temp.iter().enumerate().map(|(i, &point)| {
        (point, rgba8(255, 0, 0, (i as f32 / points as f32 * 255.) as u8))
    });
    draw.polyline().weight(3.).points_colored(vertecies);

    draw.to_frame(app, &frame).unwrap();
}
