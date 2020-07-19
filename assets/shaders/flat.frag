#version 330 core

out vec4 Color;

in VS_OUTPUT {
    vec3 color;
} IN;


void main()
{
    Color = vec4(IN.color, 1.0f);
    // Color = vec4(1.0f, 0.0f, 1.0f, 1.0f);
}
