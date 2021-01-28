extern crate nannou;
use nannou::prelude::*;
use nannou::geom::Range;


#[derive(Debug)]
struct Box {
    id: u8,
    width: f32,
    target_width: f32,
    centre: f32,
    target_centre: f32,
}

impl Box {
    fn left (&self) -> f32 { self.centre - self.width / 2.0 }
    fn right (&self) -> f32 { self.centre + self.width / 2.0 }
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
            centre: (i as f32) * standard_width - window_width/2.0 + standard_width/2.0,
            target_centre: (i as f32) * standard_width,
        }).collect(),
        box_size: 0.0
    }
}

fn is_in_box (x: f32, b: &Box) -> bool {
    x > b.left() && x < b.right()
}

fn layout_boxes(boxes: &mut Vec<Box>, window_rect: Rect, target_size: f32, target_x: f32) {
    let min_width = window_rect.w() / boxes.len() as f32;
    let active_box = boxes.into_iter().find(|b| is_in_box(target_x, b));

    if target_x < window_rect.left() || target_x > window_rect.right() { // bail out if target out of window bounds
        return;
    }

    // active_box is an Option, so need to get the result if it exists
    match active_box {
        Some(b) => {
            let target_x_relative = if target_size > min_width { target_x - target_size / 2.0 } else { b.centre };
            // b.centre = clamp(in_x, window_rect.left(), window_rect.right());
            b.target_centre = clamp(target_x, window_rect.left(), window_rect.right());
            b.target_width = clamp(target_size, min_width, target_size);
            // b.left = clamp(in_x, -window_width/2.0, window_width/2.0);
            // b.width = clamp(target_size, min_width, target_size);
        },
        _ => {}
    }

    // TODO: now the boxes to the left and right (the others)

}

fn animate_boxes(boxes: &mut Vec<Box>) {
    for b in boxes {
        {
            let range = Range { start: b.width, end: b.target_width };
            const LERP_FACTOR: f32 = 0.1;
            b.width = range.lerp(LERP_FACTOR);
        }
        {
            let range = Range { start: b.centre, end: b.target_centre };
            const LERP_FACTOR: f32 = 0.1;
            b.centre = range.lerp(LERP_FACTOR);
        }

        
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let window_rect = _app.main_window().rect();
    let box_size: f32 = map_range(_app.mouse.position().y, window_rect.top(), window_rect.bottom(), window_rect.w(), 0.0);
    _model.box_size = box_size;

    layout_boxes(&mut _model.boxes, window_rect, box_size, _app.mouse.position().x);
    animate_boxes(&mut _model.boxes);

}
fn view(_app: &App, _model: &Model, frame: Frame){
    // frame.clear(PURPLE);
    let draw = _app.draw();

    let window_rect = _app.main_window().rect();

    draw.background().color(PURPLE);

    for b in &_model.boxes {
        draw.rect()
            .color(if is_in_box(_app.mouse.position().x, b) { GREEN } else { GREY })
            .w(b.width)
            .h(window_rect.h())
            .x(b.centre)
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
