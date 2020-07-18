#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec3 FragmentPosition;
    vec3 FragmentNormal;
    vec3 LightColor;
    vec3 LightDirection;
    float LightPower;
    float LightAmbientStrength;
} IN;

out vec4 Color;

void main()
{
    float light_value = max(dot(IN.FragmentNormal, IN.LightDirection), 0.0);

    vec3 light_ambient = IN.LightAmbientStrength * IN.LightColor;
    vec3 light_diffuse = IN.LightColor * light_value * IN.LightPower;

    Color = vec4(IN.Color * (light_ambient + light_diffuse), 1.0f);
}
