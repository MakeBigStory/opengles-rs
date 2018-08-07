use super::es20::data_struct::*;
use super::data_struct::*;
use super::ffi::*;
use super::*;
use super::types::*;

use std::mem;

use std::ptr;
use std::slice;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub struct Error {}

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

/// Buffer Objects

pub enum ColorBufferMode {
    Back = GL_BACK as isize,
    None = GL_NONE as isize,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15,
    MaxColorAttachments = GL_MAX_COLOR_ATTACHMENTS,
}

pub enum BufferObjectTarget {
    ArrayBuffer = GL_ARRAY_BUFFER as GLenum,
    CopyReadBuffer = GL_COPY_READ_BUFFER as GLenum,
    CopyWriteBuffer = GL_COPY_WRITE_BUFFER as GLenum,
    ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER as GLenum,
    PixelPackBuffer = GL_PIXEL_PACK_BUFFER as GLenum,
    PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER as GLenum,
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER as GLenum,
    UniformBuffer = GL_UNIFORM_BUFFER as GLenum,
}

pub enum BufferBindingTarget {
    /// Vertex attributes
    GL_ARRAY_BUFFER,
    /// Atomic counter storage
    GL_ATOMIC_COUNTER_BUFFER,
    /// Buffer copy source
    GL_COPY_READ_BUFFER,
    /// Buffer copy destination
    GL_COPY_WRITE_BUFFER,
    /// Indirect compute dispatch commands
    GL_DISPATCH_INDIRECT_BUFFER,
    /// Indirect command arguments
    GL_DRAW_INDIRECT_BUFFER,
    /// Vertex array indices
    GL_ELEMENT_ARRAY_BUFFER,
    /// Pixel read target
    GL_PIXEL_PACK_BUFFER,
    /// Texture data source
    GL_PIXEL_UNPACK_BUFFER,
    /// Query result buffer
    GL_QUERY_BUFFER,
    /// Read-write storage for shaders
    GL_SHADER_STORAGE_BUFFER,
    /// Texture data buffer
    GL_TEXTURE_BUFFER,
    /// Transform feedback buffer
    GL_TRANSFORM_FEEDBACK_BUFFER,
    /// Uniform block storage
    GL_UNIFORM_BUFFER,
}

pub enum BufferMapTarget {
    BufferMapPointer = GL_BUFFER_MAP_POINTER,
}

pub enum MappingBit {
    InvalidateRange = GL_MAP_INVALIDATE_RANGE_BIT,
    InvalidateBuffer = GL_MAP_INVALIDATE_BUFFER_BIT,
    FlushExplicit = GL_MAP_FLUSH_EXPLICIT_BIT,
    Unsynchronized = GL_MAP_UNSYNCHRONIZED_BIT,
}

/// Samplers

pub enum SamplerParameter {
    GL_TEXTURE_WRAP_S,
    GL_TEXTURE_WRAP_T,
    GL_TEXTURE_WRAP_R,
    GL_TEXTURE_MIN_FILTER,
    GL_TEXTURE_MAG_FILTER,
    GL_TEXTURE_BORDER_COLOR,
    GL_TEXTURE_MIN_LOD,
    GL_TEXTURE_MAX_LOD,
    GL_TEXTURE_LOD_BIAS,
    GL_TEXTURE_COMPARE_MODE,
    GL_TEXTURE_COMPARE_FUNC,
}

pub enum TransformFeedbackMode {
    GL_INTERLEAVED_ATTRIBS,
    GL_SEPARATE_ATTRIBS,
}

pub enum TransformFeedbackObjectTarget {
    GL_TRANSFORM_FEEDBACK,
}

pub enum FrameBufferTarget {
    FrameBuffer = GL_FRAMEBUFFER as isize,
}

pub enum AttachmentTarget {
    Color_Attachment_0 = GL_COLOR_ATTACHMENT0,
    GL_COLOR_ATTACHMENT1,
    GL_COLOR_ATTACHMENT2,
    GL_COLOR_ATTACHMENT3,
    GL_DEPTH_ATTACHMENT,
    GL_STENCIL_ATTACHMENT,
    GL_DEPTH_STENCIL_ATTACHMENT,
}

pub enum FilterMode {
    GL_NEAREST,
    GL_LINEAR,
}

pub enum BufferMask {
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
    GL_STENCIL_BUFFER_BIT,
}

pub enum FramebufferTarget {
    GL_DRAW_FRAMEBUFFER,
    GL_READ_FRAMEBUFFER,
    /// GL_FRAMEBUFFER is equivalent to GL_DRAW_FRAMEBUFFER
    GL_FRAMEBUFFER,
}

pub struct Wrapper {}

impl Wrapper {
    pub fn gl_read_buffer(&mut self, mode: ColorBufferMode) -> Result<(), Error> {
        unsafe {
            ffi::glReadBuffer(mode as GLenum);
        }
        Ok(())
    }

    pub fn gl_draw_buffers(&mut self, bufs: &[ColorBufferMode]) -> Result<(), Error> {
        unsafe {
            let n: GLsizei = bufs.len() as i32;
            ffi::glDrawBuffers(n, bufs.as_ptr() as *const GLenum);
        }
        Ok(())
    }

    pub fn gl_unmap_buffer(&mut self, target: BufferObjectTarget) -> Result<(GLboolean), Error> {
        unsafe {
            ffi::glUnmapBuffer(target as GLenum)
        }
        Ok((GL_TRUE))
    }

    pub fn gl_copy_buffer_sub_data(&mut self,
        readTarget: BufferObjectTarget,
        writeTarget: BufferObjectTarget,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyBufferSubData(
                readTarget,
                writeTarget,
                readOffset,
                writeOffset,
                size,
            );
        }
        Ok(())
    }

    //todo : *mut *mut GLvoid
    pub fn gl_get_buffer_pointerv<T>(&mut self, target: BufferObjectTarget, pname: BufferMapTarget, params: *mut *mut GLvoid) -> Result<(), Error> {
        unsafe {
            ffi::glGetBufferPointerv(target, pname, params);
        }
        Ok(())
    }

    //todo : reference to program binary
    pub fn gl_map_buffer_range<'a, T>(&mut self,
        target: BufferBindingTarget,
        offset: GLintptr,
        length: GLsizeiptr,
        access: MappingBit,
    ) -> &'a [T] {
        unsafe {
            let ptr = ffi::glMapBufferRange(
                target,
                offset,
                length,
                access,
            );

            let count = length as usize / std::mem::size_of::<T>();
            return slice::from_raw_parts_mut(ptr as *mut T, count as usize);
        }
        Ok(())
    }

    pub fn gl_flush_mapped_buffer_range(&mut self, target: BufferBindingTarget, offset: i32, length: i32) -> Result<(), Error> {
        unsafe {
            ffi::glFlushMappedBufferRange(target, offset as GLintptr, length as GLsizeiptr);
        }
        Ok(())
    }

    // todo: target范围变小
    pub fn gl_bind_buffer_range(&mut self,
        target: BufferBindingTarget,
        index: u32,
        buffer: u32,
        offset: i32,
        size: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBindBufferRange(
                target,
                index as GLuint,
                buffer as GLuint,
                offset as GLintptr,
                size as GLsizeiptr,
            );
        }
        Ok(())
    }

    // todo: target范围变小
    pub fn gl_bind_buffer_base(&mut self, target: BufferBindingTarget, index: u32, buffer: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBindBufferBase(target as GLenum, index as GLuint, buffer as GLuint);
        }
        Ok(())
    }

    // todo: [GLint]
    pub fn gl_clear_bufferiv(&mut self, buffer: GLenum, draw_buffer: i32, value: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferiv(buffer, draw_buffer as GLint, value.as_ptr() as *const GLint);
        }
        Ok(())
    }

    pub fn gl_clear_bufferuiv(&mut self, buffer: GLenum, drawbuffer: GLint, value: &[GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferuiv(buffer, drawbuffer, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_clear_bufferfv(&mut self, buffer: GLenum, drawbuffer: GLint, value: &[GLfloat]) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferfv(buffer, drawbuffer, value.as_ptr() as *const GLfloat);
        }
        Ok(())
    }

    pub fn gl_clear_bufferfi(&mut self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glClearBufferfi(buffer, drawbuffer, depth, stencil);
        }
        Ok(())
    }

    pub fn gl_get_buffer_parameteri64v(&mut self, target: GLenum, pname: GLenum) -> GLint64 {
        unsafe {
            let mut params = 0 as GLint64;
            ffi::glGetBufferParameteri64v(target, pname, &mut params);
            params
        }
        Ok(())
    }

    pub fn gl_tex_image3d(&mut self,
        target: TextureTarget,
        level: i32,
        internal_format: i32,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: i32,
        format: GLenum,
        type_: GLenum,
        opt_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        let pdata = match opt_data {
            Some(data) => mem::transmute(data.as_ptr()),
            None => ptr::null(),
        };

        unsafe {
            ffi::glTexImage3D(target as GLenum, level as GLint, internal_format as GLint, width, height, depth, border as GLint, format, type_, pdata);
        }
        Ok(())
    }

    pub fn gl_tex_sub_image3d(&mut self,
        target: TextureTarget,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
                                 opt_data: Option<&[u8]>,
    ) -> Result<(), Error> {
        let pdata = match opt_data {
            Some(data) => mem::transmute(data.as_ptr()),
            None => ptr::null(),
        };

        unsafe {
            ffi::glTexSubImage3D(
                target as GLenum,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                type_,
                pdata,
            );
        }
        Ok(())
    }

    pub fn gl_copy_tex_sub_image3d(&mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            glCopyTexSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                x,
                y,
                width,
                height,
            );
        }
        Ok(())
    }

    pub fn gl_compressed_tex_image3d<T>(&mut self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCompressedTexImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                imageSize,
                data.as_ptr() as *const GLvoid,
            );
            ;
        }
        Ok(())
    }

    pub fn gl_compressed_tex_sub_image3d<T>(&mut self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCompressedTexSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                imageSize,
                data.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    pub fn gl_gen_queries(&mut self, size: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
            ffi::glGenQueries(size, ids.as_ptr() as *mut GLuint);
            ids
        }
        Ok(())
    }

    pub fn gl_delete_queries(&mut self, ids: &mut [GLuint]) -> Result<(), Error> {
        unsafe {
            let n: GLsizei = ids.len() as i32;
            ffi::glDeleteQueries(n, ids.as_ptr() as *mut GLuint);
        }
        Ok(())
    }

    pub fn gl_is_query(&mut self, id: GLuint) -> GLboolean {
        unsafe {
            ffi::glIsQuery(id)
        }
        Ok(())
    }

    pub fn gl_begin_query(&mut self, target: GLenum, id: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBeginQuery(target, id);
        }
        Ok(())
    }

    pub fn gl_end_query(&mut self, target: GLenum) -> Result<(), Error> {
        unsafe {
            ffi::glEndQuery(target);
        }
        Ok(())
    }

    pub fn gl_get_queryiv(&mut self, target: GLenum, pname: GLenum, params: &mut [GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glGetQueryiv(target, pname, params.as_ptr() as *mut GLint);
        }
        Ok(())
    }

    pub fn gl_get_query_objectuiv(&mut self, id: GLuint, pname: GLenum, params: &mut [GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glGetQueryObjectuiv(id, pname, params.as_mut_ptr() as *mut GLuint);
        }
        Ok(())
    }

    pub fn gl_uniform_matrix2x3fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix2x3fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix3x2fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix3x2fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix2x4fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix2x4fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix4x2fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix4x2fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix3x4fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix3x4fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_uniform_matrix4x3fv(&mut self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix4x3fv(
                location,
                count,
                transpose,
                value.as_ptr() as *const GLfloat,
            );
        }
        Ok(())
    }

    pub fn gl_renderbuffer_storage_multisample(&mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glRenderbufferStorageMultisample(
                target,
                samples,
                internalformat,
                width,
                height,
            );
        }
        Ok(())
    }


    pub fn gl_bind_vertex_array(&mut self, array: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindVertexArray(array);
        }
        Ok(())
    }

    pub fn gl_delete_vertex_arrays(&mut self, arrays: &[u32]) -> Result<(), Error> {
        unsafe {
            if arrays.len() > 0 {
                let n = arrays.len() as i32;
                ffi::glDeleteVertexArrays(n, arrays.as_ptr() as *const GLuint);
            }
        }
        Ok(())
    }

    pub fn gl_gen_vertex_arrays(&mut self, count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);
            ffi::glGenVertexArrays(count, vec.as_mut_ptr());
            vec.set_len(count as usize);
            vec
        }
        Ok(())
    }

    pub fn gl_is_vertex_array(&mut self, array: GLuint) -> GLboolean {
        unsafe {
            ffi::glIsVertexArray(array)
        }
        Ok(())
    }

    pub fn gl_get_integeri_v(&mut self, target: GLenum, index: GLuint) -> GLint {
        unsafe {
            let mut value: GLint = 0;
            ffi::glGetIntegeri_v(target, index, &mut value);
            value
        }
        Ok(())
    }


    /// Transform Feedback

    pub fn gl_end_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glEndTransformFeedback();
        }
        Ok(())
    }

    //todo: count这么写是否对？
    pub fn gl_transform_feedback_varyings(&mut self,
        program: GLuint,
        count: GLsizei,
        varyings: &Vec<String>,
        bufferMode: TransformFeedbackMode,
    ) -> Result<(), Error> {
        unsafe {
            let mut names: Vec<CString> = Vec::with_capacity(count as usize);
            let mut index = 0 as usize;
            while index < count as usize {
                names.push(CString::new(&(varyings[index])[..]).unwrap());
                index = index + 1;
            }
            index = 0;
            let ptr = names[index].as_ptr();
            let mut names_ptr: Vec<usize> = Vec::with_capacity(count as usize);

            while index < count as usize {
                names_ptr.push(names[index].as_ptr() as usize);
                index = index + 1;
            }
            ffi::glTransformFeedbackVaryings(
                program,
                count,
                names_ptr.as_ptr() as *const *const GLchar,
                bufferMode,
            );
        }
        Ok(())
    }

    pub fn gl_get_transform_feedback_varying(&mut self,
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
    ) -> Option<Active> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: GLsizei = 0;
            let mut type_: GLenum = GL_NONE;
            let mut name = String::with_capacity(256);

            ffi::glGetTransformFeedbackVarying(
                program,
                index,
                bufSize,
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
        Ok(())
    }

    pub fn gl_bind_transform_feedback(&mut self, target: TransformFeedbackObjectTarget, id: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindTransformFeedback(target, id);
        }
        Ok(())
    }

    pub fn gl_delete_transform_feedbacks(&mut self, ids: &[GLuint]) -> Result<(), Error> {
        unsafe {
            let n = ids.len() as i32;
            ffi::glDeleteTransformFeedbacks(n, ids.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_gen_transform_feedbacks(&mut self, size: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
            ffi::glGenTransformFeedbacks(size, ids.as_mut_ptr() as *mut GLuint);
            ids
        }
        Ok(())
    }

    pub fn gl_is_transform_feedback(&mut self, id: GLuint) -> GLboolean {
        unsafe { ffi::glIsTransformFeedback(id) }
        Ok(())
    }

    pub fn gl_pause_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glPauseTransformFeedback();
        }
        Ok(())
    }

    pub fn gl_resume_transform_feedback(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glResumeTransformFeedback();
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_ipointer<T>(&mut self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribIPointer(
                index,
                size,
                type_,
                stride,
                pointer.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    pub fn gl_get_vertex_attrib_iiv(&mut self, index: GLuint, pname: GLenum) -> GLint {
        unsafe {
            let mut params: GLint = 0;
            ffi::glGetVertexAttribIiv(index, pname, &mut params);
            params
        }
        Ok(())
    }

    pub fn gl_get_vertex_attrib_iuiv(&mut self, index: GLuint, pname: GLenum) -> GLuint {
        unsafe {
            let mut params: GLuint = 0;
            ffi::glGetVertexAttribIuiv(index, pname, &mut params);
            params
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4i(&mut self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4i(index, x, y, z, w);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4ui(&mut self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4ui(index, x, y, z, w);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4iv(&mut self, index: GLuint, v: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4iv(index, v.as_ptr() as *const GLint);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_i4uiv(&mut self, index: GLuint, v: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribI4uiv(index, v.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_get_uniformuiv(&mut self, program: GLuint, location: GLint) -> GLuint {
        unsafe {
            let mut value: GLuint = 0;
            glGetUniformuiv(program, location, &mut value);
            value
        }
        Ok(())
    }

    pub fn gl_get_frag_data_location(&mut self, program: GLuint, name: &str) -> GLint {
        unsafe {
            let c_str = CString::new(name).unwrap();
            ffi::glGetFragDataLocation(program, c_str.as_ptr() as *const GLchar)
        }
        Ok(())
    }

    pub fn gl_uniform1ui(&mut self, location: GLint, v0: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1ui(location, v0);
        }
        Ok(())
    }

    pub fn gl_uniform2ui(&mut self, location: GLint, v0: GLuint, v1: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2ui(location, v0, v1);
        }
        Ok(())
    }

    pub fn gl_uniform3ui(&mut self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3ui(location, v0, v1, v2);
        }
        Ok(())
    }

    pub fn gl_uniform4ui(&mut self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4ui(location, v0, v1, v2, v3);
        }
        Ok(())
    }

    pub fn gl_uniform1uiv(&mut self, location: GLint, count: GLsizei, value: &[GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1uiv(location, count, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_uniform2uiv(&mut self, location: GLint, count: GLsizei, value: &[GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2uiv(location, count, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_uniform3uiv(&mut self, location: GLint, count: GLsizei, value: &[GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3uiv(location, count, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_uniform4uiv(&mut self, location: GLint, count: GLsizei, value: &[GLuint]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4uiv(location, count, value.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_get_stringi(&mut self, name: GLenum, index: GLuint) -> Option<String> {
        unsafe {
            let c_str = ffi::glGetStringi(name, index);
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

    //todo:
    pub fn gl_get_uniform_indices(&mut self,
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: &Vec<String>,
    ) -> Vec<GLuint>
    {
        unsafe {
            let mut names: Vec<CString> = Vec::with_capacity(uniformCount as usize);
            let mut index = 0 as usize;
            while index < uniformCount as usize {
                names.push(CString::new(&(uniformNames[index])[..]).unwrap());
                index = index + 1;
            }
            index = 0;
            let ptr = names[index].as_ptr();
            let mut names_ptr: Vec<usize> = Vec::with_capacity(uniformCount as usize);

            while index < uniformCount as usize {
                names_ptr.push(names[index].as_ptr() as usize);
                index = index + 1;
            }

            let mut uniformIndices: Vec<GLuint> = Vec::with_capacity(uniformCount as usize);

            ffi::glGetUniformIndices(
                program,
                uniformCount,
                names_ptr.as_ptr() as *const *const GLchar,
                uniformIndices.as_ptr() as *mut GLuint,
            );

            uniformIndices
        }
        Ok(())
    }

    pub fn gl_get_active_uniformsiv(&mut self,
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: &[GLuint],
        pname: GLenum,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glGetActiveUniformsiv(
                program,
                uniformCount,
                uniformIndices.as_ptr() as *const GLuint,
                pname,
                params.as_mut_ptr() as *mut GLint,
            );
        }
        Ok(())
    }

    pub fn gl_get_uniform_block_index(&mut self, program: GLuint, uniformBlockName: &str) -> GLuint {
        unsafe {
            let c_str = CString::new(uniformBlockName).unwrap();
            ffi::glGetUniformBlockIndex(program, c_str.as_ptr() as *const GLchar)
        }
        Ok(())
    }

    pub fn gl_get_active_uniform_blockiv(&mut self,
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
    ) -> GLint {
        unsafe {
            let mut value = 0 as GLint;
            ffi::glGetActiveUniformBlockiv(
                program,
                uniformBlockIndex,
                pname,
                &mut value,
            );
            value
        }
        Ok(())
    }

    pub fn gl_uniform_block_binding(&mut self,
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glUniformBlockBinding(
                program,
                uniformBlockIndex,
                uniformBlockBinding,
            );
        }
        Ok(())
    }

    pub fn gl_draw_range_elements<T>(&mut self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: &[T],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawRangeElements(mode, start, end, count, type_,
                                     indices.as_ptr() as *const GLvoid);
        }
        Ok(())
    }

    pub fn gl_draw_arrays_instanced(&mut self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instance_count: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawArraysInstanced(
                mode,
                first,
                count,
                instance_count,
            );
        }
        Ok(())
    }

    pub fn gl_draw_elements_instanced<T>(&mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: &[T],
        instance_count: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glDrawElementsInstanced(
                mode,
                count,
                type_,
                indices.as_ptr() as *const GLvoid,
                instance_count,
            );
        }
        Ok(())
    }

    pub fn gl_fence_sync(&mut self, condition: GLenum, flags: GLbitfield) -> GLsync {
        unsafe {
            ffi::glFenceSync(condition, flags)
        }
        Ok(())
    }

    pub fn gl_is_sync(&mut self, sync: GLsync) -> GLboolean {
        unsafe {
            ffi::glIsSync(sync)
        }
        Ok(())
    }

    pub fn gl_delete_sync(&mut self, sync: GLsync) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteSync(sync);
        }
        Ok(())
    }

    pub fn gl_client_wait_sync(&mut self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        unsafe {
            ffi::glClientWaitSync(sync, flags, timeout)
        }
        Ok(())
    }

    pub fn gl_wait_sync(&mut self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> Result<(), Error> {
        unsafe {
            ffi::glWaitSync(sync, flags, timeout);
        }
        Ok(())
    }

    pub fn gl_get_integer64v(&mut self, pname: GLenum) -> GLint64 {
        unsafe {
            let mut value = 0 as GLint64;
            ffi::glGetInteger64v(pname, &mut value);
            value
        }
        Ok(())
    }

    //todo: 返回两个，不封装结构体，不做处理。
    pub fn gl_get_synciv(&mut self,
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: &mut GLsizei,
    ) -> Vec<GLint> {
        unsafe {
            let mut values: Vec<GLint> = Vec::with_capacity(bufSize as usize);
            ffi::glGetSynciv(
                sync,
                pname,
                bufSize,
                length as *mut GLsizei,
                values.as_mut_ptr() as *mut GLint,
            );
            values
        }
        Ok(())
    }

    pub fn gl_get_integer64i_v(&mut self, target: GLenum, index: GLuint) -> GLint64 {
        unsafe {
            let mut value = 0 as GLint64;
            ffi::glGetInteger64i_v(target, index, &mut value);
            value
        }
        Ok(())
    }

    /// Samplers

    pub fn gl_gen_samplers(&mut self, count: GLsizei) -> Vec<GLuint> {
        unsafe {
            let mut sampler: Vec<GLuint> = Vec::with_capacity(count as usize);
            ffi::glGenSamplers(count, sampler.as_mut_ptr() as *mut GLuint);
            sampler
        }
        Ok(())
    }

    pub fn gl_delete_samplers(&mut self, samplers: &[GLuint]) -> Result<(), Error> {
        unsafe {
            let count = samplers.len() as i32;
            ffi::glDeleteSamplers(count, samplers.as_ptr() as *const GLuint);
        }
        Ok(())
    }

    pub fn gl_is_sampler(&mut self, sampler: GLuint) -> GLboolean {
        unsafe {
            ffi::glIsSampler(sampler)
        }
        Ok(())
    }

    pub fn gl_bind_sampler(&mut self, unit: GLuint, sampler: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindSampler(unit, sampler);
        }
        Ok(())
    }

    pub fn gl_sampler_parameteri(&mut self, sampler: GLuint, pname: SamplerParameter, param: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameteri(sampler, pname, param);
        }
        Ok(())
    }

    pub fn gl_sampler_parameteriv(&mut self, sampler: GLuint, pname: SamplerParameter, param: &[GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameteriv(sampler, pname, param.as_ptr() as *const GLint);
        }
        Ok(())
    }

    pub fn gl_sampler_parameterf(&mut self, sampler: GLuint, pname: SamplerParameter, param: GLfloat) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterf(sampler, pname, param);
        }
        Ok(())
    }

    pub fn gl_sampler_parameterfv(&mut self, sampler: GLuint, pname: SamplerParameter, param: &[GLfloat]) -> Result<(), Error> {
        unsafe {
            ffi::glSamplerParameterfv(sampler, pname, param.as_ptr() as *const GLfloat);
        }
        Ok(())
    }

    //todo : 我怀疑是返回一个，这里需要用slice?
    pub fn gl_get_sampler_parameteriv(&mut self, sampler: GLuint, pname: SamplerParameter, params: &mut [GLint]) -> Result<(), Error> {
        unsafe {
            ffi::glGetSamplerParameteriv(sampler, pname, params.as_mut_ptr() as *mut GLint);
        }
        Ok(())
    }

    //todo : 我怀疑是返回一个，这里需要用slice?
    pub fn gl_get_sampler_parameterfv(&mut self, sampler: GLuint, pname: SamplerParameter, params: &mut [GLfloat]) -> Result<(), Error> {
        unsafe {
            ffi::glGetSamplerParameterfv(sampler, pname, params.as_mut_ptr() as *mut GLfloat);
        }
        Ok(())
    }

    pub fn gl_vertex_attrib_divisor(&mut self, index: GLuint, divisor: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribDivisor(index, divisor);
        }
        Ok(())
    }

    /// Shaders

//todo:
    pub fn gl_get_program_binary(&mut self,
        program: GLuint,
        bufSize: GLsizei,
    ) -> Option<ProgramBinary>
    {
        unsafe {
            let mut length = 0 as GLsizei;
            let mut binaryFormat = GL_NONE as GLenum;
            let mut binary: Vec<u8> = Vec::with_capacity(bufSize as usize);

            ffi::glGetProgramBinary(
                program,
                bufSize,
                &mut length as *mut GLsizei,
                &mut binaryFormat as *mut GLenum,
                binary.as_mut_ptr() as *mut GLvoid,
            );

            if length == 0 {
                None
            } else {
                Some(ProgramBinary {
                    length,
                    binaryFormat,
                    binary,
                })
            }
        }
        Ok(())
    }

    pub fn gl_program_binary(&mut self,
        program: GLuint,
        binaryFormat: GLenum,
        binary: &[u8],
        length: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glProgramBinary(
                program,
                binaryFormat,
                binary.as_ptr() as *const GLvoid,
                length,
            );
        }
        Ok(())
    }

    pub fn gl_program_parameteri(&mut self, program: GLuint, pname: GLenum, value: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glProgramParameteri(program, pname, value);
        }
        Ok(())
    }

    /// Frame Buffers

    pub fn gl_invalidate_framebuffer(&mut self,
        target: FrameBufferTarget,
        numAttachments: GLsizei,
        attachments: &[AttachmentTarget],
    ) -> Result<(), Error> {
        unsafe {
            ffi::glInvalidateFramebuffer(
                target,
                numAttachments,
                attachments.as_ptr() as *const GLenum,
            );
        }
        Ok(())
    }

    pub fn gl_invalidate_sub_framebuffer(&mut self,
        target: FrameBufferTarget,
        numAttachments: GLsizei,
        attachments: &[AttachmentTarget],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glInvalidateSubFramebuffer(
                target,
                numAttachments,
                attachments.as_ptr() as *const GLenum,
                x,
                y,
                width,
                height,
            );
        }
        Ok(())
    }

    pub fn gl_blit_framebuffer(&mut self,
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: BufferMask,
        filter: FilterMode,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glBlitFramebuffer(
                srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
            );
        }
        Ok(())
    }

    pub fn gl_framebuffer_texture_layer(&mut self,
        target: FramebufferTarget,
        attachment: AttachmentTarget,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferTextureLayer(target, attachment, texture, level, layer);
        }
        Ok(())
    }

    pub fn gl_tex_storage2d(&mut self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage2D(
                target,
                levels,
                internalformat,
                width,
                height,
            );
        }
        Ok(())
    }

    pub fn gl_tex_storage3d(&mut self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glTexStorage3D(
                target,
                levels,
                internalformat,
                width,
                height,
                depth,
            );
        }
        Ok(())
    }

    pub fn gl_get_internalformativ(&mut self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
    ) -> Vec<GLint> {
        unsafe {
            let mut params: Vec<GLint> = Vec::with_capacity(bufSize as usize);
            glGetInternalformativ(
                target,
                internalformat,
                pname,
                bufSize,
                params.as_mut_ptr() as *mut GLint,
            );
            params
        }
        Ok(())
    }
}