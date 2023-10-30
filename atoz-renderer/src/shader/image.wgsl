struct Viewport {
    width: f32,
    height: f32,
}

@group(0) @binding(0) var<uniform> viewport : Viewport;

fn convert_pxl_dcm(x: f32, y: f32) -> vec2<f32> {
    var pos: vec2<f32>;
    pos.x = (x - viewport.width / 2.0) / (viewport.width / 2.0);
    pos.y = - (y - viewport.height / 2.0) / (viewport.height / 2.0);
    return pos;
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) start: vec2<f32>,
    @location(2) size: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.tex_coords = input.position;
    output.position = vec4<f32>(
        convert_pxl_dcm(
            input.start.x + input.position.x * input.size.x,
            input.start.y + input.position.y * input.size.y,
        ), 0.0, 1.0,
    );
    return output;
}

@group(1) @binding(0)
var texture_image: texture_2d<f32>;
@group(1)@binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_image, texture_sampler, input.tex_coords);
}