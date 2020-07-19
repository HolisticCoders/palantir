#version 330 core

layout (location = 0) in vec3 va_position;
layout (location = 1) in vec3 va_normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;
uniform vec3 u_color;
uniform vec3 u_light_direction;
uniform vec3 u_light_color;
uniform float u_light_power;
uniform float u_light_ambient_strength;

out VS_OUTPUT {
    vec3 color;
    vec3 fragment_position;
    vec3 fragment_normal;
    vec3 light_color;
    vec3 light_direction;
    float light_power;
    float light_ambient_strength;
} OUT;

void main()
{

    vec4 vertex_position = u_model * vec4(va_position, 1.0);
    gl_Position = u_projection * u_view * vertex_position;

    OUT.color = u_color;

    OUT.fragment_position = vec3(vertex_position);
    OUT.fragment_normal = va_normal;

    OUT.light_ambient_strength = u_light_ambient_strength;
    OUT.light_direction = u_light_direction;
    OUT.light_color = u_light_color;
    OUT.light_power = u_light_power;
}
