use super::*;

pub const TRUE: Bool = 1;
pub const FALSE: Bool = 0;

pub use gl::{
    ACTIVE_ATTRIBUTES, ACTIVE_UNIFORMS, ALPHA, ALWAYS, ARRAY_BUFFER, BACK, BLEND, CLAMP_TO_EDGE,
    COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, COMPILE_STATUS, CULL_FACE, DECR, DECR_WRAP,
    DEPTH_ATTACHMENT, DEPTH_BUFFER_BIT, DEPTH_COMPONENT, DEPTH_STENCIL, DEPTH_STENCIL_ATTACHMENT,
    DEPTH_TEST, DST_ALPHA, DST_COLOR, DYNAMIC_DRAW, EQUAL, FLOAT, FLOAT_MAT2, FLOAT_MAT3,
    FLOAT_MAT4, FLOAT_VEC2, FLOAT_VEC3, FLOAT_VEC4, FRAGMENT_SHADER, FRAMEBUFFER,
    FRAMEBUFFER_COMPLETE, FRONT, GEQUAL, GREATER, INCR, INCR_WRAP, INT, INT_VEC2, INT_VEC3,
    INT_VEC4, INVALID_ENUM, INVALID_FRAMEBUFFER_OPERATION, INVALID_OPERATION, INVALID_VALUE,
    INVERT, KEEP, LEQUAL, LESS, LINEAR, LINEAR_MIPMAP_LINEAR, LINES, LINE_LOOP, LINE_STRIP,
    LINK_STATUS, NEAREST, NEVER, NOTEQUAL, NO_ERROR, ONE, ONE_MINUS_DST_ALPHA, ONE_MINUS_SRC_ALPHA,
    ONE_MINUS_SRC_COLOR, OUT_OF_MEMORY, POINTS, PROGRAM_POINT_SIZE, RENDERBUFFER, REPEAT, REPLACE,
    RGBA, SRC_ALPHA, SRC_ALPHA_SATURATE, SRC_COLOR, STATIC_DRAW, STENCIL_TEST, TEXTURE0,
    TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, TRIANGLES,
    TRIANGLE_FAN, TRIANGLE_STRIP, UNPACK_ALIGNMENT, UNSIGNED_BYTE, VERTEX_SHADER, ZERO,
};

#[cfg(target_os = "windows")]
pub const POINT_SPRITE: Enum = 0x8861;
