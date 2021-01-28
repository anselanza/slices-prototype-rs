extern crate nannou;
use nannou::prelude::*;

#[derive(Debug)]
struct Box {
    id: u8,
    width: f32,
    target_width: f32,
    left: f32,
    target_left: f32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    boxes: Vec<Box>,
    box_size: f32
}

fn model(_app: &App) -> Model {
    const NUM_BOXES: u8 = 10;
    let window = _app.main_window();
    let window_width = window.rect().w();
    let box_indexes: Vec<u8> = (0..NUM_BOXES).collect();
    let standard_width: f32 = window_width / (NUM_BOXES as f32);

    Model {
        boxes: box_indexes.iter().map(|&i| Box{
            id: i,
            width: standard_width,
            target_width: standard_width,
            left: (i as f32) * standard_width,
            target_left: (i as f32) * standard_width,
        }).collect(),
        box_size: 0.0
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let window_rect = _app.main_window().rect();
    let box_size: f32 = map_range(_app.mouse.position().y, window_rect.top(), window_rect.bottom(), window_rect.w(), 0.0);
    _model.box_size = box_size;

}
fn view(_app: &App, _model: &Model, frame: Frame){
    // frame.clear(PURPLE);
    let draw = _app.draw();

    let window_rect = _app.main_window().rect();

    draw.background().color(PURPLE);

    for b in &_model.boxes {
        draw.rect()
            .color(WHITE)
            .w(b.width)
            .h(window_rect.h())
            .x((window_rect.left() + b.width/2.0) + b.left)
            .y(0.0)
            .stroke_weight(2.0)
            .stroke(BLUE);
    }

    draw.rect()
        .color(RED)
        .w(_model.box_size)
        .h(_model.box_size)
        .x(_app.mouse.position().x)
        .y(_app.mouse.position().y);

    draw.to_frame(_app, &frame).unwrap();

}
