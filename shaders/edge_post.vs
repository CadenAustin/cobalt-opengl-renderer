#version 330 core
in vec2 pos;
in vec2 uv;
out vec2 v_pos;

void main() {
    gl_Position = vec4(pos, 0., 1.);
    v_pos = pos;
}