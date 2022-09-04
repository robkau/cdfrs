struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
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

	if ((rx < 0.1 || rx > 0.9) && (ry < 0.1 || ry > 0.9)) {
		return true;
	}
	return false;
}

// returns rgb from grayscale value and iteration count
fn colorize(v: f32, current_iteration: i32, max_iterations: i32) -> vec3<f32> {
    var


    hsv = [powf((i / max) * 360, 1.5) % 360, 100, (i / max) * 100]
}




struct ComplexDivisorFractalDoubleLoop {
    iterations: i32,
}

@group(1) @binding(0)
var <uniform> material: ComplexDivisorFractalDoubleLoop;

@fragment
fn fragment(at: VertexOutput) -> @location(0) vec4<f32> {
    var upto: i32 = material.iterations;
    var seenTotal: i32 = 0;
    var seenGaussian: i32 = 0;

    {
      var i: i32 = 0;
      loop {
        if (i >= upto) {
          break;
        }

        {
              var j: i32 = 0;
              loop {
                if (j >= upto) {
                  break;
                }
                var compare = vec2<f32>(f32(i), f32(j));
                var divided: vec2<f32> = divideComplex(compare, at.uv);
                if (isGaussianInteger(divided)) {
                    seenGaussian = seenGaussian + 1;
                }
                seenTotal = seenTotal + 1;
                continuing {
                  j = j + 1;
                }
              }
        }
        continuing {
          i = i + 1;
        }
      }
    }

    var gaussianRatioScale: f32 = 3.0;
    var gaussianRatio: f32 = gaussianRatioScale * f32(seenGaussian) / f32(seenTotal);
    var r: f32 = gaussianRatio;
    var g: f32 = gaussianRatio;
    var b: f32 = gaussianRatio;

    var rc: f32 = clamp(r, 0.0, 1.0);
    var gc: f32 = clamp(g, 0.0, 1.0);
    var bc: f32 = clamp(b, 0.0, 1.0);

    // colorize
    if (gaussianRatio < 0.2) {
        gc = 0.;
    }
    if (gaussianRatio < 0.7) {
        bc = 0.;
    }

    var color = vec3<f32>(rc, gc, bc);

    var output_color = vec4<f32>(color, 1.0);
    return output_color;
}


