// Fragment shader using the #include directive that Minecraft 26.3 snapshot 5 and newer
// resolve against the resource locations of other shaders, in both of its spellings

#version 330

#ifdef GLINT
#include <minecraft:globals.glsl>
#endif
#include <minecraft:fog.glsl>
#include <minecraft:dynamictransforms.glsl>
#include "minecraft:light.glsl"
  #  include   "minecraft:projection.glsl"

uniform sampler2D Sampler0;

in float sphericalVertexDistance;
in float cylindricalVertexDistance;
in vec2 texCoord0;

out vec4 fragColor;

void main() {
	// Everything but the includes is plain GLSL, so this shader would be minifiable
	// if the included files were known
	vec4 color = texture(Sampler0, texCoord0) * ColorModulator;

	fragColor = apply_fog(color, sphericalVertexDistance, cylindricalVertexDistance, FogEnvironmentalStart, FogEnvironmentalEnd, FogRenderDistanceStart, FogRenderDistanceEnd, FogColor);
}
