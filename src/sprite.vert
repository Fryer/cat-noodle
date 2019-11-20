#version 410

layout(location = 0) in vec2 vertPosition;
layout(location = 1) in vec2 vertTexCoord;
layout(location = 2) in vec3 vertColor;

out vec2 texCoord;
out vec3 color;

void main() {
    gl_Position = vec4(vertPosition.xy, 0.0, 1.0);
    texCoord = vertTexCoord;
    color = vertColor;
}
