#version 150

// shader_that_defines_main.fsh may define main, so PackSquash cannot be sure
// this shader is invalid
#moj_import <shader_that_defines_main.fsh>
