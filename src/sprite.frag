#version 330 core

uniform sampler2D texture0;

in vec2 texCoord;
in vec4 color;

layout(location = 0) out vec4 fragColor;

void main() {
    vec4 texColor = texture(texture0, texCoord);
    fragColor = color * texColor;
}
