#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D Texture;

void main() {
    Color = texture(Texture, IN.TexCoord);
}
