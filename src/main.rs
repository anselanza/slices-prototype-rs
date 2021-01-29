extern crate nannou;
use nannou::prelude::*;
use nannou::geom::Range;


#[derive(Debug, Clone)]
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
        boxes: box_indexes.iter().map(|&i| {
            let centre_x = (i as f32) * standard_width - window_width/2.0 + standard_width/2.0;

            Box{
                id: i,
                width: standard_width,
                target_width: standard_width,
                centre: centre_x,
                target_centre: centre_x,
            }
        }).collect(),
        box_size: 0.0
    }
}

fn is_in_box (x: f32, b: &Box) -> bool {
    x > b.left() && x < b.right()
}

fn layout_boxes(boxes: &Vec<Box>, active_box: &Box, window_rect: Rect, target_size: f32, target_x: f32) -> Vec<Box>{
    let min_width = window_rect.w() / boxes.len() as f32;

    let updated: Vec<Box> = boxes.into_iter().map(|b| {
        if b.id == active_box.id {
            Box {
                id: b.id,
                width: b.width,
                target_width: clamp(target_size, min_width, target_size),
                centre: b.centre,
                target_centre: clamp(target_x, window_rect.left(), window_rect.right())
            }
        } else {
            if b.id < active_box.id { // left
                let num_boxes = active_box.id;
                let left_space_remaining = abs(active_box.left() - window_rect.left());
                let left_width = left_space_remaining / num_boxes as f32;
                Box {
                    id: b.id,
                    width: b.width,
                    target_width: left_width,
                    centre: b.centre,
                    target_centre: window_rect.left() + b.id as f32 * left_width + left_width/2.0,
                }
            } else { // right
                let right_start = active_box.right();
                let right_start_index = active_box.id + 1;
                let right_space_remaining = abs(window_rect.right() - active_box.right());
                let num_boxes = boxes.len() as u8 -1 - active_box.id;
                let right_width = right_space_remaining /   num_boxes as f32;
                Box {
                    id: b.id,
                    width: b.width,
                    target_width: right_width,
                    centre: b.centre,
                    target_centre: right_start + (b.id - right_start_index) as f32 * right_width + right_width/2.0,
                }
            }
        }
    }).collect();

    // let mut box_update = active_box.clone();
    // box_update.target_centre = clamp(target_x, window_rect.left(), window_rect.right());
    // box_update.target_width = clamp(target_size, min_width, target_size);

    // let left_slices = &boxes[0.. active_box.id as usize];
    // let start_right = active_box.id + 1;
    // let right_slices = &boxes[start_right as usize.. boxes.len()];


    // let a: &[Box] = &[box_update];
    // let updated = [left_slices, a, right_slices].concat();

    return updated;

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

fn get_active_box(boxes: &Vec<Box>, target_x: f32) -> Option<&Box> {
    boxes.into_iter().find(|&b| is_in_box(target_x, b))
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let window_rect = _app.main_window().rect();
    let box_size: f32 = map_range(_app.mouse.position().y, window_rect.top(), window_rect.bottom(), window_rect.w(), 0.0);
    _model.box_size = box_size;

    let target_x = _app.mouse.position().x;

    match get_active_box(&_model.boxes, target_x) {
        Some(active_box) => {
         _model.boxes = layout_boxes(&_model.boxes, &active_box, window_rect, box_size, target_x);
        },
        _ => {}
    }
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
        let t = format!("#{}", b.id);
        draw.text(&t).x(b.centre).y(0.0);
    }

    draw.rect()
        .color(RED)
        .w(_model.box_size)
        .h(_model.box_size)
        .x(_app.mouse.position().x)
        .y(_app.mouse.position().y);

    draw.to_frame(_app, &frame).unwrap();

}
