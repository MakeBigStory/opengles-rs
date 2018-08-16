use super::ffi;
use super::ffi::GLDEBUGPROC;
use es20::data_struct::*;
use es20::wrapper::Error;
use std::result::Result;

pub struct Wrapper {}

impl Wrapper {
    pub fn gl_blend_barrier(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glBlendBarrier();
        }

        Ok(())
    }

    pub fn gl_copy_image_sub_data(
        &mut self,
        srcName: GLuint,
        srcTarget: GLenum,
        srcLevel: GLint,
        srcX: GLint,
        srcY: GLint,
        srcZ: GLint,
        dst_Name: GLuint,
        dst_Target: GLenum,
        dst_Level: GLint,
        dst_X: GLint,
        dst_Y: GLint,
        dst_Z: GLint,
        src_Width: GLsizei,
        src_Height: GLsizei,
        src_Depth: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyImageSubData(
                srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target, dst_Level,
                dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
            );
        }
        Ok(())
    }

    pub fn gl_debug_message_control(
        &mut self,
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDebugMessageControl(source, type_, severity, count, ids, enabled);
        }
        Ok(())
    }

    pub fn gl_debug_message_insert(
        &mut self,
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDebugMessageInsert(source, type_, id, severity, length, buf);
        }
        Ok(())
    }

    pub fn gl_debug_message_callback(
        &mut self,
        callback: GLDEBUGPROC,
        userParam: *const GLvoid,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDebugMessageCallback(callback, userParam);
        }
        Ok(())
    }

    pub fn gl_get_debug_message_Log(
        &mut self,
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        message_log: *mut GLchar,
    ) -> Result<u32, Error> {
        unsafe {
            let result = ffi::glGetDebugMessageLog(
                count,
                bufSize,
                sources,
                types,
                ids,
                severities,
                lengths,
                message_log,
            );
            Ok(result as u32)
        }
    }

    pub fn gl_push_debug_group(
        &mut self,
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glPushDebugGroup(source, id, length, message);
        }
        Ok(())
    }

    pub fn gl_pop_debug_group(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glPopDebugGroup();
        }
        Ok(())
    }

    pub fn gl_object_label(
        &mut self,
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glObjectLabel(identifier, name, length, label);
        }
        Ok(())
    }

    pub fn gl_get_object_label(
        &mut self,
        ptr: *const GLvoid,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
        }
        Ok(())
    }

    pub fn gl_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glObjectPtrLabel(ptr, length, label);
        }
        Ok(())
    }

    pub fn gl_get_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
        }
        Ok(())
    }

    pub fn gl_get_pointer_v(&mut self, pname: GLenum, params: *mut *mut GLvoid) -> Result<(), Error> {
        unsafe {
            ffi::glGetPointerv(pname, params);
        }
        Ok(())
    }

    pub fn gl_enable_i(&mut self, target: GLenum, index: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glEnablei(target, index);
        }
        Ok(())
    }

    pub fn gl_disable_i(&mut self, target: GLenum, index: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glDisablei(target, index);
        }
        Ok(())
    }

    pub fn gl_blend_equation_i(&mut self, buf: GLuint, mode: GLenum) -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquationi(buf, mode);
        }
        Ok(())
    }

    pub fn gl_blend_equation_separate_i(
        &mut self,
        buf: GLuint,
        mode_RGB: GLenum,
        mode_alpha: GLenum,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquationSeparatei(buf, mode_RGB, mode_alpha);
        }
        Ok(())
    }

    pub fn gl_blend_func_i(&mut self, buf: GLuint, src: GLenum, dst: GLenum) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFunci(buf, src, dst);
        }
        Ok(())
    }

    pub fn gl_blend_func_separate_i(
        &mut self,
        buf: GLuint,
        src_rgb: GLenum,
        d_stRG_b: GLenum,
        _srcAlpha: GLenum,
        d_stAlpha: GLenum,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFuncSeparatei(buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha);
        }
        Ok(())
    }

    pub fn gl_color_mask_i(
        &mut self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glColorMaski(index, r, g, b, a);
        }
        Ok(())
    }

    pub fn gl_is_enabled_i(&mut self, target: GLenum, index: GLuint) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glIsEnabledi(target, index);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_draw_elements_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawElementsBaseVertex(mode, count, type_, indices, base_vertex);
        }
        Ok(())
    }

    pub fn gl_draw_range_elements_base_vertex(
        &mut self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawRangeElementsBaseVertex(
                mode,
                start,
                end,
                count,
                type_,
                indices,
                base_vertex,
            );
        }
        Ok(())
    }

    pub fn gl_draw_elements_instanced_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        instance_count: GLsizei,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawElementsInstancedBaseVertex(
                mode,
                count,
                type_,
                indices,
                instance_count,
                base_vertex,
            );
        }
        Ok(())
    }

    pub fn gl_frame_buffer_texture(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferTexture(target, attachment, texture, level);
        }
        Ok(())
    }

    pub fn gl_primitive_bounding_box(
        &mut self,
        minX: GLfloat,
        minY: GLfloat,
        minZ: GLfloat,
        minW: GLfloat,
        maxX: GLfloat,
        maxY: GLfloat,
        maxZ: GLfloat,
        maxW: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glPrimitiveBoundingBox(minX, minY, minZ, minW, maxX, maxY, maxZ, maxW);
        }
        Ok(())
    }

    pub fn gl_get_graphics_reset_status(&mut self) -> Result<GLenum, Error> {
        unsafe {
            let result = ffi::glGetGraphicsResetStatus();
            Ok(result)
        }
    }

    pub fn gl_read_n_pixels(
        &mut self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        data: *mut GLvoid,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glReadnPixels(x, y, width, height, format, type_, bufSize, data);
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_fv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetnUniformfv(program, location, bufSize, params);
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_iv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetnUniformiv(program, location, bufSize, params);
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetnUniformuiv(program, location, bufSize, params);
        }
        Ok(())
    }

    pub fn gl_minsampleshading(&mut self, value: GLfloat) -> Result<(), Error> {
        unsafe {
            ffi::glMinSampleShading(value);
        }
        Ok(())
    }

    pub fn gl_patch_parameter_i(&mut self, pname: GLenum, value: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glPatchParameteri(pname, value);
        }
        Ok(())
    }

    pub fn gl_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexParameterIiv(target, pname, params);
        }
        Ok(())
    }

    pub fn gl_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexParameterIuiv(target, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetTexParameterIiv(target, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetTexParameterIuiv(target, pname, params);
        }

        Ok(())
    }

    pub fn gl_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterIiv(sampler, pname, param);
        }
        Ok(())
    }

    pub fn gl_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterIuiv(sampler, pname, param);
        }
        Ok(())
    }

    pub fn gl_get_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterIiv(sampler, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetSamplerParameterIuiv(sampler, pname, params);
        }
        Ok(())
    }

    pub fn gl_tex_buffer(
        &mut self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexBuffer(target, internal_format, buffer);
        }
        Ok(())
    }

    pub fn gl_tex_buffer_range(
        &mut self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexBufferRange(target, internal_format, buffer, offset, size);
        }
        Ok(())
    }

    pub fn gl_tex_storage_3D_multi_sample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixed_sample_locations: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage3DMultisample(
                target,
                samples,
                internal_format,
                width,
                height,
                depth,
                fixed_sample_locations,
            );
        }
        Ok(())
    }
}
