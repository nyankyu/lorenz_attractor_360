mod lorenz_attractor;

use lorenz_attractor::*;
use nannou::prelude::*;
use std::process::exit;

const WINDOW_H: u32 = 2160;
//const WINDOW_H: u32 = 800;
const WINDOW_W: u32 = WINDOW_H * 2;
const DELTA_THETA: f32 = 0.0003;
const SCALE: f32 = WINDOW_H as f32 / PI;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    theta: f32,
    lorenz_attractor: LorenzAttractor,
    minutes: u64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .visible(false)
        .view(view)
        .build()
        .unwrap();

    Model {
        theta: 0.0,
        lorenz_attractor: LorenzAttractor::new(),
        minutes: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let minutes = app.elapsed_frames() / 60 / 60;
    if model.minutes < minutes {
        model.minutes = minutes;
        print!("{}, ", minutes);
    }
    if minutes > 2 {
        exit(0);
    }

    model.theta += DELTA_THETA;
    model.lorenz_attractor.update();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(SCALE);

    draw.background().color(BLACK);

    model.lorenz_attractor.draw(&draw, model.theta);

    draw.to_frame(app, &frame).unwrap();

    save_frame(app);
}

#[allow(dead_code)]
fn save_frame(app: &App) {
    let frame_num = app.elapsed_frames();

    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join(frame_num.to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
