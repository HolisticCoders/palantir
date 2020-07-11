#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 light_position;
uniform vec3 light_color;
uniform float light_power;
uniform float light_ambient_strength;

out VS_OUTPUT {
    vec3 FragmentPosition;
    vec3 FragmentColor;
    vec3 FragmentNormal;
    vec3 LightColor;
    vec3 LightPosition;
    float LightPower;
    float LightAmbientStrength;
} OUT;

void main()
{

    vec4 vertex_position = model * vec4(Position, 1.0);
    gl_Position = projection * view * vertex_position;

    OUT.FragmentPosition = vec3(vertex_position);
    OUT.FragmentColor = Color;
    OUT.FragmentNormal = Normal;

    OUT.LightAmbientStrength = light_ambient_strength;
    OUT.LightPosition = light_position;
    OUT.LightColor = light_color;
    OUT.LightPower = light_power;
}
