[[block]]
struct Uniforms {
    scale: f32;
    center_x: f32;
    center_y: f32;
    width: f32;
    height: f32;
    dummy1: f32;
    dummy2: f32;
    dummy3: f32;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[stage(fragment)]]
fn main([[builtin(position)]] coord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    // Map the coordinate to the range of the Mandelbrot set
    let x: f32 = (coord.x - uniforms.width * 0.5) * uniforms.scale + uniforms.center_x;
    let y: f32 = (coord.y - uniforms.height * 0.5) * uniforms.scale + uniforms.center_y;

    // Mandelbrot set calculation
    var z_re: f32 = 0.0;
    var z_im: f32 = 0.0;
    let max_iterations: i32 = 100; // Increase this value for more detailed images

    var i: i32 = 0;
    for (; i < max_iterations;) {
        let z_re_temp = z_re * z_re - z_im * z_im + x;
        z_im = 2.0 * z_re * z_im + y;
        z_re = z_re_temp;
        if (z_re * z_re + z_im * z_im > 4.0) {
            break;
        }
        i = i + 1;
    }

    // Color the pixel based on the number of iterations
    let v: f32 = f32(i) / f32(max_iterations);
    let color: vec3<f32> = vec3<f32>(1.0 - v);

    // Output the color as a vec4<f32> with alpha set to 1.0
    return vec4<f32>(color, 1.0);
}