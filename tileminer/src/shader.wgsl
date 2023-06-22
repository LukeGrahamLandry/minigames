
@vertex
fn vs_main(
    in: VertexInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // TODO: just do these divisions in the scale factor cpu side
    out.clip_position = vec4(in.pos.x / 800.0 - 1.0, in.pos.y / -600.0 + 1.0, 0.0, 1.0);
    out.colour = in.colour;
    return out;
}

struct VertexInput {
    @location(0) colour: vec4<f32>,
    @location(1) pos: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) colour: vec4<f32>
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.colour;
}
