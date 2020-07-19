#version 330 core

layout (location = 0) in vec3 va_position;
layout (location = 1) in vec3 va_normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out VS_OUTPUT {
    vec3 fragment_normal;
} OUT;

void main()
{

    vec4 vertex_position = u_model * vec4(va_position, 1.0);
    gl_Position = u_projection * u_view * vertex_position;

    OUT.fragment_normal = va_normal;
}
