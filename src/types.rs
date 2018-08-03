// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::str::from_utf8;
use std::os::raw::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void, c_float};

//use khronos::*;
//use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

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

pub type GLintptr = isize;
pub type GLsizeiptr = isize;

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct Active {
    pub name: String,
    pub size: GLint,
    pub type_: GLenum,
    pub length: GLsizei,
}

pub struct ShaderPrecisionFormat {
    pub precision: GLint,
    pub range: [GLint; 2],
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// OpenGL ES core versions
pub const ES_VERSION_2_0: GLint = 1;

// ClearBufferMask
pub const DEPTH_BUFFER_BIT: GLuint = 0x00000100;
pub const STENCIL_BUFFER_BIT: GLuint = 0x00000400;
pub const COLOR_BUFFER_BIT: GLuint = 0x00004000;

// boolean
pub const FALSE: GLboolean = 0;
pub const TRUE: GLboolean = 1;

// BeginMode
pub const POINTS: GLuint = 0x0000;
pub const LINES: GLuint = 0x0001;
pub const LINE_LOOP: GLuint = 0x0002;
pub const LINE_STRIP: GLuint = 0x0003;
pub const TRIANGLES: GLuint = 0x0004;
pub const TRIANGLE_STRIP: GLuint = 0x0005;
pub const TRIANGLE_FAN: GLuint = 0x0006;

// StencilFunction, DepthFunction (not supported in ES20)
//      NEVER
//      LESS
//      EQUAL
//      LEQUAL
//      GREATER
//      NOTEQUAL
//      GEQUAL
//      ALWAYS

// BlendingFactorDest
pub const ZERO: GLuint = 0;
pub const ONE: GLuint = 1;
pub const SRC_COLOR: GLuint = 0x0300;
pub const ONE_MINUS_SRC_COLOR: GLuint = 0x0301;
pub const SRC_ALPHA: GLuint = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: GLuint = 0x0303;
pub const DST_ALPHA: GLuint = 0x0304;
pub const ONE_MINUS_DST_ALPHA: GLuint = 0x0305;

// BlendingFactorSrc
//      ZERO
//      ONE
pub const DST_COLOR: GLuint = 0x0306;
pub const ONE_MINUS_DST_COLOR: GLuint = 0x0307;
pub const SRC_ALPHA_SATURATE: GLuint = 0x0308;
//      SRC_ALPHA
//      ONE_MINUS_SRC_ALPHA
//      DST_ALPHA
//      ONE_MINUS_DST_ALPHA

// BlendEquationSeparate
pub const FUNC_ADD: GLuint = 0x8006;
pub const BLEND_EQUATION: GLuint = 0x8009;
pub const BLEND_EQUATION_RGB: GLuint = 0x8009; // same as BLEND_EQUATION
pub const BLEND_EQUATION_ALPHA: GLuint = 0x883D;

// BlendSubtract
pub const FUNC_SUBTRACT: GLuint = 0x800A;
pub const FUNC_REVERSE_SUBTRACT: GLuint = 0x800B;

// Separate Blend Functions
pub const BLEND_DST_RGB: GLuint = 0x80C8;
pub const BLEND_SRC_RGB: GLuint = 0x80C9;
pub const BLEND_DST_ALPHA: GLuint = 0x80CA;
pub const BLEND_SRC_ALPHA: GLuint = 0x80CB;
pub const CONSTANT_COLOR: GLuint = 0x8001;
pub const ONE_MINUS_CONSTANT_COLOR: GLuint = 0x8002;
pub const CONSTANT_ALPHA: GLuint = 0x8003;
pub const ONE_MINUS_CONSTANT_ALPHA: GLuint = 0x8004;
pub const BLEND_COLOR: GLuint = 0x8005;

// Buffer Objects
pub const ARRAY_BUFFER: GLuint = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: GLuint = 0x8893;
pub const ARRAY_BUFFER_BINDING: GLuint = 0x8894;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLuint = 0x8895;

pub const STREAM_DRAW: GLuint = 0x88E0;
pub const STATIC_DRAW: GLuint = 0x88E4;
pub const DYNAMIC_DRAW: GLuint = 0x88E8;

pub const BUFFER_SIZE: GLuint = 0x8764;
pub const BUFFER_USAGE: GLuint = 0x8765;

pub const CURRENT_VERTEX_ATTRIB: GLuint = 0x8626;

// CullFaceMode
pub const FRONT: GLuint = 0x0404;
pub const BACK: GLuint = 0x0405;
pub const FRONT_AND_BACK: GLuint = 0x0408;

// DepthFunction
//      NEVER
//      LESS
//      EQUAL
//      LEQUAL
//      GREATER
//      NOTEQUAL
//      GEQUAL
//      ALWAYS

// EnableCap
pub const TEXTURE_2D: GLuint = 0x0DE1;
pub const CULL_FACE: GLuint = 0x0B44;
pub const BLEND: GLuint = 0x0BE2;
pub const DITHER: GLuint = 0x0BD0;
pub const STENCIL_TEST: GLuint = 0x0B90;
pub const DEPTH_TEST: GLuint = 0x0B71;
pub const SCISSOR_TEST: GLuint = 0x0C11;
pub const POLYGON_OFFSET_FILL: GLuint = 0x8037;
pub const SAMPLE_ALPHA_TO_COVERAGE: GLuint = 0x809E;
pub const SAMPLE_COVERAGE: GLuint = 0x80A0;

// ErrorCode
pub const NO_ERROR: GLuint = 0;
pub const INVALID_ENUM: GLuint = 0x0500;
pub const INVALID_VALUE: GLuint = 0x0501;
pub const INVALID_OPERATION: GLuint = 0x0502;
pub const OUT_OF_MEMORY: GLuint = 0x0505;

// FrontFaceDirection
pub const CW: GLint = 0x0900;
pub const CCW: GLint = 0x0901;

// GetPName
pub const LINE_WIDTH: GLuint = 0x0B21;
pub const ALIASED_POINT_SIZE_RANGE: GLuint = 0x846D;
pub const ALIASED_LINE_WIDTH_RANGE: GLuint = 0x846E;
pub const CULL_FACE_MODE: GLuint = 0x0B45;
pub const FRONT_FACE: GLuint = 0x0B46;
pub const DEPTH_RANGE: GLuint = 0x0B70;
pub const DEPTH_WRITEMASK: GLuint = 0x0B72;
pub const DEPTH_CLEAR_VALUE: GLuint = 0x0B73;
pub const DEPTH_FUNC: GLuint = 0x0B74;
pub const STENCIL_CLEAR_VALUE: GLuint = 0x0B91;
pub const STENCIL_FUNC: GLuint = 0x0B92;
pub const STENCIL_FAIL: GLuint = 0x0B94;
pub const STENCIL_PASS_DEPTH_FAIL: GLuint = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: GLuint = 0x0B96;
pub const STENCIL_REF: GLuint = 0x0B97;
pub const STENCIL_VALUE_MASK: GLuint = 0x0B93;
pub const STENCIL_WRITEMASK: GLuint = 0x0B98;
pub const STENCIL_BACK_FUNC: GLuint = 0x8800;
pub const STENCIL_BACK_FAIL: GLuint = 0x8801;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLuint = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLuint = 0x8803;
pub const STENCIL_BACK_REF: GLuint = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: GLuint = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: GLuint = 0x8CA5;
pub const VIEWPORT: GLuint = 0x0BA2;
pub const SCISSOR_BOX: GLuint = 0x0C10;
//      SCISSOR_TEST
pub const COLOR_CLEAR_VALUE: GLuint = 0x0C22;
pub const COLOR_WRITEMASK: GLuint = 0x0C23;
pub const UNPACK_ALIGNMENT: GLuint = 0x0CF5;
pub const PACK_ALIGNMENT: GLuint = 0x0D05;
pub const MAX_TEXTURE_SIZE: GLuint = 0x0D33;
pub const MAX_VIEWPORT_DIMS: GLuint = 0x0D3A;
pub const SUBPIXEL_BITS: GLuint = 0x0D50;
pub const RED_BITS: GLuint = 0x0D52;
pub const GREEN_BITS: GLuint = 0x0D53;
pub const BLUE_BITS: GLuint = 0x0D54;
pub const ALPHA_BITS: GLuint = 0x0D55;
pub const DEPTH_BITS: GLuint = 0x0D56;
pub const STENCIL_BITS: GLuint = 0x0D57;
pub const POLYGON_OFFSET_UNITS: GLuint = 0x2A00;
//      POLYGON_OFFSET_FILL
pub const POLYGON_OFFSET_FACTOR: GLuint = 0x8038;
pub const TEXTURE_BINDING_2D: GLuint = 0x8069;
pub const SAMPLE_BUFFERS: GLuint = 0x80A8;
pub const SAMPLES: GLuint = 0x80A9;
pub const SAMPLE_COVERAGE_VALUE: GLuint = 0x80AA;
pub const SAMPLE_COVERAGE_INVERT: GLuint = 0x80AB;

// GetTextureParameter
//      TEXTURE_MAG_FILTER
//      TEXTURE_MIN_FILTER
//      TEXTURE_WRAP_S
//      TEXTURE_WRAP_T

pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLuint = 0x86A2;
pub const COMPRESSED_TEXTURE_FORMATS: GLuint = 0x86A3;

// HintMode
pub const DONT_CARE: GLuint = 0x1100;
pub const FASTEST: GLuint = 0x1101;
pub const NICEST: GLuint = 0x1102;

// HintTarget
pub const GENERATE_MIPMAP_HINT: GLint = 0x8192;

// DataType
pub const BYTE: GLuint = 0x1400;
pub const UNSIGNED_BYTE: GLuint = 0x1401;
pub const SHORT: GLuint = 0x1402;
pub const UNSIGNED_SHORT: GLuint = 0x1403;
pub const INT: GLuint = 0x1404;
pub const UNSIGNED_INT: GLuint = 0x1405;
pub const FLOAT: GLuint = 0x1406;
pub const FIXED: GLuint = 0x140C;

// PixelFormat
pub const DEPTH_COMPONENT: GLuint = 0x1902;
pub const ALPHA: GLuint = 0x1906;
pub const RGB: GLuint = 0x1907;
pub const RGBA: GLuint = 0x1908;
pub const LUMINANCE: GLuint = 0x1909;
pub const LUMINANCE_ALPHA: GLuint = 0x190A;

// PixelType
//      UNSIGNED_BYTE
pub const UNSIGNED_SHORT_4_4_4_4: GLuint = 0x8033;
pub const UNSIGNED_SHORT_5_5_5_1: GLuint = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: GLuint = 0x8363;

// Shaders
pub const FRAGMENT_SHADER: GLuint = 0x8B30;
pub const VERTEX_SHADER: GLuint = 0x8B31;
pub const MAX_VERTEX_ATTRIBS: GLuint = 0x8869;
pub const MAX_VERTEX_UNIFORM_VECTORS: GLuint = 0x8DFB;
pub const MAX_VARYING_VECTORS: GLuint = 0x8DFC;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLuint = 0x8B4D;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLuint = 0x8B4C;
pub const MAX_TEXTURE_IMAGE_UNITS: GLuint = 0x8872;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLuint = 0x8DFD;
pub const SHADER_TYPE: GLuint = 0x8B4F;
pub const DELETE_STATUS: GLuint = 0x8B80;
pub const LINK_STATUS: GLuint = 0x8B82;
pub const VALIDATE_STATUS: GLuint = 0x8B83;
pub const ATTACHED_SHADERS: GLuint = 0x8B85;
pub const ACTIVE_UNIFORMS: GLuint = 0x8B86;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLuint = 0x8B87;
pub const ACTIVE_ATTRIBUTES: GLuint = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLuint = 0x8B8A;
pub const SHADING_LANGUAGE_VERSION: GLuint = 0x8B8C;
pub const CURRENT_PROGRAM: GLuint = 0x8B8D;

// StencilFunction
pub const NEVER: GLuint = 0x0200;
pub const LESS: GLuint = 0x0201;
pub const EQUAL: GLuint = 0x0202;
pub const LEQUAL: GLuint = 0x0203;
pub const GREATER: GLuint = 0x0204;
pub const NOTEQUAL: GLuint = 0x0205;
pub const GEQUAL: GLuint = 0x0206;
pub const ALWAYS: GLuint = 0x0207;

// StencilOp
//      ZERO
pub const KEEP: GLuint = 0x1E00;
pub const REPLACE: GLuint = 0x1E01;
pub const INCR: GLuint = 0x1E02;
pub const DECR: GLuint = 0x1E03;
pub const INVERT: GLuint = 0x150A;
pub const INCR_WRAP: GLuint = 0x8507;
pub const DECR_WRAP: GLuint = 0x8508;

// StringName
pub const VENDOR: GLuint = 0x1F00;
pub const RENDERER: GLuint = 0x1F01;
pub const VERSION: GLuint = 0x1F02;
pub const EXTENSIONS: GLuint = 0x1F03;

// TextureMagFilter
pub const NEAREST: GLuint = 0x2600;
pub const LINEAR: GLuint = 0x2601;

// TextureMinFilter
//      NEAREST
//      LINEAR
pub const NEAREST_MIPMAP_NEAREST: GLuint = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: GLuint = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: GLuint = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: GLuint = 0x2703;

// TextureParameterName
pub const TEXTURE_MAG_FILTER: GLuint = 0x2800;
pub const TEXTURE_MIN_FILTER: GLuint = 0x2801;
pub const TEXTURE_WRAP_S: GLuint = 0x2802;
pub const TEXTURE_WRAP_T: GLuint = 0x2803;

// TextureTarget
//      TEXTURE_2D
pub const TEXTURE: GLuint = 0x1702;
pub const TEXTURE_CUBE_MAP: GLuint = 0x8513;
pub const TEXTURE_BINDING_CUBE_MAP: GLuint = 0x8514;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLuint = 0x8515;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLuint = 0x8516;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLuint = 0x8517;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLuint = 0x8518;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLuint = 0x8519;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLuint = 0x851A;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLuint = 0x851C;

// TextureUnit
pub const TEXTURE0: GLuint = 0x84C0;
pub const TEXTURE1: GLuint = 0x84C1;
pub const TEXTURE2: GLuint = 0x84C2;
pub const TEXTURE3: GLuint = 0x84C3;
pub const TEXTURE4: GLuint = 0x84C4;
pub const TEXTURE5: GLuint = 0x84C5;
pub const TEXTURE6: GLuint = 0x84C6;
pub const TEXTURE7: GLuint = 0x84C7;
pub const TEXTURE8: GLuint = 0x84C8;
pub const TEXTURE9: GLuint = 0x84C9;
pub const TEXTURE10: GLuint = 0x84CA;
pub const TEXTURE11: GLuint = 0x84CB;
pub const TEXTURE12: GLuint = 0x84CC;
pub const TEXTURE13: GLuint = 0x84CD;
pub const TEXTURE14: GLuint = 0x84CE;
pub const TEXTURE15: GLuint = 0x84CF;
pub const TEXTURE16: GLuint = 0x84D0;
pub const TEXTURE17: GLuint = 0x84D1;
pub const TEXTURE18: GLuint = 0x84D2;
pub const TEXTURE19: GLuint = 0x84D3;
pub const TEXTURE20: GLuint = 0x84D4;
pub const TEXTURE21: GLuint = 0x84D5;
pub const TEXTURE22: GLuint = 0x84D6;
pub const TEXTURE23: GLuint = 0x84D7;
pub const TEXTURE24: GLuint = 0x84D8;
pub const TEXTURE25: GLuint = 0x84D9;
pub const TEXTURE26: GLuint = 0x84DA;
pub const TEXTURE27: GLuint = 0x84DB;
pub const TEXTURE28: GLuint = 0x84DC;
pub const TEXTURE29: GLuint = 0x84DD;
pub const TEXTURE30: GLuint = 0x84DE;
pub const TEXTURE31: GLuint = 0x84DF;
pub const ACTIVE_TEXTURE: GLuint = 0x84E0;

// TextureWrapMode
pub const REPEAT: GLuint = 0x2901;
pub const CLAMP_TO_EDGE: GLuint = 0x812F;
pub const MIRRORED_REPEAT: GLuint = 0x8370;

// Uniform Types
pub const FLOAT_VEC2: GLuint = 0x8B50;
pub const FLOAT_VEC3: GLuint = 0x8B51;
pub const FLOAT_VEC4: GLuint = 0x8B52;
pub const INT_VEC2: GLuint = 0x8B53;
pub const INT_VEC3: GLuint = 0x8B54;
pub const INT_VEC4: GLuint = 0x8B55;
pub const BOOL: GLuint = 0x8B56;
pub const BOOL_VEC2: GLuint = 0x8B57;
pub const BOOL_VEC3: GLuint = 0x8B58;
pub const BOOL_VEC4: GLuint = 0x8B59;
pub const FLOAT_MAT2: GLuint = 0x8B5A;
pub const FLOAT_MAT3: GLuint = 0x8B5B;
pub const FLOAT_MAT4: GLuint = 0x8B5C;
pub const SAMPLER_2D: GLuint = 0x8B5E;
pub const SAMPLER_CUBE: GLuint = 0x8B60;

// Vertex Arrays
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLuint = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLuint = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLuint = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLuint = 0x8625;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLuint = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLuint = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLuint = 0x889F;

// Read Format
pub const IMPLEMENTATION_COLOR_READ_TYPE: GLuint = 0x8B9A;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLuint = 0x8B9B;

// Shader Source
pub const COMPILE_STATUS: GLuint = 0x8B81;
pub const INFO_LOG_LENGTH: GLuint = 0x8B84;
pub const SHADER_SOURCE_LENGTH: GLuint = 0x8B88;
pub const SHADER_COMPILER: GLuint = 0x8DFA;

// Shader Binary
pub const SHADER_BINARY_FORMATS: GLuint = 0x8DF8;
pub const NUM_SHADER_BINARY_FORMATS: GLuint = 0x8DF9;

// Shader Precision-Specified Types
pub const LOW_FLOAT: GLuint = 0x8DF0;
pub const MEDIUM_FLOAT: GLuint = 0x8DF1;
pub const HIGH_FLOAT: GLuint = 0x8DF2;
pub const LOW_INT: GLuint = 0x8DF3;
pub const MEDIUM_INT: GLuint = 0x8DF4;
pub const HIGH_INT: GLuint = 0x8DF5;

// Framebuffer Object.
pub const FRAMEBUFFER: GLuint = 0x8D40;
pub const RENDERBUFFER: GLuint = 0x8D41;

pub const RGBA4: GLuint = 0x8056;
pub const RGB5_A1: GLuint = 0x8057;
pub const RGB565: GLuint = 0x8D62;
pub const DEPTH_COMPONENT16: GLuint = 0x81A5;
pub const STENCIL_INDEX: GLuint = 0x1901;
pub const STENCIL_INDEX8: GLuint = 0x8D48;

pub const RENDERBUFFER_WIDTH: GLuint = 0x8D42;
pub const RENDERBUFFER_HEIGHT: GLuint = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLuint = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: GLuint = 0x8D50;
pub const RENDERBUFFER_GREEN_SIZE: GLuint = 0x8D51;
pub const RENDERBUFFER_BLUE_SIZE: GLuint = 0x8D52;
pub const RENDERBUFFER_ALPHA_SIZE: GLuint = 0x8D53;
pub const RENDERBUFFER_DEPTH_SIZE: GLuint = 0x8D54;
pub const RENDERBUFFER_STENCIL_SIZE: GLuint = 0x8D55;

pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLuint = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLuint = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLuint = 0x8CD2;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLuint = 0x8CD3;

pub const COLOR_ATTACHMENT0: GLuint = 0x8CE0;
pub const DEPTH_ATTACHMENT: GLuint = 0x8D00;
pub const STENCIL_ATTACHMENT: GLuint = 0x8D20;

pub const NONE: GLuint = 0;

pub const FRAMEBUFFER_COMPLETE: GLuint = 0x8CD5;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLuint = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLuint = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLuint = 0x8CD9;
pub const FRAMEBUFFER_UNSUPPORTED: GLuint = 0x8CDD;

pub const FRAMEBUFFER_BINDING: GLuint = 0x8CA6;
pub const RENDERBUFFER_BINDING: GLuint = 0x8CA7;
pub const MAX_RENDERBUFFER_SIZE: GLuint = 0x84E8;

pub const INVALID_FRAMEBUFFER_OPERATION: GLuint = 0x0506;


// OpenGL ES 3.0
//STRUCT
pub struct ProgramBinary{
    pub length: GLsizei,
    pub binaryFormat: GLenum,
    pub binary: Vec<u8>
}

//// -------------------------------------------------------------------------------------------------
//// CONSTANTS
//// -------------------------------------------------------------------------------------------------

// OpenGL ES core versions
pub const ES_VERSION_3_0: GLint = 1;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLint64 = i64;
pub type GLuint64 = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

pub const READ_BUFFER: GLuint = 3074;
pub const UNPACK_ROW_LENGTH: GLuint = 3314;
pub const UNPACK_SKIP_ROWS: GLuint = 3315;
pub const UNPACK_SKIP_PIXELS: GLuint = 3316;
pub const PACK_ROW_LENGTH: GLuint = 3330;
pub const PACK_SKIP_ROWS: GLuint = 3331;
pub const PACK_SKIP_PIXELS: GLuint = 3332;
pub const COLOR: GLuint = 6144;
pub const DEPTH: GLuint = 6145;
pub const STENCIL: GLuint = 6146;
pub const RED: GLuint = 6403;
pub const RGB8: GLuint = 32849;
pub const RGBA8: GLuint = 32856;
pub const RGB10_A2: GLuint = 32857;
pub const TEXTURE_BINDING_3D: GLuint = 32874;
pub const UNPACK_SKIP_IMAGES: GLuint = 32877;
pub const UNPACK_IMAGE_HEIGHT: GLuint = 32878;
pub const TEXTURE_3D: GLuint = 32879;
pub const TEXTURE_WRAP_R: GLuint = 32882;
pub const MAX_3D_TEXTURE_SIZE: GLuint = 32883;
pub const UNSIGNED_INT_2_10_10_10_REV: GLuint = 33640;
pub const MAX_ELEMENTS_VERTICES: GLuint = 33000;
pub const MAX_ELEMENTS_INDICES: GLuint = 33001;
pub const TEXTURE_MIN_LOD: GLuint = 33082;
pub const TEXTURE_MAX_LOD: GLuint = 33083;
pub const TEXTURE_BASE_LEVEL: GLuint = 33084;
pub const TEXTURE_MAX_LEVEL: GLuint = 33085;
pub const MIN: GLuint = 32775;
pub const MAX: GLuint = 32776;
pub const DEPTH_COMPONENT24: GLuint = 33190;
pub const MAX_TEXTURE_LOD_BIAS: GLuint = 34045;
pub const TEXTURE_COMPARE_MODE: GLuint = 34892;
pub const TEXTURE_COMPARE_FUNC: GLuint = 34893;
pub const CURRENT_QUERY: GLuint = 34917;
pub const QUERY_RESULT: GLuint = 34918;
pub const QUERY_RESULT_AVAILABLE: GLuint = 34919;
pub const BUFFER_MAPPED: GLuint = 35004;
pub const BUFFER_MAP_POINTER: GLuint = 35005;
pub const STREAM_READ: GLuint = 35041;
pub const STREAM_COPY: GLuint = 35042;
pub const STATIC_READ: GLuint = 35045;
pub const STATIC_COPY: GLuint = 35046;
pub const DYNAMIC_READ: GLuint = 35049;
pub const DYNAMIC_COPY: GLuint = 35050;
pub const MAX_DRAW_BUFFERS: GLuint = 34852;
pub const DRAW_BUFFER0: GLuint = 34853;
pub const DRAW_BUFFER1: GLuint = 34854;
pub const DRAW_BUFFER2: GLuint = 34855;
pub const DRAW_BUFFER3: GLuint = 34856;
pub const DRAW_BUFFER4: GLuint = 34857;
pub const DRAW_BUFFER5: GLuint = 34858;
pub const DRAW_BUFFER6: GLuint = 34859;
pub const DRAW_BUFFER7: GLuint = 34860;
pub const DRAW_BUFFER8: GLuint = 34861;
pub const DRAW_BUFFER9: GLuint = 34862;
pub const DRAW_BUFFER10: GLuint = 34863;
pub const DRAW_BUFFER11: GLuint = 34864;
pub const DRAW_BUFFER12: GLuint = 34865;
pub const DRAW_BUFFER13: GLuint = 34866;
pub const DRAW_BUFFER14: GLuint = 34867;
pub const DRAW_BUFFER15: GLuint = 34868;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLuint = 35657;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLuint = 35658;
pub const SAMPLER_3D: GLuint = 35679;
pub const SAMPLER_2D_SHADOW: GLuint = 35682;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLuint = 35723;
pub const PIXEL_PACK_BUFFER: GLuint = 35051;
pub const PIXEL_UNPACK_BUFFER: GLuint = 35052;
pub const PIXEL_PACK_BUFFER_BINDING: GLuint = 35053;
pub const PIXEL_UNPACK_BUFFER_BINDING: GLuint = 35055;
pub const FLOAT_MAT2x3: GLuint = 35685;
pub const FLOAT_MAT2x4: GLuint = 35686;
pub const FLOAT_MAT3x2: GLuint = 35687;
pub const FLOAT_MAT3x4: GLuint = 35688;
pub const FLOAT_MAT4x2: GLuint = 35689;
pub const FLOAT_MAT4x3: GLuint = 35690;
pub const SRGB: GLuint = 35904;
pub const SRGB8: GLuint = 35905;
pub const SRGB8_ALPHA8: GLuint = 35907;
pub const COMPARE_REF_TO_TEXTURE: GLuint = 34894;
pub const MAJOR_VERSION: GLuint = 33307;
pub const MINOR_VERSION: GLuint = 33308;
pub const NUM_EXTENSIONS: GLuint = 33309;
pub const RGBA32F: GLuint = 34836;
pub const RGB32F: GLuint = 34837;
pub const RGBA16F: GLuint = 34842;
pub const RGB16F: GLuint = 34843;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLuint = 35069;
pub const MAX_ARRAY_TEXTURE_LAYERS: GLuint = 35071;
pub const MIN_PROGRAM_TEXEL_OFFSET: GLuint = 35076;
pub const MAX_PROGRAM_TEXEL_OFFSET: GLuint = 35077;
pub const MAX_VARYING_COMPONENTS: GLuint = 35659;
pub const TEXTURE_2D_ARRAY: GLuint = 35866;
pub const TEXTURE_BINDING_2D_ARRAY: GLuint = 35869;
pub const R11F_G11F_B10F: GLuint = 35898;
pub const UNSIGNED_INT_10F_11F_11F_REV: GLuint = 35899;
pub const RGB9_E5: GLuint = 35901;
pub const UNSIGNED_INT_5_9_9_9_REV: GLuint = 35902;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLuint = 35958;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLuint = 35967;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLuint = 35968;
pub const TRANSFORM_FEEDBACK_VARYINGS: GLuint = 35971;
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLuint = 35972;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLuint = 35973;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLuint = 35976;
pub const RASTERIZER_DISCARD: GLuint = 35977;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLuint = 35978;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLuint = 35979;
pub const INTERLEAVED_ATTRIBS: GLuint = 35980;
pub const SEPARATE_ATTRIBS: GLuint = 35981;
pub const TRANSFORM_FEEDBACK_BUFFER: GLuint = 35982;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLuint = 35983;
pub const RGBA32UI: GLuint = 36208;
pub const RGB32UI: GLuint = 36209;
pub const RGBA16UI: GLuint = 36214;
pub const RGB16UI: GLuint = 36215;
pub const RGBA8UI: GLuint = 36220;
pub const RGB8UI: GLuint = 36221;
pub const RGBA32I: GLuint = 36226;
pub const RGB32I: GLuint = 36227;
pub const RGBA16I: GLuint = 36232;
pub const RGB16I: GLuint = 36233;
pub const RGBA8I: GLuint = 36238;
pub const RGB8I: GLuint = 36239;
pub const RED_INTEGER: GLuint = 36244;
pub const RGB_INTEGER: GLuint = 36248;
pub const RGBA_INTEGER: GLuint = 36249;
pub const SAMPLER_2D_ARRAY: GLuint = 36289;
pub const SAMPLER_2D_ARRAY_SHADOW: GLuint = 36292;
pub const SAMPLER_CUBE_SHADOW: GLuint = 36293;
pub const UNSIGNED_INT_VEC2: GLuint = 36294;
pub const UNSIGNED_INT_VEC3: GLuint = 36295;
pub const UNSIGNED_INT_VEC4: GLuint = 36296;
pub const INT_SAMPLER_2D: GLuint = 36298;
pub const INT_SAMPLER_3D: GLuint = 36299;
pub const INT_SAMPLER_CUBE: GLuint = 36300;
pub const INT_SAMPLER_2D_ARRAY: GLuint = 36303;
pub const UNSIGNED_INT_SAMPLER_2D: GLuint = 36306;
pub const UNSIGNED_INT_SAMPLER_3D: GLuint = 36307;
pub const UNSIGNED_INT_SAMPLER_CUBE: GLuint = 36308;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLuint = 36311;
pub const BUFFER_ACCESS_FLAGS: GLuint = 37151;
pub const BUFFER_MAP_LENGTH: GLuint = 37152;
pub const BUFFER_MAP_OFFSET: GLuint = 37153;
pub const DEPTH_COMPONENT32F: GLuint = 36012;
pub const DEPTH32F_STENCIL8: GLuint = 36013;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLuint = 36269;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLuint = 33296;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLuint = 33297;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLuint = 33298;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLuint = 33299;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLuint = 33300;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLuint = 33301;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLuint = 33302;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLuint = 33303;
pub const FRAMEBUFFER_DEFAULT: GLuint = 33304;
pub const FRAMEBUFFER_UNDEFINED: GLuint = 33305;
pub const DEPTH_STENCIL_ATTACHMENT: GLuint = 33306;
pub const DEPTH_STENCIL: GLuint = 34041;
pub const UNSIGNED_INT_24_8: GLuint = 34042;
pub const DEPTH24_STENCIL8: GLuint = 35056;
pub const UNSIGNED_NORMALIZED: GLuint = 35863;
pub const READ_FRAMEBUFFER: GLuint = 36008;
pub const DRAW_FRAMEBUFFER: GLuint = 36009;
pub const READ_FRAMEBUFFER_BINDING: GLuint = 36010;
pub const RENDERBUFFER_SAMPLES: GLuint = 36011;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLuint = 36052;
pub const MAX_COLOR_ATTACHMENTS: GLuint = 36063;
pub const COLOR_ATTACHMENT1: GLuint = 36065;
pub const COLOR_ATTACHMENT2: GLuint = 36066;
pub const COLOR_ATTACHMENT3: GLuint = 36067;
pub const COLOR_ATTACHMENT4: GLuint = 36068;
pub const COLOR_ATTACHMENT5: GLuint = 36069;
pub const COLOR_ATTACHMENT6: GLuint = 36070;
pub const COLOR_ATTACHMENT7: GLuint = 36071;
pub const COLOR_ATTACHMENT8: GLuint = 36072;
pub const COLOR_ATTACHMENT9: GLuint = 36073;
pub const COLOR_ATTACHMENT10: GLuint = 36074;
pub const COLOR_ATTACHMENT11: GLuint = 36075;
pub const COLOR_ATTACHMENT12: GLuint = 36076;
pub const COLOR_ATTACHMENT13: GLuint = 36077;
pub const COLOR_ATTACHMENT14: GLuint = 36078;
pub const COLOR_ATTACHMENT15: GLuint = 36079;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLuint = 36182;
pub const MAX_SAMPLES: GLuint = 36183;
pub const HALF_FLOAT: GLuint = 5131;
pub const MAP_READ_BIT: GLuint = 1;
pub const MAP_WRITE_BIT: GLuint = 2;
pub const MAP_INVALIDATE_RANGE_BIT: GLuint = 4;
pub const MAP_INVALIDATE_BUFFER_BIT: GLuint = 8;
pub const MAP_FLUSH_EXPLICIT_BIT: GLuint = 16;
pub const MAP_UNSYNCHRONIZED_BIT: GLuint = 32;
pub const RG: GLuint = 33319;
pub const RG_INTEGER: GLuint = 33320;
pub const R8: GLuint = 33321;
pub const RG8: GLuint = 33323;
pub const R16F: GLuint = 33325;
pub const R32F: GLuint = 33326;
pub const RG16F: GLuint = 33327;
pub const RG32F: GLuint = 33328;
pub const R8I: GLuint = 33329;
pub const R8UI: GLuint = 33330;
pub const R16I: GLuint = 33331;
pub const R16UI: GLuint = 33332;
pub const R32I: GLuint = 33333;
pub const R32UI: GLuint = 33334;
pub const RG8I: GLuint = 33335;
pub const RG8UI: GLuint = 33336;
pub const RG16I: GLuint = 33337;
pub const RG16UI: GLuint = 33338;
pub const RG32I: GLuint = 33339;
pub const RG32UI: GLuint = 33340;
pub const VERTEX_ARRAY_BINDING: GLuint = 34229;
pub const R8_SNORM: GLuint = 36756;
pub const RG8_SNORM: GLuint = 36757;
pub const RGB8_SNORM: GLuint = 36758;
pub const RGBA8_SNORM: GLuint = 36759;
pub const SIGNED_NORMALIZED: GLuint = 36764;
pub const PRIMITIVE_RESTART_FIXED_INDEX: GLuint = 36201;
pub const COPY_READ_BUFFER: GLuint = 36662;
pub const COPY_WRITE_BUFFER: GLuint = 36663;
pub const COPY_READ_BUFFER_BINDING: GLuint = 36662;
pub const COPY_WRITE_BUFFER_BINDING: GLuint = 36663;
pub const UNIFORM_BUFFER: GLuint = 35345;
pub const UNIFORM_BUFFER_BINDING: GLuint = 35368;
pub const UNIFORM_BUFFER_START: GLuint = 35369;
pub const UNIFORM_BUFFER_SIZE: GLuint = 35370;
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLuint = 35371;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLuint = 35373;
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLuint = 35374;
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLuint = 35375;
pub const MAX_UNIFORM_BLOCK_SIZE: GLuint = 35376;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLuint = 35377;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLuint = 35379;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLuint = 35380;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLuint = 35381;
pub const ACTIVE_UNIFORM_BLOCKS: GLuint = 35382;
pub const UNIFORM_TYPE: GLuint = 35383;
pub const UNIFORM_SIZE: GLuint = 35384;
pub const UNIFORM_NAME_LENGTH: GLuint = 35385;
pub const UNIFORM_BLOCK_INDEX: GLuint = 35386;
pub const UNIFORM_OFFSET: GLuint = 35387;
pub const UNIFORM_ARRAY_STRIDE: GLuint = 35388;
pub const UNIFORM_MATRIX_STRIDE: GLuint = 35389;
pub const UNIFORM_IS_ROW_MAJOR: GLuint = 35390;
pub const UNIFORM_BLOCK_BINDING: GLuint = 35391;
pub const UNIFORM_BLOCK_DATA_SIZE: GLuint = 35392;
pub const UNIFORM_BLOCK_NAME_LENGTH: GLuint = 35393;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLuint = 35394;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLuint = 35395;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLuint = 35396;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLuint = 35398;
pub const INVALID_INDEX: GLuint = 4294967295;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLuint = 37154;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLuint = 37157;
pub const MAX_SERVER_WAIT_TIMEOUT: GLuint = 37137;
pub const OBJECT_TYPE: GLuint = 37138;
pub const SYNC_CONDITION: GLuint = 37139;
pub const SYNC_STATUS: GLuint = 37140;
pub const SYNC_FLAGS: GLuint = 37141;
pub const SYNC_FENCE: GLuint = 37142;
pub const SYNC_GPU_COMMANDS_COMPLETE: GLuint = 37143;
pub const UNSIGNALED: GLuint = 37144;
pub const SIGNALED: GLuint = 37145;
pub const ALREADY_SIGNALED: GLuint = 37146;
pub const TIMEOUT_EXPIRED: GLuint = 37147;
pub const CONDITION_SATISFIED: GLuint = 37148;
pub const WAIT_FAILED: GLuint = 37149;
pub const SYNC_FLUSH_COMMANDS_BIT: GLuint = 1;
pub const TIMEOUT_IGNORED: i32 = -1;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLuint = 35070;
pub const ANY_SAMPLES_PASSED: GLuint = 35887;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLuint = 36202;
pub const SAMPLER_BINDING: GLuint = 35097;
pub const RGB10_A2UI: GLuint = 36975;
pub const TEXTURE_SWIZZLE_R: GLuint = 36418;
pub const TEXTURE_SWIZZLE_G: GLuint = 36419;
pub const TEXTURE_SWIZZLE_B: GLuint = 36420;
pub const TEXTURE_SWIZZLE_A: GLuint = 36421;
pub const GREEN: GLuint = 6404;
pub const BLUE: GLuint = 6405;
pub const INT_2_10_10_10_REV: GLuint = 36255;
pub const TRANSFORM_FEEDBACK: GLuint = 36386;
pub const TRANSFORM_FEEDBACK_PAUSED: GLuint = 36387;
pub const TRANSFORM_FEEDBACK_ACTIVE: GLuint = 36388;
pub const TRANSFORM_FEEDBACK_BINDING: GLuint = 36389;
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLuint = 33367;
pub const PROGRAM_BINARY_LENGTH: GLuint = 34625;
pub const NUM_PROGRAM_BINARY_FORMATS: GLuint = 34814;
pub const PROGRAM_BINARY_FORMATS: GLuint = 34815;
pub const COMPRESSED_R11_EAC: GLuint = 37488;
pub const COMPRESSED_SIGNED_R11_EAC: GLuint = 37489;
pub const COMPRESSED_RG11_EAC: GLuint = 37490;
pub const COMPRESSED_SIGNED_RG11_EAC: GLuint = 37491;
pub const COMPRESSED_RGB8_ETC2: GLuint = 37492;
pub const COMPRESSED_SRGB8_ETC2: GLuint = 37493;
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLuint = 37494;
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLuint = 37495;
pub const COMPRESSED_RGBA8_ETC2_EAC: GLuint = 37496;
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLuint = 37497;
pub const TEXTURE_IMMUTABLE_FORMAT: GLuint = 37167;
pub const MAX_ELEMENT_INDEX: GLuint = 36203;
pub const NUM_SAMPLE_COUNTS: GLuint = 37760;
pub const TEXTURE_IMMUTABLE_LEVELS: GLuint = 33503;


// OpenGL ES 3.1
pub const COMPUTE_SHADER: GLuint = 37305;
pub const MAX_COMPUTE_UNIFORM_BLOCKS: GLuint = 37307;
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLuint = 37308;
pub const MAX_COMPUTE_IMAGE_UNIFORMS: GLuint = 37309;
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLuint = 33378;
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GLuint = 33379;
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLuint = 33380;
pub const MAX_COMPUTE_ATOMIC_COUNTERS: GLuint = 33381;
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLuint = 33382;
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLuint = 37099;
pub const MAX_COMPUTE_WORK_GROUP_COUNT: GLuint = 37310;
pub const MAX_COMPUTE_WORK_GROUP_SIZE: GLuint = 37311;
pub const COMPUTE_WORK_GROUP_SIZE: GLuint = 33383;
pub const DISPATCH_INDIRECT_BUFFER: GLuint = 37102;
pub const DISPATCH_INDIRECT_BUFFER_BINDING: GLuint = 37103;
pub const COMPUTE_SHADER_BIT: GLuint = 32;
pub const DRAW_INDIRECT_BUFFER: GLuint = 36671;
pub const DRAW_INDIRECT_BUFFER_BINDING: GLuint = 36675;
pub const MAX_UNIFORM_LOCATIONS: GLuint = 33390;
pub const FRAMEBUFFER_DEFAULT_WIDTH: GLuint = 37648;
pub const FRAMEBUFFER_DEFAULT_HEIGHT: GLuint = 37649;
pub const FRAMEBUFFER_DEFAULT_SAMPLES: GLuint = 37651;
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLuint = 37652;
pub const MAX_FRAMEBUFFER_WIDTH: GLuint = 37653;
pub const MAX_FRAMEBUFFER_HEIGHT: GLuint = 37654;
pub const MAX_FRAMEBUFFER_SAMPLES: GLuint = 37656;
pub const UNIFORM: GLuint = 37601;
pub const UNIFORM_BLOCK: GLuint = 37602;
pub const PROGRAM_INPUT: GLuint = 37603;
pub const PROGRAM_OUTPUT: GLuint = 37604;
pub const BUFFER_VARIABLE: GLuint = 37605;
pub const SHADER_STORAGE_BLOCK: GLuint = 37606;
pub const ATOMIC_COUNTER_BUFFER: GLuint = 37568;
pub const TRANSFORM_FEEDBACK_VARYING: GLuint = 37620;
pub const ACTIVE_RESOURCES: GLuint = 37621;
pub const MAX_NAME_LENGTH: GLuint = 37622;
pub const MAX_NUM_ACTIVE_VARIABLES: GLuint = 37623;
pub const NAME_LENGTH: GLuint = 37625;
pub const TYPE: GLuint = 37626;
pub const ARRAY_SIZE: GLuint = 37627;
pub const OFFSET: GLuint = 37628;
pub const BLOCK_INDEX: GLuint = 37629;
pub const ARRAY_STRIDE: GLuint = 37630;
pub const MATRIX_STRIDE: GLuint = 37631;
pub const IS_ROW_MAJOR: GLuint = 37632;
pub const ATOMIC_COUNTER_BUFFER_INDEX: GLuint = 37633;
pub const BUFFER_BINDING: GLuint = 37634;
pub const BUFFER_DATA_SIZE: GLuint = 37635;
pub const NUM_ACTIVE_VARIABLES: GLuint = 37636;
pub const ACTIVE_VARIABLES: GLuint = 37637;
pub const REFERENCED_BY_VERTEX_SHADER: GLuint = 37638;
pub const REFERENCED_BY_FRAGMENT_SHADER: GLuint = 37642;
pub const REFERENCED_BY_COMPUTE_SHADER: GLuint = 37643;
pub const TOP_LEVEL_ARRAY_SIZE: GLuint = 37644;
pub const TOP_LEVEL_ARRAY_STRIDE: GLuint = 37645;
pub const LOCATION: GLuint = 37646;
pub const VERTEX_SHADER_BIT: GLuint = 1;
pub const FRAGMENT_SHADER_BIT: GLuint = 2;
pub const ALL_SHADER_BITS: GLuint = 4294967295;
pub const PROGRAM_SEPARABLE: GLuint = 33368;
pub const ACTIVE_PROGRAM: GLuint = 33369;
pub const PROGRAM_PIPELINE_BINDING: GLuint = 33370;
pub const ATOMIC_COUNTER_BUFFER_BINDING: GLuint = 37569;
pub const ATOMIC_COUNTER_BUFFER_START: GLuint = 37570;
pub const ATOMIC_COUNTER_BUFFER_SIZE: GLuint = 37571;
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLuint = 37580;
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLuint = 37584;
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLuint = 37585;
pub const MAX_VERTEX_ATOMIC_COUNTERS: GLuint = 37586;
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GLuint = 37590;
pub const MAX_COMBINED_ATOMIC_COUNTERS: GLuint = 37591;
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLuint = 37592;
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLuint = 37596;
pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLuint = 37593;
pub const UNSIGNED_INT_ATOMIC_COUNTER: GLuint = 37595;
pub const MAX_IMAGE_UNITS: GLuint = 36664;
pub const MAX_VERTEX_IMAGE_UNIFORMS: GLuint = 37066;
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: GLuint = 37070;
pub const MAX_COMBINED_IMAGE_UNIFORMS: GLuint = 37071;
pub const IMAGE_BINDING_NAME: GLuint = 36666;
pub const IMAGE_BINDING_LEVEL: GLuint = 36667;
pub const IMAGE_BINDING_LAYERED: GLuint = 36668;
pub const IMAGE_BINDING_LAYER: GLuint = 36669;
pub const IMAGE_BINDING_ACCESS: GLuint = 36670;
pub const IMAGE_BINDING_FORMAT: GLuint = 36974;
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLuint = 1;
pub const ELEMENT_ARRAY_BARRIER_BIT: GLuint = 2;
pub const UNIFORM_BARRIER_BIT: GLuint = 4;
pub const TEXTURE_FETCH_BARRIER_BIT: GLuint = 8;
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLuint = 32;
pub const COMMAND_BARRIER_BIT: GLuint = 64;
pub const PIXEL_BUFFER_BARRIER_BIT: GLuint = 128;
pub const TEXTURE_UPDATE_BARRIER_BIT: GLuint = 256;
pub const BUFFER_UPDATE_BARRIER_BIT: GLuint = 512;
pub const FRAMEBUFFER_BARRIER_BIT: GLuint = 1024;
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: GLuint = 2048;
pub const ATOMIC_COUNTER_BARRIER_BIT: GLuint = 4096;
pub const ALL_BARRIER_BITS: GLuint = 4294967295;
pub const IMAGE_2D: GLuint = 36941;
pub const IMAGE_3D: GLuint = 36942;
pub const IMAGE_CUBE: GLuint = 36944;
pub const IMAGE_2D_ARRAY: GLuint = 36947;
pub const INT_IMAGE_2D: GLuint = 36952;
pub const INT_IMAGE_3D: GLuint = 36953;
pub const INT_IMAGE_CUBE: GLuint = 36955;
pub const INT_IMAGE_2D_ARRAY: GLuint = 36958;
pub const UNSIGNED_INT_IMAGE_2D: GLuint = 36963;
pub const UNSIGNED_INT_IMAGE_3D: GLuint = 36964;
pub const UNSIGNED_INT_IMAGE_CUBE: GLuint = 36966;
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: GLuint = 36969;
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLuint = 37063;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLuint = 37064;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLuint = 37065;
pub const READ_ONLY: GLuint = 35000;
pub const WRITE_ONLY: GLuint = 35001;
pub const READ_WRITE: GLuint = 35002;
pub const SHADER_STORAGE_BUFFER: GLuint = 37074;
pub const SHADER_STORAGE_BUFFER_BINDING: GLuint = 37075;
pub const SHADER_STORAGE_BUFFER_START: GLuint = 37076;
pub const SHADER_STORAGE_BUFFER_SIZE: GLuint = 37077;
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLuint = 37078;
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLuint = 37082;
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLuint = 37083;
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLuint = 37084;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLuint = 37085;
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: GLuint = 37086;
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLuint = 37087;
pub const SHADER_STORAGE_BARRIER_BIT: GLuint = 8192;
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLuint = 36665;
pub const DEPTH_STENCIL_TEXTURE_MODE: GLuint = 37098;
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLuint = 36446;
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLuint = 36447;
pub const SAMPLE_POSITION: GLuint = 36432;
pub const SAMPLE_MASK: GLuint = 36433;
pub const SAMPLE_MASK_VALUE: GLuint = 36434;
pub const TEXTURE_2D_MULTISAMPLE: GLuint = 37120;
pub const MAX_SAMPLE_MASK_WORDS: GLuint = 36441;
pub const MAX_COLOR_TEXTURE_SAMPLES: GLuint = 37134;
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLuint = 37135;
pub const MAX_INTEGER_SAMPLES: GLuint = 37136;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLuint = 37124;
pub const TEXTURE_SAMPLES: GLuint = 37126;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLuint = 37127;
pub const TEXTURE_WIDTH: GLuint = 4096;
pub const TEXTURE_HEIGHT: GLuint = 4097;
pub const TEXTURE_DEPTH: GLuint = 32881;
pub const TEXTURE_INTERNAL_FORMAT: GLuint = 4099;
pub const TEXTURE_RED_SIZE: GLuint = 32860;
pub const TEXTURE_GREEN_SIZE: GLuint = 32861;
pub const TEXTURE_BLUE_SIZE: GLuint = 32862;
pub const TEXTURE_ALPHA_SIZE: GLuint = 32863;
pub const TEXTURE_DEPTH_SIZE: GLuint = 34890;
pub const TEXTURE_STENCIL_SIZE: GLuint = 35057;
pub const TEXTURE_SHARED_SIZE: GLuint = 35903;
pub const TEXTURE_RED_TYPE: GLuint = 35856;
pub const TEXTURE_GREEN_TYPE: GLuint = 35857;
pub const TEXTURE_BLUE_TYPE: GLuint = 35858;
pub const TEXTURE_ALPHA_TYPE: GLuint = 35859;
pub const TEXTURE_DEPTH_TYPE: GLuint = 35862;
pub const TEXTURE_COMPRESSED: GLuint = 34465;
pub const SAMPLER_2D_MULTISAMPLE: GLuint = 37128;
pub const INT_SAMPLER_2D_MULTISAMPLE: GLuint = 37129;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLuint = 37130;
pub const VERTEX_ATTRIB_BINDING: GLuint = 33492;
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: GLuint = 33493;
pub const VERTEX_BINDING_DIVISOR: GLuint = 33494;
pub const VERTEX_BINDING_OFFSET: GLuint = 33495;
pub const VERTEX_BINDING_STRIDE: GLuint = 33496;
pub const VERTEX_BINDING_BUFFER: GLuint = 36687;
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLuint = 33497;
pub const MAX_VERTEX_ATTRIB_BINDINGS: GLuint = 33498;
pub const MAX_VERTEX_ATTRIB_STRIDE: GLuint = 33509;

// OpenGL ES 3.2
pub const MULTISAMPLE_LINE_WIDTH_RANGE: GLuint = 37761;
pub const MULTISAMPLE_LINE_WIDTH_GRANULARITY: GLuint = 37762;
pub const MULTIPLY: GLuint = 37524;
pub const SCREEN: GLuint = 37525;
pub const OVERLAY: GLuint = 37526;
pub const DARKEN: GLuint = 37527;
pub const LIGHTEN: GLuint = 37528;
pub const COLORDODGE: GLuint = 37529;
pub const COLORBURN: GLuint = 37530;
pub const HARDLIGHT: GLuint = 37531;
pub const SOFTLIGHT: GLuint = 37532;
pub const DIFFERENCE: GLuint = 37534;
pub const EXCLUSION: GLuint = 37536;
pub const HSL_HUE: GLuint = 37549;
pub const HSL_SATURATION: GLuint = 37550;
pub const HSL_COLOR: GLuint = 37551;
pub const HSL_LUMINOSITY: GLuint = 37552;
pub const DEBUG_OUTPUT_SYNCHRONOUS: GLuint = 33346;
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLuint = 33347;
pub const DEBUG_CALLBACK_FUNCTION: GLuint = 33348;
pub const DEBUG_CALLBACK_USER_PARAM: GLuint = 33349;
pub const DEBUG_SOURCE_API: GLuint = 33350;
pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLuint = 33351;
pub const DEBUG_SOURCE_SHADER_COMPILER: GLuint = 33352;
pub const DEBUG_SOURCE_THIRD_PARTY: GLuint = 33353;
pub const DEBUG_SOURCE_APPLICATION: GLuint = 33354;
pub const DEBUG_SOURCE_OTHER: GLuint = 33355;
pub const DEBUG_TYPE_ERROR: GLuint = 33356;
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLuint = 33357;
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLuint = 33358;
pub const DEBUG_TYPE_PORTABILITY: GLuint = 33359;
pub const DEBUG_TYPE_PERFORMANCE: GLuint = 33360;
pub const DEBUG_TYPE_OTHER: GLuint = 33361;
pub const DEBUG_TYPE_MARKER: GLuint = 33384;
pub const DEBUG_TYPE_PUSH_GROUP: GLuint = 33385;
pub const DEBUG_TYPE_POP_GROUP: GLuint = 33386;
pub const DEBUG_SEVERITY_NOTIFICATION: GLuint = 33387;
pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLuint = 33388;
pub const DEBUG_GROUP_STACK_DEPTH: GLuint = 33389;
pub const BUFFER: GLuint = 33504;
pub const SHADER: GLuint = 33505;
pub const PROGRAM: GLuint = 33506;
pub const VERTEX_ARRAY: GLuint = 32884;
pub const QUERY: GLuint = 33507;
pub const PROGRAM_PIPELINE: GLuint = 33508;
pub const SAMPLER: GLuint = 33510;
pub const MAX_LABEL_LENGTH: GLuint = 33512;
pub const MAX_DEBUG_MESSAGE_LENGTH: GLuint = 37187;
pub const MAX_DEBUG_LOGGED_MESSAGES: GLuint = 37188;
pub const DEBUG_LOGGED_MESSAGES: GLuint = 37189;
pub const DEBUG_SEVERITY_HIGH: GLuint = 37190;
pub const DEBUG_SEVERITY_MEDIUM: GLuint = 37191;
pub const DEBUG_SEVERITY_LOW: GLuint = 37192;
pub const DEBUG_OUTPUT: GLuint = 37600;
pub const CONTEXT_FLAG_DEBUG_BIT: GLuint = 2;
pub const STACK_OVERFLOW: GLuint = 1283;
pub const STACK_UNDERFLOW: GLuint = 1284;
pub const GEOMETRY_SHADER: GLuint = 36313;
pub const GEOMETRY_SHADER_BIT: GLuint = 4;
pub const GEOMETRY_VERTICES_OUT: GLuint = 35094;
pub const GEOMETRY_INPUT_TYPE: GLuint = 35095;
pub const GEOMETRY_OUTPUT_TYPE: GLuint = 35096;
pub const GEOMETRY_SHADER_INVOCATIONS: GLuint = 34943;
pub const LAYER_PROVOKING_VERTEX: GLuint = 33374;
pub const LINES_ADJACENCY: GLuint = 10;
pub const LINE_STRIP_ADJACENCY: GLuint = 11;
pub const TRIANGLES_ADJACENCY: GLuint = 12;
pub const TRIANGLE_STRIP_ADJACENCY: GLuint = 13;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLuint = 36319;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLuint = 35372;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLuint = 35378;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLuint = 37155;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLuint = 37156;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLuint = 36320;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLuint = 36321;
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: GLuint = 36442;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLuint = 35881;
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLuint = 37583;
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GLuint = 37589;
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: GLuint = 37069;
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLuint = 37079;
pub const FIRST_VERTEX_CONVENTION: GLuint = 36429;
pub const LAST_VERTEX_CONVENTION: GLuint = 36430;
pub const UNDEFINED_VERTEX: GLuint = 33376;
pub const PRIMITIVES_GENERATED: GLuint = 35975;
pub const FRAMEBUFFER_DEFAULT_LAYERS: GLuint = 37650;
pub const MAX_FRAMEBUFFER_LAYERS: GLuint = 37655;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLuint = 36264;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLuint = 36263;
pub const REFERENCED_BY_GEOMETRY_SHADER: GLuint = 37641;
pub const PRIMITIVE_BOUNDING_BOX: GLuint = 37566;
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLuint = 4;
pub const CONTEXT_FLAGS: GLuint = 33310;
pub const LOSE_CONTEXT_ON_RESET: GLuint = 33362;
pub const GUILTY_CONTEXT_RESET: GLuint = 33363;
pub const INNOCENT_CONTEXT_RESET: GLuint = 33364;
pub const UNKNOWN_CONTEXT_RESET: GLuint = 33365;
pub const RESET_NOTIFICATION_STRATEGY: GLuint = 33366;
pub const NO_RESET_NOTIFICATION: GLuint = 33377;
pub const CONTEXT_LOST: GLuint = 1287;
pub const SAMPLE_SHADING: GLuint = 35894;
pub const MIN_SAMPLE_SHADING_VALUE: GLuint = 35895;
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLuint = 36443;
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLuint = 36444;
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLuint = 36445;
pub const PATCHES: GLuint = 14;
pub const PATCH_VERTICES: GLuint = 36466;
pub const TESS_CONTROL_OUTPUT_VERTICES: GLuint = 36469;
pub const TESS_GEN_MODE: GLuint = 36470;
pub const TESS_GEN_SPACING: GLuint = 36471;
pub const TESS_GEN_VERTEX_ORDER: GLuint = 36472;
pub const TESS_GEN_POINT_MODE: GLuint = 36473;
pub const ISOLINES: GLuint = 36474;
pub const QUADS: GLuint = 7;
pub const FRACTIONAL_ODD: GLuint = 36475;
pub const FRACTIONAL_EVEN: GLuint = 36476;
pub const MAX_PATCH_VERTICES: GLuint = 36477;
pub const MAX_TESS_GEN_LEVEL: GLuint = 36478;
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLuint = 36479;
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLuint = 36480;
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLuint = 36481;
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLuint = 36482;
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLuint = 36483;
pub const MAX_TESS_PATCH_COMPONENTS: GLuint = 36484;
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLuint = 36485;
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLuint = 36486;
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLuint = 36489;
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLuint = 36490;
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLuint = 34924;
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLuint = 34925;
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLuint = 36382;
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLuint = 36383;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLuint = 37581;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLuint = 37582;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLuint = 37587;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLuint = 37588;
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLuint = 37067;
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLuint = 37068;
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLuint = 37080;
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLuint = 37081;
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLuint = 33313;
pub const IS_PER_PATCH: GLuint = 37607;
pub const REFERENCED_BY_TESS_CONTROL_SHADER: GLuint = 37639;
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: GLuint = 37640;
pub const TESS_CONTROL_SHADER: GLuint = 36488;
pub const TESS_EVALUATION_SHADER: GLuint = 36487;
pub const TESS_CONTROL_SHADER_BIT: GLuint = 8;
pub const TESS_EVALUATION_SHADER_BIT: GLuint = 16;
pub const TEXTURE_BORDER_COLOR: GLuint = 4100;
pub const CLAMP_TO_BORDER: GLuint = 33069;
pub const TEXTURE_BUFFER: GLuint = 35882;
pub const TEXTURE_BUFFER_BINDING: GLuint = 35882;
pub const MAX_TEXTURE_BUFFER_SIZE: GLuint = 35883;
pub const TEXTURE_BINDING_BUFFER: GLuint = 35884;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLuint = 35885;
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLuint = 37279;
pub const SAMPLER_BUFFER: GLuint = 36290;
pub const INT_SAMPLER_BUFFER: GLuint = 36304;
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLuint = 36312;
pub const IMAGE_BUFFER: GLuint = 36945;
pub const INT_IMAGE_BUFFER: GLuint = 36956;
pub const UNSIGNED_INT_IMAGE_BUFFER: GLuint = 36967;
pub const TEXTURE_BUFFER_OFFSET: GLuint = 37277;
pub const TEXTURE_BUFFER_SIZE: GLuint = 37278;
pub const COMPRESSED_RGBA_ASTC_4x4: GLuint = 37808;
pub const COMPRESSED_RGBA_ASTC_5x4: GLuint = 37809;
pub const COMPRESSED_RGBA_ASTC_5x5: GLuint = 37810;
pub const COMPRESSED_RGBA_ASTC_6x5: GLuint = 37811;
pub const COMPRESSED_RGBA_ASTC_6x6: GLuint = 37812;
pub const COMPRESSED_RGBA_ASTC_8x5: GLuint = 37813;
pub const COMPRESSED_RGBA_ASTC_8x6: GLuint = 37814;
pub const COMPRESSED_RGBA_ASTC_8x8: GLuint = 37815;
pub const COMPRESSED_RGBA_ASTC_10x5: GLuint = 37816;
pub const COMPRESSED_RGBA_ASTC_10x6: GLuint = 37817;
pub const COMPRESSED_RGBA_ASTC_10x8: GLuint = 37818;
pub const COMPRESSED_RGBA_ASTC_10x10: GLuint = 37819;
pub const COMPRESSED_RGBA_ASTC_12x10: GLuint = 37820;
pub const COMPRESSED_RGBA_ASTC_12x12: GLuint = 37821;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GLuint = 37840;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GLuint = 37841;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GLuint = 37842;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GLuint = 37843;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GLuint = 37844;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GLuint = 37845;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GLuint = 37846;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GLuint = 37847;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GLuint = 37848;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GLuint = 37849;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GLuint = 37850;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GLuint = 37851;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GLuint = 37852;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GLuint = 37853;
pub const TEXTURE_CUBE_MAP_ARRAY: GLuint = 36873;
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLuint = 36874;
pub const SAMPLER_CUBE_MAP_ARRAY: GLuint = 36876;
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLuint = 36877;
pub const INT_SAMPLER_CUBE_MAP_ARRAY: GLuint = 36878;
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLuint = 36879;
pub const IMAGE_CUBE_MAP_ARRAY: GLuint = 36948;
pub const INT_IMAGE_CUBE_MAP_ARRAY: GLuint = 36959;
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLuint = 36970;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLuint = 37122;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLuint = 37125;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37131;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37132;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLuint = 37133;