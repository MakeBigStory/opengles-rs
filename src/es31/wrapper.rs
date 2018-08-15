use super::ffi::*;
use std;
use types::*;
use super::super::es20::data_struct::GL_TRUE;

struct Wrapper {
    glDispatchComputePtr: *const c_void,
    glDispatchComputeIndirectPtr: *const c_void,
    glDrawArraysIndirectPtr: *const c_void,
    glDrawElementsIndirectPtr: *const c_void,
    glFramebufferParameteriPtr: *const c_void,
    glGetFramebufferParameterivPtr: *const c_void,
    glGetProgramInterfaceivPtr: *const c_void,
    glGetProgramResourceIndexPtr: *const c_void,
    glGetProgramResourceNamePtr: *const c_void,
    glGetProgramResourceivPtr: *const c_void,
    glGetProgramResourceLocationPtr: *const c_void,
    glUseProgramStagesPtr: *const c_void,
    glActiveShaderProgramPtr: *const c_void,
    glCreateShaderProgramvPtr: *const c_void,
    glBindProgramPipelinePtr: *const c_void,
    glDeleteProgramPipelinesPtr: *const c_void,
    glGenProgramPipelinesPtr: *const c_void,
    glIsProgramPipelinePtr: *const c_void,
    glGetProgramPipelineivPtr: *const c_void,
    glProgramUniform1iPtr: *const c_void,
    glProgramUniform2iPtr: *const c_void,
    glProgramUniform3iPtr: *const c_void,
    glProgramUniform4iPtr: *const c_void,
    glProgramUniform1uiPtr: *const c_void,
    glProgramUniform2uiPtr: *const c_void,
    glProgramUniform3uiPtr: *const c_void,
    glProgramUniform4uiPtr: *const c_void,
    glProgramUniform1fPtr: *const c_void,
    glProgramUniform2fPtr: *const c_void,
    glProgramUniform3fPtr: *const c_void,
    glProgramUniform4fPtr: *const c_void,
    glProgramUniform1ivPtr: *const c_void,
    glProgramUniform2ivPtr: *const c_void,
    glProgramUniform3ivPtr: *const c_void,
    glProgramUniform4ivPtr: *const c_void,
    glProgramUniform1uivPtr: *const c_void,
    glProgramUniform2uivPtr: *const c_void,
    glProgramUniform3uivPtr: *const c_void,
    glProgramUniform4uivPtr: *const c_void,
    glProgramUniform1fvPtr: *const c_void,
    glProgramUniform2fvPtr: *const c_void,
    glProgramUniform3fvPtr: *const c_void,
    glProgramUniform4fvPtr: *const c_void,
    glProgramUniformMatrix2fvPtr: *const c_void,
    glProgramUniformMatrix3fvPtr: *const c_void,
    glProgramUniformMatrix4fvPtr: *const c_void,
    glProgramUniformMatrix2x3fvPtr: *const c_void,
    glProgramUniformMatrix3x2fvPtr: *const c_void,
    glProgramUniformMatrix2x4fvPtr: *const c_void,
    glProgramUniformMatrix4x2fvPtr: *const c_void,
    glProgramUniformMatrix3x4fvPtr: *const c_void,
    glProgramUniformMatrix4x3fvPtr: *const c_void,
    glValidateProgramPipelinePtr: *const c_void,
    glGetProgramPipelineInfoLogPtr: *const c_void,
    glBindImageTexturePtr: *const c_void,
    glGetBooleani_vPtr: *const c_void,
    glMemoryBarrierPtr: *const c_void,
    glMemoryBarrierByRegionPtr: *const c_void,
    glTexStorage2DMultisamplePtr: *const c_void,
    glGetMultisamplefvPtr: *const c_void,
    glSampleMaskiPtr: *const c_void,
    glGetTexLevelParameterivPtr: *const c_void,
    glGetTexLevelParameterfvPtr: *const c_void,
    glBindVertexBufferPtr: *const c_void,
    glVertexAttribFormatPtr: *const c_void,
    glVertexAttribIFormatPtr: *const c_void,
    glVertexAttribBindingPtr: *const c_void,
    glVertexBindingDivisorPtr: *const c_void,
}

impl Wrapper {
    pub fn gl_dispatch_compute(
        &self,
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint,
    ) {
        unsafe {
            if self.glDispatchComputePtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
                    self.glDispatchComputePtr,
                )(num_groups_x, num_groups_y, num_groups_z);
            }
        }
    }

    pub fn gl_dispatch_compute_indirect(&self, indirect: GLintptr) {
        unsafe {
            if self.glDispatchComputeIndirectPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(
                    self.glDispatchComputeIndirectPtr,
                )(indirect);
            }
        }
    }

    pub fn gl_draw_arrays_indirect(&self, mode: GLenum, indirect: *const ::std::os::raw::c_void) {
        unsafe {
            if self.glDrawArraysIndirectPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, *const c_void) -> ()>(
                    self.glDrawArraysIndirectPtr,
                )(mode as GLenum, indirect);
            }
        }
    }

    pub fn gl_draw_elements_indirect(&self, mode: GLenum, type_: GLenum, indirect: *const GLvoid) {
        unsafe {
            if self.glDrawElementsIndirectPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLvoid) -> ()>(
                    self.glDrawElementsIndirectPtr,
                )(mode, type_, indirect);
            }
        }
    }

    pub fn gl_framebuffer_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        unsafe {
            if self.glFramebufferParameteriPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
                    self.glFramebufferParameteriPtr,
                )(target, pname, param);
            }
        }
    }

    pub fn gl_get_framebuffer_parameteriv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe {
            if self.glGetFramebufferParameterivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                    self.glGetFramebufferParameterivPtr,
                )(target, pname, params);
            }
        }
    }

    pub fn gl_get_program_interfaceiv(
        &self,
        program: GLuint,
        program_interface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe {
            if self.glGetProgramInterfaceivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> (),
                >(self.glGetProgramInterfaceivPtr)(
                    program, program_interface, pname, params
                );
            }
        }
    }

    pub fn gl_get_program_resource_index(
        &self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> u32 {
        unsafe {
            if self.glGetProgramResourceIndexPtr != 0 as *const c_void {
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
                >(self.glGetProgramResourceIndexPtr)(
                    program, program_interface, name
                ) as u32;
                result
            } else {
                0
            }
        }
    }

    pub fn gl_get_program_resource_name(
        &self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        unsafe {
            if self.glGetProgramResourceivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar)
                        -> (),
                >(self.glGetProgramResourceivPtr)(
                    program,
                    program_interface,
                    index,
                    buf_size,
                    length,
                    name,
                );
            }
        }
    }

    pub fn gl_get_program_resourceiv(
        &self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        propCount: GLsizei,
        props: *const GLenum,
        buf_size: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ) {
        unsafe {
            if self.glGetProgramResourceivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLuint,
                        GLenum,
                        GLuint,
                        GLsizei,
                        *const GLenum,
                        GLsizei,
                        *mut GLsizei,
                        *mut GLint,
                    ) -> (),
                >(self.glGetProgramResourceivPtr)(
                    program,
                    program_interface,
                    index,
                    propCount,
                    props,
                    buf_size,
                    length,
                    params,
                );
            }
        }
    }

    pub fn gl_get_program_resource_location(
        &self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> i32 {
        unsafe {
            if self.glGetProgramResourceLocationPtr != 0 as *const c_void {
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
                >(self.glGetProgramResourceLocationPtr)(
                    program, program_interface, name
                ) as i32;
                result
            } else {
                0
            }
        }
    }

    pub fn gl_use_program_stages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) {
        unsafe {
            if self.glUseProgramStagesPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(
                    self.glUseProgramStagesPtr,
                )(pipeline, stages, program);
            }
        }
    }

    pub fn gl_active_shader_program(&self, pipeline: GLuint, program: GLuint) {
        unsafe {
            if self.glActiveShaderProgramPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glActiveShaderProgramPtr,
                )(pipeline, program);
            }
        }
    }

    pub fn gl_create_shader_program_v(
        &self,
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> u32 {
        unsafe {
            if self.glCreateShaderProgramvPtr != 0 as *const c_void {
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint,
                >(self.glCreateShaderProgramvPtr)(type_, count, strings)
                    as u32;
                result
            } else {
                0
            }
        }
    }

    pub fn gl_bind_program_pipeline(&self, pipeline: GLuint) {
        unsafe {
            if self.glBindProgramPipelinePtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                    self.glBindProgramPipelinePtr,
                )(pipeline);
            }
        }
    }

    pub fn gl_delete_program_pipelines(&self, n: GLsizei, pipelines: *const GLuint) {
        unsafe {
            if self.glDeleteProgramPipelinesPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                    self.glDeleteProgramPipelinesPtr,
                )(n, pipelines);
            }
        }
    }

    pub fn gl_gen_program_pipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
        unsafe {
            if self.glGenProgramPipelinesPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenProgramPipelinesPtr,
                )(n, pipelines);
            }
        }
    }

    pub fn gl_is_program_pipeline(&self, pipeline: GLuint) -> bool {
        unsafe {
            if self.glIsProgramPipelinePtr != 0 as *const c_void {
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsProgramPipelinePtr,
                )(pipeline);
                if result == GL_TRUE {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    }

    pub fn gl_get_program_pipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe {
            if self.glGetProgramPipelineivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                    self.glGetProgramPipelineivPtr,
                )(pipeline, pname, params);
            }
        }
    }

    pub fn gl_program_uniform1i(&self, program: GLuint, location: GLint, v0: GLint) {
        unsafe {
            if self.glProgramUniform1iPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(
                    self.glProgramUniform1iPtr,
                )(program, location, v0);
            }
        }
    }

    pub fn gl_program_uniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) {
        unsafe {
            if self.glProgramUniform2iPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(
                    self.glProgramUniform2iPtr,
                )(program, location, v0, v1);
            }
        }
    }

    pub fn gl_program_uniform3i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) {
        unsafe {
            if self.glProgramUniform3iPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> (),
                >(self.glProgramUniform3iPtr)(program, location, v0, v1, v2);
            }
        }
    }

    pub fn gl_program_uniform4i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) {
        unsafe {
            if self.glProgramUniform4iPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> (),
                >(self.glProgramUniform4iPtr)(program, location, v0, v1, v2, v3);
            }
        }
    }

    pub fn gl_program_uniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) {
        unsafe {
            if self.glProgramUniform1uiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(
                    self.glProgramUniform1uiPtr,
                )(program, location, v0);
            }
        }
    }

    pub fn gl_program_uniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
        unsafe {
            if self.glProgramUniform2uiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(
                    self.glProgramUniform2uiPtr,
                )(program, location, v0, v1);
            }
        }
    }

    pub fn gl_program_uniform3ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        unsafe {
            if self.glProgramUniform3uiPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> (),
                >(self.glProgramUniform3uiPtr)(program, location, v0, v1, v2);
            }
        }
    }

    pub fn gl_program_uniform4ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        unsafe {
            if self.glProgramUniform4uiPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> (),
                >(self.glProgramUniform4uiPtr)(program, location, v0, v1, v2, v3);
            }
        }
    }

    pub fn gl_program_uniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) {
        unsafe {
            if self.glProgramUniform1fPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(
                    self.glProgramUniform1fPtr,
                )(program, location, v0);
            }
        }
    }

    pub fn gl_program_uniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
        unsafe {
            if self.glProgramUniform2fPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>(
                    self.glProgramUniform2fPtr,
                )(program, location, v0, v1);
            }
        }
    }

    pub fn gl_program_uniform3f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform3fPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> (),
                >(self.glProgramUniform3fPtr)(program, location, v0, v1, v2);
            }
        }
    }

    pub fn gl_program_uniform4f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform4fPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
                >(self.glProgramUniform4fPtr)(program, location, v0, v1, v2, v3);
            }
        }
    }

    pub fn gl_program_uniform1iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        unsafe {
            if self.glProgramUniform1ivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform1ivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform2iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        unsafe {
            if self.glProgramUniform2ivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform2ivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform3iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        unsafe {
            if self.glProgramUniform3ivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform3ivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform4iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) {
        unsafe {
            if self.glProgramUniform4ivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform4ivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform1uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        unsafe {
            if self.glProgramUniform1uivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform1uivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform2uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        unsafe {
            if self.glProgramUniform2uivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform2uivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform3uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        unsafe {
            if self.glProgramUniform3uivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform3uivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform4uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) {
        unsafe {
            if self.glProgramUniform4uivPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform4uivPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform1fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform4fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform4fvPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform2fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform2fvPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform3fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform3fvPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniform4fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform4fvPtr)(program, location, count, value);
            }
        }
    }

    pub fn gl_program_uniform_matrix2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix2fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix3fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix4fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix2x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix2x3fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2x3fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix3x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix3x2fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3x2fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix2x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix2x4fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2x4fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix4x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix4x2fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4x2fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix3x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix3x4fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3x4fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_program_uniform_matrix4x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            if self.glProgramUniformMatrix4x3fvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4x3fvPtr)(
                    program, location, count, transpose, value
                );
            }
        }
    }

    pub fn gl_validate_program_pipeline(&self, pipeline: GLuint) {
        unsafe {
            if self.glValidateProgramPipelinePtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                    self.glValidateProgramPipelinePtr,
                )(pipeline);
            }
        }
    }

    pub fn gl_get_program_pipeline_info_log(
        &self,
        pipeline: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) {
        unsafe {
            if self.glGetProgramPipelineInfoLogPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetProgramPipelineInfoLogPtr)(
                    pipeline, buf_size, length, infoLog
                );
            }
        }
    }

    pub fn gl_bind_image_texture(
        &self,
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum,
    ) {
        unsafe {
            if self.glBindImageTexturePtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum)
                        -> (),
                >(self.glBindImageTexturePtr)(
                    unit, texture, level, layered, layer, access, format
                );
            }
        }
    }

    pub fn gl_get_boolean_iv(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
        unsafe {
            if self.glGetBooleani_vPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(
                    self.glGetBooleani_vPtr,
                )(target, index, data);
            }
        }
    }

    pub fn gl_memory_barrier(&self, barriers: GLbitfield) {
        unsafe {
            if self.glMemoryBarrierPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                    self.glMemoryBarrierPtr,
                )(barriers);
            }
        }
    }

    pub fn gl_memory_barrier_by_region(&self, barriers: GLbitfield) {
        unsafe {
            if self.glMemoryBarrierByRegionPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                    self.glMemoryBarrierByRegionPtr,
                )(barriers);
            }
        }
    }

    pub fn gl_tex_storage2D_multi_sample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) {
        unsafe {
            if self.glTexStorage2DMultisamplePtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> (),
                >(self.glTexStorage2DMultisamplePtr)(
                    target,
                    samples,
                    internalformat,
                    width,
                    height,
                    fixedsamplelocations,
                );
            }
        }
    }

    pub fn gl_get_multisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
        unsafe {
            if self.glGetMultisamplefvPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(
                    self.glGetMultisamplefvPtr,
                )(pname, index, val);
            }
        }
    }

    pub fn gl_sample_mask_i(&self, maskNumber: GLuint, mask: GLbitfield) {
        unsafe {
            if self.glSampleMaskiPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(
                    self.glSampleMaskiPtr,
                )(maskNumber, mask);
            }
        }
    }

    pub fn gl_get_tex_level_parameter_iv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe {
            if self.glGetTexLevelParameterivPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(
                    self.glGetTexLevelParameterivPtr,
                )(target, level, pname, params);
            }
        }
    }

    pub fn gl_get_tex_level_parameter_fv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) {
        unsafe {
            if self.glGetTexLevelParameterfvPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> (),
                >(self.glGetTexLevelParameterfvPtr)(target, level, pname, params);
            }
        }
    }

    pub fn gl_bind_vertex_buffer(
        &self,
        binding_index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        unsafe {
            if self.glBindVertexBufferPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>(
                    self.glBindVertexBufferPtr,
                )(binding_index, buffer, offset, stride);
            }
        }
    }

    pub fn gl_vertex_attrib_format(
        &self,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) {
        unsafe {
            if self.glVertexAttribFormatPtr != 0 as *const c_void {
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> (),
                >(self.glVertexAttribFormatPtr)(
                    attribindex,
                    size,
                    type_,
                    normalized,
                    relativeoffset,
                );
            }
        }
    }

    pub fn gl_vertex_attrib_I_format(
        &self,
        attri_bindex: GLuint,
        size: GLint,
        type_: GLenum,
        relative_offset: GLuint,
    ) {
        unsafe {
            if self.glVertexAttribIFormatPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(
                    self.glVertexAttribIFormatPtr,
                )(attri_bindex, size, type_, relative_offset);
            }
        }
    }

    pub fn gl_vertex_attrib_binding(&self, attri_bindex: GLuint, binding_index: GLuint) {
        unsafe {
            if self.glVertexAttribBindingPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glVertexAttribBindingPtr,
                )(attri_bindex, binding_index);
            }
        }
    }

    pub fn gl_vertex_binding_divisor(&self, binding_index: GLuint, divisor: GLuint) {
        unsafe {
            if self.glVertexBindingDivisorPtr != 0 as *const c_void {
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glVertexBindingDivisorPtr,
                )(binding_index, divisor);
            }
        }
    }
}
