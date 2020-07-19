#version 330 core

uniform vec3 u_color;
uniform vec3 u_light_direction;
uniform vec3 u_light_color;
uniform float u_light_power;
uniform float u_light_ambient_strength;

in VS_OUTPUT {
    vec3 fragment_normal;
} IN;

out vec4 fragment_color;

void main()
{
    float light_value = max(dot(IN.fragment_normal, u_light_direction), 0.0);

    vec3 light_ambient = u_light_ambient_strength * u_light_color;
    vec3 light_diffuse = u_light_color * light_value * u_light_power;

    fragment_color = vec4(u_color * (light_ambient + light_diffuse), 1.0f);
}
