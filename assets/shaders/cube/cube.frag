#version 330 core

in VS_OUTPUT {
    vec2 tex_coord;
    vec3 normal;
    vec3 frag_pos;
} IN;

out vec4 Color;

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

uniform sampler2D wall;
uniform sampler2D face;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform Material material;

void main() {
    const float MAX_DISTANCE = 20.0;

    vec3 normal = normalize(IN.normal);
    vec3 light_direction = normalize(light_pos - IN.frag_pos);
    vec3 view_direction = normalize(-IN.frag_pos);
    float light_distance = length(light_pos - IN.frag_pos);
    float distance_effect = min(mix(1.0, 0.1, light_distance / MAX_DISTANCE), 1.0);

    // Diffuse
    float diffuse_impact = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = material.diffuse * diffuse_impact * light_color;

    // Ambient
    vec3 ambient = material.ambient * light_color;

    // Specular
    vec3 reflection = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflection), 0.0), 32);
    vec3 specular = material.specular * spec * light_color;

    Color = vec4(ambient + (diffuse + specular) * distance_effect, 1.0) *
            mix(texture(wall, IN.tex_coord), texture(face, IN.tex_coord - 0.5), 0.2);
}
