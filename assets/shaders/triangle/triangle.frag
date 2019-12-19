#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D wall;
uniform sampler2D face;

void main() {
    Color = mix(texture(wall, IN.TexCoord), texture(face, IN.TexCoord - 0.5), 0.2);
}
