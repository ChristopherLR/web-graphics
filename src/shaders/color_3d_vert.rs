pub const SHADER: &str = r#"#version 300 es

layout(location = 0) in vec3 a_position;
out vec3 pos;
uniform mat4 u_transform;

void main() {
  gl_Position = vec4(a_position, 1.0) * u_transform;
  pos = a_position;
}
"#;