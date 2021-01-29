extern crate nannou;
use nannou::prelude::*;
use nannou::geom::Range;

const LERP_FACTOR: f32 = 0.1;
const NUM_BOXES: u8 = 10;


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
    box_size: f32,
    frame_rate: f64
}

fn model(_app: &App) -> Model {
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
        box_size: 0.0,
        frame_rate: 0.0
    }
}

fn is_in_box (x: f32, b: &Box) -> bool {
    x > b.left() && x < b.right()
}

fn squeeze_box_to_fit (b: &Box, start_x: f32, offset_index: i8, count: u8, space_remaining: f32) -> Box {
    let new_width = space_remaining / count as f32;
    Box {
        id: b.id,
        width: b.width,
        target_width: new_width,
        centre: b.centre,
        target_centre: start_x + new_width/2.0 + (offset_index + b.id as i8) as f32 * new_width,
    }
}

fn layout_boxes(boxes: &Vec<Box>, active_box: &Box, window_rect: Rect, target_size: f32, target_x: f32) -> Vec<Box>{
    let min_width = window_rect.w() / boxes.len() as f32;
    let max_width = window_rect.w() / 2.0;

    let updated: Vec<Box> = boxes.into_iter().map(|b| {
        if b.id == active_box.id {
            Box {
                id: b.id,
                width: b.width,
                target_width: clamp(target_size, min_width, max_width),
                centre: b.centre,
                target_centre: clamp(target_x, window_rect.left(), window_rect.right())
            }
        } else {
            if b.id < active_box.id { // left
                let num_boxes = active_box.id;
                let left_space_remaining = abs(active_box.left() - window_rect.left());
                return squeeze_box_to_fit(b, window_rect.left(), 0, num_boxes, left_space_remaining);
            } else { // right
                let right_start_index: i8 = active_box.id as i8 + 1;
                let right_space_remaining = abs(window_rect.right() - active_box.right());
                let num_boxes = boxes.len () as u8 -1 - active_box.id;
                return squeeze_box_to_fit(b, active_box.right(), -right_start_index, num_boxes, right_space_remaining);
            }
        }
    }).collect();

    return updated;

}

fn animate_boxes(boxes: &mut Vec<Box>) {
    for b in boxes {
        {
            let range = Range { start: b.width, end: b.target_width };
            b.width = range.lerp(LERP_FACTOR);
        }
        {
            let range = Range { start: b.centre, end: b.target_centre };
            b.centre = range.lerp(LERP_FACTOR);
        }
    }
}

fn get_active_box(boxes: &Vec<Box>, target_x: f32) -> Option<&Box> {
    boxes.into_iter().find(|&b| is_in_box(target_x, b))
}

fn update(app: &App, model: &mut Model, update: Update) {
    let window_rect = app.main_window().rect();
    let box_size: f32 = map_range(app.mouse.position().y, window_rect.top(), window_rect.bottom(), window_rect.w(), 0.0);
    model.box_size = box_size;

    let target_x = app.mouse.position().x;

    match get_active_box(&model.boxes, target_x) {
        Some(active_box) => {
         model.boxes = layout_boxes(&model.boxes, &active_box, window_rect, box_size, target_x);
        },
        _ => {}
    }
    animate_boxes(&mut model.boxes);

    let s = update.since_last.secs();
    let fps = 1.0 / s;

    model.frame_rate = fps;
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
        .rgba(1.0, 0.0, 0.0, 0.5)
        .w(_model.box_size)
        .h(_model.box_size)
        .x(_app.mouse.position().x)
        .y(_app.mouse.position().y);

    {
        let t = format!("{:.1} fps", _model.frame_rate);
        draw.text(&t).x(window_rect.left()+ 32.0).y(window_rect.top() -16.0);
    }

    draw.to_frame(_app, &frame).unwrap();

}
