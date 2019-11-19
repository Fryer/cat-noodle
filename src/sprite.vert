#version 410

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 vertColor;

layout(location = 0) out vec3 color;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    color = vertColor;
}
