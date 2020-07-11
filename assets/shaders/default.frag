#version 330 core

in VS_OUTPUT {
    vec3 FragmentPosition;
    vec3 FragmentColor;
    vec3 FragmentNormal;
    vec3 LightColor;
    vec3 LightPosition;
    float LightPower;
    float LightAmbientStrength;
} IN;

out vec4 Color;

void main()
{
    vec3 light_direction = normalize(IN.LightPosition - IN.FragmentPosition);
    float light_value = max(dot(IN.FragmentNormal, light_direction), 0.0);

    vec3 light_ambient = IN.LightAmbientStrength * IN.LightColor;
    vec3 light_diffuse = IN.LightColor * light_value * IN.LightPower;

    Color = vec4(IN.FragmentColor * (light_ambient + light_diffuse), 1.0f);
}
