#version 450 core

layout (location = 0) in vec3 oColor;
layout (location = 1) in vec2 oUv;

layout (location = 0) out vec4 fColor;

layout (binding = 0) uniform sampler2D texBrickWall;

void main()
{
	fColor = mix(texture(texBrickWall, oUv), vec4(oColor, 1.0), 0.5);
}