#version 410

uniform sampler2D texture0;

in vec2 texCoord;
in vec3 color;

layout(location = 0) out vec4 fragColor;

void main() {
    vec4 texColor = texture(texture0, texCoord);
    if(texColor.a == 0.) discard;
    fragColor = vec4(color, 1.0) * texColor;
}
