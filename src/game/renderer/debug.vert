#version 330 core

uniform mat3x2 transform;

layout(location = 0) in vec2 vertPosition;
layout(location = 1) in vec4 vertColor;

out vec4 color;

void main() {
    gl_Position = vec4(transform * vec3(vertPosition, 1.0), 0.0, 1.0);
    color = vertColor;
}
