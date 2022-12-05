#version 330 core

uniform float offset;
uniform sampler2D tex;
in vec2 texcoord;

void main() {
    vec2 tex_coords = texcoord;
  tex_coords.x += sin(tex_coords.y * 4*2*3.14159 + offset) / 100;
  gl_FragColor = texture2D(tex, tex_coords);
}