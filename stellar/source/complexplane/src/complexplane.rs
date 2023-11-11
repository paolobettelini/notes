use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use nannou::winit;
use nannou::text::Font;
use std::cell::RefCell;
use std::sync::Arc;

/*
lazy_static! {
    pub static ref CMFONT: Arc<Font> = {
        let font_data: &[u8] = include_bytes!("cmunbmr.ttf");
        Arc::new(Font::from_bytes(font_data).unwrap())
    };
}*/

struct Model {
    dragging: bool,
    point_pos: (f32, f32),
}

pub async fn run_app() {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    app::Builder::new_async(move |app| {
        Box::new(async move {
            let model = model(&app).await;
            MODEL.with(|m| m.borrow_mut().replace(model));
            //create_window(app).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    //.update(update)
    .event(event)
    .view(view)
    .run_async()
    .await;
}

async fn model(app: &App) -> Model {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    let width = 600;
    let height = 400;

    let w_id = app
        .new_window()
        .device_descriptor(device_desc)
        .size(width, height)
        .build_async()
        .await
        .unwrap();

    Model {
        dragging: false,
        point_pos: (0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    
    draw.background().color(WHITE);

    // Define the coordinates of the complex plane.
    let x_min = -5.5;
    let x_max = 5.5;
    let y_min = -5.5;
    let y_max = 5.5;

    // Draw the x-axis.
    draw.line()
        .start(pt2(win.left(), 0.0))
        .end(pt2(win.right(), 0.0))
        .color(BLACK)
        .weight(2.0);

    // Draw the y-axis.
    draw.line()
        .start(pt2(0.0, win.bottom()))
        .end(pt2(0.0, win.top()))
        .color(BLACK)
        .weight(2.0);

    // TODO fix the real axis

    // Draw ticks and labels on the x-axis.
    for x in (x_min as i32)..=(x_max as i32) {
        let x_pos = map_range(x as f32, x_min, x_max, win.left(), win.right());
        if x == 0 {
            continue; // Skip 0
        }
        draw.line()
            .start(pt2(x_pos, -5.0))
            .end(pt2(x_pos, 5.0))
            .color(BLACK)
            .weight(2.0);
        draw.text(&x.to_string())
            .xy(pt2(x_pos, 15.0))
            .color(BLACK)
            .font_size(12);
    }

    // Draw ticks and labels on the y-axis.
    for y in (y_min as i32)..=(y_max as i32) {
        let y_pos = map_range(y as f32, y_min, y_max, win.bottom(), win.top());
        if y == 0 {
            continue; // Skip 0
        }
        draw.line()
            .start(pt2(-5.0, y_pos))
            .end(pt2(5.0, y_pos))
            .color(BLACK)
            .weight(2.0);
        draw.text(&format!("{y}i"))
            .xy(pt2(20.0, y_pos))
            .color(BLACK)
            //.font(CMFONT.as_ref().clone())
            .font_size(12);
    }

    // Origin
    draw.text("0")
        .xy(pt2(12.0, 12.0))
        .color(BLACK)
        .font_size(12);

    // Draw point
    draw.ellipse()
        .xy(pt2(model.point_pos.0, model.point_pos.1))
        .radius(5.0)
        .color(BLACK);
    let re = map_range(model.point_pos.0, win.bottom(), win.top(), x_min, x_max);
    let im = map_range(model.point_pos.1, win.bottom(), win.top(), y_min, y_max);
    let coord = complex_to_string(re, im);
    draw.text(&coord)
        .xy(pt2(model.point_pos.0, model.point_pos.1 + 15.0))
        .color(BLACK)
        .font_size(12);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id, simple } => {
            if let Some(window_event) = simple {
                match window_event {
                    // Mouse pressed event
                    WindowEvent::MousePressed(btn) => {
                        if btn == MouseButton::Left {
                            model.dragging = true;
                        }
                    }
                    // Mouse released event
                    WindowEvent::MouseReleased(btn) => {
                        if btn == MouseButton::Left {
                            model.dragging = false;
                        }
                    }
                    // Mouse moved event (coordinate)
                    WindowEvent::MouseMoved(point) => {
                        if model.dragging {
                            model.point_pos = (point.x, point.y);
                        }
                    }
                    // Windows resize (e.g. scale)
                    WindowEvent::Resized(size) => {
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn complex_to_string(re: f32, im: f32) -> String {
    let mut result = String::new();

    // Real part
    if re == 0.0 {
        result.push('0');
    } else {
        result.push_str(&format!("{:.1}", re));
    }

    // Imaginary part
    if im == 0.0 {
        // Do nothing if the imaginary part is 0.
    } else if im > 0.0 {
        result.push('+');
        if im != 1.0 {
            result.push_str(&format!("{:.1}", im));
        }
        result.push('i');
    } else {
        result.push_str(&format!("{:.1}i", im));
    }

    result
}