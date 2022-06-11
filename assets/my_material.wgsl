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
		return r;
	}

	return vec2<f32>((r.x*u.x + r.y*u.y) / bottom, (r.y*u.x - r.x*u.y) / bottom);
}

fn isGaussianInteger(r: vec2<f32>) -> bool {
    var rx: f32 = fract(r.x);
	var ry: f32 = fract(r.y);

	if ((rx < 0.1 || rx > 0.9) && (ry < 0.1 || ry > 0.9)) {
		return true;
	}
	return false;
}

[[stage(fragment)]]
fn fragment(at: VertexOutput) -> [[location(0)]] vec4<f32> {

    var upto: i32 = 100;
    var seenTotal: i32 = 0;
    var seenGaussian: i32 = 0;

    {
      var i: i32 = 0;
      loop {
        if (i >= upto) {
          break;
        }

        var compare = vec2<f32>(f32(i), f32(i));

        var divided: vec2<f32> = divideComplex(compare, at.uv);

        if (isGaussianInteger(divided)) {
            seenGaussian = seenGaussian + 1;
        }
        seenTotal = seenTotal + 1;

        continuing {
          i = i + 1;
        }
      }
    }

    var gaussianRatio: f32 = f32(25 * seenGaussian / seenTotal);
    var color = vec3<f32>(0.2 * gaussianRatio, 0.2 * gaussianRatio, 1.2 * gaussianRatio);

    var output_color = vec4<f32>(color, 1.0);
    return output_color;
}


