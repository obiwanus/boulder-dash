#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D texture0;
uniform sampler2D texture1;

void main() {
    Color = mix(texture(texture0, IN.TexCoord), texture(texture1, IN.TexCoord - 0.5), 0.2);
}
