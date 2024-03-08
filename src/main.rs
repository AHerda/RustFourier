use nannou::{glam::Vec3, App, Frame};
use Fourier::helpers::sinus_circle::SinusCircle;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let win =
        app
            .window_rect();
    let draw = app.draw();
    let t = app.time;
    draw.background().color(nannou::color::rgba8(0, 0, 0, 0));

    let mut scs: Vec<SinusCircle> = Vec::new();

    scs.push(SinusCircle::new(&draw, 0., 0., 40., t, 4.));
    scs.push(SinusCircle::new(&draw, scs[0].end.x, scs[0].end.y, 20., t, -4.));

    draw.to_frame(app, &frame).unwrap();
}
