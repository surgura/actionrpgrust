#version 430

layout(location = 0) uniform vec3 position;

in vec4 fragColor;

out vec4 finalColor;

void main()
{
    finalColor = vec4(position.xyz / 400.0,1);
    //finalColor = fragColor;
}
