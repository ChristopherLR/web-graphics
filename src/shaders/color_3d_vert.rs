pub const SHADER: &str = r#"#version 300 es

in vec3 a_position;

uniform mat4 u_model;
uniform mat4 u_proj;
uniform mat4 u_view;

out vec4 v_position;

void main() {
  vec4 a_position = vec4(a_position, 1.0);
  gl_Position = u_proj * u_view * u_model * a_position;

  v_position = a_position;
}
"#;