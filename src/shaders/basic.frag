#version 450 core

layout (location = 0) in vec2 oUv;

layout (location = 0) out vec4 fColor;

layout (binding = 0) uniform sampler2D texBrickWall;

void main()
{
	fColor = texture(texBrickWall, oUv);
}