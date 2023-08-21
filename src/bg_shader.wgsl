// Author(s): Dylan Turner <dylan.turner@tutanota.com>
// Description: Draw a square background behind text

var<private> pos: array<vec2<f32>, 6> = array(
    vec2<f32>(-0.86, 0.96),
    vec2<f32>(0.86, 0.96),
    vec2<f32>(0.86, -0.95),
    vec2<f32>(0.86, -0.95),
    vec2<f32>(-0.86, -0.95),
    vec2<f32>(-0.86, 0.96)
);

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(pos[vertex_index].x, pos[vertex_index].y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4(0.0, 0.0, 0.6, 1.0);
}

