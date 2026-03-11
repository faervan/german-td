#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::decal::forward::get_forward_decal_info

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> hover: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	let decal_info = get_forward_decal_info(mesh);
	let decal_alpha = vec4f(1., 1., 1., decal_info.alpha);

	let x = decal_info.uv.x - 0.5;
	let y = decal_info.uv.y - 0.5;
	let distance_from_origin = sqrt(x * x + y * y);

	let hovered_color = vec3f(0.2, 0.2, 0.);
	let unhovered_color = vec3f(0.1, 0.05, 0.);

	if hover == 1. {
		return draw(distance_from_origin, hovered_color) * decal_alpha;
	} else if hover == 0. {
		return draw(distance_from_origin, unhovered_color) * decal_alpha;
	}

	let hovered = draw(distance_from_origin, hovered_color);
	let unhovered = draw(distance_from_origin, unhovered_color);
	return (hovered * hover + unhovered * (1. - hover)) * decal_alpha;
}

fn draw(distance_from_origin: f32, base_color: vec3<f32>) -> vec4<f32> {
	// Distance of the ring from the origin
	let ring = 0.3;

	let distance_from_ring = abs(distance_from_origin - ring);
	// The "strong" part of the ring with full opacity
	let strong = 0.01;
	// The padding around the strong ring, with full opaqueness adjacent to the strong ring,
	// which fades into full opacity
	let fading = 0.15;

	let inner_opacity = 0.4;

	if distance_from_ring < strong {
		return vec4f(base_color, 1.);
	} else if distance_from_ring < fading {
		let alpha = 1. - (distance_from_ring - strong) / (fading - strong);
		if alpha < inner_opacity && distance_from_origin < ring {
			return vec4f(base_color, inner_opacity);
		}
		return vec4f(base_color, alpha);
	} else if distance_from_origin > ring {
		return vec4f(0., 0., 0., 0.);
	} else {
		return vec4f(base_color, inner_opacity);
	}
}
