#version 330

// minecraft:semicolon.glsl may define SEMICOLON to be ;, which would make
// this shader valid, but PackSquash does not know what the included file contains
#include <minecraft:semicolon.glsl>

out vec4 fragColor;

void main() {
	vec4 color = vec4(1.0) SEMICOLON
	fragColor = color;
}
