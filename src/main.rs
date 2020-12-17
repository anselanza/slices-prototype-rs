extern crate nannou;
use nannou::prelude::*;

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
    let num_boxes = 8;
    let window_width = 640.0;
    let box_indexes = [0,1,2,3,4,5,6,7,8,9];
    let standard_width: f32 = window_width / (num_boxes as f32);

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
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}
