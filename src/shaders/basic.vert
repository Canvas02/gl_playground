#version 450 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 uv;

layout (location = 0) out vec3 oColor;
layout (location = 1) out vec2 oUv;

uniform mat4 uProjView;

void main()
{
	gl_Position = uProjView * vec4(position, 1.0);
	oColor = color;
	oUv = uv;
}