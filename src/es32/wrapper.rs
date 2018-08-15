use super::super::es20::data_struct::*;
use super::super::es20::wrapper::Error;
use super::ffi::*;
use libc::c_void;
use std;
use std::result::Result;

pub struct Wrapper {
    pub glBlendBarrierPtr: *const c_void,
    pub glCopyImageSubDataPtr: *const c_void,
    pub glDebugMessageControlPtr: *const c_void,
    pub glDebugMessageInsertPtr: *const c_void,
    pub glDebugMessageCallbackPtr: *const c_void,
    pub glGetDebugMessageLogPtr: *const c_void,
    pub glPushDebugGroupPtr: *const c_void,
    pub glPopDebugGroupPtr: *const c_void,
    pub glObjectLabelPtr: *const c_void,
    pub glGetObjectLabelPtr: *const c_void,
    pub glObjectPtrLabelPtr: *const c_void,
    pub glGetObjectPtrLabelPtr: *const c_void,
    pub glGetPointervPtr: *const c_void,
    pub glEnableiPtr: *const c_void,
    pub glDisableiPtr: *const c_void,
    pub glBlendEquationiPtr: *const c_void,
    pub glBlendEquationSeparateiPtr: *const c_void,
    pub glBlendFunciPtr: *const c_void,
    pub glBlendFuncSeparateiPtr: *const c_void,
    pub glColorMaskiPtr: *const c_void,
    pub glIsEnablediPtr: *const c_void,
    pub glDrawElementsBaseVertexPtr: *const c_void,
    pub glDrawRangeElementsBaseVertexPtr: *const c_void,
    pub glDrawElementsInstancedBaseVertexPtr: *const c_void,
    pub glFramebufferTexturePtr: *const c_void,
    pub glPrimitiveBoundingBoxPtr: *const c_void,
    pub glGetGraphicsResetStatusPtr: *const c_void,
    pub glReadnPixelsPtr: *const c_void,
    pub glGetnUniformfvPtr: *const c_void,
    pub glGetnUniformivPtr: *const c_void,
    pub glGetnUniformuivPtr: *const c_void,
    pub glMinSampleShadingPtr: *const c_void,
    pub glPatchParameteriPtr: *const c_void,
    pub glTexParameterIivPtr: *const c_void,
    pub glTexParameterIuivPtr: *const c_void,
    pub glGetTexParameterIivPtr: *const c_void,
    pub glGetTexParameterIuivPtr: *const c_void,
    pub glSamplerParameterIivPtr: *const c_void,
    pub glSamplerParameterIuivPtr: *const c_void,
    pub glGetSamplerParameterIivPtr: *const c_void,
    pub glGetSamplerParameterIuivPtr: *const c_void,
    pub glTexBufferPtr: *const c_void,
    pub glTexBufferRangePtr: *const c_void,
    pub glTexStorage3DMultisamplePtr: *const c_void,
}

impl Wrapper {
    pub fn gl_blend_barrier(&self) -> Result<(), Error> {
        unsafe {
            if self.glBlendBarrierPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn() -> ()>(self.glBlendBarrierPtr)();
            }
        }
        Ok(())
    }

    pub fn gl_copy_image_sub_data(
        &self,
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
            if self.glCopyImageSubDataPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLuint,
                        GLenum,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLuint,
                        GLenum,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLsizei,
                        GLsizei,
                        GLsizei,
                    ) -> (),
                >(self.glCopyImageSubDataPtr)(
                    srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target,
                    dst_Level, dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
                );
            }
        }
        Ok(())
    }

    pub fn gl_debug_message_control(
        &self,
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDebugMessageControlPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean)
                        -> (),
                >(self.glDebugMessageControlPtr)(
                    source, type_, severity, count, ids, enabled
                );
            }
        }
        Ok(())
    }

    pub fn gl_debug_message_insert(
        &self,
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDebugMessageInsertPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar)
                        -> (),
                >(self.glDebugMessageInsertPtr)(
                    source, type_, id, severity, length, buf
                );
            }
        }
        Ok(())
    }

    pub fn gl_debug_message_callback(
        &self,
        callback: GLDEBUGPROC,
        userParam: *const c_void,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDebugMessageCallbackPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const c_void) -> ()>(
                    self.glDebugMessageCallbackPtr,
                )(callback, userParam);
            }
        }
        Ok(())
    }

    pub fn gl_get_debug_message_Log(
        &self,
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
            if self.glGetDebugMessageLogPtr != 0 as *const c_void {
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLuint,
                        GLsizei,
                        *mut GLenum,
                        *mut GLenum,
                        *mut GLuint,
                        *mut GLenum,
                        *mut GLsizei,
                        *mut GLchar,
                    ) -> (GLuint),
                >(self.glGetDebugMessageLogPtr)(
                    count,
                    bufSize,
                    sources,
                    types,
                    ids,
                    severities,
                    lengths,
                    message_log,
                ) as u32;
                Ok(result)
            } else {
                Err(Error {})
            }
        }
    }

    pub fn gl_push_debug_group(
        &self,
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glPushDebugGroupPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                >(self.glPushDebugGroupPtr)(source, id, length, message);
            }
        }
        Ok(())
    }

    pub fn gl_pop_debug_group(&self) -> Result<(), Error> {
        unsafe {
            if self.glPopDebugGroupPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn() -> ()>(self.glPopDebugGroupPtr)();
            }
        }
        Ok(())
    }

    pub fn gl_object_label(
        &self,
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glObjectLabelPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                >(self.glObjectLabelPtr)(identifier, name, length, label);
            }
        }
        Ok(())
    }

    pub fn gl_get_object_label(
        &self,
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetObjectPtrLabelPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetObjectPtrLabelPtr)(
                    identifier, name, bufSize, length, label
                );
            }
        }
        Ok(())
    }

    pub fn gl_object_ptr_label(
        &self,
        ptr: *const c_void,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glObjectPtrLabelPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(*const c_void, GLsizei, *const GLchar) -> (),
                >(self.glObjectPtrLabelPtr)(ptr, length, label);
            }
        }
        Ok(())
    }

    pub fn gl_get_object_ptr_label(
        &self,
        ptr: *const c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetObjectPtrLabelPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetObjectPtrLabelPtr)(ptr, bufSize, length, label);
            }
        }
        Ok(())
    }

    pub fn gl_get_pointer_v(&self, pname: GLenum, params: *mut *mut c_void) -> Result<(), Error> {
        unsafe {
            if self.glGetPointervPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, *mut *mut c_void) -> ()>(
                    self.glGetPointervPtr,
                )(pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_enable_i(&self, target: GLenum, index: GLuint) -> Result<(), Error> {
        unsafe {
            if self.glEnableiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glEnableiPtr,
                )(target, index);
            }
        }
        Ok(())
    }

    pub fn gl_disable_i(&self, target: GLenum, index: GLuint) -> Result<(), Error> {
        unsafe {
            if self.glDisableiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glDisableiPtr,
                )(target, index);
            }
        }
        Ok(())
    }

    pub fn gl_blend_equation_i(&self, buf: GLuint, mode: GLenum) -> Result<(), Error> {
        unsafe {
            if self.glBlendEquationiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
                    self.glBlendEquationiPtr,
                )(buf, mode);
            }
        }
        Ok(())
    }

    pub fn gl_blend_equation_separate_i(
        &self,
        buf: GLuint,
        mode_RGB: GLenum,
        mode_alpha: GLenum,
    ) -> Result<(), Error> {
        unsafe {
            if self.glBlendEquationSeparateiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                    self.glBlendEquationSeparateiPtr,
                )(buf, mode_RGB, mode_alpha);
            }
        }
        Ok(())
    }

    pub fn gl_blend_func_i(&self, buf: GLuint, src: GLenum, dst: GLenum) -> Result<(), Error> {
        unsafe {
            if self.glBlendFunciPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                    self.glBlendFunciPtr,
                )(buf, src, dst);
            }
        }
        Ok(())
    }

    pub fn gl_blend_func_separate_i(
        &self,
        buf: GLuint,
        src_rgb: GLenum,
        d_stRG_b: GLenum,
        _srcAlpha: GLenum,
        d_stAlpha: GLenum,
    ) -> Result<(), Error> {
        unsafe {
            if self.glBlendFuncSeparateiPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> (),
                >(self.glBlendFuncSeparateiPtr)(
                    buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha
                );
            }
        }
        Ok(())
    }

    pub fn gl_color_mask_i(
        &self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            if self.glColorMaskiPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> (),
                >(self.glColorMaskiPtr)(index, r, g, b, a);
            }
        }
        Ok(())
    }

    pub fn gl_is_enabled_i(&self, target: GLenum, index: GLuint) -> Result<bool, Error> {
        unsafe {
            if self.glIsEnablediPtr != 0 as *const c_void {
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint) -> (GLboolean),
                >(self.glIsEnablediPtr)(target, index);
                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(Error {})
            }
        }
    }

    pub fn gl_draw_elements_base_vertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDrawElementsBaseVertexPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint) -> (),
                >(self.glDrawElementsBaseVertexPtr)(
                    mode, count, type_, indices, base_vertex
                );
            }
        }
        Ok(())
    }

    pub fn gl_draw_range_elements_base_vertex(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDrawRangeElementsBaseVertexPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLuint,
                        GLuint,
                        GLsizei,
                        GLenum,
                        *const c_void,
                        GLint,
                    ) -> (),
                >(self.glDrawRangeElementsBaseVertexPtr)(
                    mode,
                    start,
                    end,
                    count,
                    type_,
                    indices,
                    base_vertex,
                );
            }
        }
        Ok(())
    }

    pub fn gl_draw_elements_instanced_base_vertex(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instance_count: GLsizei,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glDrawElementsInstancedBaseVertexPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint)
                        -> (),
                >(self.glDrawElementsInstancedBaseVertexPtr)(
                    mode,
                    count,
                    type_,
                    indices,
                    instance_count,
                    base_vertex,
                );
            }
        }
        Ok(())
    }

    pub fn gl_frame_buffer_texture(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glFramebufferTexturePtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(
                    self.glFramebufferTexturePtr,
                )(target, attachment, texture, level);
            }
        }
        Ok(())
    }

    pub fn gl_primitive_bounding_box(
        &self,
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
            if self.glPrimitiveBoundingBoxPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLfloat,
                        GLfloat,
                        GLfloat,
                        GLfloat,
                        GLfloat,
                        GLfloat,
                        GLfloat,
                        GLfloat,
                    ) -> (),
                >(self.glPrimitiveBoundingBoxPtr)(
                    minX, minY, minZ, minW, maxX, maxY, maxZ, maxW
                );
            }
        }
        Ok(())
    }

    pub fn gl_get_graphics_reset_status(&self) -> Result<GLenum, Error> {
        unsafe {
            if self.glGetGraphicsResetStatusPtr != 0 as *const c_void {
                let result = std::mem::transmute::<_, extern "system" fn() -> (GLenum)>(
                    self.glGetGraphicsResetStatusPtr,
                )();
                Ok(result)
            } else {
                Err(Error {})
            }
        }
    }

    pub fn gl_read_n_pixels(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        data: *mut c_void,
    ) -> Result<(), Error> {
        unsafe {
            if self.glReadnPixelsPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLint,
                        GLint,
                        GLsizei,
                        GLsizei,
                        GLenum,
                        GLenum,
                        GLsizei,
                        *mut c_void,
                    ) -> (),
                >(self.glReadnPixelsPtr)(
                    x, y, width, height, format, type_, bufSize, data
                );
            }
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_fv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetnUniformfvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> (),
                >(self.glGetnUniformfvPtr)(program, location, bufSize, params);
            }
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_iv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetnUniformivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> (),
                >(self.glGetnUniformivPtr)(program, location, bufSize, params);
            }
        }
        Ok(())
    }

    pub fn gl_get_n_uniform_uiv(
        &self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetnUniformuivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> (),
                >(self.glGetnUniformuivPtr)(program, location, bufSize, params);
            }
        }
        Ok(())
    }

    pub fn gl_minsampleshading(&self, value: GLfloat) -> Result<(), Error> {
        unsafe {
            if self.glMinSampleShadingPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
                    self.glMinSampleShadingPtr,
                )(value);
            }
        }
        Ok(())
    }

    pub fn gl_patch_parameter_i(&self, pname: GLenum, value: GLint) -> Result<(), Error> {
        unsafe {
            if self.glPatchParameteriPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
                    self.glPatchParameteriPtr,
                )(pname, value);
            }
        }
        Ok(())
    }

    pub fn gl_tex_parameter_iiv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glTexParameterIivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
                    self.glTexParameterIivPtr,
                )(target, pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_tex_parameter_iuiv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glTexParameterIuivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
                    self.glTexParameterIuivPtr,
                )(target, pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_get_tex_parameter_iiv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetTexParameterIivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                    self.glGetTexParameterIivPtr,
                )(target, pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_get_tex_parameter_iuiv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetTexParameterIuivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(
                    self.glGetTexParameterIuivPtr,
                )(target, pname, params);
            }
        }

        Ok(())
    }

    pub fn gl_sampler_parameter_iiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glSamplerParameterIivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                    self.glSamplerParameterIivPtr,
                )(sampler, pname, param);
            }
        }
        Ok(())
    }

    pub fn gl_sampler_parameter_iuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glSamplerParameterIuivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(
                    self.glSamplerParameterIuivPtr,
                )(sampler, pname, param);
            }
        }
        Ok(())
    }

    pub fn gl_get_sampler_parameter_iiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetSamplerParameterIivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                    self.glSamplerParameterIivPtr,
                )(sampler, pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_get_sampler_parameter_iuiv(
        &self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glGetSamplerParameterIuivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                    self.glGetSamplerParameterIuivPtr,
                )(sampler, pname, params);
            }
        }
        Ok(())
    }

    pub fn gl_tex_buffer(
        &self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            if self.glTexBufferPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
                    self.glTexBufferPtr,
                )(target, internal_format, buffer);
            }
        }
        Ok(())
    }

    pub fn gl_tex_buffer_range(
        &self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        unsafe {
            if self.glTexBufferRangePtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> (),
                >(self.glTexBufferRangePtr)(
                    target, internal_format, buffer, offset, size
                );
            }
        }
        Ok(())
    }

    pub fn gl_tex_storage_3D_multi_sample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixed_sample_locations: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            if self.glTexStorage3DMultisamplePtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLsizei,
                        GLenum,
                        GLsizei,
                        GLsizei,
                        GLsizei,
                        GLboolean,
                    ) -> (),
                >(self.glTexStorage3DMultisamplePtr)(
                    target,
                    samples,
                    internal_format,
                    width,
                    height,
                    depth,
                    fixed_sample_locations,
                );
            }
        }
        Ok(())
    }
}
