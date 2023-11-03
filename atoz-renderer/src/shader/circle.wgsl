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
            input.center.x + (input.radius + 1.0) * input.position.x,
            input.center.y + (input.radius + 1.0) * input.position.y,
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

    var diff_1 = vec2<f32>(
        input.center.x - (input.position.x + 0.25),
        input.center.y - (input.position.y + 0.25),
    );
    var diff_2 = vec2<f32>(
        input.center.x - (input.position.x + 0.25),
        input.center.y - (input.position.y + 0.75),
    );
    var diff_3 = vec2<f32>(
        input.center.x - (input.position.x + 0.75),
        input.center.y - (input.position.y + 0.25),
    );
    var diff_4 = vec2<f32>(
        input.center.x - (input.position.x + 0.75),
        input.center.y - (input.position.y + 0.75),
    );
    var radiuses = vec4<f32>(
        sqrt(
            diff_1.x * diff_1.x + diff_1.y * diff_1.y
        ),
        sqrt(
            diff_2.x * diff_2.x + diff_2.y * diff_2.y
        ),
        sqrt(
            diff_3.x * diff_3.x + diff_3.y * diff_3.y
        ),
        sqrt(
            diff_4.x * diff_4.x + diff_4.y * diff_4.y
        ),
    );
    var fill_alpha = (
        step(radiuses[0], input.radius - input.thickness) +
        step(radiuses[1], input.radius - input.thickness) +
        step(radiuses[2], input.radius - input.thickness) +
        step(radiuses[3], input.radius - input.thickness)
    ) / 4.0;

    var line_alpha = (
        step(radiuses[0], input.radius) +
        step(radiuses[1], input.radius) +
        step(radiuses[2], input.radius) +
        step(radiuses[3], input.radius)
    ) / 4.0 - fill_alpha;

    color = line_alpha * input.line_color + fill_alpha * input.fill_color;

    return color;
}