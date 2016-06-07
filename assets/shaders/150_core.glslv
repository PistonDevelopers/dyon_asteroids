#version 150 core

in vec3 pos;
in vec3 norm;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(pos, 1.0);
}
