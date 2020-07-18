#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 color;

out VS_OUTPUT {
    vec3 Color;
} OUT;


void main()
{
    vec4 vertex_position = model * vec4(Position, 1.0);
    OUT.Color = color;
    gl_Position = projection * view * vertex_position;
}
