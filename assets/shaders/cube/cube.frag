#version 330 core

in VS_OUTPUT {
    vec2 tex_coord;
    vec3 normal;
    vec3 frag_pos;
} IN;

out vec4 Color;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    vec3 position;
};

uniform Material material;
uniform Light light;

void main() {
    const float MAX_DISTANCE = 20.0;

    vec3 normal = normalize(IN.normal);
    vec3 light_direction = normalize(light.position - IN.frag_pos);
    vec3 view_direction = normalize(-IN.frag_pos);
    float light_distance = length(light.position - IN.frag_pos);
    float distance_effect = min(mix(1.0, 0.1, light_distance / MAX_DISTANCE), 1.0);

    vec3 material_color = texture(material.diffuse, IN.tex_coord).xyz;
    vec3 material_specular = texture(material.specular, IN.tex_coord).xyz;

    // Diffuse
    float diffuse_impact = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = material_color * diffuse_impact * light.diffuse;

    // Ambient
    vec3 ambient = material_color * light.ambient;

    // Specular
    vec3 reflection = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflection), 0.0), material.shininess);
    vec3 specular = material_specular * spec * light.specular;

    Color = vec4(ambient + (diffuse + specular) * distance_effect, 1.0);
}
