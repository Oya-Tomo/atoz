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

fn smoothstep_x2(n1: f32, n2: f32, n3: f32, n4: f32, x: f32) -> f32 {
    return smoothstep(n1, n2, x) * (1.0 - smoothstep(n3, n4, x));
}

const ANTI_ALIASING_WIDTH = 0.5;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) thickness: f32,
    @location(4) fill_color: vec4<f32>,
    @location(5) line_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) center: vec2<f32>,
    @location(1) radius: f32,
    @location(2) thickness: f32,
    @location(3) fill_color: vec4<f32>,
    @location(4) line_color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(
        convert_pxl_dcm(
            input.center.x + input.radius * input.position.x,
            input.center.y + input.radius * input.position.y,
        ), 0.0, 1.0,
    );
    output.center = input.center;
    output.radius = input.radius;
    output.thickness = input.thickness;
    output.fill_color = input.fill_color;
    output.line_color = input.line_color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32>;

    var diff_x = input.center.x - input.position.x;
    var diff_y = input.center.y - input.position.y;
    var current_radius = sqrt(
        diff_x * diff_x + diff_y * diff_y
    );

    var line_alpha = smoothstep_x2(
        input.radius - input.thickness - ANTI_ALIASING_WIDTH,
        input.radius - input.thickness,
        input.radius - ANTI_ALIASING_WIDTH,
        input.radius,
        current_radius,
    );
    var fill_alpha = 1.0 - smoothstep(
        input.radius - input.thickness - ANTI_ALIASING_WIDTH,
        input.radius - input.thickness,
        current_radius,
    );

    color = line_alpha * input.line_color + fill_alpha * input.fill_color;

    return color;
}