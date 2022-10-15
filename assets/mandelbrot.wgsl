struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(at: VertexOutput) -> @location(0) vec4<f32> {
    //    x0 := scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.00, 0.47))
    //    y0 := scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1.12, 1.12))
    if (at.uv.x < -2.00 || at.uv.x > 0.47) {
          var output_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
          return output_color;
    }
    if (at.uv.y < -1.12 || at.uv.y > 1.12) {
          var output_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
          return output_color;
    }

    // todo https://blocr.github.io/posts/fractal_generation.html

    var x = 0.0;
    var y = 0.0;
    var iteration = 0;
    var max_iteration = 1000;

    loop {
        if (x*x + y*y > 2.0*2.0 || iteration >= max_iteration) {
            break;
        }

        var xtemp = x*x - y*y + at.uv.x;
        y = 2.0*x*y + at.uv.y;
        x = xtemp;

        continuing {
          iteration = iteration + 1;
        }
    }

    var color_value = f32(iteration%255) / 255.;

    var output_color = vec4<f32>(color_value, color_value, color_value, 1.0);
    return output_color;
}




