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
    boxes: Vec<Box>
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
        }).collect()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // print!("boxes: {:?}", _model.boxes);
}
fn view(_app: &App, _model: &Model, frame: Frame){
    // frame.clear(PURPLE);
    let draw = _app.draw();

    draw.background().color(PURPLE);

    for b in &_model.boxes {
        draw.ellipse()
            .color(WHITE)
            .w(8.0)
            .h(8.0)
            .x_y(b.left/4.0, 25.0);
    }

    draw.to_frame(_app, &frame).unwrap();

}
