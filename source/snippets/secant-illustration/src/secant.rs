use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use nannou::winit;
use nannou::text::Font;
use std::cell::RefCell;
use std::sync::Arc;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

const DOT_RADIUS: f32 = 6.5;
const POINT_A_INIT_X: f32 = 0.15;
const POINT_B_INIT_X: f32 = 0.75;

lazy_static! {
    pub static ref FONT: Arc<Font> = {
        let font_data: &[u8] = include_bytes!("lmmath-regular.ttf");
        Arc::new(Font::from_bytes(font_data).unwrap())
    };
}

struct Model {
    touched: bool, // if the user has already dragged something
    dragging_a: bool,
    dragging_b: bool,
    point_pos_a: (f32, f32),
    point_pos_b: (f32, f32),
    cursor_pos: (f32, f32),
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
    .update(update)
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

    let w_id = app
        .new_window()
        .device_descriptor(device_desc)
        .size(WIDTH, HEIGHT)
        .build_async()
        .await
        .unwrap();

    let point_a_x = POINT_A_INIT_X * WIDTH as f32 - (WIDTH as f32) * 0.5;
    let point_a_y = function(POINT_A_INIT_X) * HEIGHT as f32 - (HEIGHT as f32) * 0.5;

    let point_b_x = POINT_B_INIT_X * WIDTH as f32 - (WIDTH as f32) * 0.5;
    let point_b_y = function(POINT_B_INIT_X) * HEIGHT as f32 - (HEIGHT as f32) * 0.5;

    Model {
        touched: false,
        dragging_a: false,
        dragging_b: false,
        point_pos_a: (point_a_x, point_a_y),
        point_pos_b: (point_b_x, point_b_y),
        cursor_pos: (0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if !model.touched {
        let d = update.since_start.as_millis() as u32 as f32;

        // There may be inital lag for all the page loading
        // so start the animation after 2 seconds
        let initial_wait = 2000.0;
        let d = d - initial_wait;
        if d < 0.0 {
            return;
        }

        let offset = (d * 0.0025).sin() * 0.1;

        let x = POINT_B_INIT_X + offset;
        model.point_pos_b.0 = x * WIDTH as f32 - (WIDTH as f32) * 0.5;
        model.point_pos_b.1 = function(x) * HEIGHT as f32 - (HEIGHT as f32) * 0.5;
    }
}

fn function(x: f32) -> f32 {
    let a = 1.4 * x - 1.0;
    let y = (std::f64::consts::E as f32).pow(-a*a) - 0.2;
    y
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    let font_size = 56;
    let weight = 3.0;
    
    draw.background().color(WHITE);

    // Draw the x-axis.
    draw.line()
        .start(pt2(win.left(), win.bottom() + 10.0))
        .end(pt2(win.right(), win.bottom() + 10.0))
        .color(BLACK)
        .weight(2.0);

    // Draw the y-axis.
    draw.line()
        .start(pt2(win.left() + 10.0, win.bottom()))
        .end(pt2(win.left() + 10.0, win.top()))
        .color(BLACK)
        .weight(2.0);

    // Draw the function from x=0 to x=1, stretched to the size of the window
    let num_points = 25;
    let step = 1.0 / num_points as f32;
    for i in 0..num_points {
        let x1 = i as f32 * step;
        let x2 = (i + 1) as f32 * step;

        let y1 = function(x1);
        let y2 = function(x2);

        let x1_mapped = map_range(x1, 0.0, 1.0, win.left(), win.right());
        let x2_mapped = map_range(x2, 0.0, 1.0, win.left(), win.right());

        let y1_mapped = map_range(y1, 0.0, 1.0, win.bottom(), win.top());
        let y2_mapped = map_range(y2, 0.0, 1.0, win.bottom(), win.top());

        draw.line()
            .start(pt2(x1_mapped, y1_mapped))
            .end(pt2(x2_mapped, y2_mapped))
            .color(RED)
            .weight(weight);
    }

    // Draw fixed point
    draw.ellipse()
        .xy(pt2(model.point_pos_a.0, model.point_pos_a.1))
        .radius(DOT_RADIUS)
        .color(BLACK);
        
    // Draw draggable point
    draw.ellipse()
        .xy(pt2(model.point_pos_b.0, model.point_pos_b.1))
        .radius(DOT_RADIUS)
        .color(BLACK);

    // Line between A and B
    draw.line()
        .start(pt2(model.point_pos_a.0, model.point_pos_a.1))
        .end(pt2(model.point_pos_b.0, model.point_pos_a.1))
        .color(BLACK)
        .weight(1.0);

    // Delta x line
    draw.line()
        .start(pt2(model.point_pos_b.0, model.point_pos_a.1))
        .end(pt2(model.point_pos_b.0, model.point_pos_b.1))
        .color(BLACK)
        .weight(1.0);

    // Delta y line
    draw.line()
        .start(pt2(model.point_pos_a.0, model.point_pos_a.1))
        .end(pt2(model.point_pos_b.0, model.point_pos_b.1))
        .color(BLUE)
        .weight(weight);

    draw.text("A")
        .xy(pt2(model.point_pos_a.0, model.point_pos_a.1 + 40.0))
        .color(BLACK)
        .font(FONT.as_ref().clone())
        .font_size(font_size);

    draw.text("B")
        .xy(pt2(model.point_pos_b.0, model.point_pos_b.1 + 40.0))
        .color(BLACK)
        .font(FONT.as_ref().clone())
        .font_size(font_size);

    draw.text("Δx")
        .xy(pt2((model.point_pos_b.0 + model.point_pos_a.0) * 0.5, model.point_pos_a.1 - 10.0))
        .color(BLACK)
        .font(FONT.as_ref().clone())
        .font_size(font_size);

    draw.text("Δy")
        .xy(pt2(model.point_pos_b.0 + 20.0, (model.point_pos_b.1 + model.point_pos_a.1) * 0.5 + 10.0))
        .color(BLACK)
        .font(FONT.as_ref().clone())
        .font_size(font_size);

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
                            // if the user clicked the A dot
                            let delta_a_x = model.point_pos_a.0 - model.cursor_pos.0;
                            let delta_a_y = model.point_pos_a.1 - model.cursor_pos.1;
                            let r = DOT_RADIUS + 3.0; // add some margin
                            if delta_a_x * delta_a_x + delta_a_y * delta_a_y <= r * r {
                                model.dragging_a = true;
                                model.touched = true;
                            }

                            // if the user clicked the B dot
                            let delta_b_x = model.point_pos_b.0 - model.cursor_pos.0;
                            let delta_b_y = model.point_pos_b.1 - model.cursor_pos.1;
                            let r = DOT_RADIUS + 3.0; // add some margin
                            if delta_b_x * delta_b_x + delta_b_y * delta_b_y <= r * r {
                                model.dragging_b = true;
                                model.touched = true;
                            }
                        }
                    }
                    // Mouse released event
                    WindowEvent::MouseReleased(btn) => {
                        if btn == MouseButton::Left {
                            model.dragging_a = false;
                            model.dragging_b = false;
                        }
                    }
                    // Mouse moved event (coordinate)
                    WindowEvent::MouseMoved(point) => {
                        model.cursor_pos = (point.x, point.y);

                        if model.dragging_a {
                            // convert nannou coordinate to [0;1]
                            let w = WIDTH as f32;
                            let h = HEIGHT as f32;

                            // xConv * WIDTH as f32 = point.x + (WIDTH as f32) * 0.5

                            let x = (point.x + w * 0.5) / w;
                            let y = function(x) * h - h * 0.5;

                            model.point_pos_a = (point.x, y);
                        } else if model.dragging_b {
                            // convert nannou coordinate to [0;1]
                            let w = WIDTH as f32;
                            let h = HEIGHT as f32;

                            // xConv * WIDTH as f32 = point.x + (WIDTH as f32) * 0.5

                            let x = (point.x + w * 0.5) / w;
                            let y = function(x) * h - h * 0.5;

                            model.point_pos_b = (point.x, y);
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