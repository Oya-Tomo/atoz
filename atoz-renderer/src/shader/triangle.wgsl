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

fn dst_pos_to_line(func: vec3<f32>, x: f32, y: f32) -> f32 {
    return abs(func.x * x + func.y * y + func.z) / sqrt(func.x * func.x + func.y * func.y);
}

fn pnt_to_func(p1: vec2<f32>, p2: vec2<f32>) -> vec3<f32> {
    var res: vec3<f32>;
    // y = ax + b
    var a = (p2.y - p1.y) / (p2.x - p1.x);
    var b = p1.y - a * p1.x;
    return vec3<f32>(a, -1.0, b);
}

const ANTI_ALIASING_WIDTH = 0.5;

struct VertexInput {
    @location(0) index: u32,
    @location(1) point1: vec2<f32>,
    @location(2) point2: vec2<f32>,
    @location(3) point3: vec2<f32>,
    @location(4) thickness: f32,
    @location(5) fill_color: vec4<f32>,
    @location(6) line_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) point1: vec2<f32>,
    @location(1) point2: vec2<f32>,
    @location(2) point3: vec2<f32>,
    @location(3) thickness: f32,
    @location(4) fill_color: vec4<f32>,
    @location(5) line_color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    var points = mat3x2<f32>(
        input.point1,
        input.point2,
        input.point3,
    );

    output.position = vec4<f32>(
        convert_pxl_dcm(points[input.index].x, points[input.index].y), 0.0, 1.0,
    );
    output.point1 = input.point1;
    output.point2 = input.point2;
    output.point3 = input.point3;
    output.thickness = input.thickness;
    output.fill_color = input.fill_color;
    output.line_color = input.line_color;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32>;

    var func_1_2 = pnt_to_func(input.point1, input.point2);
    var func_2_3 = pnt_to_func(input.point2, input.point3);
    var func_3_1 = pnt_to_func(input.point3, input.point1);

    var dst_1_2 = dst_pos_to_line(func_1_2, input.position.x, input.position.y);
    var dst_2_3 = dst_pos_to_line(func_2_3, input.position.x, input.position.y);
    var dst_3_1 = dst_pos_to_line(func_3_1, input.position.x, input.position.y);

    var fill_alpha = smoothstep(
        input.thickness - ANTI_ALIASING_WIDTH,
        input.thickness,
        dst_1_2
    ) * smoothstep(
        input.thickness - ANTI_ALIASING_WIDTH,
        input.thickness,
        dst_2_3
    ) * smoothstep(
        input.thickness - ANTI_ALIASING_WIDTH,
        input.thickness,
        dst_3_1
    );

    return input.fill_color * fill_alpha + input.line_color * (1.0 - fill_alpha);
}