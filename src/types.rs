// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------
pub use std::os::raw::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void, c_float};

use super::es20::data_struct::*;

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------
pub type GLbitfield = c_uint;
pub type GLboolean = c_uchar;
pub type GLbyte = c_char;
pub type GLchar = c_char;
pub type GLclampf = c_float;
pub type GLenum = c_uint;
pub type GLfloat = c_float;
pub type GLint = c_int;
pub type GLshort = c_short;
pub type GLsizei = c_int;
pub type GLubyte = c_uchar;
pub type GLuint = c_uint;
pub type GLushort = c_ushort;
pub type GLvoid = c_void;
pub type GLcharARB = c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = c_uint;

pub type GLhalfARB = c_ushort;
pub type GLhalf = c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

pub type GLDEBUGPROC = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureUnit {
    Texture0 = GL_TEXTURE0 as isize,
    Texture1 = GL_TEXTURE1 as isize,
    Texture2 = GL_TEXTURE2 as isize,
    Texture3 = GL_TEXTURE3 as isize,
    Texture4 = GL_TEXTURE4 as isize,
    Texture5 = GL_TEXTURE5 as isize,
    Texture6 = GL_TEXTURE6 as isize,
    Texture7 = GL_TEXTURE7 as isize,
    Texture8 = GL_TEXTURE8 as isize,
    Texture9 = GL_TEXTURE9 as isize,
    Texture10 = GL_TEXTURE10 as isize,
    Texture11 = GL_TEXTURE11 as isize,
    Texture12 = GL_TEXTURE12 as isize,
    Texture13 = GL_TEXTURE13 as isize,
    Texture14 = GL_TEXTURE14 as isize,
    Texture15 = GL_TEXTURE15 as isize,
    Texture16 = GL_TEXTURE16 as isize,
    Texture17 = GL_TEXTURE17 as isize,
    Texture18 = GL_TEXTURE18 as isize,
    Texture19 = GL_TEXTURE19 as isize,
    Texture20 = GL_TEXTURE20 as isize,
    Texture21 = GL_TEXTURE21 as isize,
    Texture22 = GL_TEXTURE22 as isize,
    Texture23 = GL_TEXTURE23 as isize,
    Texture24 = GL_TEXTURE24 as isize,
    Texture25 = GL_TEXTURE25 as isize,
    Texture26 = GL_TEXTURE26 as isize,
    Texture27 = GL_TEXTURE27 as isize,
    Texture28 = GL_TEXTURE28 as isize,
    Texture29 = GL_TEXTURE29 as isize,
    Texture30 = GL_TEXTURE30 as isize,
    Texture31 = GL_TEXTURE31 as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferBindTarget {
    ARRAY_BUFFER = GL_ARRAY_BUFFER_BINDING as isize,
    ELEMENT_ARRAY_BUFFER = GL_ELEMENT_ARRAY_BUFFER_BINDING as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferBindTarget {
    FRAMEBUFFER = GL_FRAMEBUFFER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderBufferBindTarget {
    RENDERBUFFER = GL_RENDERBUFFER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureBindTarget {
    TEXTURE_2D = GL_TEXTURE_BINDING_2D as isize,
    TEXTURE_CUBE_MAP = GL_TEXTURE_BINDING_CUBE_MAP as isize

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureTarget {
    TEXTURE_2D = GL_TEXTURE_2D as isize,
    TEXTURE_CUBE_MAP_POSITIVE_X = GL_TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_X = GL_TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
    TEXTURE_CUBE_MAP_POSITIVE_Y = GL_TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_Y = GL_TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,
    TEXTURE_CUBE_MAP_POSITIVE_Z = GL_TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_Z = GL_TEXTURE_CUBE_MAP_NEGATIVE_Z as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendEquationMode {
    FUNC_ADD = GL_FUNC_ADD as isize,
    FUNC_SUBTRACT = GL_FUNC_SUBTRACT as isize,
    FUNC_REVERSE_SUBTRACT = GL_FUNC_REVERSE_SUBTRACT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendFactor {
    ZERO = GL_ZERO as isize,
    ONE = GL_ONE as isize,
    SRC_COLOR = GL_SRC_COLOR as isize,
    ONE_MINUS_SRC_COLOR = GL_ONE_MINUS_SRC_COLOR as isize,
    DST_COLOR = GL_DST_COLOR as isize,
    ONE_MINUS_DST_COLOR = GL_ONE_MINUS_DST_COLOR as isize,
    SRC_ALPHA = GL_SRC_ALPHA as isize,
    ONE_MINUS_SRC_ALPHA = GL_ONE_MINUS_SRC_ALPHA as isize,
    DST_ALPHA = GL_DST_ALPHA as isize,
    ONE_MINUS_DST_ALPHA = GL_ONE_MINUS_DST_ALPHA as isize,
    CONSTANT_COLOR = GL_CONSTANT_COLOR as isize,
    ONE_MINUS_CONSTANT_COLOR = GL_ONE_MINUS_CONSTANT_COLOR as isize,
    CONSTANT_ALPHA = GL_CONSTANT_ALPHA as isize,
    ONE_MINUS_CONSTANT_ALPHA = GL_ONE_MINUS_CONSTANT_ALPHA as isize,
    SRC_ALPHA_SATURATE = GL_SRC_ALPHA_SATURATE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferUsage {
    STREAM_DRAW = GL_STREAM_DRAW as isize,
    STATIC_DRAW = GL_STATIC_DRAW as isize,
    DYNAMIC_DRAW = GL_DYNAMIC_DRAW as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferStatus {
    FRAMEBUFFER_COMPLETE = GL_FRAMEBUFFER_COMPLETE as isize,
    FRAMEBUFFER_INCOMPLETE_ATTACHMENT = GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT as isize,
    FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT = GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT as isize,
    FRAMEBUFFER_INCOMPLETE_DIMENSIONS = GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS as isize,
    FRAMEBUFFER_UNSUPPORTED = GL_FRAMEBUFFER_UNSUPPORTED as isize
}

impl From<GLuint> for FrameBufferStatus {
    fn from(id: GLuint) -> Self {
        match id {
            GL_FRAMEBUFFER_COMPLETE => FrameBufferStatus::FRAMEBUFFER_COMPLETE,
            GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
            GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
            GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_DIMENSIONS,
            _ => FrameBufferStatus::FRAMEBUFFER_UNSUPPORTED
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderType {
    FRAGMENT_SHADER = GL_FRAGMENT_SHADER as isize,
    VERTEX_SHADER = GL_VERTEX_SHADER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CullFaceMode {
    FRONT = GL_FRONT as isize,
    BACK = GL_BACK as isize,
    FRONT_AND_BACK= GL_FRONT_AND_BACK as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DepthFunc {
    NEVER = GL_NEVER as isize,
    LESS = GL_LESS as isize,
    EQUAL = GL_EQUAL as isize,
    LEQUAL = GL_LEQUAL as isize,
    GREATER = GL_GREATER as isize,
    NOTEQUAL = GL_NOTEQUAL as isize,
    GEQUAL = GL_GEQUAL as isize,
    ALWAYS = GL_ALWAYS as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Feature {
    BLEND = GL_BLEND as isize,
    CULL_FACE = GL_CULL_FACE as isize,
    DEPTH_TEST = GL_DEPTH_TEST as isize,
    DITHER = GL_DITHER as isize,
    POLYGON_OFFSET_FILL = GL_POLYGON_OFFSET_FILL as isize,
    SAMPLE_ALPHA_TO_COVERAGE = GL_SAMPLE_ALPHA_TO_COVERAGE as isize,
    SAMPLE_COVERAGE = GL_SAMPLE_COVERAGE as isize,
    SCISSOR_TEST = GL_SCISSOR_TEST as isize,
    STENCIL_TEST = GL_STENCIL_TEST as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BeginMode {
    POINTS = GL_POINTS as isize,
    LINES = GL_LINES as isize,
    LINE_LOOP = GL_LINE_LOOP as isize,
    LINE_STRIP = GL_LINE_STRIP as isize,
    TRIANGLES = GL_TRIANGLES as isize,
    TRIANGLE_STRIP = GL_TRIANGLE_STRIP as isize,
    TRIANGLE_FAN = GL_TRIANGLE_FAN as isize
}