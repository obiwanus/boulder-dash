#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec4 Position;
} IN;

out vec4 Color;

uniform vec4 solid_color;

void main() {
    Color = IN.Position;
}
