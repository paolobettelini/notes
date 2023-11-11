use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use nannou::winit;
use std::cell::RefCell;

struct Model {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    //
    uniforms: Uniforms,
    dragging: bool,
    last_cursor_pos: (f32, f32),
}

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
// Data size must be at least UNIFORM_BLOCK_DATA_SIZE (32 bytes)
pub struct Uniforms {
    scale: f32,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    dummy1: f32,
    dummy2: f32,
    dummy3: f32,
}

fn uniforms_as_bytes(uniforms: &Uniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(uniforms) }
}

// The vertices that make up our triangle.
const VERTICES: [Vertex; 6] = [
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
];

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
        .view(view)
        .build_async()
        .await
        .unwrap();

    // The gpu device associated with the window's swapchain
    let window = app.window(w_id).unwrap();
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let sample_count = window.msaa_samples();

    // Load shader modules.
    let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
    let fs_desc = wgpu::include_wgsl!("shaders/fs.wgsl");
    let vs_mod = device.create_shader_module(&vs_desc);
    let fs_mod = device.create_shader_module(&fs_desc);

    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsages::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });

    let uniforms = Uniforms {
        scale: 0.005,
        center_x: 0.0,
        center_y: 0.0,
        width: width as f32,
        height: height as f32,
        dummy1: 1.0,
        dummy2: 1.0,
        dummy3: 1.0,
    };

    let uniforms_bytes = uniforms_as_bytes(&uniforms);
    let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
    let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: uniforms_bytes,
        usage,
    });

    // Create the render pipeline.
    let bind_group_layout = create_bind_group_layout(device);
    let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);
    let pipeline_layout = wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .build(device);

    Model {
        bind_group,
        vertex_buffer,
        render_pipeline,
        uniform_buffer,
        uniforms,
        dragging: false,
        last_cursor_pos: (0.0, 0.0),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {}

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
                    // Mouse scroll event
                    WindowEvent::MouseWheel(delta, touch_phase) => {
                        if let MouseScrollDelta::PixelDelta(phyiscal_pos) = delta {
                            // Change scale
                            let old_scale = model.uniforms.scale; 
                            model.uniforms.scale *= if phyiscal_pos.y > 0.0 { 0.9 } else { 1.1 };

                            // Move the center such that the zoom approaches the cursor.
                            // Note that y is inverted
                            let old_point_x =  model.last_cursor_pos.0 * old_scale + model.uniforms.center_x;
                            let old_point_y = -model.last_cursor_pos.1 * old_scale + model.uniforms.center_y;
                            let new_point_x =  model.last_cursor_pos.0 * model.uniforms.scale + model.uniforms.center_x;
                            let new_point_y = -model.last_cursor_pos.1 * model.uniforms.scale + model.uniforms.center_y;

                            // Update the center position based on the zoom and cursor position
                            model.uniforms.center_x -= new_point_x - old_point_x;
                            model.uniforms.center_y -= new_point_y - old_point_y;
                        }
                    }
                    // Mouse moved event (coordinate)
                    WindowEvent::MouseMoved(point) => {
                        model.last_cursor_pos = (point.x, point.y);
                    }
                    // Windows resize (e.g. scale)
                    WindowEvent::Resized(size) => {
                        // TODO fix zoom in this case
                    }
                    _ => {}
                }
            }
        }
        Event::DeviceEvent(_device_id, event) => match event {
            // Mouse move event (delta)
            winit::event::DeviceEvent::MouseMotion { delta } => {
                if model.dragging {
                    model.uniforms.center_x -= model.uniforms.scale * delta.0 as f32;
                    model.uniforms.center_y -= model.uniforms.scale * delta.1 as f32;
                }
            }
            _ => {}
        },
        _ => {}
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Using this we will encode commands that will be submitted to the GPU.
    let mut encoder = frame.command_encoder();
    let uniforms = model.uniforms;

    // Update
    let device = frame.device_queue_pair().device();
    let uniforms_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;
    let uniforms_bytes = uniforms_as_bytes(&uniforms);
    let usage = wgpu::BufferUsages::COPY_SRC;
    let new_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: uniforms_bytes,
        usage,
    });
    let mut g = &model.uniform_buffer;
    encoder.copy_buffer_to_buffer(&new_uniform_buffer, 0, &g, 0, uniforms_size);

    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        .begin(&mut encoder);

    render_pass.set_bind_group(0, &model.bind_group, &[]);
    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));

    // We want to draw the whole range of vertices, and we're only drawing one instance of them.
    let vertex_range = 0..VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);

    // Now we're done! The commands we added will be submitted after `view` completes.
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .buffer::<Uniforms>(uniform_buffer, 0..1)
        .build(device, layout)
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    };
    device.create_pipeline_layout(&desc)
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            false,
        )
        .build(device)
}
