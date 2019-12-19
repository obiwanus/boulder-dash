#version 330 core

in VS_OUTPUT {
    vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D wall;
uniform sampler2D face;
uniform vec3 light_color;

void main() {
    Color = vec4(light_color, 1.0) * mix(texture(wall, IN.TexCoord), texture(face, IN.TexCoord - 0.5), 0.2);
}
