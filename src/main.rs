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

    let mut box_update = active_box.clone();
    box_update.target_centre = clamp(target_x, window_rect.left(), window_rect.right());
    box_update.target_width = clamp(target_size, min_width, target_size);

    let left_slices = &boxes[0.. active_box.id as usize];
    let start_right = active_box.id + 1;
    let right_slices = &boxes[start_right as usize.. boxes.len()];

    let a: &[Box] = &[box_update];
    let updated = [left_slices, a, right_slices].concat();


    // let updated: Vec<Box> = boxes.iter().map(|b| {
    //     if b.id == active_box.id {
    //         let mut box_update = b.clone();
    //         box_update.target_centre = clamp(target_x, window_rect.left(), window_rect.right());
    //         box_update.target_width = clamp(target_size, min_width, target_size);
    //         return box_update;
    //     } else {
    //         if b.id < active_box.id {
    //             let num_slices = active_box.id;
    //             let left_width = active_box.centre / num_slices as f32;
    //             Box {
    //                 id: b.id,
    //                 width: b.width,
    //                 target_width: b.target_width,
    //                 centre: b.centre,
    //                 target_centre: b.target_centre,
    //             }
    //         } else {
    //             Box {
    //                 id: b.id,
    //                 width: b.width,
    //                 target_width: b.target_width,
    //                 centre: b.centre,
    //                 target_centre: b.target_centre,
    //             }
    //         }
    //     }
        
    // }).collect();

    return updated;


    // if target_x < window_rect.left() || target_x > window_rect.right() { // bail out if target out of window bounds
    //     return;
    // }

    // let target_x_relative = if target_size > min_width { target_x - target_size / 2.0 } else { active_box.centre };
    // active_box.target_centre = clamp(target_x, window_rect.left(), window_rect.right());
    // active_box.target_width = clamp(target_size, min_width, target_size);

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

    let target_x = _app.mouse.position().x;

    let boxes = &_model.boxes;
    let active_box_finder = boxes.into_iter().find(|b| is_in_box(target_x, b));

    match active_box_finder {
        Some(active_box) => {
         _model.boxes = layout_boxes(boxes, &active_box, window_rect, box_size, target_x);
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
    }

    draw.rect()
        .color(RED)
        .w(_model.box_size)
        .h(_model.box_size)
        .x(_app.mouse.position().x)
        .y(_app.mouse.position().y);

    draw.to_frame(_app, &frame).unwrap();

}
