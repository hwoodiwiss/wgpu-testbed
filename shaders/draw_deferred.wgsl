struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coord: vec2<f32>;
};


struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vertex_main(model: VertexInput) -> VertexOutput {
	var out: VertexOutput;
	out.position = in.position;
	out.tex_coord = in.tex_coord;
	return out;
}

[[group(0), binding(0)]]
var frame_texture: texture_2d<f32>;
[[group(0), binding(1)]]
var frame_sampler: sampler;

[[stage(fragment)]]
fn fragment_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	let object_colour: vec4<f32> = textureSample(frame_texture, frame_sampler, in.tex_coords);
	return object_colour;
}