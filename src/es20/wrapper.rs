use super::data_struct::*;
use super::ffi::*;
use super::*;
use types::TextureUnit;
use types::BufferBindTarget;
use types::FrameBufferBindTarget;
use types::RenderBufferBindTarget;
use types::TextureBindTarget;
use types::BlendEquationMode;
use types::BlendFactor;
use types::BufferUsage;
use types::FrameBufferStatus;
use types::TextureTarget;
use types::ShaderType;
use types::CullFaceMode;
use types::DepthFunc;
use types::Feature;
use types::BeginMode;

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

pub struct Error {

}

pub struct Wrapper {
}

trait Interceptor {
    fn intercept(&mut self, fun_name: &str) -> Result<(), Error>;
}

impl Wrapper {
    pub fn gl_active_texture(&mut self, texture_unit: TextureUnit) -> Result<(), Error> {
        unsafe {
            ffi::glActiveTexture(texture_unit as GLenum);
        }

        Ok(())
    }

    pub fn gl_attach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glAttachShader(program as GLuint, shader as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_attrib_location(&mut self, program: u32, index: u32, name: &str) -> Result<(), Error> {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glBindAttribLocation(program as GLuint, index as GLuint,
                                      c_str.as_ptr() as *const c_char);
        }

        Ok(())
    }

    pub fn gl_bind_buffer(&mut self, target: BufferBindTarget, buffer: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindBuffer(target as GLenum, buffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_framebuffer(&mut self, target: FrameBufferBindTarget, framebuffer: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindFramebuffer(target as GLenum, framebuffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_renderbuffer(&mut self, target: RenderBufferBindTarget, renderbuffer: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBindRenderbuffer(target as GLenum, renderbuffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_texture(&mut self, target: TextureBindTarget, texture: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBindTexture(target as GLenum, texture as GLuint)
        }

        Ok(())
    }

    pub fn gl_blend_color(&mut self, red: f32, green: f32, blue: f32,
                          alpha: f32) -> Result<(), Error> {
        unsafe {
            ffi::glBlendColor(red as GLclampf, green as GLclampf,
                              blue as GLclampf, alpha as GLclampf)
        }

        Ok(())
    }

    pub fn gl_blend_equation(&mut self, mode: BlendEquationMode) -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquation(mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_equation_separate(&mut self, mode_rgb: BlendEquationMode, mode_alpha: BlendEquationMode)
                                      -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquationSeparate(mode_rgb as GLenum, mode_alpha as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_func(&mut self, src_factor: BlendFactor, dst_factor: BlendFactor) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFunc(src_factor as GLenum, dst_factor as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_func_separate(&mut self, src_rgb: BlendFactor, dst_rgb: BlendFactor,
                                  src_alpha: BlendFactor, dst_alpha: BlendFactor) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFuncSeparate(src_rgb as GLenum, dst_rgb as GLenum,
                                     src_alpha as GLenum, dst_alpha as GLenum)
        }

        Ok(())
    }

    pub fn gl_buffer_data<T>(&mut self, target: BufferBindTarget,
                             buffer: &[T], usage: BufferUsage) -> Result<(), Error> {
        unsafe {
            let t_size = size_of::<T>();

            ffi::glBufferData(
                target as GLenum,
                (buffer.len() * t_size) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
                usage as GLenum,
            )
        }

        Ok(())
    }

    pub fn gl_buffer_sub_data<T>(&mut self, target: BufferBindTarget, offset: u32, buffer: &[T])
                                 -> Result<(), Error> {
        unsafe {
            let t_size = size_of::<T>();

            ffi::glBufferSubData(
                target as GLenum,
                (offset * (t_size as u32)) as GLintptr,
                (buffer.len() * t_size) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_check_framebuffer_status(&mut self, target: FrameBufferBindTarget) -> Result<FrameBufferStatus, Error> {
        unsafe {
            let status = ffi::glCheckFramebufferStatus(target as GLenum);

            Ok(FrameBufferStatus::from(status))
        }
    }

    pub fn gl_clear(&mut self, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glClear(mask as GLbitfield)
        }

        Ok(())
    }

    pub fn gl_clear_color(&mut self, red: f32, green: f32,
                          blue: f32, alpha: f32) -> Result<(), Error> {
        unsafe {
            ffi::glClearColor(red as GLclampf, green as GLclampf, blue as GLclampf,
                              alpha as GLclampf)
        }

        Ok(())
    }

    pub fn gl_clear_depthf(&mut self, depth: f32) -> Result<(), Error> {
        unsafe {
            ffi::glClearDepthf(depth as GLclampf)
        }

        Ok(())
    }

    pub fn gl_clear_stencil(&mut self, stencil: i32) -> Result<(), Error> {
        unsafe {
            ffi::glClearStencil(stencil as GLint)
        }

        Ok(())
    }


    pub fn gl_color_mask(&mut self, red: bool, green: bool, blue: bool, alpha: bool) -> Result<(), Error> {
        unsafe {
            ffi::glColorMask(
                red as GLboolean,
                green as GLboolean,
                blue as GLboolean,
                alpha as GLboolean,
            )
        }

        Ok(())
    }

    pub fn gl_compile_shader(&mut self, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glCompileShader(shader as GLuint)
        }

        Ok(())
    }

    // TODO: internal_format 仍然是GLenum
    // OpenGL ES defines no specific compressed texture formats,
    // but does provide a mechanism to obtain symbolic constants
    // for such formats provided by extensions. The number of compressed
    // texture formats supported can be obtained by querying the value of
    // GL_NUM_COMPRESSED_TEXTURE_FORMATS. The list of specific compressed
    // texture formats supported can be obtained by querying the value of
    // GL_COMPRESSED_TEXTURE_FORMATS.

    pub fn gl_compressed_tex_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        internal_format: GLenum,
        width: u32,
        height: u32,
        border: u32,
        image_size: u32,
        buffer: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCompressedTexImage2D(
                target as GLenum,
                level as GLint,
                internal_format,
                width as GLsizei,
                height as GLsizei,
                border as GLint,
                image_size as GLsizei,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_compressed_tex_sub_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: GLenum,
        image_size: u32,
        buffer: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCompressedTexSubImage2D(
                target as GLenum,
                level as GLint,
                x_offset as GLint,
                y_offset as GLint,
                width as GLsizei,
                height as GLsizei,
                format,
                image_size as GLsizei,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_copy_tex_image_2d(
        &mut self,
        target: TextureTarget,
        level: u32,
        internal_format: GLenum,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        border: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyTexImage2D(target as GLenum,
                                  level as GLint,
                                  internal_format,
                                  x as GLint, y as GLint,
                                  width as GLsizei, height as GLsizei, border as GLint)
        }

        Ok(())
    }

    pub fn gl_copy_tex_sub_image_2d(
        &mut self,
        target: TextureTarget,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyTexSubImage2D(target as GLenum,
                                     level as GLint,
                                     x_offset as GLint, y_offset as GLint,
                                     x as GLint, y as GLint,
                                     width as GLsizei, height as GLsizei)
        }

        Ok(())
    }

    pub fn gl_create_program(&mut self) -> Result<u32, Error> {
        unsafe {
            let program_id = ffi::glCreateProgram();

            Ok(program_id as u32)
        }
    }

    pub fn gl_create_shader(&mut self, type_: ShaderType) -> Result<u32, Error> {
        unsafe {
            let shader_id = ffi::glCreateShader(type_ as GLenum);

            Ok(shader_id as u32)
        }
    }

    pub fn gl_cull_face(&mut self, mode: CullFaceMode) -> Result<(), Error> {
        unsafe {
            ffi::glCullFace(mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_delete_buffers(&mut self, buffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_framebuffers(&mut self, framebuffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_program(&mut self, program: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteProgram(program as GLuint)
        }

        Ok(())
    }

    pub fn gl_delete_renderbuffers(&mut self, renderbuffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei,
                                       renderbuffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_shader(&mut self, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteShader(shader as GLuint)
        }

        Ok(())
    }

    pub fn gl_delete_textures(&mut self, textures: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr())
        }

        Ok(())
    }

    pub fn gl_depth_func(&mut self, func: DepthFunc) -> Result<(), Error> {
        unsafe {
            ffi::glDepthFunc(func as GLenum)
        }

        Ok(())
    }

    pub fn gl_depth_mask(&mut self, flag: bool) -> Result<(), Error> {
        unsafe {
            ffi::glDepthMask(flag as GLboolean)
        }

        Ok(())
    }


    pub fn gl_depth_rangef(&mut self, z_near: f32, z_far: f32) -> Result<(), Error> {
        unsafe {
            ffi::glDepthRangef(z_near as GLclampf, z_far as GLclampf)
        }

        Ok(())
    }

    pub fn gl_detach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDetachShader(program as GLuint, shader as GLuint)
        }

        Ok(())
    }

    pub fn gl_disable(&mut self, feature: Feature) -> Result<(), Error> {
        unsafe {
            ffi::glDisable(feature as GLenum)
        }

        Ok(())
    }

    pub fn gl_disable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDisableVertexAttribArray(index as GLuint)
        }

        Ok(())
    }

    pub fn gl_draw_arrays(&mut self, mode: BeginMode, first: i32, count: i32) -> Result<(), Error> {
        unsafe {
            ffi::glDrawArrays(mode as GLenum, first as GLint, count as GLsizei)
        }

        Ok(())
    }

    // TODO: type_ & T is reasonable ?
    pub fn gl_draw_elements<T>(&mut self, mode: BeginMode, count: i32, type_: GLenum, indices: &[T]) -> Result<(), Error> {
        unsafe {
            ffi::glDrawElements(mode as GLenum, count as GLsizei,
                                type_, indices.as_ptr() as *const GLvoid)
        }

        Ok(())
    }

    pub fn gl_enable(&mut self, feature: Feature) -> Result<(), Error> {
        unsafe {
            ffi::glEnable(feature as GLenum)
        }

        Ok(())
    }

    pub fn gl_enable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        unsafe {
            ffi::glEnableVertexAttribArray(index as GLuint)
        }

        Ok(())
    }

    pub fn gl_finish(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glFinish()
        }

        Ok(())
    }

    pub fn gl_flush(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glFlush()
        }

        Ok(())
    }
}

pub fn gl_framebuffer_renderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffer_target: GLenum,
    renderbuffer: GLuint,
) {
    unsafe { ffi::glFramebufferRenderbuffer(target, attachment, renderbuffer_target, renderbuffer) }
}

pub fn gl_framebuffer_texture_2d(
    target: GLenum,
    attachment: GLenum,
    texture_target: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe { ffi::glFramebufferTexture2D(target, attachment, texture_target, texture, level) }
}

pub fn gl_front_face(mode: GLenum) {
    unsafe { ffi::glFrontFace(mode) }
}

pub fn gl_gen_buffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenBuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gl_generate_mipmap(target: GLenum) {
    unsafe { ffi::glGenerateMipmap(target) }
}

pub fn gl_gen_framebuffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenFramebuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gl_gen_renderbuffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenRenderbuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gl_gen_textures(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenTextures(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gl_get_active_attrib(program: GLuint, index: GLuint) -> Option<Active> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut size: GLint = 0;
        let mut type_: GLenum = 0;

        let mut name = String::with_capacity(256);

        ffi::glGetActiveAttrib(
            program,
            index,
            256,
            &mut length,
            &mut size,
            &mut type_,
            name.as_mut_vec().as_mut_ptr() as *mut GLchar,
        );

        if length > 0 {
            name.as_mut_vec().set_len(length as usize);
            name.truncate(length as usize);

            Some(Active {
                name,
                size,
                type_,
                length,
            })
        } else {
            None
        }
    }
}

pub fn gl_get_active_uniform(program: GLuint, index: GLuint) -> Option<Active> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut size: GLint = 0;
        let mut type_: GLenum = 0;

        let mut name = String::with_capacity(256);

        ffi::glGetActiveUniform(
            program,
            index,
            256,
            &mut length,
            &mut size,
            &mut type_,
            name.as_mut_vec().as_mut_ptr() as *mut GLchar,
        );

        if length > 0 {
            name.as_mut_vec().set_len(length as usize);
            name.truncate(length as usize);

            Some(Active {
                name,
                size,
                type_,
                length,
            })
        } else {
            None
        }
    }
}

pub fn gl_get_attached_shaders(program: GLuint, max_count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut count: GLsizei = 0;
        let mut vec: Vec<GLuint> = Vec::with_capacity(max_count as usize);

        ffi::glGetAttachedShaders(program, max_count, &mut count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec.truncate(count as usize);
        vec
    }
}

pub fn gl_get_attrib_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let c_str = CString::new(name).unwrap();

        ffi::glGetAttribLocation(program, c_str.as_ptr() as *const c_char)
    }
}

pub fn gl_get_booleanv(name: GLenum) -> bool {
    unsafe {
        let mut value: GLboolean = 0;

        ffi::glGetBooleanv(name, &mut value);

        value == GL_TRUE
    }
}

pub fn gl_get_buffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetBufferParameteriv(target, name, &mut value);

        value
    }
}

pub fn gl_get_error() -> GLenum {
    unsafe { ffi::glGetError() }
}

pub fn gl_get_floatv(name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetFloatv(name, &mut value);

        value
    }
}

pub fn gl_get_framebuffer_attachment_parameteriv(
    target: GLenum,
    attachment: GLenum,
    name: GLenum,
) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetFramebufferAttachmentParameteriv(target, attachment, name, &mut value);

        value
    }
}

pub fn gl_get_integerv(name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetIntegerv(name, &mut value);

        value
    }
}

pub fn gl_get_programiv(program: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetProgramiv(program, name, &mut value);

        value
    }
}

pub fn gl_get_program_info_log(program: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut log = String::with_capacity(max_length as usize);

        #[cfg(target_os = "ios")]
        ffi::glGetProgramInfoLog(
            program,
            max_length,
            &mut length,
            log.as_mut_vec().as_mut_ptr() as *mut i8,
        );

        #[cfg(target_os = "android")]
        ffi::glGetProgramInfoLog(
            program,
            max_length,
            &mut length,
            log.as_mut_vec().as_mut_ptr() as *mut u8,
        );

        if length > 0 {
            log.as_mut_vec().set_len(length as usize);
            log.truncate(length as usize);

            Some(log)
        } else {
            None
        }
    }
}

pub fn gl_get_renderbuffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetRenderbufferParameteriv(target, name, &mut value);

        value
    }
}

pub fn gl_get_shaderiv(shader: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetShaderiv(shader, name, &mut value);

        value
    }
}

#[warn(unused_variables)]
pub fn gl_get_shader_info_log(shader: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut log = String::with_capacity(max_length as usize);

        #[cfg(target_os = "ios")]
        ffi::glGetShaderInfoLog(
            shader,
            max_length,
            &mut length,
            log.as_mut_vec().as_mut_ptr() as *mut i8,
        );
        #[cfg(target_os = "android")]
        ffi::glGetShaderInfoLog(
            shader,
            max_length,
            &mut length,
            log.as_mut_vec().as_mut_ptr() as *mut u8,
        );

        if length > 0 {
            log.as_mut_vec().set_len(length as usize);
            log.truncate(length as usize);

            Some(log)
        } else {
            None
        }
    }
}

pub fn gl_get_shader_precision_format(
    shader_type: GLenum,
    precision_type: GLenum,
) -> ShaderPrecisionFormat {
    unsafe {
        let mut precision: GLint = 0;
        let mut range: [GLint; 2] = [0, 0];

        ffi::glGetShaderPrecisionFormat(
            shader_type,
            precision_type,
            range.as_mut_ptr(),
            &mut precision,
        );

        ShaderPrecisionFormat {
            precision: precision,
            range: range,
        }
    }
}

pub fn gl_get_shader_source(shader: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut source = String::with_capacity(max_length as usize);

        ffi::glGetShaderSource(
            shader,
            max_length,
            &mut length,
            source.as_mut_vec().as_mut_ptr() as *mut GLchar,
        );

        if length > 0 {
            source.as_mut_vec().set_len(length as usize);
            source.truncate(length as usize);

            Some(source)
        } else {
            None
        }
    }
}

pub fn gl_get_string(name: GLenum) -> Option<String> {
    unsafe {
        let c_str = ffi::glGetString(name);
        //todo : can't guarantee the lifetime, because the memory is allocated by C
        if !c_str.is_null() {
            match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                Ok(s) => Some(s.to_string()),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub fn gl_get_tex_parameterfv(target: GLenum, name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetTexParameterfv(target, name, &mut value);

        value
    }
}

pub fn gl_get_tex_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetTexParameteriv(target, name, &mut value);

        value
    }
}

pub fn gl_get_uniformfv(program: GLuint, location: GLint) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetUniformfv(program, location, &mut value);

        value
    }
}

pub fn gl_get_uniformiv(program: GLuint, location: GLint) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetUniformiv(program, location, &mut value);

        value
    }
}

pub fn gl_get_uniform_location(program: GLuint, name: &GLchar) -> GLint {
    unsafe { ffi::glGetUniformLocation(program, name) }
}

pub fn gl_get_vertex_attribfv(index: GLuint, name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetVertexAttribfv(index, name, &mut value);

        value
    }
}

pub fn gl_get_vertex_attribiv(index: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetVertexAttribiv(index, name, &mut value);

        value
    }
}

pub fn gl_hint(target: GLenum, mode: GLenum) {
    unsafe { ffi::glHint(target, mode) }
}

pub fn gl_is_buffer(buffer: GLuint) -> bool {
    unsafe { ffi::glIsBuffer(buffer) == GL_TRUE }
}

pub fn gl_is_enabled(feature: GLenum) -> bool {
    unsafe { ffi::glIsEnabled(feature) == GL_TRUE }
}

pub fn gl_is_framebuffer(framebuffer: GLuint) -> bool {
    unsafe { ffi::glIsFramebuffer(framebuffer) == GL_TRUE }
}

pub fn gl_is_program(program: GLuint) -> bool {
    unsafe { ffi::glIsProgram(program) == GL_TRUE }
}

pub fn gl_is_renderbuffer(renderbuffer: GLuint) -> bool {
    unsafe { ffi::glIsRenderbuffer(renderbuffer) == GL_TRUE }
}

pub fn gl_is_shader(shader: GLuint) -> bool {
    unsafe { ffi::glIsShader(shader) == GL_TRUE }
}

pub fn gl_is_texture(texture: GLuint) -> bool {
    unsafe { ffi::glIsTexture(texture) == GL_TRUE }
}

pub fn gl_line_width(width: GLfloat) {
    unsafe { ffi::glLineWidth(width) }
}

pub fn gl_link_program(program: GLuint) {
    unsafe { ffi::glLinkProgram(program) }
}

pub fn gl_pixel_storei(name: GLenum, param: GLint) {
    unsafe { ffi::glPixelStorei(name, param) }
}

pub fn gl_polygon_offset(factor: GLfloat, units: GLfloat) {
    unsafe { ffi::glPolygonOffset(factor, units) }
}

pub fn gl_read_pixels<T>(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    buffer: &mut [T],
) {
    unsafe {
        ffi::glReadPixels(
            x,
            y,
            width,
            height,
            format,
            type_,
            buffer.as_mut_ptr() as *mut GLvoid,
        )
    }
}

pub fn gl_read_pixels_rgba(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> Vec<u8> {
    unsafe {
        let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        ffi::glReadPixels(
            x,
            y,
            width,
            height,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            buffer.as_mut_ptr() as *mut GLvoid,
        );

        buffer.set_len((width * height * 4) as usize);
        buffer
    }
}

pub fn gl_release_shader_compiler() {
    unsafe { ffi::glReleaseShaderCompiler() }
}

pub fn gl_renderbuffer_storage(
    target: GLenum,
    internal_format: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe { ffi::glRenderbufferStorage(target, internal_format, width, height) }
}

pub fn gl_sample_coverage(value: GLclampf, invert: bool) {
    unsafe { ffi::glSampleCoverage(value, invert as GLboolean) }
}

pub fn gl_scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { ffi::glScissor(x, y, width, height) }
}

pub fn gl_shader_binary<T>(shaders: &[GLuint], data_format: GLenum, data: &[T], length: GLsizei) {
    unsafe {
        ffi::glShaderBinary(
            shaders.len() as GLsizei,
            shaders.as_ptr(),
            data_format,
            data.as_ptr() as *const GLvoid,
            length,
        )
    }
}

pub fn gl_shader_source(shader: GLuint, source: &[u8]) {
    unsafe {
        let length: GLsizei = source.len() as GLsizei;

        ffi::glShaderSource(shader, 1, &(source.as_ptr() as *const GLchar), &length)
    }
}

pub fn gl_stencil_func(func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe { ffi::glStencilFunc(func, ref_, mask) }
}

pub fn gl_stencil_func_separate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe { ffi::glStencilFuncSeparate(face, func, ref_, mask) }
}

pub fn gl_stencil_mask(mask: GLuint) {
    unsafe { ffi::glStencilMask(mask) }
}

pub fn gl_stencil_mask_separate(face: GLenum, mask: GLuint) {
    unsafe { ffi::glStencilMaskSeparate(face, mask) }
}

pub fn gl_stencil_op(fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
    unsafe { ffi::glStencilOp(fail, z_fail, z_pass) }
}

pub fn gl_stencil_op_separate(face: GLenum, fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
    unsafe { ffi::glStencilOpSeparate(face, fail, z_fail, z_pass) }
}

pub fn gl_tex_image_2d<T>(
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    buffer: &[T],
) {
    unsafe {
        ffi::glTexImage2D(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            type_,
            buffer.as_ptr() as *const GLvoid,
        )
    }
}

pub fn gl_tex_parameterf(target: GLenum, name: GLenum, value: GLfloat) {
    unsafe { ffi::glTexParameterf(target, name, value) }
}

pub fn gl_tex_parameterfv(target: GLenum, name: GLenum, value: &GLfloat) {
    unsafe { ffi::glTexParameterfv(target, name, value) }
}

pub fn gl_tex_parameteri(target: GLenum, name: GLenum, value: GLint) {
    unsafe { ffi::glTexParameteri(target, name, value) }
}

pub fn gl_tex_parameteriv(target: GLenum, name: GLenum, value: &GLint) {
    unsafe { ffi::glTexParameteriv(target, name, value) }
}

pub fn gl_tex_sub_image_2d<T>(
    target: GLenum,
    level: GLint,
    x_offset: GLint,
    y_offset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    buffer: &[T],
) {
    unsafe {
        ffi::glTexSubImage2D(
            target,
            level,
            x_offset,
            y_offset,
            width,
            height,
            format,
            type_,
            buffer.as_ptr() as *const GLvoid,
        )
    }
}

pub fn gl_uniform1f(location: GLint, x: GLfloat) {
    unsafe { ffi::glUniform1f(location, x) }
}

pub fn gl_uniform1fv(location: GLint, values: &[GLfloat]) {
    unsafe { ffi::glUniform1fv(location, values.len() as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform1i(location: GLint, x: GLint) {
    unsafe { ffi::glUniform1i(location, x) }
}

pub fn gl_uniform1iv(location: GLint, values: &[GLint]) {
    unsafe { ffi::glUniform1iv(location, values.len() as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform2f(location: GLint, x: GLfloat, y: GLfloat) {
    unsafe { ffi::glUniform2f(location, x, y) }
}

pub fn gl_uniform2fv(location: GLint, values: &[GLfloat]) {
    unsafe { ffi::glUniform2fv(location, (values.len() / 2) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform2i(location: GLint, x: GLint, y: GLint) {
    unsafe { ffi::glUniform2i(location, x, y) }
}

pub fn gl_uniform2iv(location: GLint, values: &[GLint]) {
    unsafe { ffi::glUniform2iv(location, (values.len() / 2) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe { ffi::glUniform3f(location, x, y, z) }
}

pub fn gl_uniform3fv(location: GLint, values: &[GLfloat]) {
    unsafe { ffi::glUniform3fv(location, (values.len() / 3) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
    unsafe { ffi::glUniform3i(location, x, y, z) }
}

pub fn gl_uniform3iv(location: GLint, values: &[GLint]) {
    unsafe { ffi::glUniform3iv(location, (values.len() / 3) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe { ffi::glUniform4f(location, x, y, z, w) }
}

pub fn gl_uniform4fv(location: GLint, values: &[GLfloat]) {
    unsafe { ffi::glUniform4fv(location, (values.len() / 4) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe { ffi::glUniform4i(location, x, y, z, w) }
}

pub fn gl_uniform4iv(location: GLint, values: &[GLint]) {
    unsafe { ffi::glUniform4iv(location, (values.len() / 4) as GLsizei, values.as_ptr()) }
}

pub fn gl_uniform_matrix2fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix2fv(
            location,
            (values.len() / 2) as GLsizei,
            transpose as GLboolean,
            values.as_ptr() as *const GLfloat,
        )
    }
}

pub fn gl_uniform_matrix3fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix3fv(
            location,
            (values.len() / 3) as GLsizei,
            transpose as GLboolean,
            values.as_ptr() as *const GLfloat,
        )
    }
}

pub fn gl_uniform_matrix4fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix4fv(
            location,
            (values.len() / 4) as GLsizei,
            transpose as GLboolean,
            values.as_ptr() as *const GLfloat,
        )
    }
}

pub fn gl_use_program(program: GLuint) {
    unsafe { ffi::glUseProgram(program) }
}

pub fn gl_validate_program(program: GLuint) {
    unsafe { ffi::glValidateProgram(program) }
}

pub fn gl_vertex_attrib1f(index: GLuint, x: GLfloat) {
    unsafe { ffi::glVertexAttrib1f(index, x) }
}

pub fn gl_vertex_attrib1fv(index: GLuint, values: &[GLfloat]) {
    unsafe { ffi::glVertexAttrib1fv(index, values.as_ptr()) }
}

pub fn gl_vertex_attrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    unsafe { ffi::glVertexAttrib2f(index, x, y) }
}

pub fn gl_vertex_attrib2fv(index: GLuint, values: &[GLfloat]) {
    unsafe { ffi::glVertexAttrib2fv(index, values.as_ptr()) }
}

pub fn gl_vertex_attrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe { ffi::glVertexAttrib3f(index, x, y, z) }
}

pub fn gl_vertex_attrib3fv(index: GLuint, values: &[GLfloat]) {
    unsafe { ffi::glVertexAttrib3fv(index, values.as_ptr()) }
}

pub fn gl_vertex_attrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe { ffi::glVertexAttrib4f(index, x, y, z, w) }
}

pub fn gl_vertex_attrib4fv(index: GLuint, values: &[GLfloat]) {
    unsafe { ffi::glVertexAttrib4fv(index, values.as_ptr()) }
}

pub fn gl_vertex_attrib_pointer<T>(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: bool,
    stride: GLsizei,
    buffer: &[T],
) {
    unsafe {
        if buffer.len() == 0 {
            ffi::glVertexAttribPointer(
                index,
                size,
                type_,
                normalized as GLboolean,
                stride,
                &0 as *const i32 as *const GLvoid,
            )
        } else {
            ffi::glVertexAttribPointer(
                index,
                size,
                type_,
                normalized as GLboolean,
                stride,
                buffer.as_ptr() as *const GLvoid,
            )
        }
    }
}

pub fn gl_vertex_attrib_pointer_offset(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: bool,
    stride: GLsizei,
    offset: GLuint,
) {
    unsafe {
        ffi::glVertexAttribPointer(
            index,
            size,
            type_,
            normalized as GLboolean,
            stride,
            offset as *const GLvoid,
        )
    }
}

pub fn gl_viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { ffi::glViewport(x, y, width, height) }
}
