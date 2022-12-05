#version 330 core

in vec3 pos;
in vec3 normal;
in vec2 texcoord;

out vec3 v_normal;
out vec2 v_tex;
out vec3 FragPos;

uniform mat4 view_proj;
uniform mat4 model_matrix;

void main() {
    gl_Position = view_proj * model_matrix * vec4(pos, 1);
    FragPos = vec3(model_matrix * vec4(pos, 1));
    v_normal = mat3(transpose(inverse(model_matrix))) * normal;
    v_tex = texcoord;
}