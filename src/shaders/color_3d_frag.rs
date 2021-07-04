pub const SHADER: &str = r#"#version 300 es
precision mediump float;

in vec4 v_position;
out vec4 fragColor;

void main() {
    fragColor = vec4(v_position.xyz * 0.5 + 0.5, 1.0);
}
"#;