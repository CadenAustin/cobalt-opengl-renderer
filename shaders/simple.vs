#version 330 core

uniform mat4 model_matrix;
uniform mat4 proj_view;

in vec3 pos;

void main() {
    gl_Position = model_matrix * proj_view * vec4(pos, 1);
}