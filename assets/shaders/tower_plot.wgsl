#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> hovered: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	let x = mesh.uv.x - 0.5;
	let y = mesh.uv.y - 0.5;
	if sqrt(x * x + y * y) > 0.5 {
		return vec4f(0., 0., 0., 0.);
	} else if hovered == 1. {
		return vec4f(1., 0., 0., 1.);
	} else {
		return vec4f(0., 0., 0., 1.);
	}
}
