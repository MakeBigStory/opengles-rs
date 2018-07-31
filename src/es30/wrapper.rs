use super::es20::data_struct::*;
use super::data_struct::*;
use super::ffi::*;
use super::*;

use std::ptr;
use std::slice;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub fn gl_read_buffer(mode: GLenum) {
    unsafe {
        ffi::glReadBuffer(mode);
    }
}

pub fn gl_draw_range_elements<T>(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: &[T],
) {
    unsafe {
        ffi::glDrawRangeElements(mode, start, end, count, type_,
                                 indices.as_ptr() as *const GLvoid);
    }
}

pub fn gl_tex_image3d<T>(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: &[T],
) {
    unsafe {
        ffi::glTexImage3D(target, level, internalformat, width, height, depth, border, format, type_,
                          pixels.as_ptr() as *const GLvoid);
    }
}

pub fn gl_tex_sub_image3d<T>(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: &[T],
) {
    unsafe {
        ffi::glTexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            type_,
            pixels.as_ptr() as *const GLvoid,
        );
    }
}

pub fn gl_copy_tex_sub_image3d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
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
}

pub fn gl_compressed_tex_image3d<T>(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: &[T],
) {
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
}

pub fn gl_compressed_tex_sub_image3d<T>(
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
) {
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
}

pub fn gl_gen_queries(size: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
        ffi::glGenQueries(size, ids.as_ptr() as *mut GLuint);
        ids
    }
}

pub fn gl_delete_queries(ids: &mut [GLuint]) {
    unsafe {
        let n: GLsizei = ids.len() as i32;
        ffi::glDeleteQueries(n, ids.as_ptr() as *mut GLuint);
    }
}

pub fn gl_is_query(id: GLuint) -> GLboolean {
    unsafe {
        ffi::glIsQuery(id)
    }
}

pub fn gl_begin_query(target: GLenum, id: GLuint) {
    unsafe {
        ffi::glBeginQuery(target, id);
    }
}

pub fn gl_end_query(target: GLenum) {
    unsafe {
        ffi::glEndQuery(target);
    }
}

pub fn gl_get_queryiv(target: GLenum, pname: GLenum, params: &mut [GLint]) {
    unsafe {
        ffi::glGetQueryiv(target, pname, params.as_ptr() as *mut GLint);
    }
}

pub fn gl_get_query_objectuiv(id: GLuint, pname: GLenum, params: &mut [GLuint]) {
    unsafe {
        ffi::glGetQueryObjectuiv(id, pname, params.as_mut_ptr() as *mut GLuint);
    }
}

pub fn gl_unmap_buffer(target: GLenum) -> GLboolean {
    unsafe {
        ffi::glUnmapBuffer(target)
    }
}

//todo : *mut *mut GLvoid
pub fn gl_get_buffer_pointerv<T>(target: GLenum, pname: GLenum, params: *mut *mut GLvoid) {
    unsafe {
        //ffi::glGetBufferPointerv(target, pname, params: *mut *mut GLvoid);
        ffi::glGetBufferPointerv(target, pname, params);
    }
}

pub fn gl_draw_buffers(bufs: &[GLenum]) {
    unsafe {
        let n: GLsizei = bufs.len() as i32;
        ffi::glDrawBuffers(n, bufs.as_ptr() as *const GLenum);
    }
}

pub fn gl_uniform_matrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix2x3fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_uniform_matrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix3x2fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_uniform_matrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix2x4fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_uniform_matrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix4x2fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_uniform_matrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix3x4fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_uniform_matrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: &[GLfloat],
) {
    unsafe {
        ffi::glUniformMatrix4x3fv(
            location,
            count,
            transpose,
            value.as_ptr() as *const GLfloat,
        );
    }
}

pub fn gl_blit_framebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    unsafe {
        ffi::glBlitFramebuffer(
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask,
            filter,
        );
    }
}

pub fn gl_renderbuffer_storage_multisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        ffi::glRenderbufferStorageMultisample(
            target,
            samples,
            internalformat,
            width,
            height,
        );
    }
}

pub fn gl_framebuffer_texture_layer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    unsafe {
        ffi::glFramebufferTextureLayer(
            target,
            attachment,
            texture,
            level,
            layer,
        );
    }
}

//todo :
pub fn gl_map_buffer_range<'a,T>(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> &'a[T] {
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
}

pub fn gl_flush_mapped_buffer_range(target: GLenum, offset: GLintptr, length: GLsizeiptr) {
    unsafe {
        ffi::glFlushMappedBufferRange(target, offset, length);
    }
}

pub fn gl_bind_vertex_array(array: GLuint) {
    unsafe {
        ffi::glBindVertexArray(array);
    }
}

pub fn gl_delete_vertex_arrays(arrays: &[u32]) {
    unsafe {
        if arrays.len() > 0 {
            let n = arrays.len() as i32;
            ffi::glDeleteVertexArrays(n, arrays.as_ptr() as *const GLuint);
        }
    }
}

pub fn gl_gen_vertex_arrays(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);
        ffi::glGenVertexArrays(count, vec.as_mut_ptr());
        vec.set_len(count as usize);
        vec
    }
}

pub fn gl_is_vertex_array(array: GLuint) -> GLboolean {
    unsafe {
        ffi::glIsVertexArray(array)
    }
}

pub fn gl_get_integeri_v(target: GLenum, index: GLuint) -> GLint {
    unsafe {
        let mut value: GLint = 0;
        ffi::glGetIntegeri_v(target, index, &mut value);
        value
    }
}

pub fn gl_end_transform_feedback() {
    unsafe {
        ffi::glEndTransformFeedback();
    }
}

pub fn gl_bind_buffer_range(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        ffi::glBindBufferRange(
            target,
            index,
            buffer,
            offset,
            size,
        );
    }
}

pub fn gl_bind_buffer_base(target: GLenum, index: GLuint, buffer: GLuint) {
    unsafe {
        ffi::glBindBufferBase(target, index, buffer);
    }
}

//todo: count这么写是否对？
pub fn gl_transform_feedback_varyings(
    program: GLuint,
    count: GLsizei,
    varyings: &Vec<String>,
    bufferMode: GLenum,
)
{
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
}

pub fn gl_get_transform_feedback_varying(
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
}

pub fn gl_vertex_attrib_ipointer<T>(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: &[T],
) {
    unsafe {
        ffi::glVertexAttribIPointer(
            index,
            size,
            type_,
            stride,
            pointer.as_ptr() as *const GLvoid,
        );
    }
}

pub fn gl_get_vertex_attrib_iiv(index: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let mut params: GLint = 0;
        ffi::glGetVertexAttribIiv(index, pname, &mut params);
        params
    }
}

pub fn gl_get_vertex_attrib_iuiv(index: GLuint, pname: GLenum) -> GLuint {
    unsafe {
        let mut params: GLuint = 0;
        ffi::glGetVertexAttribIuiv(index, pname, &mut params);
        params
    }
}

pub fn gl_vertex_attrib_i4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe {
        ffi::glVertexAttribI4i(index, x, y, z, w);
    }
}

pub fn gl_vertex_attrib_i4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    unsafe {
        ffi::glVertexAttribI4ui(index, x, y, z, w);
    }
}

pub fn gl_vertex_attrib_i4iv(index: GLuint, v: &[GLint]) {
    unsafe {
        ffi::glVertexAttribI4iv(index, v.as_ptr() as *const GLint);
    }
}

pub fn gl_vertex_attrib_i4uiv(index: GLuint, v: &[GLint]) {
    unsafe {
        ffi::glVertexAttribI4uiv(index, v.as_ptr() as *const GLuint);
    }
}

pub fn gl_get_uniformuiv(program: GLuint, location: GLint) -> GLuint {
    unsafe {
        let mut value: GLuint = 0;
        ffi::glGetUniformuiv(program, location, &mut value);
        value
    }
}

pub fn gl_get_frag_data_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let c_str = CString::new(name).unwrap();
        ffi::glGetFragDataLocation(program, c_str.as_ptr() as *const GLchar)
    }
}

pub fn gl_uniform1ui(location: GLint, v0: GLuint) {
    unsafe {
        ffi::glUniform1ui(location, v0);
    }
}

pub fn gl_uniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    unsafe {
        ffi::glUniform2ui(location, v0, v1);
    }
}

pub fn gl_uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    unsafe {
        ffi::glUniform3ui(location, v0, v1, v2);
    }
}

pub fn gl_uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    unsafe {
        ffi::glUniform4ui(location, v0, v1, v2, v3);
    }
}

pub fn gl_uniform1uiv(location: GLint, count: GLsizei, value: &[GLuint]) {
    unsafe {
        ffi::glUniform1uiv(location, count, value.as_ptr() as *const GLuint);
    }
}

pub fn gl_uniform2uiv(location: GLint, count: GLsizei, value: &[GLuint]) {
    unsafe {
        ffi::glUniform2uiv(location, count, value.as_ptr() as *const GLuint);
    }
}

pub fn gl_uniform3uiv(location: GLint, count: GLsizei, value: &[GLuint]) {
    unsafe {
        ffi::glUniform3uiv(location, count, value.as_ptr() as *const GLuint);
    }
}

pub fn gl_uniform4uiv(location: GLint, count: GLsizei, value: &[GLuint]) {
    unsafe {
        ffi::glUniform4uiv(location, count, value.as_ptr() as *const GLuint);
    }
}

pub fn gl_clear_bufferiv(buffer: GLenum, drawbuffer: GLint, value: &[GLint]) {
    unsafe {
        ffi::glClearBufferiv(buffer, drawbuffer, value.as_ptr() as *const GLint);
    }
}

pub fn gl_clear_bufferuiv(buffer: GLenum, drawbuffer: GLint, value: &[GLuint]) {
    unsafe {
        ffi::glClearBufferuiv(buffer, drawbuffer, value.as_ptr() as *const GLuint);
    }
}

pub fn gl_clear_bufferfv(buffer: GLenum, drawbuffer: GLint, value: &[GLfloat]) {
    unsafe {
        ffi::glClearBufferfv(buffer, drawbuffer, value.as_ptr() as *const GLfloat);
    }
}

pub fn gl_clear_bufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    unsafe {
        ffi::glClearBufferfi(buffer, drawbuffer, depth, stencil);
    }
}

pub fn gl_get_stringi(name: GLenum, index: GLuint) -> Option<String> {
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

pub fn gl_copy_buffer_sub_data(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        ffi::glCopyBufferSubData(
            readTarget,
            writeTarget,
            readOffset,
            writeOffset,
            size,
        );
    }
}

//todo:
pub fn gl_get_uniform_indices(
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
}

pub fn gl_get_active_uniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: &[GLuint],
    pname: GLenum,
    params: &mut [GLint],
) {
    unsafe {
        ffi::glGetActiveUniformsiv(
            program,
            uniformCount,
            uniformIndices.as_ptr() as *const GLuint,
            pname,
            params.as_mut_ptr() as *mut GLint,
        );
    }
}

pub fn gl_get_uniform_block_index(program: GLuint, uniformBlockName: &str) -> GLuint {
    unsafe {
        let c_str = CString::new(uniformBlockName).unwrap();
        ffi::glGetUniformBlockIndex(program, c_str.as_ptr() as *const GLchar)
    }
}

pub fn gl_get_active_uniform_blockiv(
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
}

pub fn gl_uniform_block_binding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
) {
    unsafe {
        ffi::glUniformBlockBinding(
            program,
            uniformBlockIndex,
            uniformBlockBinding,
        );
    }
}

pub fn gl_draw_arrays_instanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
) {
    unsafe {
        ffi::glDrawArraysInstanced(
            mode,
            first,
            count,
            instancecount,
        );
    }
}

pub fn gl_draw_elements_instanced<T>(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: &[T],
    instancecount: GLsizei,
) {
    unsafe {
        ffi::glDrawElementsInstanced(
            mode,
            count,
            type_,
            indices.as_ptr() as *const GLvoid,
            instancecount,
        );
    }
}

pub fn gl_fence_sync(condition: GLenum, flags: GLbitfield) -> GLsync {
    unsafe {
        ffi::glFenceSync(condition, flags)
    }
}

pub fn gl_is_sync(sync: GLsync) -> GLboolean {
    unsafe {
        ffi::glIsSync(sync)
    }
}

pub fn gl_delete_sync(sync: GLsync) {
    unsafe {
        ffi::glDeleteSync(sync);
    }
}

pub fn gl_client_wait_sync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
    unsafe {
        ffi::glClientWaitSync(sync, flags, timeout)
    }
}

pub fn gl_wait_sync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    unsafe {
        ffi::glWaitSync(sync, flags, timeout);
    }
}

pub fn gl_get_integer64v(pname: GLenum) -> GLint64 {
    unsafe {
        let mut value = 0 as GLint64;
        ffi::glGetInteger64v(pname, &mut value);
        value
    }
}

//todo: 返回两个，不封装结构体，不做处理。
pub fn gl_get_synciv(
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
}

pub fn gl_get_integer64i_v(target: GLenum, index: GLuint) -> GLint64 {
    unsafe {
        let mut value = 0 as GLint64;
        ffi::glGetInteger64i_v(target, index, &mut value);
        value
    }
}

pub fn gl_get_buffer_parameteri64v(target: GLenum, pname: GLenum) -> GLint64 {
    unsafe {
        let mut params = 0 as GLint64;
        ffi::glGetBufferParameteri64v(target, pname, &mut params);
        params
    }
}

pub fn gl_gen_samplers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut sampler: Vec<GLuint> = Vec::with_capacity(count as usize);
        ffi::glGenSamplers(count, sampler.as_mut_ptr() as *mut GLuint);
        sampler
    }
}

pub fn gl_delete_samplers(samplers: &[GLuint]) {
    unsafe {
        let count = samplers.len() as i32;
        ffi::glDeleteSamplers(count, samplers.as_ptr() as *const GLuint);
    }
}

pub fn gl_is_sampler(sampler: GLuint) -> GLboolean {
    unsafe {
        ffi::glIsSampler(sampler)
    }
}

pub fn gl_bind_sampler(unit: GLuint, sampler: GLuint) {
    unsafe {
        ffi::glBindSampler(unit, sampler);
    }
}

pub fn gl_sampler_parameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    unsafe {
        ffi::glSamplerParameteri(sampler, pname, param);
    }
}

pub fn gl_sampler_parameteriv(sampler: GLuint, pname: GLenum, param: &[GLint]) {
    unsafe {
        ffi::glSamplerParameteriv(sampler, pname, param.as_ptr() as *const GLint);
    }
}

pub fn gl_sampler_parameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    unsafe {
        ffi::glSamplerParameterf(sampler, pname, param);
    }
}

pub fn gl_sampler_parameterfv(sampler: GLuint, pname: GLenum, param: &[GLfloat]) {
    unsafe {
        ffi::glSamplerParameterfv(sampler, pname, param.as_ptr() as *const GLfloat);
    }
}

//todo : 我怀疑是返回一个，这里需要用slice?
pub fn gl_get_sampler_parameteriv(sampler: GLuint, pname: GLenum, params: &mut [GLint]) {
    unsafe {
        ffi::glGetSamplerParameteriv(sampler, pname, params.as_mut_ptr() as *mut GLint);
    }
}

//todo : 我怀疑是返回一个，这里需要用slice?
pub fn gl_get_sampler_parameterfv(sampler: GLuint, pname: GLenum, params: &mut [GLfloat]) {
    unsafe {
        ffi::glGetSamplerParameterfv(sampler, pname, params.as_mut_ptr() as *mut GLfloat);
    }
}

pub fn gl_vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
    unsafe {
        ffi::glVertexAttribDivisor(index, divisor);
    }
}

pub fn gl_bind_transform_feedback(target: GLenum, id: GLuint) {
    unsafe {
        ffi::glBindTransformFeedback(target, id);
    }
}

pub fn gl_delete_transform_feedbacks(ids: &[GLuint]) {
    unsafe {
        let n = ids.len() as i32;
        ffi::glDeleteTransformFeedbacks(n, ids.as_ptr() as *const GLuint);
    }
}

pub fn gl_gen_transform_feedbacks(size: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
        ffi::glGenTransformFeedbacks(size, ids.as_mut_ptr() as *mut GLuint);
        ids
    }
}

pub fn gl_is_transform_feedback(id: GLuint) -> GLboolean {
    unsafe {
        ffi::glIsTransformFeedback(id)
    }
}

pub fn gl_pause_transform_feedback() {
    unsafe {
        ffi::glPauseTransformFeedback();
    }
}

pub fn gl_resume_transform_feedback() {
    unsafe {
        ffi::glResumeTransformFeedback();
    }
}

//todo:
pub fn gl_get_program_binary(
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
}

pub fn gl_program_binary(
    program: GLuint,
    binaryFormat: GLenum,
    binary: &[u8],
    length: GLsizei,
) {
    unsafe {
        ffi::glProgramBinary(
            program,
            binaryFormat,
            binary.as_ptr() as *const GLvoid,
            length,
        );
    }
}

pub fn gl_program_parameteri(program: GLuint, pname: GLenum, value: GLint) {
    unsafe {
        ffi::glProgramParameteri(program, pname, value);
    }
}

pub fn gl_invalidate_framebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: &[GLenum],
) {
    unsafe {
        ffi::glInvalidateFramebuffer(
            target,
            numAttachments,
            attachments.as_ptr() as *const GLenum,
        );
    }
}

pub fn gl_invalidate_sub_framebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: &[GLenum],
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
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
}

pub fn gl_tex_storage2d(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        ffi::glTexStorage2D(
            target,
            levels,
            internalformat,
            width,
            height,
        );
    }
}

pub fn gl_tex_storage3d(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
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
}

pub fn gl_get_internalformativ(
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
}