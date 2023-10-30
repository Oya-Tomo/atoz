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

// radius: [top-left, bottom-left, bottom-right, top-left]

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) start: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) thickness: f32,
    @location(4) radius: vec4<f32>,
    @location(5) fill_color: vec4<f32>,
    @location(6) line_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) start: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) thickness: f32,
    @location(3) radius: vec4<f32>,
    @location(4) fill_color: vec4<f32>,
    @location(5) line_color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(
        convert_pxl_dcm(
            input.start.x + input.position.x * input.size.x,
            input.start.y + input.position.y * input.size.y,
        ), 0.0, 1.0,
    );
    output.start = input.start;
    output.size = input.size;
    output.thickness = input.thickness;
    output.radius = input.radius;
    output.fill_color = input.fill_color;
    output.line_color = input.line_color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32>;

    var min_radius = min(input.size.x, input.size.y) / 2.0;
    var radius = vec4<f32>(
        min(min_radius, input.radius.x),
        min(min_radius, input.radius.y),
        min(min_radius, input.radius.z),
        min(min_radius, input.radius.w),
    );

    var round_centers = mat4x2<f32>(
        input.start.x + radius.x, input.start.y + radius.x,
        input.start.x + radius.y, input.start.y + input.size.y - radius.y,
        input.start.x + input.size.x - radius.z, input.start.y + input.size.y - radius.z,
        input.start.x + input.size.x - radius.w, input.start.y + radius.w,
    );

    var in_top_left = step(input.position.x, round_centers[0][0]) * step(input.position.y, round_centers[0][1]);
    var in_btm_left = step(input.position.x, round_centers[1][0]) * step(round_centers[1][1], input.position.y);
    var in_btm_right = step(round_centers[2][0], input.position.x) * step(round_centers[2][1], input.position.y);
    var in_top_right = step(round_centers[3][0], input.position.x) * step(input.position.y, round_centers[3][1]);

    if in_top_left + in_btm_left + in_btm_right + in_top_right > 0.0 { // rounded corner
        var selected_center = round_centers[0] * in_top_left + round_centers[1] * in_btm_left + round_centers[2] * in_btm_right + round_centers[3] * in_top_right;
        var selected_radius = radius.x * in_top_left + radius.y * in_btm_left + radius.z * in_btm_right + radius.w * in_top_right;
        var diff_x = selected_center.x - input.position.x;
        var diff_y = selected_center.y - input.position.y;
        var current_radius = sqrt(
            diff_x * diff_x + diff_y * diff_y
        );

        var fill_alpha = 1.0 - smoothstep(
            selected_radius - input.thickness - ANTI_ALIASING_WIDTH,
            selected_radius - input.thickness,
            current_radius,
        );
        var line_alpha = smoothstep_x2(
            selected_radius - input.thickness - ANTI_ALIASING_WIDTH,
            selected_radius - input.thickness,
            selected_radius - ANTI_ALIASING_WIDTH,
            selected_radius,
            current_radius,
        );
        color = fill_alpha * input.fill_color + line_alpha * input.line_color;
    } else {
        var fill_alpha = smoothstep_x2(
            input.start.x + input.thickness,
            input.start.x + input.thickness + ANTI_ALIASING_WIDTH,
            input.start.x + input.size.x - input.thickness - ANTI_ALIASING_WIDTH,
            input.start.x + input.size.x - input.thickness,
            input.position.x,
        ) * smoothstep_x2(
            input.start.y + input.thickness,
            input.start.y + input.thickness + ANTI_ALIASING_WIDTH,
            input.start.y + input.size.y - input.thickness - ANTI_ALIASING_WIDTH,
            input.start.y + input.size.y - input.thickness,
            input.position.y,
        );
        var line_alpha = 1.0 - fill_alpha;
        color = fill_alpha * input.fill_color + line_alpha * input.line_color;
    }

    return color;
}

