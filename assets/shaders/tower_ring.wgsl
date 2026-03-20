#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	let x = mesh.uv.x - 0.5;
	let y = mesh.uv.y - 0.5;
	let distance_from_origin = sqrt(x * x + y * y);

	if distance_from_origin < 0.5 && distance_from_origin > 0.4 || abs(y) < 0.06 {
		return vec4f(0.1, 0.05, 0.1, 1.);
	} else {
		return vec4f(0., 0., 0., 0.);
	}
}
