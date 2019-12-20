#version 330 core

in VS_OUTPUT {
    vec2 tex_coord;
    vec3 normal;
    vec3 frag_pos;
} IN;

out vec4 Color;

uniform sampler2D wall;
uniform sampler2D face;
uniform vec3 light_color;
uniform vec3 light_pos;

void main() {
    // Ambient
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    // Diffuse
    const float MAX_DISTANCE = 7.0;
    vec3 normal = normalize(IN.normal);
    vec3 light_direction = normalize(light_pos - IN.frag_pos);
    float light_distance = length(light_pos - IN.frag_pos);
    float diffuse_impact =
        max(dot(normal, light_direction), 0.0) *
        max((1 - light_distance / MAX_DISTANCE), 0.0);
    vec3 diffuse = diffuse_impact * light_color;

    Color = vec4(ambient + diffuse, 1.0) *
            mix(texture(wall, IN.tex_coord), texture(face, IN.tex_coord - 0.5), 0.2);
}
