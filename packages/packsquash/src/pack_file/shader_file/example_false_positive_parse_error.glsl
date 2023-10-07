// unresolvable.glsl may be resolvable by the game, but
// not by us, and contain preprocessor directives that
// may render otherwise invalid GLSL source valid. In
// the example below, unresolvable.glsl may define
// SEMICOLON to be ;
#moj_import <unresolvable.glsl>

vec2 var SEMICOLON
