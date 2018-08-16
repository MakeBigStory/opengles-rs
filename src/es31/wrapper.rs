use super::ffi;
use es20::data_struct::*;
use es20::wrapper::Error;
use std::result::Result;

struct Wrapper {}

impl Wrapper {
    pub fn gl_dispatch_compute(
        &self,
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDispatchCompute(num_groups_x, num_groups_y, num_groups_z);
        }
        Ok(())
    }

    pub fn gl_dispatch_compute_indirect(&self, indirect: GLintptr) -> Result<(), Error> {
        unsafe {
            ffi::glDispatchComputeIndirect(indirect);
        }
        Ok(())
    }

    pub fn gl_draw_arrays_indirect(
        &self,
        mode: GLenum,
        indirect: *const ::std::os::raw::c_void,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawArraysIndirect(mode as GLenum, indirect as *const GLvoid);
        }
        Ok(())
    }

    pub fn gl_draw_elements_indirect(
        &self,
        mode: GLenum,
        type_: GLenum,
        indirect: *const GLvoid,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawElementsIndirect(mode, type_, indirect);
        }
        Ok(())
    }

    pub fn gl_framebuffer_parameteri(
        &self,
        target: GLenum,
        pname: GLenum,
        param: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferParameteri(target, pname, param);
        }
        Ok(())
    }

    pub fn gl_get_framebuffer_parameteriv(
        &self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetFramebufferParameteriv(target, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_program_interfaceiv(
        &self,
        program: GLuint,
        program_interface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetProgramInterfaceivPtr(program, program_interface, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_program_resource_index(
        &self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> Result<u32, Error> {
        unsafe {
            let result = ffi::glGetProgramResourceIndex(program, program_interface, name) as u32;
            Ok(result)
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
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetProgramResourceivPtr(
                program,
                program_interface,
                index,
                buf_size,
                length,
                name,
            );
        }
        Ok(())
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
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetProgramResourceivPtr(
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
        Ok(())
    }

    pub fn gl_get_program_resource_location(
        &self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> Result<i32, Error> {
        unsafe {
            let result =
                ffi::glGetProgramResourceLocationPtr(program, program_interface, name) as i32;
            Ok(result)
        }
    }

    pub fn gl_use_program_stages(
        &self,
        pipeline: GLuint,
        stages: GLbitfield,
        program: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUseProgramStages(pipeline, stages, program);
        }
        Ok(())
    }

    pub fn gl_active_shader_program(&self, pipeline: GLuint, program: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glActiveShaderProgram(pipeline, program);
        }
        Ok(())
    }

    pub fn gl_create_shader_program_v(
        &self,
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> Result<u32, Error> {
        unsafe {
            let result = ffi::glCreateShaderProgramvPtr(type_, count, strings) as u32;
            Ok(result)
        }
    }

    pub fn gl_bind_program_pipeline(&self, pipeline: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindProgramPipeline(pipeline);
        }
        Ok(())
    }

    pub fn gl_delete_program_pipelines(
        &self,
        n: GLsizei,
        pipelines: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteProgramPipelines(n, pipelines);
        }
        Ok(())
    }

    pub fn gl_gen_program_pipelines(
        &self,
        n: GLsizei,
        pipelines: *mut GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGenProgramPipelines(n, pipelines);
        }
        Ok(())
    }

    pub fn gl_is_program_pipeline(&self, pipeline: GLuint) -> Result<bool, Error> {
        unsafe {
            let result = ffi::glIsProgramPipeline(pipeline);
            if result == GL_TRUE {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn gl_get_program_pipelineiv(
        &self,
        pipeline: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetProgramPipelineiv(pipeline, pname, params);
        }
        Ok(())
    }

    pub fn gl_program_uniform1i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform1i(program, location, v0);
        }
        Ok(())
    }

    pub fn gl_program_uniform2i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2i(program, location, v0, v1);
        }
        Ok(())
    }

    pub fn gl_program_uniform3i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3iPtr(program, location, v0, v1, v2);
        }
        Ok(())
    }

    pub fn gl_program_uniform4i(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4iPtr(program, location, v0, v1, v2, v3);
        }
        Ok(())
    }

    pub fn gl_program_uniform1ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform1ui(program, location, v0);
        }
        Ok(())
    }

    pub fn gl_program_uniform2ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2ui(program, location, v0, v1);
        }
        Ok(())
    }

    pub fn gl_program_uniform3ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3uiPtr(program, location, v0, v1, v2);
        }
        Ok(())
    }

    pub fn gl_program_uniform4ui(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4uiPtr(program, location, v0, v1, v2, v3);
        }
        Ok(())
    }

    pub fn gl_program_uniform1f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform1f(program, location, v0);
        }
        Ok(())
    }

    pub fn gl_program_uniform2f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2f(program, location, v0, v1);
        }
        Ok(())
    }

    pub fn gl_program_uniform3f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3fPtr(program, location, v0, v1, v2);
        }
        Ok(())
    }

    pub fn gl_program_uniform4f(
        &self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4fPtr(program, location, v0, v1, v2, v3);
        }
        Ok(())
    }

    pub fn gl_program_uniform1iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform1ivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform2iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2ivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform3iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3ivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform4iv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4ivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform1uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform1uivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform2uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2uivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform3uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3uivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform4uiv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4uivPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform1fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4fvPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform2fvPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform3fvPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniform4fvPtr(program, location, count, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix2fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix3fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix4fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix2x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix2x3fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix3x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix3x2fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix2x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix2x4fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix4x2fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix4x2fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix3x4fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix3x4fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_program_uniform_matrix4x3fv(
        &self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramUniformMatrix4x3fvPtr(program, location, count, transpose, value);
        }
        Ok(())
    }

    pub fn gl_validate_program_pipeline(&self, pipeline: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glValidateProgramPipeline(pipeline);
        }
        Ok(())
    }

    pub fn gl_get_program_pipeline_info_log(
        &self,
        pipeline: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetProgramPipelineInfoLogPtr(pipeline, buf_size, length, infoLog);
        }
        Ok(())
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
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindImageTexturePtr(unit, texture, level, layered, layer, access, format);
        }
        Ok(())
    }

    pub fn gl_get_boolean_iv(
        &self,
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetBooleani_v(target, index, data);
        }
        Ok(())
    }

    pub fn gl_memory_barrier(&self, barriers: GLbitfield) -> Result<(), Error> {
        unsafe {
            ffi::glMemoryBarrier(barriers);
        }
        Ok(())
    }

    pub fn gl_memory_barrier_by_region(&self, barriers: GLbitfield) -> Result<(), Error> {
        unsafe {
            ffi::glMemoryBarrierByRegion(barriers);
        }
        Ok(())
    }

    pub fn gl_tex_storage2D_multi_sample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage2DMultisamplePtr(
                target,
                samples,
                internalformat,
                width,
                height,
                fixedsamplelocations,
            );
        }
        Ok(())
    }

    pub fn gl_get_multisamplefv(
        &self,
        pname: GLenum,
        index: GLuint,
        val: *mut GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetMultisamplefv(pname, index, val);
        }
        Ok(())
    }

    pub fn gl_sample_mask_i(&self, maskNumber: GLuint, mask: GLbitfield) -> Result<(), Error> {
        unsafe {
            ffi::glSampleMaski(maskNumber, mask);
        }
        Ok(())
    }

    pub fn gl_get_tex_level_parameter_iv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetTexLevelParameteriv(target, level, pname, params);
        }
        Ok(())
    }

    pub fn gl_get_tex_level_parameter_fv(
        &self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetTexLevelParameterfvPtr(target, level, pname, params);
        }
        Ok(())
    }

    pub fn gl_bind_vertex_buffer(
        &self,
        binding_index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindVertexBuffer(binding_index, buffer, offset, stride);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_format(
        &self,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribFormatPtr(attribindex, size, type_, normalized, relativeoffset);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_I_format(
        &self,
        attri_bindex: GLuint,
        size: GLint,
        type_: GLenum,
        relative_offset: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribIFormat(attri_bindex, size, type_, relative_offset);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_binding(
        &self,
        attri_bindex: GLuint,
        binding_index: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribBinding(attri_bindex, binding_index);
        }
        Ok(())
    }

    pub fn gl_vertex_binding_divisor(
        &self,
        binding_index: GLuint,
        divisor: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexBindingDivisor(binding_index, divisor);
        }
        Ok(())
    }
}
