#version 430

layout(location = 0) uniform vec3 position;

// defined by Raylib default vert shader
in vec4 fragColor;

layout(location = 0) out vec4 finalColor;

void main()
{
    finalColor = fragColor;
    gl_FragDepth = 0.5 + (position.y - position.z) / 10000.0f;
}
