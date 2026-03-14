#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> hover: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	let x = mesh.uv.x - 0.5;
	let y = mesh.uv.y - 0.5;
	let distance_from_origin = sqrt(x * x + y * y);

	if distance_from_origin > 0.5 {
		return vec4f(0., 0., 0., 0.);
	} else if distance_from_origin > 0.35 {
		return vec4f(0.05, 0.05, 0.05, 1.);
	} else {
		let icon_scale = 1.3;
		let uv = (mesh.uv - vec2f(0.5, 0.5)) * icon_scale + vec2f(0.5, 0.5);
		var color = textureSample(base_color_texture, base_color_sampler, uv);
		if color.a == 0. {
			return vec4f(0.1, 0.1, 0.1, 1.);
		} else {
			color.r = 0.5 * hover;
			color.g = 0.5 * hover;
			return color;
		}
	}
}
