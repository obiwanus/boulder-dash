#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(IN.Color.g, IN.Color.r, 0.5f, 1.0f);
}
