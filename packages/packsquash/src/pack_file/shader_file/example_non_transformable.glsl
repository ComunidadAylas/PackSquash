#version 120

void main() {
    // Preprocessor directives outside of external declaration position
    // (i.e., within statements or other GLSL symbols) or unresolvable
    // #moj_import cause PackSquash to fall back to no source
    // transformation
    #moj_import <mist.glsl>
}
