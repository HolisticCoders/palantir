#vertex

#version 330 core

layout (location = 0) in vec3 va_position;
layout (location = 1) in vec3 va_normal;
layout (location = 2) in vec2 va_texture_coordinates;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out VS_OUTPUT {
    vec3 fragment_normal;
    vec2 texture_coordinates;
} OUT;

void main()
{

    vec4 vertex_position = u_model * vec4(va_position, 1.0);
    gl_Position = u_projection * u_view * vertex_position;

    OUT.fragment_normal = va_normal;
    OUT.texture_coordinates = va_texture_coordinates;
}

#fragment

#version 330 core
struct Material {
    vec3 diffuse;
    sampler2D diffuse_texture;
    bool use_diffuse_texture;
};

// uniform vec3 u_color;
// uniform sampler2D u_texture;
// uniform bool u_use_texture;
uniform Material material;

uniform vec3 u_light_direction;
uniform vec3 u_light_color;
uniform float u_light_power;
uniform float u_light_ambient_strength;

in VS_OUTPUT {
    vec3 fragment_normal;
    vec2 texture_coordinates;
} IN;

out vec4 fragment_color;

void main()
{
    float light_value = max(dot(IN.fragment_normal, u_light_direction), 0.0);

    vec3 light_ambient = u_light_ambient_strength * u_light_color;
    vec3 light_diffuse = u_light_color * light_value * u_light_power;

    vec4 color;
    if (material.use_diffuse_texture) {
        color = texture(material.diffuse_texture, IN.texture_coordinates);
    } else {
        color = vec4(material.diffuse, 1.0);
    }
    fragment_color = color * vec4(light_ambient + light_diffuse, 1.0);
}
