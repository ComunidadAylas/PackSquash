#version 120
varying vec4 texcoord;
uniform sampler2D gcolor;
void main() {
vec2 point = texcoord.st;
vec3 color = texture2D(gcolor, point).rgb;
color.r = 1-color.r;
color.g = 1-color.g;
color.b = 1-color.b;
gl_FragColor = vec4(color, 1.);
}
