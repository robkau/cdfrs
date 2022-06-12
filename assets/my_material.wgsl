//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

fn divideComplex(r: vec2<f32>, u: vec2<f32>) -> vec2<f32> {
    var bottom: f32 = u.x*u.x + u.y*u.y;
	if (bottom == 0.0) {
		//return r;
		return vec2<f32>(0.0, 0.0);
	}

	return vec2<f32>((r.x*u.x + r.y*u.y) / bottom, (r.y*u.x - r.x*u.y) / bottom);
}

fn isGaussianInteger(r: vec2<f32>) -> bool {
    var rx: f32 = fract(r.x);
	var ry: f32 = fract(r.y);

	//if (i32(r.x) % 7 == 3 && i32(r.y) % 7 == 0) {
	//    return true;
	//}

	if ((rx < 0.1 || rx > 0.9) && (ry < 0.1 || ry > 0.9)) {
		return true;
	}
	return false;
}

// todo pass in number of iterations. https://github.com/bevyengine/bevy/blob/main/assets/shaders/animate_shader.wgsl https://github.com/bevyengine/bevy/blob/main/examples/shader/animate_shader.rs
// todo pass in camera offset and scale.
// todo pass in color hint
[[stage(fragment)]]
fn fragment(at: VertexOutput) -> [[location(0)]] vec4<f32> {

    //var scale: f32 = 0.1;
    //at.uv.x = at.uv.x * scale;
    //at.uv.y = at.uv.y * scale;

    var upto: i32 = 3;
    var seenTotal: i32 = 0;
    var seenGaussian: i32 = 0;

    {
      var i: i32 = 0;
      loop {
        if (i >= upto) {
          break;
        }


        {
              //var j: i32 = 0;
              //loop {
              //  if (j >= upto) {
              //    break;
              //  }
                var compare = vec2<f32>(f32(i), f32(i));
                var divided: vec2<f32> = divideComplex(compare, at.uv);
                if (isGaussianInteger(divided)) {
                    seenGaussian = seenGaussian + 1;
                }
                seenTotal = seenTotal + 1;
              //  continuing {
              //    j = j + 1;
              //  }
              //}
        }
        continuing {
          i = i + 1;
        }
      }
    }

    var gaussianRatio: f32 = f32(seenGaussian) / f32(seenTotal);
    var r: f32 = gaussianRatio;
    var g: f32 = gaussianRatio;
    var b: f32 =gaussianRatio;

    var color = vec3<f32>(clamp(r, 0.0, 1.0), clamp(g, 0.0, 1.0), clamp(b, 0.0, 1.0));

    var output_color = vec4<f32>(color, 1.0);
    return output_color;
}


