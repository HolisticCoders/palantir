#version 330 core

in VS_OUTPUT {
    vec3 color;
    vec3 fragment_position;
    vec3 fragment_normal;
    vec3 light_color;
    vec3 light_direction;
    float light_power;
    float light_ambient_strength;
} IN;

out vec4 Color;

void main()
{
    float light_value = max(dot(IN.fragment_normal, IN.light_direction), 0.0);

    vec3 light_ambient = IN.light_ambient_strength * IN.light_color;
    vec3 light_diffuse = IN.light_color * light_value * IN.light_power;

    Color = vec4(IN.color * (light_ambient + light_diffuse), 1.0f);
}
