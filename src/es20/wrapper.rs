use es30::ffi::*;
use es31::ffi::*;
use es32::ffi::*;

use consts::*;
use enums::*;
use std::mem::size_of;
use std::str::from_utf8;
use types::*;

use super::ffi::*;
use super::*;
use egl::ffi::eglGetProcAddress;

use types::GLDEBUGPROC;

use std::mem;

use std::ptr;
use std::slice;

use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

#[derive(Debug)]
pub struct Active {
    pub name: String,
    pub size: i32,
    pub type_: DataType,
    pub length: i32,
}

#[derive(Debug)]
pub struct ShaderPrecisionFormat {
    pub precision: i32,
    pub range: [i32; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramBinary {
    pub length: GLsizei,
    pub binary_format: GLenum,
    pub binary: Vec<u8>,
}

#[derive(Debug)]
pub struct Error {}

use std::collections::HashMap;
use std::boxed::Box;

trait Interceptor {

    fn get_name(&self) -> String;

    fn enable(&mut self, enable: bool);

    fn is_enabled(&self) -> bool;

    fn pre_intercept(&mut self, func_info: &FuncInfo) -> Result<(), Error>;

    fn post_intercept(&mut self, func_info: &FuncInfo, res_desc: &str) -> Result<(), Error>;
}

pub struct Wrapper {
    es_version: ES_VERSION,
    interceptors: HashMap<String, Box<Interceptor>>,
    debug: bool,
    glReadBuffer_ptr: *const c_void,
    glDrawBuffers_ptr: *const c_void,
    glUnmapBuffer_ptr: *const c_void,
    glCopyBufferSubData_ptr: *const c_void,
    glGetBufferPointerv_ptr: *const c_void,
    glMapBufferRange_ptr: *const c_void,
    glFlushMappedBufferRange_ptr: *const c_void,
    glBindBufferRange_ptr: *const c_void,
    glBindBufferBase_ptr: *const c_void,
    glClearBufferiv_ptr: *const c_void,
    glClearBufferuiv_ptr: *const c_void,
    glClearBufferfv_ptr: *const c_void,
    glClearBufferfi_ptr: *const c_void,
    glGetBufferParameteri64v_ptr: *const c_void,
    glTexImage3D_ptr: *const c_void,
    glTexSubImage3D_ptr: *const c_void,
    glCopyTexSubImage3D_ptr: *const c_void,
    glCompressedTexImage3D_ptr: *const c_void,
    glCompressedTexSubImage3D_ptr: *const c_void,
    glGenQueries_ptr: *const c_void,
    glDeleteQueries_ptr: *const c_void,
    glIsQuery_ptr: *const c_void,
    glBeginQuery_ptr: *const c_void,
    glEndQuery_ptr: *const c_void,
    glGetQueryiv_ptr: *const c_void,
    glGetQueryObjectuiv_ptr: *const c_void,
    glUniformMatrix2x3fv_ptr: *const c_void,
    glUniformMatrix3x2fv_ptr: *const c_void,
    glUniformMatrix2x4fv_ptr: *const c_void,
    glUniformMatrix4x2fv_ptr: *const c_void,
    glUniformMatrix3x4fv_ptr: *const c_void,
    glUniformMatrix4x3fv_ptr: *const c_void,
    glRenderbufferStorageMultisample_ptr: *const c_void,
    glBindVertexArray_ptr: *const c_void,
    glDeleteVertexArrays_ptr: *const c_void,
    glGenVertexArrays_ptr: *const c_void,
    glIsVertexArray_ptr: *const c_void,
    glGetIntegeri_v_ptr: *const c_void,
    glEndTransformFeedback_ptr: *const c_void,
    glTransformFeedbackVaryings_ptr: *const c_void,
    glGetTransformFeedbackVarying_ptr: *const c_void,
    glBindTransformFeedback_ptr: *const c_void,
    glDeleteTransformFeedbacks_ptr: *const c_void,
    glGenTransformFeedbacks_ptr: *const c_void,
    glIsTransformFeedback_ptr: *const c_void,
    glPauseTransformFeedback_ptr: *const c_void,
    glResumeTransformFeedback_ptr: *const c_void,
    glVertexAttribIPointer_ptr: *const c_void,
    glGetVertexAttribIiv_ptr: *const c_void,
    glGetVertexAttribIuiv_ptr: *const c_void,
    glVertexAttribI4i_ptr: *const c_void,
    glVertexAttribI4ui_ptr: *const c_void,
    glVertexAttribI4iv_ptr: *const c_void,
    glVertexAttribI4uiv_ptr: *const c_void,
    glGetUniformuiv_ptr: *const c_void,
    glGetFragDataLocation_ptr: *const c_void,
    glUniform1ui_ptr: *const c_void,
    glUniform2ui_ptr: *const c_void,
    glUniform3ui_ptr: *const c_void,
    glUniform4ui_ptr: *const c_void,
    glUniform1uiv_ptr: *const c_void,
    glUniform2uiv_ptr: *const c_void,
    glUniform3uiv_ptr: *const c_void,
    glUniform4uiv_ptr: *const c_void,
    glGetStringi_ptr: *const c_void,
    glGetUniformIndices_ptr: *const c_void,
    glGetActiveUniformsiv_ptr: *const c_void,
    glGetUniformBlockIndex_ptr: *const c_void,
    glGetActiveUniformBlockiv_ptr: *const c_void,
    glUniformBlockBinding_ptr: *const c_void,
    glDrawRangeElements_ptr: *const c_void,
    glDrawArraysInstanced_ptr: *const c_void,
    glDrawElementsInstanced_ptr: *const c_void,
    glFenceSync_ptr: *const c_void,
    glIsSync_ptr: *const c_void,
    glDeleteSync_ptr: *const c_void,
    glClientWaitSync_ptr: *const c_void,
    glWaitSync_ptr: *const c_void,
    glGetInteger64v_ptr: *const c_void,
    glGetSynciv_ptr: *const c_void,
    glGetInteger64i_v_ptr: *const c_void,
    glGenSamplers_ptr: *const c_void,
    glDeleteSamplers_ptr: *const c_void,
    glIsSampler_ptr: *const c_void,
    glBindSampler_ptr: *const c_void,
    glSamplerParameteri_ptr: *const c_void,
    glSamplerParameteriv_ptr: *const c_void,
    glSamplerParameterf_ptr: *const c_void,
    glSamplerParameterfv_ptr: *const c_void,
    glGetSamplerParameteriv_ptr: *const c_void,
    glGetSamplerParameterfv_ptr: *const c_void,
    glVertexAttribDivisor_ptr: *const c_void,
    glGetProgramBinary_ptr: *const c_void,
    glProgramBinary_ptr: *const c_void,
    glProgramParameteri_ptr: *const c_void,
    glInvalidateFramebuffer_ptr: *const c_void,
    glInvalidateSubFramebuffer_ptr: *const c_void,
    glBlitFramebuffer_ptr: *const c_void,
    glFramebufferTextureLayer_ptr: *const c_void,
    glTexStorage2D_ptr: *const c_void,
    glTexStorage3D_ptr: *const c_void,
    glGetInternalformativ_ptr: *const c_void,
    glDispatchCompute_ptr: *const c_void,
    glDispatchComputeIndirect_ptr: *const c_void,
    glDrawArraysIndirect_ptr: *const c_void,
    glDrawElementsIndirect_ptr: *const c_void,
    glFramebufferParameteri_ptr: *const c_void,
    glGetFramebufferParameteriv_ptr: *const c_void,
    glGetProgramInterfaceiv_ptr: *const c_void,
    glGetProgramResourceIndex_ptr: *const c_void,
    glGetProgramResourceName_ptr: *const c_void,
    glGetProgramResourceiv_ptr: *const c_void,
    glGetProgramResourceLocation_ptr: *const c_void,
    glUseProgramStages_ptr: *const c_void,
    glActiveShaderProgram_ptr: *const c_void,
    glCreateShaderProgramv_ptr: *const c_void,
    glBindProgramPipeline_ptr: *const c_void,
    glDeleteProgramPipelines_ptr: *const c_void,
    glGenProgramPipelines_ptr: *const c_void,
    glIsProgramPipeline_ptr: *const c_void,
    glGetProgramPipelineiv_ptr: *const c_void,
    glProgramUniform1i_ptr: *const c_void,
    glProgramUniform2i_ptr: *const c_void,
    glProgramUniform3i_ptr: *const c_void,
    glProgramUniform4i_ptr: *const c_void,
    glProgramUniform1ui_ptr: *const c_void,
    glProgramUniform2ui_ptr: *const c_void,
    glProgramUniform3ui_ptr: *const c_void,
    glProgramUniform4ui_ptr: *const c_void,
    glProgramUniform1f_ptr: *const c_void,
    glProgramUniform2f_ptr: *const c_void,
    glProgramUniform3f_ptr: *const c_void,
    glProgramUniform4f_ptr: *const c_void,
    glProgramUniform1iv_ptr: *const c_void,
    glProgramUniform2iv_ptr: *const c_void,
    glProgramUniform3iv_ptr: *const c_void,
    glProgramUniform4iv_ptr: *const c_void,
    glProgramUniform1uiv_ptr: *const c_void,
    glProgramUniform2uiv_ptr: *const c_void,
    glProgramUniform3uiv_ptr: *const c_void,
    glProgramUniform4uiv_ptr: *const c_void,
    glProgramUniform4fv_ptr: *const c_void,
    glProgramUniform2fv_ptr: *const c_void,
    glProgramUniform3fv_ptr: *const c_void,
    glProgramUniformMatrix2fv_ptr: *const c_void,
    glProgramUniformMatrix3fv_ptr: *const c_void,
    glProgramUniformMatrix4fv_ptr: *const c_void,
    glProgramUniformMatrix2x3fv_ptr: *const c_void,
    glProgramUniformMatrix3x2fv_ptr: *const c_void,
    glProgramUniformMatrix2x4fv_ptr: *const c_void,
    glProgramUniformMatrix4x2fv_ptr: *const c_void,
    glProgramUniformMatrix3x4fv_ptr: *const c_void,
    glProgramUniformMatrix4x3fv_ptr: *const c_void,
    glValidateProgramPipeline_ptr: *const c_void,
    glGetProgramPipelineInfoLog_ptr: *const c_void,
    glBindImageTexture_ptr: *const c_void,
    glGetBooleani_v_ptr: *const c_void,
    glMemoryBarrier_ptr: *const c_void,
    glMemoryBarrierByRegion_ptr: *const c_void,
    glTexStorage2DMultisample_ptr: *const c_void,
    glGetMultisamplefv_ptr: *const c_void,
    glSampleMaski_ptr: *const c_void,
    glGetTexLevelParameteriv_ptr: *const c_void,
    glGetTexLevelParameterfv_ptr: *const c_void,
    glBindVertexBuffer_ptr: *const c_void,
    glVertexAttribFormat_ptr: *const c_void,
    glVertexAttribIFormat_ptr: *const c_void,
    glVertexAttribBinding_ptr: *const c_void,
    glVertexBindingDivisor_ptr: *const c_void,
    glBlendBarrier_ptr: *const c_void,
    glCopyImageSubData_ptr: *const c_void,
    glDebugMessageControl_ptr: *const c_void,
    glDebugMessageInsert_ptr: *const c_void,
    glDebugMessageCallback_ptr: *const c_void,
    glGetDebugMessageLog_ptr: *const c_void,
    glPushDebugGroup_ptr: *const c_void,
    glPopDebugGroup_ptr: *const c_void,
    glObjectLabel_ptr: *const c_void,
    glGetObjectPtrLabel_ptr: *const c_void,
    glObjectPtrLabel_ptr: *const c_void,
    glGetPointerv_ptr: *const c_void,
    glEnablei_ptr: *const c_void,
    glDisablei_ptr: *const c_void,
    glBlendEquationi_ptr: *const c_void,
    glBlendEquationSeparatei_ptr: *const c_void,
    glBlendFunci_ptr: *const c_void,
    glBlendFuncSeparatei_ptr: *const c_void,
    glColorMaski_ptr: *const c_void,
    glIsEnabledi_ptr: *const c_void,
    glDrawElementsBaseVertex_ptr: *const c_void,
    glDrawRangeElementsBaseVertex_ptr: *const c_void,
    glDrawElementsInstancedBaseVertex_ptr: *const c_void,
    glFramebufferTexture_ptr: *const c_void,
    glPrimitiveBoundingBox_ptr: *const c_void,
    glGetGraphicsResetStatus_ptr: *const c_void,
    glReadnPixels_ptr: *const c_void,
    glGetnUniformfv_ptr: *const c_void,
    glGetnUniformiv_ptr: *const c_void,
    glGetnUniformuiv_ptr: *const c_void,
    glMinSampleShading_ptr: *const c_void,
    glPatchParameteri_ptr: *const c_void,
    glTexParameterIiv_ptr: *const c_void,
    glTexParameterIuiv_ptr: *const c_void,
    glGetTexParameterIiv_ptr: *const c_void,
    glGetTexParameterIuiv_ptr: *const c_void,
    glSamplerParameterIiv_ptr: *const c_void,
    glSamplerParameterIuiv_ptr: *const c_void,
    glGetSamplerParameterIuiv_ptr: *const c_void,
    glTexBuffer_ptr: *const c_void,
    glTexBufferRange_ptr: *const c_void,
    glTexStorage3DMultisample_ptr: *const c_void,
}

trait Param: std::fmt::Debug {}

impl Param for i32 {}

impl Param for u32 {}

impl Param for f32 {}

impl Param for bool {}

impl Param for String {}

impl Param for TextureUnit {}

impl Param for BufferTarget {}

impl Param for FrameBufferTarget {}

impl Param for RenderBufferTarget {}

impl Param for TextureBindTarget {}

impl Param for BlendEquationMode {}

impl Param for BlendFactor {}

impl Param for BufferUsage {}

impl Param for FrameBufferStatus {}

impl Param for TextureTarget {}

impl Param for ShaderType {}

impl Param for FaceMode {}

impl Param for FuncType {}

impl Param for FeatureType {}

impl Param for BeginMode {}

impl Param for FrameBufferAttachmentType {}

impl Param for FrontFaceDirection {}

impl Param for StateType {}

impl Param for BufferParamName {}

impl Param for ErrorType {}

impl Param for FrameBufferAttachmentParamType {}

impl Param for ProgramParamType {}

impl Param for RenderBufferParamType {}

impl Param for ShaderParamType {}

impl Param for ShaderPrecisionType {}

impl Param for ConstantType {}

impl Param for TextureParamType {}

impl Param for VertexAttributeParamType {}

impl Param for HintTargetType {}

impl Param for HintBehaviorType {}

impl Param for PackParamType {}

impl Param for PixelFormat {}

impl Param for PixelDataType {}

impl Param for ActionType {}

impl Param for DataType {}

impl Param for SamplerParameter {}

impl Param for BufferMask {}

impl Param for FilterMode {}

impl Param for FramebufferTarget {}

impl Param for AttachmentTarget {}

impl Param for ColorBufferMode {}

impl Param for BufferObjectTarget {}

impl Param for BufferMapTarget {}

impl Param for MappingBit {}

impl Param for TransformFeedbackMode {}

impl Param for PixelDataFormat {}

impl Param for TransformFeedbackObjectTarget {}

impl Param for u64 {}

impl Param for *const types::__GLsync {}

impl Param for *mut i32 {}

impl Param for *mut f32 {}

impl Param for *mut u32 {}

impl Param for *mut i8 {}

impl Param for *mut u8 {}

impl Param for *const i8 {}

impl Param for *const i32 {}

impl Param for *const u32 {}

impl Param for *const f32 {}

impl Param for *const u8 {}

impl Param for *const *const u8 {}

impl Param for isize {}

impl Param for u8 {}

impl Param for *const std::os::raw::c_void {}

impl Param for *mut std::os::raw::c_void {}

impl Param for *mut *mut std::os::raw::c_void {}

impl Param for *const *const i8 {}

impl Param for *const libc::c_void {}

impl Param for *mut *mut libc::c_void {}

impl Param for *mut libc::c_void {}

impl<T> Param for Vec<T>
where
    T: std::fmt::Debug,
{
}

#[derive(Debug)]
struct ParamInfo {
    param_type: String,
    param_name: String,
}

impl ParamInfo {
    fn new(param_name: &str, param_type: &str) -> Self {
        ParamInfo {
            param_type: param_type.to_string(),
            param_name: param_name.to_string(),
        }
    }
}

#[derive(Debug)]
struct FuncInfo<'a> {
    func_name: String,
    func_param_infos: Vec<&'a ParamInfo>,
    func_param_values: Vec<&'a Param>,
}

impl<'a> FuncInfo<'a> {
    fn new() -> Self {
        FuncInfo {
            func_name: "".to_string(),
            func_param_infos: vec![],
            func_param_values: vec![],
        }
    }
}

struct InitError {
    desc: String,
}

pub struct InitResult {
    es_version: ES_VERSION,
    desc: String,
}

impl Wrapper {

    pub fn get_es_version(&self) -> ES_VERSION {
        return self.es_version;
    }

    fn add_interceptor<T>(&mut self, interceptor: T) where T: Interceptor + 'static {
        let new_interceptor : Box<Interceptor> = Box::new(interceptor);
        self.interceptors.insert(new_interceptor.get_name(), new_interceptor);
    }

    fn remove_interceptor<T>(&mut self, name: &str) {
        self.interceptors.remove(name);
    }

    fn pre_process(&mut self, func_info: &FuncInfo) -> Result<(), Error> {
        for interceptor in self.interceptors.values_mut() {
            if interceptor.is_enabled() {
                interceptor.pre_intercept(func_info)?;
            }
        }

        Ok(())
    }

    fn post_process(&mut self, func_info: &FuncInfo, res_desc: &str) -> Result<(), Error> {
        for interceptor in self.interceptors.values_mut() {
            if interceptor.is_enabled() {
                interceptor.post_intercept(func_info, res_desc)?;
            }
        }

        Ok(())
    }

    fn is_debug(&self) -> bool {
        self.debug
    }

    fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn new() -> Self {
        Wrapper {
            es_version: ES_VERSION::ES20,
            interceptors: HashMap::new(),
            debug: false,
            glReadBuffer_ptr: 0 as *const c_void,
            glDrawBuffers_ptr: 0 as *const c_void,
            glUnmapBuffer_ptr: 0 as *const c_void,
            glCopyBufferSubData_ptr: 0 as *const c_void,
            glGetBufferPointerv_ptr: 0 as *const c_void,
            glMapBufferRange_ptr: 0 as *const c_void,
            glFlushMappedBufferRange_ptr: 0 as *const c_void,
            glBindBufferRange_ptr: 0 as *const c_void,
            glBindBufferBase_ptr: 0 as *const c_void,
            glClearBufferiv_ptr: 0 as *const c_void,
            glClearBufferuiv_ptr: 0 as *const c_void,
            glClearBufferfv_ptr: 0 as *const c_void,
            glClearBufferfi_ptr: 0 as *const c_void,
            glGetBufferParameteri64v_ptr: 0 as *const c_void,
            glTexImage3D_ptr: 0 as *const c_void,
            glTexSubImage3D_ptr: 0 as *const c_void,
            glCopyTexSubImage3D_ptr: 0 as *const c_void,
            glCompressedTexImage3D_ptr: 0 as *const c_void,
            glCompressedTexSubImage3D_ptr: 0 as *const c_void,
            glGenQueries_ptr: 0 as *const c_void,
            glDeleteQueries_ptr: 0 as *const c_void,
            glIsQuery_ptr: 0 as *const c_void,
            glBeginQuery_ptr: 0 as *const c_void,
            glEndQuery_ptr: 0 as *const c_void,
            glGetQueryiv_ptr: 0 as *const c_void,
            glGetQueryObjectuiv_ptr: 0 as *const c_void,
            glUniformMatrix2x3fv_ptr: 0 as *const c_void,
            glUniformMatrix3x2fv_ptr: 0 as *const c_void,
            glUniformMatrix2x4fv_ptr: 0 as *const c_void,
            glUniformMatrix4x2fv_ptr: 0 as *const c_void,
            glUniformMatrix3x4fv_ptr: 0 as *const c_void,
            glUniformMatrix4x3fv_ptr: 0 as *const c_void,
            glRenderbufferStorageMultisample_ptr: 0 as *const c_void,
            glBindVertexArray_ptr: 0 as *const c_void,
            glDeleteVertexArrays_ptr: 0 as *const c_void,
            glGenVertexArrays_ptr: 0 as *const c_void,
            glIsVertexArray_ptr: 0 as *const c_void,
            glGetIntegeri_v_ptr: 0 as *const c_void,
            glEndTransformFeedback_ptr: 0 as *const c_void,
            glTransformFeedbackVaryings_ptr: 0 as *const c_void,
            glGetTransformFeedbackVarying_ptr: 0 as *const c_void,
            glBindTransformFeedback_ptr: 0 as *const c_void,
            glDeleteTransformFeedbacks_ptr: 0 as *const c_void,
            glGenTransformFeedbacks_ptr: 0 as *const c_void,
            glIsTransformFeedback_ptr: 0 as *const c_void,
            glPauseTransformFeedback_ptr: 0 as *const c_void,
            glResumeTransformFeedback_ptr: 0 as *const c_void,
            glVertexAttribIPointer_ptr: 0 as *const c_void,
            glGetVertexAttribIiv_ptr: 0 as *const c_void,
            glGetVertexAttribIuiv_ptr: 0 as *const c_void,
            glVertexAttribI4i_ptr: 0 as *const c_void,
            glVertexAttribI4ui_ptr: 0 as *const c_void,
            glVertexAttribI4iv_ptr: 0 as *const c_void,
            glVertexAttribI4uiv_ptr: 0 as *const c_void,
            glGetUniformuiv_ptr: 0 as *const c_void,
            glGetFragDataLocation_ptr: 0 as *const c_void,
            glUniform1ui_ptr: 0 as *const c_void,
            glUniform2ui_ptr: 0 as *const c_void,
            glUniform3ui_ptr: 0 as *const c_void,
            glUniform4ui_ptr: 0 as *const c_void,
            glUniform1uiv_ptr: 0 as *const c_void,
            glUniform2uiv_ptr: 0 as *const c_void,
            glUniform3uiv_ptr: 0 as *const c_void,
            glUniform4uiv_ptr: 0 as *const c_void,
            glGetStringi_ptr: 0 as *const c_void,
            glGetUniformIndices_ptr: 0 as *const c_void,
            glGetActiveUniformsiv_ptr: 0 as *const c_void,
            glGetUniformBlockIndex_ptr: 0 as *const c_void,
            glGetActiveUniformBlockiv_ptr: 0 as *const c_void,
            glUniformBlockBinding_ptr: 0 as *const c_void,
            glDrawRangeElements_ptr: 0 as *const c_void,
            glDrawArraysInstanced_ptr: 0 as *const c_void,
            glDrawElementsInstanced_ptr: 0 as *const c_void,
            glFenceSync_ptr: 0 as *const c_void,
            glIsSync_ptr: 0 as *const c_void,
            glDeleteSync_ptr: 0 as *const c_void,
            glClientWaitSync_ptr: 0 as *const c_void,
            glWaitSync_ptr: 0 as *const c_void,
            glGetInteger64v_ptr: 0 as *const c_void,
            glGetSynciv_ptr: 0 as *const c_void,
            glGetInteger64i_v_ptr: 0 as *const c_void,
            glGenSamplers_ptr: 0 as *const c_void,
            glDeleteSamplers_ptr: 0 as *const c_void,
            glIsSampler_ptr: 0 as *const c_void,
            glBindSampler_ptr: 0 as *const c_void,
            glSamplerParameteri_ptr: 0 as *const c_void,
            glSamplerParameteriv_ptr: 0 as *const c_void,
            glSamplerParameterf_ptr: 0 as *const c_void,
            glSamplerParameterfv_ptr: 0 as *const c_void,
            glGetSamplerParameteriv_ptr: 0 as *const c_void,
            glGetSamplerParameterfv_ptr: 0 as *const c_void,
            glVertexAttribDivisor_ptr: 0 as *const c_void,
            glGetProgramBinary_ptr: 0 as *const c_void,
            glProgramBinary_ptr: 0 as *const c_void,
            glProgramParameteri_ptr: 0 as *const c_void,
            glInvalidateFramebuffer_ptr: 0 as *const c_void,
            glInvalidateSubFramebuffer_ptr: 0 as *const c_void,
            glBlitFramebuffer_ptr: 0 as *const c_void,
            glFramebufferTextureLayer_ptr: 0 as *const c_void,
            glTexStorage2D_ptr: 0 as *const c_void,
            glTexStorage3D_ptr: 0 as *const c_void,
            glGetInternalformativ_ptr: 0 as *const c_void,
            glDispatchCompute_ptr: 0 as *const c_void,
            glDispatchComputeIndirect_ptr: 0 as *const c_void,
            glDrawArraysIndirect_ptr: 0 as *const c_void,
            glDrawElementsIndirect_ptr: 0 as *const c_void,
            glFramebufferParameteri_ptr: 0 as *const c_void,
            glGetFramebufferParameteriv_ptr: 0 as *const c_void,
            glGetProgramInterfaceiv_ptr: 0 as *const c_void,
            glGetProgramResourceIndex_ptr: 0 as *const c_void,
            glGetProgramResourceName_ptr: 0 as *const c_void,
            glGetProgramResourceiv_ptr: 0 as *const c_void,
            glGetProgramResourceLocation_ptr: 0 as *const c_void,
            glUseProgramStages_ptr: 0 as *const c_void,
            glActiveShaderProgram_ptr: 0 as *const c_void,
            glCreateShaderProgramv_ptr: 0 as *const c_void,
            glBindProgramPipeline_ptr: 0 as *const c_void,
            glDeleteProgramPipelines_ptr: 0 as *const c_void,
            glGenProgramPipelines_ptr: 0 as *const c_void,
            glIsProgramPipeline_ptr: 0 as *const c_void,
            glGetProgramPipelineiv_ptr: 0 as *const c_void,
            glProgramUniform1i_ptr: 0 as *const c_void,
            glProgramUniform2i_ptr: 0 as *const c_void,
            glProgramUniform3i_ptr: 0 as *const c_void,
            glProgramUniform4i_ptr: 0 as *const c_void,
            glProgramUniform1ui_ptr: 0 as *const c_void,
            glProgramUniform2ui_ptr: 0 as *const c_void,
            glProgramUniform3ui_ptr: 0 as *const c_void,
            glProgramUniform4ui_ptr: 0 as *const c_void,
            glProgramUniform1f_ptr: 0 as *const c_void,
            glProgramUniform2f_ptr: 0 as *const c_void,
            glProgramUniform3f_ptr: 0 as *const c_void,
            glProgramUniform4f_ptr: 0 as *const c_void,
            glProgramUniform1iv_ptr: 0 as *const c_void,
            glProgramUniform2iv_ptr: 0 as *const c_void,
            glProgramUniform3iv_ptr: 0 as *const c_void,
            glProgramUniform4iv_ptr: 0 as *const c_void,
            glProgramUniform1uiv_ptr: 0 as *const c_void,
            glProgramUniform2uiv_ptr: 0 as *const c_void,
            glProgramUniform3uiv_ptr: 0 as *const c_void,
            glProgramUniform4uiv_ptr: 0 as *const c_void,
            glProgramUniform4fv_ptr: 0 as *const c_void,
            glProgramUniform2fv_ptr: 0 as *const c_void,
            glProgramUniform3fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix2fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix3fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix4fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix2x3fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix3x2fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix2x4fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix4x2fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix3x4fv_ptr: 0 as *const c_void,
            glProgramUniformMatrix4x3fv_ptr: 0 as *const c_void,
            glValidateProgramPipeline_ptr: 0 as *const c_void,
            glGetProgramPipelineInfoLog_ptr: 0 as *const c_void,
            glBindImageTexture_ptr: 0 as *const c_void,
            glGetBooleani_v_ptr: 0 as *const c_void,
            glMemoryBarrier_ptr: 0 as *const c_void,
            glMemoryBarrierByRegion_ptr: 0 as *const c_void,
            glTexStorage2DMultisample_ptr: 0 as *const c_void,
            glGetMultisamplefv_ptr: 0 as *const c_void,
            glSampleMaski_ptr: 0 as *const c_void,
            glGetTexLevelParameteriv_ptr: 0 as *const c_void,
            glGetTexLevelParameterfv_ptr: 0 as *const c_void,
            glBindVertexBuffer_ptr: 0 as *const c_void,
            glVertexAttribFormat_ptr: 0 as *const c_void,
            glVertexAttribIFormat_ptr: 0 as *const c_void,
            glVertexAttribBinding_ptr: 0 as *const c_void,
            glVertexBindingDivisor_ptr: 0 as *const c_void,
            glBlendBarrier_ptr: 0 as *const c_void,
            glCopyImageSubData_ptr: 0 as *const c_void,
            glDebugMessageControl_ptr: 0 as *const c_void,
            glDebugMessageInsert_ptr: 0 as *const c_void,
            glDebugMessageCallback_ptr: 0 as *const c_void,
            glGetDebugMessageLog_ptr: 0 as *const c_void,
            glPushDebugGroup_ptr: 0 as *const c_void,
            glPopDebugGroup_ptr: 0 as *const c_void,
            glObjectLabel_ptr: 0 as *const c_void,
            glGetObjectPtrLabel_ptr: 0 as *const c_void,
            glObjectPtrLabel_ptr: 0 as *const c_void,
            glGetPointerv_ptr: 0 as *const c_void,
            glEnablei_ptr: 0 as *const c_void,
            glDisablei_ptr: 0 as *const c_void,
            glBlendEquationi_ptr: 0 as *const c_void,
            glBlendEquationSeparatei_ptr: 0 as *const c_void,
            glBlendFunci_ptr: 0 as *const c_void,
            glBlendFuncSeparatei_ptr: 0 as *const c_void,
            glColorMaski_ptr: 0 as *const c_void,
            glIsEnabledi_ptr: 0 as *const c_void,
            glDrawElementsBaseVertex_ptr: 0 as *const c_void,
            glDrawRangeElementsBaseVertex_ptr: 0 as *const c_void,
            glDrawElementsInstancedBaseVertex_ptr: 0 as *const c_void,
            glFramebufferTexture_ptr: 0 as *const c_void,
            glPrimitiveBoundingBox_ptr: 0 as *const c_void,
            glGetGraphicsResetStatus_ptr: 0 as *const c_void,
            glReadnPixels_ptr: 0 as *const c_void,
            glGetnUniformfv_ptr: 0 as *const c_void,
            glGetnUniformiv_ptr: 0 as *const c_void,
            glGetnUniformuiv_ptr: 0 as *const c_void,
            glMinSampleShading_ptr: 0 as *const c_void,
            glPatchParameteri_ptr: 0 as *const c_void,
            glTexParameterIiv_ptr: 0 as *const c_void,
            glTexParameterIuiv_ptr: 0 as *const c_void,
            glGetTexParameterIiv_ptr: 0 as *const c_void,
            glGetTexParameterIuiv_ptr: 0 as *const c_void,
            glSamplerParameterIiv_ptr: 0 as *const c_void,
            glSamplerParameterIuiv_ptr: 0 as *const c_void,
            glGetSamplerParameterIuiv_ptr: 0 as *const c_void,
            glTexBuffer_ptr: 0 as *const c_void,
            glTexBufferRange_ptr: 0 as *const c_void,
            glTexStorage3DMultisample_ptr: 0 as *const c_void,
        }
    }

    #[cfg(target_os = "android")]
    fn init_es30(&mut self) -> Result<(), InitError> {
        self.glReadBuffer_ptr = self.get_proc_address("glReadBuffer")?;
        self.glDrawBuffers_ptr = self.get_proc_address("glDrawBuffers")?;
        self.glUnmapBuffer_ptr = self.get_proc_address("glUnmapBuffer")?;
        self.glCopyBufferSubData_ptr = self.get_proc_address("glCopyBufferSubData")?;
        self.glGetBufferPointerv_ptr = self.get_proc_address("glGetBufferPointerv")?;
        self.glMapBufferRange_ptr = self.get_proc_address("glMapBufferRange")?;
        self.glFlushMappedBufferRange_ptr = self.get_proc_address("glFlushMappedBufferRange")?;
        self.glBindBufferRange_ptr = self.get_proc_address("glBindBufferRange")?;
        self.glBindBufferBase_ptr = self.get_proc_address("glBindBufferBase")?;
        self.glClearBufferiv_ptr = self.get_proc_address("glClearBufferiv")?;
        self.glClearBufferuiv_ptr = self.get_proc_address("glClearBufferuiv")?;
        self.glClearBufferfv_ptr = self.get_proc_address("glClearBufferfv")?;
        self.glClearBufferfi_ptr = self.get_proc_address("glClearBufferfi")?;
        self.glGetBufferParameteri64v_ptr = self.get_proc_address("glGetBufferParameteri64v")?;
        self.glTexImage3D_ptr = self.get_proc_address("glTexImage3D")?;
        self.glTexSubImage3D_ptr = self.get_proc_address("glTexSubImage3D")?;
        self.glCopyTexSubImage3D_ptr = self.get_proc_address("glCopyTexSubImage3D")?;
        self.glCompressedTexImage3D_ptr = self.get_proc_address("glCompressedTexImage3D")?;
        self.glCompressedTexSubImage3D_ptr = self.get_proc_address("glCompressedTexSubImage3D")?;
        self.glGenQueries_ptr = self.get_proc_address("glGenQueries")?;
        self.glDeleteQueries_ptr = self.get_proc_address("glDeleteQueries")?;
        self.glIsQuery_ptr = self.get_proc_address("glIsQuery")?;
        self.glBeginQuery_ptr = self.get_proc_address("glBeginQuery")?;
        self.glEndQuery_ptr = self.get_proc_address("glEndQuery")?;
        self.glGetQueryiv_ptr = self.get_proc_address("glGetQueryiv")?;
        self.glGetQueryObjectuiv_ptr = self.get_proc_address("glGetQueryObjectuiv")?;
        self.glUniformMatrix2x3fv_ptr = self.get_proc_address("glUniformMatrix2x3fv")?;
        self.glUniformMatrix3x2fv_ptr = self.get_proc_address("glUniformMatrix3x2fv")?;
        self.glUniformMatrix2x4fv_ptr = self.get_proc_address("glUniformMatrix2x4fv")?;
        self.glUniformMatrix4x2fv_ptr = self.get_proc_address("glUniformMatrix4x2fv")?;
        self.glUniformMatrix3x4fv_ptr = self.get_proc_address("glUniformMatrix3x4fv")?;
        self.glUniformMatrix4x3fv_ptr = self.get_proc_address("glUniformMatrix4x3fv")?;
        self.glRenderbufferStorageMultisample_ptr =
            self.get_proc_address("glRenderbufferStorageMultisample")?;
        self.glBindVertexArray_ptr = self.get_proc_address("glBindVertexArray")?;
        self.glDeleteVertexArrays_ptr = self.get_proc_address("glDeleteVertexArrays")?;
        self.glGenVertexArrays_ptr = self.get_proc_address("glGenVertexArrays")?;
        self.glIsVertexArray_ptr = self.get_proc_address("glIsVertexArray")?;
        self.glGetIntegeri_v_ptr = self.get_proc_address("glGetIntegeri_v")?;
        self.glEndTransformFeedback_ptr = self.get_proc_address("glEndTransformFeedback")?;
        self.glTransformFeedbackVaryings_ptr =
            self.get_proc_address("glTransformFeedbackVaryings")?;
        self.glGetTransformFeedbackVarying_ptr =
            self.get_proc_address("glGetTransformFeedbackVarying")?;
        self.glBindTransformFeedback_ptr = self.get_proc_address("glBindTransformFeedback")?;
        self.glDeleteTransformFeedbacks_ptr = self.get_proc_address("glDeleteTransformFeedbacks")?;
        self.glGenTransformFeedbacks_ptr = self.get_proc_address("glGenTransformFeedbacks")?;
        self.glIsTransformFeedback_ptr = self.get_proc_address("glIsTransformFeedback")?;
        self.glPauseTransformFeedback_ptr = self.get_proc_address("glPauseTransformFeedback")?;
        self.glResumeTransformFeedback_ptr = self.get_proc_address("glResumeTransformFeedback")?;
        self.glVertexAttribIPointer_ptr = self.get_proc_address("glVertexAttribIPointer")?;
        self.glGetVertexAttribIiv_ptr = self.get_proc_address("glGetVertexAttribIiv")?;
        self.glGetVertexAttribIuiv_ptr = self.get_proc_address("glGetVertexAttribIuiv")?;
        self.glVertexAttribI4i_ptr = self.get_proc_address("glVertexAttribI4i")?;
        self.glVertexAttribI4ui_ptr = self.get_proc_address("glVertexAttribI4ui")?;
        self.glVertexAttribI4iv_ptr = self.get_proc_address("glVertexAttribI4iv")?;
        self.glVertexAttribI4uiv_ptr = self.get_proc_address("glVertexAttribI4uiv")?;
        self.glGetUniformuiv_ptr = self.get_proc_address("glGetUniformuiv")?;
        self.glGetFragDataLocation_ptr = self.get_proc_address("glGetFragDataLocation")?;
        self.glUniform1ui_ptr = self.get_proc_address("glUniform1ui")?;
        self.glUniform2ui_ptr = self.get_proc_address("glUniform2ui")?;
        self.glUniform3ui_ptr = self.get_proc_address("glUniform3ui")?;
        self.glUniform4ui_ptr = self.get_proc_address("glUniform4ui")?;
        self.glUniform1uiv_ptr = self.get_proc_address("glUniform1uiv")?;
        self.glUniform2uiv_ptr = self.get_proc_address("glUniform2uiv")?;
        self.glUniform3uiv_ptr = self.get_proc_address("glUniform3uiv")?;
        self.glUniform4uiv_ptr = self.get_proc_address("glUniform4uiv")?;
        self.glGetStringi_ptr = self.get_proc_address("glGetStringi")?;
        self.glGetUniformIndices_ptr = self.get_proc_address("glGetUniformIndices")?;
        self.glGetActiveUniformsiv_ptr = self.get_proc_address("glGetActiveUniformsiv")?;
        self.glGetUniformBlockIndex_ptr = self.get_proc_address("glGetUniformBlockIndex")?;
        self.glGetActiveUniformBlockiv_ptr = self.get_proc_address("glGetActiveUniformBlockiv")?;
        self.glUniformBlockBinding_ptr = self.get_proc_address("glUniformBlockBinding")?;
        self.glDrawRangeElements_ptr = self.get_proc_address("glDrawRangeElements")?;
        self.glDrawArraysInstanced_ptr = self.get_proc_address("glDrawArraysInstanced")?;
        self.glDrawElementsInstanced_ptr = self.get_proc_address("glDrawElementsInstanced")?;
        self.glFenceSync_ptr = self.get_proc_address("glFenceSync")?;
        self.glIsSync_ptr = self.get_proc_address("glIsSync")?;
        self.glDeleteSync_ptr = self.get_proc_address("glDeleteSync")?;
        self.glClientWaitSync_ptr = self.get_proc_address("glClientWaitSync")?;
        self.glWaitSync_ptr = self.get_proc_address("glWaitSync")?;
        self.glGetInteger64v_ptr = self.get_proc_address("glGetInteger64v")?;
        self.glGetSynciv_ptr = self.get_proc_address("glGetSynciv")?;
        self.glGetInteger64i_v_ptr = self.get_proc_address("glGetInteger64i_v")?;
        self.glGenSamplers_ptr = self.get_proc_address("glGenSamplers")?;
        self.glDeleteSamplers_ptr = self.get_proc_address("glDeleteSamplers")?;
        self.glIsSampler_ptr = self.get_proc_address("glIsSampler")?;
        self.glBindSampler_ptr = self.get_proc_address("glBindSampler")?;
        self.glSamplerParameteri_ptr = self.get_proc_address("glSamplerParameteri")?;
        self.glSamplerParameteriv_ptr = self.get_proc_address("glSamplerParameteriv")?;
        self.glSamplerParameterf_ptr = self.get_proc_address("glSamplerParameterf")?;
        self.glSamplerParameterfv_ptr = self.get_proc_address("glSamplerParameterfv")?;
        self.glGetSamplerParameteriv_ptr = self.get_proc_address("glGetSamplerParameteriv")?;
        self.glGetSamplerParameterfv_ptr = self.get_proc_address("glGetSamplerParameterfv")?;
        self.glVertexAttribDivisor_ptr = self.get_proc_address("glVertexAttribDivisor")?;
        self.glGetProgramBinary_ptr = self.get_proc_address("glGetProgramBinary")?;
        self.glProgramBinary_ptr = self.get_proc_address("glProgramBinary")?;
        self.glProgramParameteri_ptr = self.get_proc_address("glProgramParameteri")?;
        self.glInvalidateFramebuffer_ptr = self.get_proc_address("glInvalidateFramebuffer")?;
        self.glInvalidateSubFramebuffer_ptr = self.get_proc_address("glInvalidateSubFramebuffer")?;
        self.glBlitFramebuffer_ptr = self.get_proc_address("glBlitFramebuffer")?;
        self.glFramebufferTextureLayer_ptr = self.get_proc_address("glFramebufferTextureLayer")?;
        self.glTexStorage2D_ptr = self.get_proc_address("glTexStorage2D")?;
        self.glTexStorage3D_ptr = self.get_proc_address("glTexStorage3D")?;
        self.glGetInternalformativ_ptr = self.get_proc_address("glGetInternalformativ")?;

        Ok(())
    }

    #[cfg(target_os = "android")]
    fn init_es31(&mut self) -> Result<(), InitError> {
        self.glDispatchCompute_ptr = self.get_proc_address("glDispatchCompute")?;
        self.glDispatchComputeIndirect_ptr = self.get_proc_address("glDispatchComputeIndirect")?;
        self.glDrawArraysIndirect_ptr = self.get_proc_address("glDrawArraysIndirect")?;
        self.glDrawElementsIndirect_ptr = self.get_proc_address("glDrawElementsIndirect")?;
        self.glFramebufferParameteri_ptr = self.get_proc_address("glFramebufferParameteri")?;
        self.glGetFramebufferParameteriv_ptr =
            self.get_proc_address("glGetFramebufferParameteriv")?;
        self.glGetProgramInterfaceiv_ptr = self.get_proc_address("glGetProgramInterfaceiv")?;
        self.glGetProgramResourceIndex_ptr = self.get_proc_address("glGetProgramResourceIndex")?;
        self.glGetProgramResourceName_ptr = self.get_proc_address("glGetProgramResourceName")?;
        self.glGetProgramResourceiv_ptr = self.get_proc_address("glGetProgramResourceiv")?;
        self.glGetProgramResourceLocation_ptr =
            self.get_proc_address("glGetProgramResourceLocation")?;
        self.glUseProgramStages_ptr = self.get_proc_address("glUseProgramStages")?;
        self.glActiveShaderProgram_ptr = self.get_proc_address("glActiveShaderProgram")?;
        self.glCreateShaderProgramv_ptr = self.get_proc_address("glCreateShaderProgramv")?;
        self.glBindProgramPipeline_ptr = self.get_proc_address("glBindProgramPipeline")?;
        self.glDeleteProgramPipelines_ptr = self.get_proc_address("glDeleteProgramPipelines")?;
        self.glGenProgramPipelines_ptr = self.get_proc_address("glGenProgramPipelines")?;
        self.glIsProgramPipeline_ptr = self.get_proc_address("glIsProgramPipeline")?;
        self.glGetProgramPipelineiv_ptr = self.get_proc_address("glGetProgramPipelineiv")?;
        self.glProgramUniform1i_ptr = self.get_proc_address("glProgramUniform1i")?;
        self.glProgramUniform2i_ptr = self.get_proc_address("glProgramUniform2i")?;
        self.glProgramUniform3i_ptr = self.get_proc_address("glProgramUniform3i")?;
        self.glProgramUniform4i_ptr = self.get_proc_address("glProgramUniform4i")?;
        self.glProgramUniform1ui_ptr = self.get_proc_address("glProgramUniform1ui")?;
        self.glProgramUniform2ui_ptr = self.get_proc_address("glProgramUniform2ui")?;
        self.glProgramUniform3ui_ptr = self.get_proc_address("glProgramUniform3ui")?;
        self.glProgramUniform4ui_ptr = self.get_proc_address("glProgramUniform4ui")?;
        self.glProgramUniform1f_ptr = self.get_proc_address("glProgramUniform1f")?;
        self.glProgramUniform2f_ptr = self.get_proc_address("glProgramUniform2f")?;
        self.glProgramUniform3f_ptr = self.get_proc_address("glProgramUniform3f")?;
        self.glProgramUniform4f_ptr = self.get_proc_address("glProgramUniform4f")?;
        self.glProgramUniform1iv_ptr = self.get_proc_address("glProgramUniform1iv")?;
        self.glProgramUniform2iv_ptr = self.get_proc_address("glProgramUniform2iv")?;
        self.glProgramUniform3iv_ptr = self.get_proc_address("glProgramUniform3iv")?;
        self.glProgramUniform4iv_ptr = self.get_proc_address("glProgramUniform4iv")?;
        self.glProgramUniform1uiv_ptr = self.get_proc_address("glProgramUniform1uiv")?;
        self.glProgramUniform2uiv_ptr = self.get_proc_address("glProgramUniform2uiv")?;
        self.glProgramUniform3uiv_ptr = self.get_proc_address("glProgramUniform3uiv")?;
        self.glProgramUniform4uiv_ptr = self.get_proc_address("glProgramUniform4uiv")?;
        self.glProgramUniform4fv_ptr = self.get_proc_address("glProgramUniform4fv")?;
        self.glProgramUniform2fv_ptr = self.get_proc_address("glProgramUniform2fv")?;
        self.glProgramUniform3fv_ptr = self.get_proc_address("glProgramUniform3fv")?;
        self.glProgramUniformMatrix2fv_ptr = self.get_proc_address("glProgramUniformMatrix2fv")?;
        self.glProgramUniformMatrix3fv_ptr = self.get_proc_address("glProgramUniformMatrix3fv")?;
        self.glProgramUniformMatrix4fv_ptr = self.get_proc_address("glProgramUniformMatrix4fv")?;
        self.glProgramUniformMatrix2x3fv_ptr =
            self.get_proc_address("glProgramUniformMatrix2x3fv")?;
        self.glProgramUniformMatrix3x2fv_ptr =
            self.get_proc_address("glProgramUniformMatrix3x2fv")?;
        self.glProgramUniformMatrix2x4fv_ptr =
            self.get_proc_address("glProgramUniformMatrix2x4fv")?;
        self.glProgramUniformMatrix4x2fv_ptr =
            self.get_proc_address("glProgramUniformMatrix4x2fv")?;
        self.glProgramUniformMatrix3x4fv_ptr =
            self.get_proc_address("glProgramUniformMatrix3x4fv")?;
        self.glProgramUniformMatrix4x3fv_ptr =
            self.get_proc_address("glProgramUniformMatrix4x3fv")?;
        self.glValidateProgramPipeline_ptr = self.get_proc_address("glValidateProgramPipeline")?;
        self.glGetProgramPipelineInfoLog_ptr =
            self.get_proc_address("glGetProgramPipelineInfoLog")?;
        self.glBindImageTexture_ptr = self.get_proc_address("glBindImageTexture")?;
        self.glGetBooleani_v_ptr = self.get_proc_address("glGetBooleani_v")?;
        self.glMemoryBarrier_ptr = self.get_proc_address("glMemoryBarrier")?;
        self.glMemoryBarrierByRegion_ptr = self.get_proc_address("glMemoryBarrierByRegion")?;
        self.glTexStorage2DMultisample_ptr = self.get_proc_address("glTexStorage2DMultisample")?;
        self.glGetMultisamplefv_ptr = self.get_proc_address("glGetMultisamplefv")?;
        self.glSampleMaski_ptr = self.get_proc_address("glSampleMaski")?;
        self.glGetTexLevelParameteriv_ptr = self.get_proc_address("glGetTexLevelParameteriv")?;
        self.glGetTexLevelParameterfv_ptr = self.get_proc_address("glGetTexLevelParameterfv")?;
        self.glBindVertexBuffer_ptr = self.get_proc_address("glBindVertexBuffer")?;
        self.glVertexAttribFormat_ptr = self.get_proc_address("glVertexAttribFormat")?;
        self.glVertexAttribIFormat_ptr = self.get_proc_address("glVertexAttribIFormat")?;
        self.glVertexAttribBinding_ptr = self.get_proc_address("glVertexAttribBinding")?;
        self.glVertexBindingDivisor_ptr = self.get_proc_address("glVertexBindingDivisor")?;

        Ok(())
    }

    #[cfg(target_os = "android")]
    fn init_es32(&mut self) -> Result<(), InitError> {
        self.glBlendBarrier_ptr = self.get_proc_address("glBlendBarrier")?;
        self.glCopyImageSubData_ptr = self.get_proc_address("glCopyImageSubData")?;
        self.glDebugMessageControl_ptr = self.get_proc_address("glDebugMessageControl")?;
        self.glDebugMessageInsert_ptr = self.get_proc_address("glDebugMessageInsert")?;
        self.glDebugMessageCallback_ptr = self.get_proc_address("glDebugMessageCallback")?;
        self.glGetDebugMessageLog_ptr = self.get_proc_address("glGetDebugMessageLog")?;
        self.glPushDebugGroup_ptr = self.get_proc_address("glPushDebugGroup")?;
        self.glPopDebugGroup_ptr = self.get_proc_address("glPopDebugGroup")?;
        self.glObjectLabel_ptr = self.get_proc_address("glObjectLabel")?;
        self.glGetObjectPtrLabel_ptr = self.get_proc_address("glGetObjectPtrLabel")?;
        self.glObjectPtrLabel_ptr = self.get_proc_address("glObjectPtrLabel")?;
        self.glGetPointerv_ptr = self.get_proc_address("glGetPointerv")?;
        self.glEnablei_ptr = self.get_proc_address("glEnablei")?;
        self.glDisablei_ptr = self.get_proc_address("glDisablei")?;
        self.glBlendEquationi_ptr = self.get_proc_address("glBlendEquationi")?;
        self.glBlendEquationSeparatei_ptr = self.get_proc_address("glBlendEquationSeparatei")?;
        self.glBlendFunci_ptr = self.get_proc_address("glBlendFunci")?;
        self.glBlendFuncSeparatei_ptr = self.get_proc_address("glBlendFuncSeparatei")?;
        self.glColorMaski_ptr = self.get_proc_address("glColorMaski")?;
        self.glIsEnabledi_ptr = self.get_proc_address("glIsEnabledi")?;
        self.glDrawElementsBaseVertex_ptr = self.get_proc_address("glDrawElementsBaseVertex")?;
        self.glDrawRangeElementsBaseVertex_ptr =
            self.get_proc_address("glDrawRangeElementsBaseVertex")?;
        self.glDrawElementsInstancedBaseVertex_ptr =
            self.get_proc_address("glDrawElementsInstancedBaseVertex")?;
        self.glFramebufferTexture_ptr = self.get_proc_address("glFramebufferTexture")?;
        self.glPrimitiveBoundingBox_ptr = self.get_proc_address("glPrimitiveBoundingBox")?;
        self.glGetGraphicsResetStatus_ptr = self.get_proc_address("glGetGraphicsResetStatus")?;
        self.glReadnPixels_ptr = self.get_proc_address("glReadnPixels")?;
        self.glGetnUniformfv_ptr = self.get_proc_address("glGetnUniformfv")?;
        self.glGetnUniformiv_ptr = self.get_proc_address("glGetnUniformiv")?;
        self.glGetnUniformuiv_ptr = self.get_proc_address("glGetnUniformuiv")?;
        self.glMinSampleShading_ptr = self.get_proc_address("glMinSampleShading")?;
        self.glPatchParameteri_ptr = self.get_proc_address("glPatchParameteri")?;
        self.glTexParameterIiv_ptr = self.get_proc_address("glTexParameterIiv")?;
        self.glTexParameterIuiv_ptr = self.get_proc_address("glTexParameterIuiv")?;
        self.glGetTexParameterIiv_ptr = self.get_proc_address("glGetTexParameterIiv")?;
        self.glGetTexParameterIuiv_ptr = self.get_proc_address("glGetTexParameterIuiv")?;
        self.glSamplerParameterIiv_ptr = self.get_proc_address("glSamplerParameterIiv")?;
        self.glSamplerParameterIuiv_ptr = self.get_proc_address("glSamplerParameterIuiv")?;
        self.glGetSamplerParameterIuiv_ptr = self.get_proc_address("glGetSamplerParameterIuiv")?;
        self.glTexBuffer_ptr = self.get_proc_address("glTexBuffer")?;
        self.glTexBufferRange_ptr = self.get_proc_address("glTexBufferRange")?;
        self.glTexStorage3DMultisample_ptr = self.get_proc_address("glTexStorage3DMultisample")?;

        Ok(())
    }

    pub fn init(&mut self) -> InitResult {
        #[cfg(target_os = "ios")] {
            return InitResult {
                es_version: ES_VERSION::ES30,
                desc: "".to_string(),
            };
        }

        #[cfg(target_os = "android")] {
            let mut result = InitResult {
                es_version: ES_VERSION::ES20,
                desc: "".to_string(),
            };

            match self.init_es30() {
                Ok(()) => {
                    self.es_version = ES_VERSION::ES30;
                    result.es_version = ES_VERSION::ES30;
                },
                Err(error) => {
                    result.desc = error.desc.clone();
                    return result;
                }
            }

            match self.init_es31() {
                Ok(()) => {
                    self.es_version = ES_VERSION::ES31;
                    result.es_version = ES_VERSION::ES31;
                },
                Err(error) => {
                    result.desc = error.desc.clone();
                    return result;
                }
            }

            match self.init_es32() {
                Ok(()) => {
                    self.es_version = ES_VERSION::ES32;
                    result.es_version = ES_VERSION::ES32;
                },
                Err(error) => {
                    result.desc = error.desc.clone();
                    return result;
                }
            }

            return result;
        }
    }


    #[cfg(target_os = "android")]
    fn get_proc_address(&mut self, proc_name: &str) -> Result<*const c_void, InitError> {
        unsafe {
            let string = CString::new(proc_name).unwrap();

            let address = eglGetProcAddress(string.as_ptr());

            if address.is_null() {
                return Err(InitError {
                    desc: proc_name.to_string(),
                });
            } else {
                return Ok(address);
            }
        }
    }

    pub fn gl_active_texture(&mut self, texture_unit: TextureUnit) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_texture_unit = ParamInfo::new("TextureUnit", "texture_unit");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_active_texture".to_string();

            param_infos.push(&param_info_texture_unit);
            param_values.push(&texture_unit);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glActiveTexture(texture_unit as GLenum);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glActiveTexture(texture_unit as GLenum);
            }

            Ok(())
        }
    }
    pub fn gl_attach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_attach_shader".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glAttachShader(program as GLuint, shader as GLuint);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glAttachShader(program as GLuint, shader as GLuint);
            }

            Ok(())
        }
    }
    pub fn gl_bind_attrib_location(
        &mut self,
        program: u32,
        index: u32,
        name: &str,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_name = ParamInfo::new("&str", "name");
            let param_value_name = name.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_attrib_location".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_name);
            param_values.push(&param_value_name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let c_str = CString::new(name).unwrap();

                    ffi::glBindAttribLocation(
                        program as GLuint,
                        index as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let c_str = CString::new(name).unwrap();

                ffi::glBindAttribLocation(
                    program as GLuint,
                    index as GLuint,
                    c_str.as_ptr() as *const GLchar,
                );
            }

            Ok(())
        }
    }
    pub fn gl_bind_buffer(&mut self, target: BufferTarget, buffer: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferTarget", "target");

            let mut param_info_buffer = ParamInfo::new("GLuint", "buffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_buffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glBindBuffer(target as GLenum, buffer as GLuint);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glBindBuffer(target as GLenum, buffer as GLuint);
            }

            Ok(())
        }
    }
    pub fn gl_bind_framebuffer(
        &mut self,
        target: FrameBufferTarget,
        framebuffer: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_framebuffer = ParamInfo::new("GLuint", "framebuffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_framebuffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_framebuffer);
            param_values.push(&framebuffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glBindFramebuffer(target as GLenum, framebuffer as GLuint);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glBindFramebuffer(target as GLenum, framebuffer as GLuint);
            }

            Ok(())
        }
    }
    pub fn gl_bind_renderbuffer(
        &mut self,
        target: RenderBufferTarget,
        renderbuffer: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("RenderBufferTarget", "target");

            let mut param_info_renderbuffer = ParamInfo::new("u32", "renderbuffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_renderbuffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_renderbuffer);
            param_values.push(&renderbuffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glBindRenderbuffer(target as GLenum, renderbuffer as GLuint);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glBindRenderbuffer(target as GLenum, renderbuffer as GLuint);
            }

            Ok(())
        }
    }
    pub fn gl_bind_texture(
        &mut self,
        target: TextureBindTarget,
        texture: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_info_texture = ParamInfo::new("u32", "texture");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_texture".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glBindTexture(target as GLenum, texture as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glBindTexture(target as GLenum, texture as GLuint) }

            Ok(())
        }
    }
    pub fn gl_blend_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_red = ParamInfo::new("f32", "red");

            let mut param_info_green = ParamInfo::new("f32", "green");

            let mut param_info_blue = ParamInfo::new("f32", "blue");

            let mut param_info_alpha = ParamInfo::new("f32", "alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_color".to_string();

            param_infos.push(&param_info_red);
            param_values.push(&red);

            param_infos.push(&param_info_green);
            param_values.push(&green);

            param_infos.push(&param_info_blue);
            param_values.push(&blue);

            param_infos.push(&param_info_alpha);
            param_values.push(&alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glBlendColor(
                        red as GLclampf,
                        green as GLclampf,
                        blue as GLclampf,
                        alpha as GLclampf,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glBlendColor(
                    red as GLclampf,
                    green as GLclampf,
                    blue as GLclampf,
                    alpha as GLclampf,
                )
            }

            Ok(())
        }
    }
    pub fn gl_blend_equation(&mut self, mode: BlendEquationMode) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BlendEquationMode", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_equation".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glBlendEquation(mode as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glBlendEquation(mode as GLenum) }

            Ok(())
        }
    }
    pub fn gl_blend_equation_separate(
        &mut self,
        mode_rgb: BlendEquationMode,
        mode_alpha: BlendEquationMode,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode_rgb = ParamInfo::new("BlendEquationMode", "mode_rgb");

            let mut param_info_mode_alpha = ParamInfo::new("BlendEquationMode", "mode_alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_equation_separate".to_string();

            param_infos.push(&param_info_mode_rgb);
            param_values.push(&mode_rgb);

            param_infos.push(&param_info_mode_alpha);
            param_values.push(&mode_alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glBlendEquationSeparate(mode_rgb as GLenum, mode_alpha as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glBlendEquationSeparate(mode_rgb as GLenum, mode_alpha as GLenum) }

            Ok(())
        }
    }
    pub fn gl_blend_func(
        &mut self,
        src_factor: BlendFactor,
        dst_factor: BlendFactor,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_src_factor = ParamInfo::new("BlendFactor", "src_factor");

            let mut param_info_dst_factor = ParamInfo::new("BlendFactor", "dst_factor");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_func".to_string();

            param_infos.push(&param_info_src_factor);
            param_values.push(&src_factor);

            param_infos.push(&param_info_dst_factor);
            param_values.push(&dst_factor);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glBlendFunc(src_factor as GLenum, dst_factor as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glBlendFunc(src_factor as GLenum, dst_factor as GLenum) }

            Ok(())
        }
    }
    pub fn gl_blend_func_separate(
        &mut self,
        src_rgb: BlendFactor,
        dst_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dst_alpha: BlendFactor,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_src_rgb = ParamInfo::new("BlendFactor", "src_rgb");

            let mut param_info_dst_rgb = ParamInfo::new("BlendFactor", "dst_rgb");

            let mut param_info_src_alpha = ParamInfo::new("BlendFactor", "src_alpha");

            let mut param_info_dst_alpha = ParamInfo::new("BlendFactor", "dst_alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_func_separate".to_string();

            param_infos.push(&param_info_src_rgb);
            param_values.push(&src_rgb);

            param_infos.push(&param_info_dst_rgb);
            param_values.push(&dst_rgb);

            param_infos.push(&param_info_src_alpha);
            param_values.push(&src_alpha);

            param_infos.push(&param_info_dst_alpha);
            param_values.push(&dst_alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glBlendFuncSeparate(
                        src_rgb as GLenum,
                        dst_rgb as GLenum,
                        src_alpha as GLenum,
                        dst_alpha as GLenum,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glBlendFuncSeparate(
                    src_rgb as GLenum,
                    dst_rgb as GLenum,
                    src_alpha as GLenum,
                    dst_alpha as GLenum,
                )
            }

            Ok(())
        }
    }
    pub fn gl_buffer_data<T>(
        &mut self,
        target: BufferTarget,
        buffer: &[T],
        usage: BufferUsage,
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferTarget", "target");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_info_usage = ParamInfo::new("BufferUsage", "usage");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_buffer_data".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            param_infos.push(&param_info_usage);
            param_values.push(&usage);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
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
    }
    pub fn gl_buffer_sub_data<T>(
        &mut self,
        target: BufferTarget,
        offset: u32,
        buffer: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferTarget", "target");

            let mut param_info_offset = ParamInfo::new("u32", "offset");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_buffer_sub_data".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
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
    }
    pub fn gl_check_framebuffer_status(
        &mut self,
        target: FrameBufferTarget,
    ) -> Result<FrameBufferStatus, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_check_framebuffer_status".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let status = ffi::glCheckFramebufferStatus(target as GLenum);

                    Ok(FrameBufferStatus::from(status))
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let status = ffi::glCheckFramebufferStatus(target as GLenum);

                Ok(FrameBufferStatus::from(status))
            }
        }
    }
    pub fn gl_clear(&mut self, mask: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mask = ParamInfo::new("u32", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear".to_string();

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glClear(mask as GLbitfield) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glClear(mask as GLbitfield) }

            Ok(())
        }
    }
    pub fn gl_clear_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_red = ParamInfo::new("f32", "red");

            let mut param_info_green = ParamInfo::new("f32", "green");

            let mut param_info_blue = ParamInfo::new("f32", "blue");

            let mut param_info_alpha = ParamInfo::new("f32", "alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_color".to_string();

            param_infos.push(&param_info_red);
            param_values.push(&red);

            param_infos.push(&param_info_green);
            param_values.push(&green);

            param_infos.push(&param_info_blue);
            param_values.push(&blue);

            param_infos.push(&param_info_alpha);
            param_values.push(&alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glClearColor(
                        red as GLclampf,
                        green as GLclampf,
                        blue as GLclampf,
                        alpha as GLclampf,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glClearColor(
                    red as GLclampf,
                    green as GLclampf,
                    blue as GLclampf,
                    alpha as GLclampf,
                )
            }

            Ok(())
        }
    }
    pub fn gl_clear_depthf(&mut self, depth: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_depth = ParamInfo::new("f32", "depth");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_depthf".to_string();

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glClearDepthf(depth as GLclampf) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glClearDepthf(depth as GLclampf) }

            Ok(())
        }
    }
    pub fn gl_clear_stencil(&mut self, stencil: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_stencil = ParamInfo::new("i32", "stencil");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_stencil".to_string();

            param_infos.push(&param_info_stencil);
            param_values.push(&stencil);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glClearStencil(stencil as GLint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glClearStencil(stencil as GLint) }

            Ok(())
        }
    }
    pub fn gl_color_mask(
        &mut self,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_red = ParamInfo::new("bool", "red");

            let mut param_info_green = ParamInfo::new("bool", "green");

            let mut param_info_blue = ParamInfo::new("bool", "blue");

            let mut param_info_alpha = ParamInfo::new("bool", "alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_color_mask".to_string();

            param_infos.push(&param_info_red);
            param_values.push(&red);

            param_infos.push(&param_info_green);
            param_values.push(&green);

            param_infos.push(&param_info_blue);
            param_values.push(&blue);

            param_infos.push(&param_info_alpha);
            param_values.push(&alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glColorMask(
                        red as GLboolean,
                        green as GLboolean,
                        blue as GLboolean,
                        alpha as GLboolean,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
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
    }
    pub fn gl_compile_shader(&mut self, shader: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_compile_shader".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glCompileShader(shader as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glCompileShader(shader as GLuint) }

            Ok(())
        }
    }
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
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("i32", "level");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_width = ParamInfo::new("u32", "width");

            let mut param_info_height = ParamInfo::new("u32", "height");

            let mut param_info_border = ParamInfo::new("u32", "border");

            let mut param_info_image_size = ParamInfo::new("u32", "image_size");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_compressed_tex_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_border);
            param_values.push(&border);

            param_infos.push(&param_info_image_size);
            param_values.push(&image_size);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
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
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("u32", "level");

            let mut param_info_x_offset = ParamInfo::new("u32", "x_offset");

            let mut param_info_y_offset = ParamInfo::new("u32", "y_offset");

            let mut param_info_width = ParamInfo::new("u32", "width");

            let mut param_info_height = ParamInfo::new("u32", "height");

            let mut param_info_format = ParamInfo::new("GLenum", "format");

            let mut param_info_image_size = ParamInfo::new("u32", "image_size");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_compressed_tex_sub_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_x_offset);
            param_values.push(&x_offset);

            param_infos.push(&param_info_y_offset);
            param_values.push(&y_offset);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_image_size);
            param_values.push(&image_size);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
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
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("u32", "level");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_x = ParamInfo::new("u32", "x");

            let mut param_info_y = ParamInfo::new("u32", "y");

            let mut param_info_width = ParamInfo::new("u32", "width");

            let mut param_info_height = ParamInfo::new("u32", "height");

            let mut param_info_border = ParamInfo::new("u32", "border");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_copy_tex_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_border);
            param_values.push(&border);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glCopyTexImage2D(
                        target as GLenum,
                        level as GLint,
                        internal_format,
                        x as GLint,
                        y as GLint,
                        width as GLsizei,
                        height as GLsizei,
                        border as GLint,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glCopyTexImage2D(
                    target as GLenum,
                    level as GLint,
                    internal_format,
                    x as GLint,
                    y as GLint,
                    width as GLsizei,
                    height as GLsizei,
                    border as GLint,
                )
            }

            Ok(())
        }
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
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("u32", "level");

            let mut param_info_x_offset = ParamInfo::new("u32", "x_offset");

            let mut param_info_y_offset = ParamInfo::new("u32", "y_offset");

            let mut param_info_x = ParamInfo::new("u32", "x");

            let mut param_info_y = ParamInfo::new("u32", "y");

            let mut param_info_width = ParamInfo::new("u32", "width");

            let mut param_info_height = ParamInfo::new("u32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_copy_tex_sub_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_x_offset);
            param_values.push(&x_offset);

            param_infos.push(&param_info_y_offset);
            param_values.push(&y_offset);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glCopyTexSubImage2D(
                        target as GLenum,
                        level as GLint,
                        x_offset as GLint,
                        y_offset as GLint,
                        x as GLint,
                        y as GLint,
                        width as GLsizei,
                        height as GLsizei,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glCopyTexSubImage2D(
                    target as GLenum,
                    level as GLint,
                    x_offset as GLint,
                    y_offset as GLint,
                    x as GLint,
                    y as GLint,
                    width as GLsizei,
                    height as GLsizei,
                )
            }

            Ok(())
        }
    }
    pub fn gl_create_program(&mut self) -> Result<u32, Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_create_program".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let program_id = ffi::glCreateProgram();

                    Ok(program_id as u32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let program_id = ffi::glCreateProgram();

                Ok(program_id as u32)
            }
        }
    }
    pub fn gl_create_shader(&mut self, type_: ShaderType) -> Result<u32, Error> {
        if self.is_debug() {
            let mut param_info_type_ = ParamInfo::new("ShaderType", "type_");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_create_shader".to_string();

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let shader_id = ffi::glCreateShader(type_ as GLenum);

                    Ok(shader_id as u32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let shader_id = ffi::glCreateShader(type_ as GLenum);

                Ok(shader_id as u32)
            }
        }
    }
    pub fn gl_cull_face(&mut self, mode: FaceMode) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("FaceMode", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_cull_face".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glCullFace(mode as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glCullFace(mode as GLenum) }

            Ok(())
        }
    }
    pub fn gl_delete_buffers(&mut self, buffers: &[u32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buffers = ParamInfo::new("&[u32]", "buffers");
            let param_value_buffers = buffers.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_buffers".to_string();

            param_infos.push(&param_info_buffers);
            param_values.push(&param_value_buffers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_delete_framebuffers(&mut self, framebuffers: &[u32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_framebuffers = ParamInfo::new("&[u32]", "framebuffers");
            let param_value_framebuffers = framebuffers.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_framebuffers".to_string();

            param_infos.push(&param_info_framebuffers);
            param_values.push(&param_value_framebuffers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
            }

            Ok(())
        }
    }
    pub fn gl_delete_program(&mut self, program: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_program".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDeleteProgram(program as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDeleteProgram(program as GLuint) }

            Ok(())
        }
    }
    pub fn gl_delete_renderbuffers(&mut self, renderbuffers: &[u32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_renderbuffers = ParamInfo::new("&[u32]", "renderbuffers");
            let param_value_renderbuffers = renderbuffers.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_renderbuffers".to_string();

            param_infos.push(&param_info_renderbuffers);
            param_values.push(&param_value_renderbuffers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glDeleteRenderbuffers(
                        renderbuffers.len() as GLsizei,
                        renderbuffers.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei, renderbuffers.as_ptr())
            }

            Ok(())
        }
    }
    pub fn gl_delete_shader(&mut self, shader: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_shader".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDeleteShader(shader as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDeleteShader(shader as GLuint) }

            Ok(())
        }
    }
    pub fn gl_delete_textures(&mut self, textures: &[u32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_textures = ParamInfo::new("&[u32]", "textures");
            let param_value_textures = textures.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_textures".to_string();

            param_infos.push(&param_info_textures);
            param_values.push(&param_value_textures);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_depth_func(&mut self, func: FuncType) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_func = ParamInfo::new("FuncType", "func");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_depth_func".to_string();

            param_infos.push(&param_info_func);
            param_values.push(&func);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDepthFunc(func as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDepthFunc(func as GLenum) }

            Ok(())
        }
    }
    pub fn gl_depth_mask(&mut self, flag: bool) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_flag = ParamInfo::new("bool", "flag");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_depth_mask".to_string();

            param_infos.push(&param_info_flag);
            param_values.push(&flag);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDepthMask(flag as GLboolean) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDepthMask(flag as GLboolean) }

            Ok(())
        }
    }
    pub fn gl_depth_rangef(&mut self, z_near: f32, z_far: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_z_near = ParamInfo::new("f32", "z_near");

            let mut param_info_z_far = ParamInfo::new("f32", "z_far");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_depth_rangef".to_string();

            param_infos.push(&param_info_z_near);
            param_values.push(&z_near);

            param_infos.push(&param_info_z_far);
            param_values.push(&z_far);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDepthRangef(z_near as GLclampf, z_far as GLclampf) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDepthRangef(z_near as GLclampf, z_far as GLclampf) }

            Ok(())
        }
    }
    pub fn gl_detach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_detach_shader".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDetachShader(program as GLuint, shader as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDetachShader(program as GLuint, shader as GLuint) }

            Ok(())
        }
    }
    pub fn gl_disable(&mut self, feature: FeatureType) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_feature = ParamInfo::new("FeatureType", "feature");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_disable".to_string();

            param_infos.push(&param_info_feature);
            param_values.push(&feature);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDisable(feature as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDisable(feature as GLenum) }

            Ok(())
        }
    }
    pub fn gl_disable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_disable_vertex_attrib_array".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDisableVertexAttribArray(index as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDisableVertexAttribArray(index as GLuint) }

            Ok(())
        }
    }
    pub fn gl_draw_arrays(&mut self, mode: BeginMode, first: i32, count: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BeginMode", "mode");

            let mut param_info_first = ParamInfo::new("i32", "first");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_arrays".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_first);
            param_values.push(&first);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glDrawArrays(mode as GLenum, first as GLint, count as GLsizei) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glDrawArrays(mode as GLenum, first as GLint, count as GLsizei) }

            Ok(())
        }
    }
    pub fn gl_draw_elements<T>(
        &mut self,
        mode: BeginMode,
        count: i32,
        type_: GLenum,
        indices: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BeginMode", "mode");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("&[T]", "indices");
            let param_value_indices = indices.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_elements".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&param_value_indices);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glDrawElements(
                        mode as GLenum,
                        count as GLsizei,
                        type_,
                        indices.as_ptr() as *const GLvoid,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glDrawElements(
                    mode as GLenum,
                    count as GLsizei,
                    type_,
                    indices.as_ptr() as *const GLvoid,
                )
            }

            Ok(())
        }
    }
    pub fn gl_enable(&mut self, feature: FeatureType) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_feature = ParamInfo::new("FeatureType", "feature");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_enable".to_string();

            param_infos.push(&param_info_feature);
            param_values.push(&feature);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glEnable(feature as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glEnable(feature as GLenum) }

            Ok(())
        }
    }
    pub fn gl_enable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_enable_vertex_attrib_array".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glEnableVertexAttribArray(index as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glEnableVertexAttribArray(index as GLuint) }

            Ok(())
        }
    }
    pub fn gl_finish(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_finish".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glFinish() }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glFinish() }

            Ok(())
        }
    }
    pub fn gl_flush(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_flush".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glFlush() }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glFlush() }

            Ok(())
        }
    }
    pub fn gl_framebuffer_renderbuffer(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        renderbuffer_target: RenderBufferTarget,
        renderbuffer: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_attachment =
                ParamInfo::new("FrameBufferAttachmentType", "attachment");

            let mut param_info_renderbuffer_target =
                ParamInfo::new("RenderBufferTarget", "renderbuffer_target");

            let mut param_info_renderbuffer = ParamInfo::new("u32", "renderbuffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_framebuffer_renderbuffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_attachment);
            param_values.push(&attachment);

            param_infos.push(&param_info_renderbuffer_target);
            param_values.push(&renderbuffer_target);

            param_infos.push(&param_info_renderbuffer);
            param_values.push(&renderbuffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glFramebufferRenderbuffer(
                        target as GLenum,
                        attachment as GLenum,
                        renderbuffer_target as GLenum,
                        renderbuffer as GLuint,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glFramebufferRenderbuffer(
                    target as GLenum,
                    attachment as GLenum,
                    renderbuffer_target as GLenum,
                    renderbuffer as GLuint,
                )
            }

            Ok(())
        }
    }
    pub fn gl_framebuffer_texture_2d(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        texture_target: TextureTarget,
        texture: u32,
        level: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_attachment =
                ParamInfo::new("FrameBufferAttachmentType", "attachment");

            let mut param_info_texture_target = ParamInfo::new("TextureTarget", "texture_target");

            let mut param_info_texture = ParamInfo::new("u32", "texture");

            let mut param_info_level = ParamInfo::new("i32", "level");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_framebuffer_texture_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_attachment);
            param_values.push(&attachment);

            param_infos.push(&param_info_texture_target);
            param_values.push(&texture_target);

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glFramebufferTexture2D(
                        target as GLenum,
                        attachment as GLenum,
                        texture_target as GLenum,
                        texture as GLuint,
                        level as GLint,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glFramebufferTexture2D(
                    target as GLenum,
                    attachment as GLenum,
                    texture_target as GLenum,
                    texture as GLuint,
                    level as GLint,
                )
            }

            Ok(())
        }
    }
    pub fn gl_front_face(&mut self, mode: FrontFaceDirection) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("FrontFaceDirection", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_front_face".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glFrontFace(mode as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glFrontFace(mode as GLenum) }

            Ok(())
        }
    }
    pub fn gl_gen_buffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("u32", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_buffers".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                    ffi::glGenBuffers(count as GLsizei, vec.as_mut_ptr());

                    vec.set_len(count as usize);

                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                ffi::glGenBuffers(count as GLsizei, vec.as_mut_ptr());

                vec.set_len(count as usize);

                Ok(vec)
            }
        }
    }
    pub fn gl_generate_mipmap(&mut self, target: TextureBindTarget) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_generate_mipmap".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glGenerateMipmap(target as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glGenerateMipmap(target as GLenum) }

            Ok(())
        }
    }
    pub fn gl_gen_framebuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("u32", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_framebuffers".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                    ffi::glGenFramebuffers(count as GLsizei, vec.as_mut_ptr());

                    vec.set_len(count as usize);
                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                ffi::glGenFramebuffers(count as GLsizei, vec.as_mut_ptr());

                vec.set_len(count as usize);
                Ok(vec)
            }
        }
    }
    pub fn gl_gen_renderbuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("u32", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_renderbuffers".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                    ffi::glGenRenderbuffers(count as GLsizei, vec.as_mut_ptr());

                    vec.set_len(count as usize);
                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                ffi::glGenRenderbuffers(count as GLsizei, vec.as_mut_ptr());

                vec.set_len(count as usize);
                Ok(vec)
            }
        }
    }
    pub fn gl_gen_textures(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("u32", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_textures".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                    ffi::glGenTextures(count as GLsizei, vec.as_mut_ptr());

                    vec.set_len(count as usize);
                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

                ffi::glGenTextures(count as GLsizei, vec.as_mut_ptr());

                vec.set_len(count as usize);
                Ok(vec)
            }
        }
    }
    pub fn gl_get_active_attrib(&mut self, program: u32, index: u32) -> Result<Active, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_active_attrib".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut size: GLint = 0;
                    let mut attrib_type: GLenum = 0;

                    let mut name = String::with_capacity(256);

                    ffi::glGetActiveAttrib(
                        program as GLuint,
                        index as GLuint,
                        256,
                        &mut length,
                        &mut size,
                        &mut attrib_type,
                        name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        name.as_mut_vec().set_len(length as usize);
                        name.truncate(length as usize);

                        let type_ = DataType::from(attrib_type);

                        Ok(Active {
                            name,
                            size,
                            type_,
                            length,
                        })
                    } else {
                        // TODO: error desc
                        Err(Error {})
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut size: GLint = 0;
                let mut attrib_type: GLenum = 0;

                let mut name = String::with_capacity(256);

                ffi::glGetActiveAttrib(
                    program as GLuint,
                    index as GLuint,
                    256,
                    &mut length,
                    &mut size,
                    &mut attrib_type,
                    name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    name.as_mut_vec().set_len(length as usize);
                    name.truncate(length as usize);

                    let type_ = DataType::from(attrib_type);

                    Ok(Active {
                        name,
                        size,
                        type_,
                        length,
                    })
                } else {
                    // TODO: error desc
                    Err(Error {})
                }
            }
        }
    }
    pub fn gl_get_active_uniform(&mut self, program: u32, index: u32) -> Result<Active, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_active_uniform".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut size: GLint = 0;
                    let mut uniform_data_type: GLenum = 0;

                    let mut name = String::with_capacity(256);

                    ffi::glGetActiveUniform(
                        program as GLuint,
                        index as GLuint,
                        256,
                        &mut length,
                        &mut size,
                        &mut uniform_data_type,
                        name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        name.as_mut_vec().set_len(length as usize);
                        name.truncate(length as usize);

                        let type_ = DataType::from(uniform_data_type);

                        Ok(Active {
                            name,
                            size,
                            type_,
                            length,
                        })
                    } else {
                        // TODO: error desc
                        Err(Error {})
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut size: GLint = 0;
                let mut uniform_data_type: GLenum = 0;

                let mut name = String::with_capacity(256);

                ffi::glGetActiveUniform(
                    program as GLuint,
                    index as GLuint,
                    256,
                    &mut length,
                    &mut size,
                    &mut uniform_data_type,
                    name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    name.as_mut_vec().set_len(length as usize);
                    name.truncate(length as usize);

                    let type_ = DataType::from(uniform_data_type);

                    Ok(Active {
                        name,
                        size,
                        type_,
                        length,
                    })
                } else {
                    // TODO: error desc
                    Err(Error {})
                }
            }
        }
    }
    pub fn gl_get_attached_shaders(
        &mut self,
        program: u32,
        max_count: i32,
    ) -> Result<Vec<u32>, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_max_count = ParamInfo::new("i32", "max_count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_attached_shaders".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_max_count);
            param_values.push(&max_count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut count: GLsizei = 0;
                    let mut vec: Vec<u32> = Vec::with_capacity(max_count as usize);

                    ffi::glGetAttachedShaders(
                        program as GLuint,
                        max_count as GLsizei,
                        &mut count,
                        vec.as_mut_ptr(),
                    );

                    vec.set_len(count as usize);
                    vec.truncate(count as usize);
                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut count: GLsizei = 0;
                let mut vec: Vec<u32> = Vec::with_capacity(max_count as usize);

                ffi::glGetAttachedShaders(
                    program as GLuint,
                    max_count as GLsizei,
                    &mut count,
                    vec.as_mut_ptr(),
                );

                vec.set_len(count as usize);
                vec.truncate(count as usize);
                Ok(vec)
            }
        }
    }
    pub fn gl_get_attrib_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_name = ParamInfo::new("&str", "name");
            let param_value_name = name.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_attrib_location".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_name);
            param_values.push(&param_value_name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let c_str = CString::new(name).unwrap();

                    let loc = ffi::glGetAttribLocation(
                        program as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );

                    Ok(loc as i32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let c_str = CString::new(name).unwrap();

                let loc =
                    ffi::glGetAttribLocation(program as GLuint, c_str.as_ptr() as *const GLchar);

                Ok(loc as i32)
            }
        }
    }
    pub fn gl_get_booleanv(&mut self, name: StateType) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("StateType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_booleanv".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLboolean = 0;

                unsafe {
                    ffi::glGetBooleanv(name as GLenum, &mut value);
                }

                Ok(value == GL_TRUE)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLboolean = 0;

            unsafe {
                ffi::glGetBooleanv(name as GLenum, &mut value);
            }

            Ok(value == GL_TRUE)
        }
    }
    pub fn gl_get_buffer_parameteriv(
        &mut self,
        target: BufferTarget,
        name: BufferParamName,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferTarget", "target");

            let mut param_info_name = ParamInfo::new("BufferParamName", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_buffer_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetBufferParameteriv(target as GLenum, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetBufferParameteriv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_error(&mut self) -> ErrorType {
        let mut error = GL_NO_ERROR;

        unsafe {
            error = ffi::glGetError();
        }

        ErrorType::from(error)
    }
    pub fn gl_get_floatv(&mut self, name: StateType) -> Result<f32, Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("StateType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_floatv".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLfloat = 0.0;

                unsafe {
                    ffi::glGetFloatv(name as GLenum, &mut value);
                }

                Ok(value as f32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetFloatv(name as GLenum, &mut value);
            }

            Ok(value as f32)
        }
    }
    pub fn gl_get_framebuffer_attachment_parameteriv(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        name: FrameBufferAttachmentParamType,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_attachment =
                ParamInfo::new("FrameBufferAttachmentType", "attachment");

            let mut param_info_name = ParamInfo::new("FrameBufferAttachmentParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_framebuffer_attachment_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_attachment);
            param_values.push(&attachment);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetFramebufferAttachmentParameteriv(
                        target as GLenum,
                        attachment as GLenum,
                        name as GLenum,
                        &mut value,
                    );
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetFramebufferAttachmentParameteriv(
                    target as GLenum,
                    attachment as GLenum,
                    name as GLenum,
                    &mut value,
                );
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_integerv(&mut self, name: StateType) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("StateType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_integerv".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetIntegerv(name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetIntegerv(name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_programiv(&mut self, program: u32, name: ProgramParamType) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_name = ParamInfo::new("ProgramParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_programiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetProgramiv(program as GLuint, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetProgramiv(program as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_program_info_log(
        &mut self,
        program: u32,
        max_length: i32,
    ) -> Result<String, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_max_length = ParamInfo::new("i32", "max_length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_info_log".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_max_length);
            param_values.push(&max_length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut log = String::with_capacity(max_length as usize);

                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramInfoLog(
                        program as GLuint,
                        max_length as GLsizei,
                        &mut length,
                        log.as_mut_vec().as_mut_ptr() as *mut i8,
                    );

                    #[cfg(target_os = "android")]
                    ffi::glGetProgramInfoLog(
                        program as GLuint,
                        max_length as GLsizei,
                        &mut length,
                        log.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        log.as_mut_vec().set_len(length as usize);
                        log.truncate(length as usize);

                        Ok(log)
                    } else {
                        Ok("".to_string())
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut log = String::with_capacity(max_length as usize);

                #[cfg(target_os = "ios")]
                ffi::glGetProgramInfoLog(
                    program as GLuint,
                    max_length as GLsizei,
                    &mut length,
                    log.as_mut_vec().as_mut_ptr() as *mut i8,
                );

                #[cfg(target_os = "android")]
                ffi::glGetProgramInfoLog(
                    program as GLuint,
                    max_length as GLsizei,
                    &mut length,
                    log.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    log.as_mut_vec().set_len(length as usize);
                    log.truncate(length as usize);

                    Ok(log)
                } else {
                    Ok("".to_string())
                }
            }
        }
    }
    pub fn gl_get_renderbuffer_parameteriv(
        &mut self,
        target: RenderBufferTarget,
        name: RenderBufferParamType,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("RenderBufferTarget", "target");

            let mut param_info_name = ParamInfo::new("RenderBufferParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_renderbuffer_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetRenderbufferParameteriv(target as GLenum, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetRenderbufferParameteriv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_shaderiv(&mut self, shader: u32, name: ShaderParamType) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_info_name = ParamInfo::new("ShaderParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_shaderiv".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetShaderiv(shader as GLuint, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetShaderiv(shader as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_shader_info_log(
        &mut self,
        shader: u32,
        max_length: i32,
    ) -> Result<String, Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_info_max_length = ParamInfo::new("i32", "max_length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_shader_info_log".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            param_infos.push(&param_info_max_length);
            param_values.push(&max_length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut log = String::with_capacity(max_length as usize);

                    #[cfg(target_os = "ios")]
                    ffi::glGetShaderInfoLog(
                        shader as GLuint,
                        max_length as GLsizei,
                        &mut length,
                        log.as_mut_vec().as_mut_ptr() as *mut i8,
                    );
                    #[cfg(target_os = "android")]
                    ffi::glGetShaderInfoLog(
                        shader as GLuint,
                        max_length as GLsizei,
                        &mut length,
                        log.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        log.as_mut_vec().set_len(length as usize);
                        log.truncate(length as usize);

                        Ok(log)
                    } else {
                        Ok("".to_string())
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut log = String::with_capacity(max_length as usize);

                #[cfg(target_os = "ios")]
                ffi::glGetShaderInfoLog(
                    shader as GLuint,
                    max_length as GLsizei,
                    &mut length,
                    log.as_mut_vec().as_mut_ptr() as *mut i8,
                );
                #[cfg(target_os = "android")]
                ffi::glGetShaderInfoLog(
                    shader as GLuint,
                    max_length as GLsizei,
                    &mut length,
                    log.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    log.as_mut_vec().set_len(length as usize);
                    log.truncate(length as usize);

                    Ok(log)
                } else {
                    Ok("".to_string())
                }
            }
        }
    }
    pub fn gl_get_shader_precision_format(
        &mut self,
        shader_type: ShaderType,
        precision_type: ShaderPrecisionType,
    ) -> Result<ShaderPrecisionFormat, Error> {
        if self.is_debug() {
            let mut param_info_shader_type = ParamInfo::new("ShaderType", "shader_type");

            let mut param_info_precision_type =
                ParamInfo::new("ShaderPrecisionType", "precision_type");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_shader_precision_format".to_string();

            param_infos.push(&param_info_shader_type);
            param_values.push(&shader_type);

            param_infos.push(&param_info_precision_type);
            param_values.push(&precision_type);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut precision: GLint = 0;
                let mut range: [GLint; 2] = [0, 0];

                unsafe {
                    ffi::glGetShaderPrecisionFormat(
                        shader_type as GLenum,
                        precision_type as GLenum,
                        range.as_mut_ptr(),
                        &mut precision,
                    );
                }

                Ok(ShaderPrecisionFormat {
                    precision: precision,
                    range: range,
                })
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut precision: GLint = 0;
            let mut range: [GLint; 2] = [0, 0];

            unsafe {
                ffi::glGetShaderPrecisionFormat(
                    shader_type as GLenum,
                    precision_type as GLenum,
                    range.as_mut_ptr(),
                    &mut precision,
                );
            }

            Ok(ShaderPrecisionFormat {
                precision: precision,
                range: range,
            })
        }
    }
    pub fn gl_get_shader_source(&mut self, shader: u32, max_length: i32) -> Result<String, Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_info_max_length = ParamInfo::new("i32", "max_length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_shader_source".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            param_infos.push(&param_info_max_length);
            param_values.push(&max_length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut source = String::with_capacity(max_length as usize);

                    ffi::glGetShaderSource(
                        shader as GLuint,
                        max_length as GLsizei,
                        &mut length,
                        source.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        source.as_mut_vec().set_len(length as usize);
                        source.truncate(length as usize);

                        Ok(source)
                    } else {
                        Ok("".to_string())
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut source = String::with_capacity(max_length as usize);

                ffi::glGetShaderSource(
                    shader as GLuint,
                    max_length as GLsizei,
                    &mut length,
                    source.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    source.as_mut_vec().set_len(length as usize);
                    source.truncate(length as usize);

                    Ok(source)
                } else {
                    Ok("".to_string())
                }
            }
        }
    }
    pub fn gl_get_string(&mut self, name: ConstantType) -> Result<String, Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("ConstantType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_string".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let c_str = ffi::glGetString(name as GLenum);
                    //todo : can't guarantee the lifetime, because the memory is allocated by C
                    if !c_str.is_null() {
                        match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                            Ok(s) => Ok(s.to_string()),
                            // TODO: error desc
                            Err(_) => Err(Error {}),
                        }
                    } else {
                        // TODO: Ok is not proper ?
                        Ok("".to_string())
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let c_str = ffi::glGetString(name as GLenum);
                //todo : can't guarantee the lifetime, because the memory is allocated by C
                if !c_str.is_null() {
                    match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                        Ok(s) => Ok(s.to_string()),
                        // TODO: error desc
                        Err(_) => Err(Error {}),
                    }
                } else {
                    // TODO: Ok is not proper ?
                    Ok("".to_string())
                }
            }
        }
    }
    pub fn gl_get_tex_parameterfv(
        &mut self,
        target: TextureTarget,
        name: TextureParamType,
    ) -> Result<f32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_parameterfv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLfloat = 0.0;

                unsafe {
                    ffi::glGetTexParameterfv(target as GLenum, name as GLenum, &mut value);
                }

                Ok(value as f32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetTexParameterfv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as f32)
        }
    }
    pub fn gl_get_tex_parameteriv(
        &mut self,
        target: TextureTarget,
        name: TextureParamType,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetTexParameteriv(target as GLenum, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetTexParameteriv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_uniformfv(&mut self, program: u32, location: i32) -> Result<f32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_uniformfv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLfloat = 0.0;
                unsafe {
                    ffi::glGetUniformfv(program as GLuint, location as GLint, &mut value);
                }

                Ok(value as f32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLfloat = 0.0;
            unsafe {
                ffi::glGetUniformfv(program as GLuint, location as GLint, &mut value);
            }

            Ok(value as f32)
        }
    }
    pub fn gl_get_uniformiv(&mut self, program: u32, location: i32) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_uniformiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetUniformiv(program as GLuint, location as GLint, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetUniformiv(program as GLuint, location as GLint, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_get_uniform_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_name = ParamInfo::new("&str", "name");
            let param_value_name = name.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_uniform_location".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_name);
            param_values.push(&param_value_name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut loc: GLint = 0;

                unsafe {
                    let name_c_str = CString::new(name).unwrap();

                    loc = ffi::glGetUniformLocation(
                        program as GLuint,
                        name_c_str.as_ptr() as *const GLchar,
                    );
                }

                Ok(loc as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut loc: GLint = 0;

            unsafe {
                let name_c_str = CString::new(name).unwrap();

                loc = ffi::glGetUniformLocation(
                    program as GLuint,
                    name_c_str.as_ptr() as *const GLchar,
                );
            }

            Ok(loc as i32)
        }
    }
    pub fn gl_get_vertex_attribfv(
        &mut self,
        index: u32,
        name: VertexAttributeParamType,
    ) -> Result<f32, Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_name = ParamInfo::new("VertexAttributeParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_vertex_attribfv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLfloat = 0.0;

                unsafe {
                    ffi::glGetVertexAttribfv(index as GLuint, name as GLenum, &mut value);
                }

                Ok(value as f32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetVertexAttribfv(index as GLuint, name as GLenum, &mut value);
            }

            Ok(value as f32)
        }
    }
    pub fn gl_get_vertex_attribiv(
        &mut self,
        index: u32,
        name: VertexAttributeParamType,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_name = ParamInfo::new("VertexAttributeParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_vertex_attribiv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut value: GLint = 0;

                unsafe {
                    ffi::glGetVertexAttribiv(index as GLuint, name as GLenum, &mut value);
                }

                Ok(value as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut value: GLint = 0;

            unsafe {
                ffi::glGetVertexAttribiv(index as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)
        }
    }
    pub fn gl_hint(&mut self, target: HintTargetType, mode: HintBehaviorType) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("HintTargetType", "target");

            let mut param_info_mode = ParamInfo::new("HintBehaviorType", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_hint".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glHint(target as GLenum, mode as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glHint(target as GLenum, mode as GLenum) }

            Ok(())
        }
    }
    pub fn gl_is_buffer(&mut self, buffer: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_buffer = ParamInfo::new("u32", "buffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_buffer".to_string();

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsBuffer(buffer as GLuint) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsBuffer(buffer as GLuint) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_enabled(&mut self, feature: FeatureType) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_feature = ParamInfo::new("FeatureType", "feature");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_enabled".to_string();

            param_infos.push(&param_info_feature);
            param_values.push(&feature);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsEnabled(feature as GLenum) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsEnabled(feature as GLenum) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_framebuffer(&mut self, framebuffer: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_framebuffer = ParamInfo::new("u32", "framebuffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_framebuffer".to_string();

            param_infos.push(&param_info_framebuffer);
            param_values.push(&framebuffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsFramebuffer(framebuffer as GLuint) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsFramebuffer(framebuffer as GLuint) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_program(&mut self, program: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_program".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsProgram(program as GLuint) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsProgram(program as GLuint) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_renderbuffer(&mut self, renderbuffer: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_renderbuffer = ParamInfo::new("u32", "renderbuffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_renderbuffer".to_string();

            param_infos.push(&param_info_renderbuffer);
            param_values.push(&renderbuffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsRenderbuffer(renderbuffer as u32) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsRenderbuffer(renderbuffer as u32) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_shader(&mut self, shader: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_shader".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsShader(shader as u32) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsShader(shader as u32) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_is_texture(&mut self, texture: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_texture = ParamInfo::new("u32", "texture");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_texture".to_string();

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let mut res = false;

                unsafe {
                    res = ffi::glIsTexture(texture as u32) == GL_TRUE;
                }

                Ok(res)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let mut res = false;

            unsafe {
                res = ffi::glIsTexture(texture as u32) == GL_TRUE;
            }

            Ok(res)
        }
    }
    pub fn gl_line_width(&mut self, width: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_width = ParamInfo::new("f32", "width");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_line_width".to_string();

            param_infos.push(&param_info_width);
            param_values.push(&width);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glLineWidth(width as GLfloat);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glLineWidth(width as GLfloat);
            }

            Ok(())
        }
    }
    pub fn gl_link_program(&mut self, program: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_link_program".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glLinkProgram(program as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glLinkProgram(program as GLuint) }

            Ok(())
        }
    }
    pub fn gl_pixel_storei(&mut self, name: PackParamType, param: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("PackParamType", "name");

            let mut param_info_param = ParamInfo::new("i32", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_pixel_storei".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glPixelStorei(name as GLenum, param as GLint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glPixelStorei(name as GLenum, param as GLint) }

            Ok(())
        }
    }
    pub fn gl_polygon_offset(&mut self, factor: f32, units: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_factor = ParamInfo::new("f32", "factor");

            let mut param_info_units = ParamInfo::new("f32", "units");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_polygon_offset".to_string();

            param_infos.push(&param_info_factor);
            param_values.push(&factor);

            param_infos.push(&param_info_units);
            param_values.push(&units);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glPolygonOffset(factor as GLfloat, units as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glPolygonOffset(factor as GLfloat, units as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_read_pixels<T>(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &mut [T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_format = ParamInfo::new("PixelFormat", "format");

            let mut param_info_type_ = ParamInfo::new("PixelDataType", "type_");

            let mut param_info_buffer = ParamInfo::new("&mut [T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_read_pixels".to_string();

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glReadPixels(
                        x as GLint,
                        y as GLint,
                        width as GLsizei,
                        height as GLsizei,
                        format as GLenum,
                        type_ as GLenum,
                        buffer.as_mut_ptr() as *mut GLvoid,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glReadPixels(
                    x as GLint,
                    y as GLint,
                    width as GLsizei,
                    height as GLsizei,
                    format as GLenum,
                    type_ as GLenum,
                    buffer.as_mut_ptr() as *mut GLvoid,
                )
            }

            Ok(())
        }
    }
    pub fn gl_release_shader_compiler(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_release_shader_compiler".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glReleaseShaderCompiler() }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glReleaseShaderCompiler() }

            Ok(())
        }
    }
    pub fn gl_renderbuffer_storage(
        &mut self,
        target: RenderBufferTarget,
        internal_format: PixelFormat,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("RenderBufferTarget", "target");

            let mut param_info_internal_format = ParamInfo::new("PixelFormat", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_renderbuffer_storage".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glRenderbufferStorage(
                        target as GLenum,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glRenderbufferStorage(
                    target as GLenum,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                )
            }

            Ok(())
        }
    }
    pub fn gl_sample_coverage(&mut self, value: f32, invert: bool) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_value = ParamInfo::new("f32", "value");

            let mut param_info_invert = ParamInfo::new("bool", "invert");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sample_coverage".to_string();

            param_infos.push(&param_info_value);
            param_values.push(&value);

            param_infos.push(&param_info_invert);
            param_values.push(&invert);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glSampleCoverage(value as GLclampf, invert as GLboolean) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glSampleCoverage(value as GLclampf, invert as GLboolean) }

            Ok(())
        }
    }
    pub fn gl_scissor(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_scissor".to_string();

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glScissor(x as GLint, y as GLint, width as GLsizei, height as GLsizei)
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glScissor(x as GLint, y as GLint, width as GLsizei, height as GLsizei) }

            Ok(())
        }
    }
    pub fn gl_shader_binary<T>(
        &mut self,
        shaders: &[u32],
        data_format: GLenum,
        data: &[T],
        length: i32,
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_shaders = ParamInfo::new("&[u32]", "shaders");
            let param_value_shaders = shaders.to_vec();

            let mut param_info_data_format = ParamInfo::new("GLenum", "data_format");

            let mut param_info_data = ParamInfo::new("&[T]", "data");
            let param_value_data = data.to_vec();

            let mut param_info_length = ParamInfo::new("i32", "length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_shader_binary".to_string();

            param_infos.push(&param_info_shaders);
            param_values.push(&param_value_shaders);

            param_infos.push(&param_info_data_format);
            param_values.push(&data_format);

            param_infos.push(&param_info_data);
            param_values.push(&param_value_data);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glShaderBinary(
                        shaders.len() as GLsizei,
                        shaders.as_ptr(),
                        data_format,
                        data.as_ptr() as *const GLvoid,
                        length as GLsizei,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glShaderBinary(
                    shaders.len() as GLsizei,
                    shaders.as_ptr(),
                    data_format,
                    data.as_ptr() as *const GLvoid,
                    length as GLsizei,
                )
            }

            Ok(())
        }
    }
    pub fn gl_shader_source(&mut self, shader: u32, source: &str) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_shader = ParamInfo::new("u32", "shader");

            let mut param_info_source = ParamInfo::new("&str", "source");
            let param_value_source = source.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_shader_source".to_string();

            param_infos.push(&param_info_shader);
            param_values.push(&shader);

            param_infos.push(&param_info_source);
            param_values.push(&param_value_source);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let length: GLsizei = source.len() as GLsizei;

                    ffi::glShaderSource(
                        shader as GLuint,
                        1,
                        &(source.as_ptr() as *const GLchar),
                        &length,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let length: GLsizei = source.len() as GLsizei;

                ffi::glShaderSource(
                    shader as GLuint,
                    1,
                    &(source.as_ptr() as *const GLchar),
                    &length,
                )
            }

            Ok(())
        }
    }
    pub fn gl_stencil_func(&mut self, func: FuncType, ref_: i32, mask: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_func = ParamInfo::new("FuncType", "func");

            let mut param_info_ref_ = ParamInfo::new("i32", "ref_");

            let mut param_info_mask = ParamInfo::new("u32", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_func".to_string();

            param_infos.push(&param_info_func);
            param_values.push(&func);

            param_infos.push(&param_info_ref_);
            param_values.push(&ref_);

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glStencilFunc(func as GLenum, ref_ as GLint, mask as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glStencilFunc(func as GLenum, ref_ as GLint, mask as GLuint) }

            Ok(())
        }
    }
    pub fn gl_stencil_func_separate(
        &mut self,
        face: FaceMode,
        func: FuncType,
        ref_: i32,
        mask: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_face = ParamInfo::new("FaceMode", "face");

            let mut param_info_func = ParamInfo::new("FuncType", "func");

            let mut param_info_ref_ = ParamInfo::new("i32", "ref_");

            let mut param_info_mask = ParamInfo::new("u32", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_func_separate".to_string();

            param_infos.push(&param_info_face);
            param_values.push(&face);

            param_infos.push(&param_info_func);
            param_values.push(&func);

            param_infos.push(&param_info_ref_);
            param_values.push(&ref_);

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glStencilFuncSeparate(
                        face as GLenum,
                        func as GLenum,
                        ref_ as GLint,
                        mask as GLuint,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glStencilFuncSeparate(
                    face as GLenum,
                    func as GLenum,
                    ref_ as GLint,
                    mask as GLuint,
                )
            }

            Ok(())
        }
    }
    pub fn gl_stencil_mask(&mut self, mask: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mask = ParamInfo::new("u32", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_mask".to_string();

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glStencilMask(mask as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glStencilMask(mask as GLuint) }

            Ok(())
        }
    }
    pub fn gl_stencil_mask_separate(&mut self, face: FaceMode, mask: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_face = ParamInfo::new("FaceMode", "face");

            let mut param_info_mask = ParamInfo::new("u32", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_mask_separate".to_string();

            param_infos.push(&param_info_face);
            param_values.push(&face);

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glStencilMaskSeparate(face as GLenum, mask as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glStencilMaskSeparate(face as GLenum, mask as GLuint) }

            Ok(())
        }
    }
    pub fn gl_stencil_op(
        &mut self,
        s_fail: ActionType,
        dp_fail: ActionType,
        dp_pass: ActionType,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_s_fail = ParamInfo::new("ActionType", "s_fail");

            let mut param_info_dp_fail = ParamInfo::new("ActionType", "dp_fail");

            let mut param_info_dp_pass = ParamInfo::new("ActionType", "dp_pass");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_op".to_string();

            param_infos.push(&param_info_s_fail);
            param_values.push(&s_fail);

            param_infos.push(&param_info_dp_fail);
            param_values.push(&dp_fail);

            param_infos.push(&param_info_dp_pass);
            param_values.push(&dp_pass);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glStencilOp(s_fail as GLenum, dp_fail as GLenum, dp_pass as GLenum) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glStencilOp(s_fail as GLenum, dp_fail as GLenum, dp_pass as GLenum) }

            Ok(())
        }
    }
    pub fn gl_stencil_op_separate(
        &mut self,
        face: FaceMode,
        s_fail: ActionType,
        dp_fail: ActionType,
        dp_pass: ActionType,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_face = ParamInfo::new("FaceMode", "face");

            let mut param_info_s_fail = ParamInfo::new("ActionType", "s_fail");

            let mut param_info_dp_fail = ParamInfo::new("ActionType", "dp_fail");

            let mut param_info_dp_pass = ParamInfo::new("ActionType", "dp_pass");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_stencil_op_separate".to_string();

            param_infos.push(&param_info_face);
            param_values.push(&face);

            param_infos.push(&param_info_s_fail);
            param_values.push(&s_fail);

            param_infos.push(&param_info_dp_fail);
            param_values.push(&dp_fail);

            param_infos.push(&param_info_dp_pass);
            param_values.push(&dp_pass);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glStencilOpSeparate(
                        face as GLenum,
                        s_fail as GLenum,
                        dp_fail as GLenum,
                        dp_pass as GLenum,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glStencilOpSeparate(
                    face as GLenum,
                    s_fail as GLenum,
                    dp_fail as GLenum,
                    dp_pass as GLenum,
                )
            }

            Ok(())
        }
    }
    pub fn gl_tex_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        internal_format: GLint,
        width: i32,
        height: i32,
        border: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("i32", "level");

            let mut param_info_internal_format = ParamInfo::new("GLint", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_border = ParamInfo::new("i32", "border");

            let mut param_info_format = ParamInfo::new("PixelFormat", "format");

            let mut param_info_type_ = ParamInfo::new("PixelDataType", "type_");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_border);
            param_values.push(&border);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glTexImage2D(
                        target as GLenum,
                        level as GLint,
                        internal_format,
                        width as GLsizei,
                        height as GLsizei,
                        border as GLint,
                        format as GLenum,
                        type_ as GLenum,
                        buffer.as_ptr() as *const GLvoid,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glTexImage2D(
                    target as GLenum,
                    level as GLint,
                    internal_format,
                    width as GLsizei,
                    height as GLsizei,
                    border as GLint,
                    format as GLenum,
                    type_ as GLenum,
                    buffer.as_ptr() as *const GLvoid,
                )
            }

            Ok(())
        }
    }
    pub fn gl_tex_parameterf(
        &mut self,
        target: TextureBindTarget,
        name: TextureParamType,
        value: f32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_info_value = ParamInfo::new("f32", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameterf".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glTexParameterf(target as GLenum, name as GLenum, value as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glTexParameterf(target as GLenum, name as GLenum, value as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_tex_parameterfv(
        &mut self,
        target: TextureBindTarget,
        name: TextureParamType,
    ) -> Result<f32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameterfv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let res: GLfloat = 0.0;
                unsafe { ffi::glTexParameterfv(target as GLenum, name as GLenum, &res) }

                Ok(res as f32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let res: GLfloat = 0.0;
            unsafe { ffi::glTexParameterfv(target as GLenum, name as GLenum, &res) }

            Ok(res as f32)
        }
    }
    pub fn gl_tex_parameteri(
        &mut self,
        target: TextureBindTarget,
        name: TextureParamType,
        value: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_info_value = ParamInfo::new("GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameteri".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glTexParameteri(target as GLenum, name as GLenum, value) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glTexParameteri(target as GLenum, name as GLenum, value) }

            Ok(())
        }
    }
    pub fn gl_tex_parameteriv(
        &mut self,
        target: TextureBindTarget,
        name: TextureParamType,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureBindTarget", "target");

            let mut param_info_name = ParamInfo::new("TextureParamType", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                let res: GLint = 0;

                unsafe { ffi::glTexParameteriv(target as GLenum, name as GLenum, &res) }

                Ok(res as i32)
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            let res: GLint = 0;

            unsafe { ffi::glTexParameteriv(target as GLenum, name as GLenum, &res) }

            Ok(res as i32)
        }
    }
    pub fn gl_tex_sub_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("i32", "level");

            let mut param_info_x_offset = ParamInfo::new("i32", "x_offset");

            let mut param_info_y_offset = ParamInfo::new("i32", "y_offset");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_format = ParamInfo::new("PixelFormat", "format");

            let mut param_info_type_ = ParamInfo::new("PixelDataType", "type_");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_sub_image_2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_x_offset);
            param_values.push(&x_offset);

            param_infos.push(&param_info_y_offset);
            param_values.push(&y_offset);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glTexSubImage2D(
                        target as GLenum,
                        level as GLint,
                        x_offset as GLint,
                        y_offset as GLint,
                        width as GLsizei,
                        height as GLsizei,
                        format as GLenum,
                        type_ as GLenum,
                        buffer.as_ptr() as *const GLvoid,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glTexSubImage2D(
                    target as GLenum,
                    level as GLint,
                    x_offset as GLint,
                    y_offset as GLint,
                    width as GLsizei,
                    height as GLsizei,
                    format as GLenum,
                    type_ as GLenum,
                    buffer.as_ptr() as *const GLvoid,
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform1f(&mut self, location: i32, x: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1f".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUniform1f(location as GLint, x as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform1f(location as GLint, x as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_uniform1fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform1fv(location as GLint, values.len() as GLsizei, values.as_ptr())
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform1fv(location as GLint, values.len() as GLsizei, values.as_ptr())
            }

            Ok(())
        }
    }
    pub fn gl_uniform1i(&mut self, location: i32, x: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1i".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUniform1i(location as GLint, x as GLint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform1i(location as GLint, x as GLint) }

            Ok(())
        }
    }
    pub fn gl_uniform1iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[i32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1iv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform1iv(location as GLint, values.len() as GLsizei, values.as_ptr())
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform1iv(location as GLint, values.len() as GLsizei, values.as_ptr())
            }

            Ok(())
        }
    }
    pub fn gl_uniform2f(&mut self, location: i32, x: f32, y: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2f".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUniform2f(location as GLint, x as GLfloat, y as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform2f(location as GLint, x as GLfloat, y as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_uniform2fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform2fv(
                        location as GLint,
                        (values.len() / 2) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform2fv(
                    location as GLint,
                    (values.len() / 2) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform2i(&mut self, location: i32, x: i32, y: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2i".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUniform2i(location as GLint, x as GLint, y as GLint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform2i(location as GLint, x as GLint, y as GLint) }

            Ok(())
        }
    }
    pub fn gl_uniform2iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[i32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2iv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform2iv(
                        location as GLint,
                        (values.len() / 2) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform2iv(
                    location as GLint,
                    (values.len() / 2) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform3f(&mut self, location: i32, x: f32, y: f32, z: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_info_z = ParamInfo::new("f32", "z");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3f".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform3f(location as GLint, x as GLfloat, y as GLfloat, z as GLfloat)
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform3f(location as GLint, x as GLfloat, y as GLfloat, z as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_uniform3fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform3fv(
                        location as GLint,
                        (values.len() / 3) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform3fv(
                    location as GLint,
                    (values.len() / 3) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform3i(&mut self, location: i32, x: i32, y: i32, z: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_info_z = ParamInfo::new("i32", "z");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3i".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUniform3i(location as GLint, x as GLint, y as GLint, z as GLint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUniform3i(location as GLint, x as GLint, y as GLint, z as GLint) }

            Ok(())
        }
    }
    pub fn gl_uniform3iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[i32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3iv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform3iv(
                        location as GLint,
                        (values.len() / 3) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform3iv(
                    location as GLint,
                    (values.len() / 3) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform4f(
        &mut self,
        location: i32,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_info_z = ParamInfo::new("f32", "z");

            let mut param_info_w = ParamInfo::new("f32", "w");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4f".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            param_infos.push(&param_info_w);
            param_values.push(&w);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform4f(
                        location as GLint,
                        x as GLfloat,
                        y as GLfloat,
                        z as GLfloat,
                        w as GLfloat,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform4f(
                    location as GLint,
                    x as GLfloat,
                    y as GLfloat,
                    z as GLfloat,
                    w as GLfloat,
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform4fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform4fv(
                        location as GLint,
                        (values.len() / 4) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform4fv(
                    location as GLint,
                    (values.len() / 4) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform4i(
        &mut self,
        location: i32,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_info_z = ParamInfo::new("i32", "z");

            let mut param_info_w = ParamInfo::new("i32", "w");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4i".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            param_infos.push(&param_info_w);
            param_values.push(&w);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform4i(
                        location as GLint,
                        x as GLint,
                        y as GLint,
                        z as GLint,
                        w as GLint,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform4i(
                    location as GLint,
                    x as GLint,
                    y as GLint,
                    z as GLint,
                    w as GLint,
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform4iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_values = ParamInfo::new("&[i32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4iv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniform4iv(
                        location as GLint,
                        (values.len() / 4) as GLsizei,
                        values.as_ptr(),
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniform4iv(
                    location as GLint,
                    (values.len() / 4) as GLsizei,
                    values.as_ptr(),
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform_matrix2fv(
        &mut self,
        location: i32,
        transpose: bool,
        values: &[f32],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix2fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniformMatrix2fv(
                        location as i32,
                        (values.len() / 2 * 2) as GLsizei,
                        transpose as GLboolean,
                        values.as_ptr() as *const GLfloat,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniformMatrix2fv(
                    location as i32,
                    (values.len() / 2 * 2) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform_matrix3fv(
        &mut self,
        location: i32,
        transpose: bool,
        values: &[f32],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix3fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniformMatrix3fv(
                        location as GLint,
                        (values.len() / 3 * 3) as GLsizei,
                        transpose as GLboolean,
                        values.as_ptr() as *const GLfloat,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniformMatrix3fv(
                    location as GLint,
                    (values.len() / 3 * 3) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())
        }
    }
    pub fn gl_uniform_matrix4fv(
        &mut self,
        location: i32,
        transpose: bool,
        values: &[f32],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix4fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glUniformMatrix4fv(
                        location as GLint,
                        (values.len() / 4 * 4) as GLsizei,
                        transpose as GLboolean,
                        values.as_ptr() as *const GLfloat,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glUniformMatrix4fv(
                    location as GLint,
                    (values.len() / 4 * 4) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())
        }
    }
    pub fn gl_use_program(&mut self, program: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_use_program".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glUseProgram(program as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glUseProgram(program as GLuint) }

            Ok(())
        }
    }
    pub fn gl_validate_program(&mut self, program: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_validate_program".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glValidateProgram(program as GLuint) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glValidateProgram(program as GLuint) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib1f(&mut self, index: u32, x: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib1f".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib1f(index as GLuint, x as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib1f(index as GLuint, x as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib1fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib1fv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib1fv(index as GLuint, values.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib1fv(index as GLuint, values.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib2f(&mut self, index: u32, x: f32, y: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib2f".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib2f(index as GLuint, x as GLfloat, y as GLfloat) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib2f(index as GLuint, x as GLfloat, y as GLfloat) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib2fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib2fv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib2fv(index as GLuint, values.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib2fv(index as GLuint, values.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib3f(&mut self, index: u32, x: f32, y: f32, z: f32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_info_z = ParamInfo::new("f32", "z");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib3f".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glVertexAttrib3f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat)
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glVertexAttrib3f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat)
            }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib3fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib3fv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib3fv(index as GLuint, values.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib3fv(index as GLuint, values.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib4f(
        &mut self,
        index: u32,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("f32", "x");

            let mut param_info_y = ParamInfo::new("f32", "y");

            let mut param_info_z = ParamInfo::new("f32", "z");

            let mut param_info_w = ParamInfo::new("f32", "w");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib4f".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            param_infos.push(&param_info_w);
            param_values.push(&w);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glVertexAttrib4f(
                        index as GLuint,
                        x as GLfloat,
                        y as GLfloat,
                        z as GLfloat,
                        w as GLfloat,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glVertexAttrib4f(
                    index as GLuint,
                    x as GLfloat,
                    y as GLfloat,
                    z as GLfloat,
                    w as GLfloat,
                )
            }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib4fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_values = ParamInfo::new("&[f32]", "values");
            let param_value_values = values.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib4fv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_values);
            param_values.push(&param_value_values);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe { ffi::glVertexAttrib4fv(index as GLuint, values.as_ptr()) }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glVertexAttrib4fv(index as GLuint, values.as_ptr()) }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib_pointer<T>(
        &mut self,
        index: u32,
        size: i32,
        type_: DataType,
        normalized: bool,
        stride: i32,
        buffer: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_size = ParamInfo::new("i32", "size");

            let mut param_info_type_ = ParamInfo::new("DataType", "type_");

            let mut param_info_normalized = ParamInfo::new("bool", "normalized");

            let mut param_info_stride = ParamInfo::new("i32", "stride");

            let mut param_info_buffer = ParamInfo::new("&[T]", "buffer");
            let param_value_buffer = buffer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_pointer".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_normalized);
            param_values.push(&normalized);

            param_infos.push(&param_info_stride);
            param_values.push(&stride);

            param_infos.push(&param_info_buffer);
            param_values.push(&param_value_buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    if buffer.len() == 0 {
                        ffi::glVertexAttribPointer(
                            index as GLuint,
                            size as GLint,
                            type_ as GLenum,
                            normalized as GLboolean,
                            stride as GLsizei,
                            &0 as *const i32 as *const GLvoid,
                        )
                    } else {
                        ffi::glVertexAttribPointer(
                            index as GLuint,
                            size as GLint,
                            type_ as GLenum,
                            normalized as GLboolean,
                            stride as GLsizei,
                            buffer.as_ptr() as *const GLvoid,
                        )
                    }
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                if buffer.len() == 0 {
                    ffi::glVertexAttribPointer(
                        index as GLuint,
                        size as GLint,
                        type_ as GLenum,
                        normalized as GLboolean,
                        stride as GLsizei,
                        &0 as *const i32 as *const GLvoid,
                    )
                } else {
                    ffi::glVertexAttribPointer(
                        index as GLuint,
                        size as GLint,
                        type_ as GLenum,
                        normalized as GLboolean,
                        stride as GLsizei,
                        buffer.as_ptr() as *const GLvoid,
                    )
                }
            }

            Ok(())
        }
    }
    pub fn gl_vertex_attrib_pointer_offset(
        &mut self,
        index: u32,
        size: i32,
        type_: DataType,
        normalized: bool,
        stride: i32,
        offset: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_size = ParamInfo::new("i32", "size");

            let mut param_info_type_ = ParamInfo::new("DataType", "type_");

            let mut param_info_normalized = ParamInfo::new("bool", "normalized");

            let mut param_info_stride = ParamInfo::new("i32", "stride");

            let mut param_info_offset = ParamInfo::new("u32", "offset");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_pointer_offset".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_normalized);
            param_values.push(&normalized);

            param_infos.push(&param_info_stride);
            param_values.push(&stride);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glVertexAttribPointer(
                        index as GLuint,
                        size as GLint,
                        type_ as GLenum,
                        normalized as GLboolean,
                        stride as i32,
                        offset as *const GLvoid,
                    )
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                ffi::glVertexAttribPointer(
                    index as GLuint,
                    size as GLint,
                    type_ as GLenum,
                    normalized as GLboolean,
                    stride as i32,
                    offset as *const GLvoid,
                )
            }

            Ok(())
        }
    }
    pub fn gl_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_x = ParamInfo::new("i32", "x");

            let mut param_info_y = ParamInfo::new("i32", "y");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_viewport".to_string();

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    ffi::glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei)
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe { ffi::glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei) }

            Ok(())
        }
    }
    pub fn gl_read_buffer(&mut self, mode: ColorBufferMode) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("ColorBufferMode", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_read_buffer".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glReadBuffer(mode as GLenum);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
                        self.glReadBuffer_ptr,
                    )(mode as GLenum);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glReadBuffer(mode as GLenum);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum) -> ()>(self.glReadBuffer_ptr)(
                    mode as GLenum,
                );
            }
            Ok(())
        }
    }
    pub fn gl_draw_buffers(&mut self, bufs: &[ColorBufferMode]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_bufs = ParamInfo::new("&[ColorBufferMode]", "bufs");
            let param_value_bufs = bufs.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_buffers".to_string();

            param_infos.push(&param_info_bufs);
            param_values.push(&param_value_bufs);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let n: GLsizei = bufs.len() as i32;
                    #[cfg(target_os = "ios")]
                    ffi::glDrawBuffers(n, bufs.as_ptr() as *const GLenum);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(
                        self.glDrawBuffers_ptr,
                    )(n, bufs.as_ptr() as *const GLenum);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let n: GLsizei = bufs.len() as i32;
                #[cfg(target_os = "ios")]
                ffi::glDrawBuffers(n, bufs.as_ptr() as *const GLenum);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(
                    self.glDrawBuffers_ptr,
                )(n, bufs.as_ptr() as *const GLenum);
            }
            Ok(())
        }
    }
    pub fn gl_unmap_buffer(&mut self, target: BufferObjectTarget) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_unmap_buffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glUnmapBuffer(target as GLenum);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(
                        self.glUnmapBuffer_ptr,
                    )(target as GLenum);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glUnmapBuffer(target as GLenum);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(
                    self.glUnmapBuffer_ptr,
                )(target as GLenum);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
    pub fn gl_copy_buffer_sub_data(
        &mut self,
        read_target: BufferObjectTarget,
        write_target: BufferObjectTarget,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_read_target = ParamInfo::new("BufferObjectTarget", "read_target");

            let mut param_info_write_target = ParamInfo::new("BufferObjectTarget", "write_target");

            let mut param_info_read_offset = ParamInfo::new("GLintptr", "read_offset");

            let mut param_info_write_offset = ParamInfo::new("GLintptr", "write_offset");

            let mut param_info_size = ParamInfo::new("GLsizeiptr", "size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_copy_buffer_sub_data".to_string();

            param_infos.push(&param_info_read_target);
            param_values.push(&read_target);

            param_infos.push(&param_info_write_target);
            param_values.push(&write_target);

            param_infos.push(&param_info_read_offset);
            param_values.push(&read_offset);

            param_infos.push(&param_info_write_offset);
            param_values.push(&write_offset);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glCopyBufferSubData(
                        read_target as GLenum,
                        write_target as GLenum,
                        read_offset,
                        write_offset,
                        size,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> (),
                    >(self.glCopyBufferSubData_ptr)(
                        read_target as GLenum,
                        write_target as GLenum,
                        read_offset,
                        write_offset,
                        size,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glCopyBufferSubData(
                    read_target as GLenum,
                    write_target as GLenum,
                    read_offset,
                    write_offset,
                    size,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> (),
                >(self.glCopyBufferSubData_ptr)(
                    read_target as GLenum,
                    write_target as GLenum,
                    read_offset,
                    write_offset,
                    size,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_buffer_pointerv<T>(
        &mut self,
        target: BufferObjectTarget,
        pname: BufferMapTarget,
        params: *mut *mut GLvoid,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");

            let mut param_info_pname = ParamInfo::new("BufferMapTarget", "pname");

            let mut param_info_params = ParamInfo::new("*mut *mut GLvoid", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_buffer_pointerv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetBufferPointerv(target as GLenum, pname as GLenum, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, *mut *mut GLvoid) -> (),
                    >(self.glGetBufferPointerv_ptr)(
                        target as GLenum, pname as GLenum, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetBufferPointerv(target as GLenum, pname as GLenum, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut *mut GLvoid) -> ()>(
                    self.glGetBufferPointerv_ptr,
                )(target as GLenum, pname as GLenum, params);
            }
            Ok(())
        }
    }
//    pub fn gl_map_buffer_range<'a, T>(
//        &mut self,
//        target: BufferObjectTarget,
//        offset: GLintptr,
//        length: GLsizeiptr,
//        access: MappingBit,
//    ) -> Result<&'a [T], Error>
//    where
//        T: std::fmt::Debug + Clone,
//    {
//        if self.is_debug() {
//            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");
//
//            let mut param_info_offset = ParamInfo::new("GLintptr", "offset");
//
//            let mut param_info_length = ParamInfo::new("GLsizeiptr", "length");
//
//            let mut param_info_access = ParamInfo::new("MappingBit", "access");
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_map_buffer_range".to_string();
//
//            param_infos.push(&param_info_target);
//            param_values.push(&target);
//
//            param_infos.push(&param_info_offset);
//            param_values.push(&offset);
//
//            param_infos.push(&param_info_length);
//            param_values.push(&length);
//
//            param_infos.push(&param_info_access);
//            param_values.push(&access);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    #[cfg(target_os = "ios")]
//                    let ptr =
//                        ffi::glMapBufferRange(target as GLenum, offset, length, access as GLenum);
//                    #[cfg(target_os = "android")]
//                    let ptr = std::mem::transmute::<
//                        _,
//                        extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut GLvoid,
//                    >(self.glMapBufferRange_ptr)(
//                        target as GLenum,
//                        offset,
//                        length,
//                        access as GLenum,
//                    );
//
//                    let count = length as usize / std::mem::size_of::<T>();
//                    Ok(slice::from_raw_parts_mut(ptr as *mut T, count as usize))
//                }
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                #[cfg(target_os = "ios")]
//                let ptr = ffi::glMapBufferRange(target as GLenum, offset, length, access as GLenum);
//                #[cfg(target_os = "android")]
//                let ptr = std::mem::transmute::<
//                    _,
//                    extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut GLvoid,
//                >(self.glMapBufferRange_ptr)(
//                    target as GLenum, offset, length, access as GLenum
//                );
//
//                let count = length as usize / std::mem::size_of::<T>();
//                Ok(slice::from_raw_parts_mut(ptr as *mut T, count as usize))
//            }
//        }
//    }
    pub fn gl_flush_mapped_buffer_range(
        &mut self,
        target: BufferObjectTarget,
        offset: i32,
        length: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");

            let mut param_info_offset = ParamInfo::new("i32", "offset");

            let mut param_info_length = ParamInfo::new("i32", "length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_flush_mapped_buffer_range".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glFlushMappedBufferRange(
                        target as GLenum,
                        offset as GLintptr,
                        length as GLsizeiptr,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(
                        self.glFlushMappedBufferRange_ptr,
                    )(
                        target as GLenum, offset as GLintptr, length as GLsizeiptr
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glFlushMappedBufferRange(
                    target as GLenum,
                    offset as GLintptr,
                    length as GLsizeiptr,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(
                    self.glFlushMappedBufferRange_ptr,
                )(target as GLenum, offset as GLintptr, length as GLsizeiptr);
            }
            Ok(())
        }
    }
    pub fn gl_bind_buffer_range(
        &mut self,
        target: BufferObjectTarget,
        index: u32,
        buffer: u32,
        offset: i32,
        size: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_buffer = ParamInfo::new("u32", "buffer");

            let mut param_info_offset = ParamInfo::new("i32", "offset");

            let mut param_info_size = ParamInfo::new("i32", "size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_buffer_range".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindBufferRange(
                        target as GLenum,
                        index as GLuint,
                        buffer as GLuint,
                        offset as GLintptr,
                        size as GLsizeiptr,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> (),
                    >(self.glBindBufferRange_ptr)(
                        target as GLenum,
                        index as GLuint,
                        buffer as GLuint,
                        offset as GLintptr,
                        size as GLsizeiptr,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindBufferRange(
                    target as GLenum,
                    index as GLuint,
                    buffer as GLuint,
                    offset as GLintptr,
                    size as GLsizeiptr,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> (),
                >(self.glBindBufferRange_ptr)(
                    target as GLenum,
                    index as GLuint,
                    buffer as GLuint,
                    offset as GLintptr,
                    size as GLsizeiptr,
                );
            }
            Ok(())
        }
    }
    pub fn gl_bind_buffer_base(
        &mut self,
        target: BufferObjectTarget,
        index: u32,
        buffer: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("BufferObjectTarget", "target");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_buffer = ParamInfo::new("u32", "buffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_buffer_base".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindBufferBase(target as GLenum, index as GLuint, buffer as GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
                        self.glBindBufferBase_ptr,
                    )(target as GLenum, index as GLuint, buffer as GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindBufferBase(target as GLenum, index as GLuint, buffer as GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
                    self.glBindBufferBase_ptr,
                )(target as GLenum, index as GLuint, buffer as GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_clear_bufferiv(
        &mut self,
        buffer: GLenum,
        draw_buffer: i32,
        value: &[GLint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buffer = ParamInfo::new("GLenum", "buffer");

            let mut param_info_draw_buffer = ParamInfo::new("i32", "draw_buffer");

            let mut param_info_value = ParamInfo::new("&[GLint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_bufferiv".to_string();

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_draw_buffer);
            param_values.push(&draw_buffer);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glClearBufferiv(
                        buffer,
                        draw_buffer as GLint,
                        value.as_ptr() as *const GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(
                        self.glClearBufferiv_ptr,
                    )(
                        buffer, draw_buffer as GLint, value.as_ptr() as *const GLint
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glClearBufferiv(buffer, draw_buffer as GLint, value.as_ptr() as *const GLint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(
                    self.glClearBufferiv_ptr,
                )(buffer, draw_buffer as GLint, value.as_ptr() as *const GLint);
            }
            Ok(())
        }
    }
    pub fn gl_clear_bufferuiv(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buffer = ParamInfo::new("GLenum", "buffer");

            let mut param_info_drawbuffer = ParamInfo::new("i32", "drawbuffer");

            let mut param_info_value = ParamInfo::new("&[GLuint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_bufferuiv".to_string();

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_drawbuffer);
            param_values.push(&drawbuffer);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glClearBufferuiv(
                        buffer,
                        drawbuffer as i32,
                        value.as_ptr() as *const GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(
                        self.glClearBufferuiv_ptr,
                    )(
                        buffer, drawbuffer as i32, value.as_ptr() as *const GLuint
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glClearBufferuiv(buffer, drawbuffer as i32, value.as_ptr() as *const GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(
                    self.glClearBufferuiv_ptr,
                )(buffer, drawbuffer as i32, value.as_ptr() as *const GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_clear_bufferfv(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buffer = ParamInfo::new("GLenum", "buffer");

            let mut param_info_drawbuffer = ParamInfo::new("i32", "drawbuffer");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_bufferfv".to_string();

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_drawbuffer);
            param_values.push(&drawbuffer);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glClearBufferfv(
                        buffer,
                        drawbuffer as i32,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(
                        self.glClearBufferfv_ptr,
                    )(
                        buffer, drawbuffer as i32, value.as_ptr() as *const GLfloat
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glClearBufferfv(buffer, drawbuffer as i32, value.as_ptr() as *const GLfloat);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(
                    self.glClearBufferfv_ptr,
                )(buffer, drawbuffer as i32, value.as_ptr() as *const GLfloat);
            }
            Ok(())
        }
    }
    pub fn gl_clear_bufferfi(
        &mut self,
        buffer: GLenum,
        drawbuffer: i32,
        depth: GLfloat,
        stencil: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buffer = ParamInfo::new("GLenum", "buffer");

            let mut param_info_drawbuffer = ParamInfo::new("i32", "drawbuffer");

            let mut param_info_depth = ParamInfo::new("GLfloat", "depth");

            let mut param_info_stencil = ParamInfo::new("GLint", "stencil");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_clear_bufferfi".to_string();

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_drawbuffer);
            param_values.push(&drawbuffer);

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            param_infos.push(&param_info_stencil);
            param_values.push(&stencil);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glClearBufferfi(buffer, drawbuffer as i32, depth, stencil);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(
                        self.glClearBufferfi_ptr,
                    )(buffer, drawbuffer as i32, depth, stencil);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glClearBufferfi(buffer, drawbuffer as i32, depth, stencil);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(
                    self.glClearBufferfi_ptr,
                )(buffer, drawbuffer as i32, depth, stencil);
            }
            Ok(())
        }
    }
    pub fn gl_get_buffer_parameteri64v(
        &mut self,
        target: GLenum,
        pname: GLenum,
    ) -> Result<i64, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_buffer_parameteri64v".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut params = 0 as GLint64;
                    #[cfg(target_os = "ios")]
                    ffi::glGetBufferParameteri64v(target as GLenum, pname as GLenum, &mut params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(
                        self.glGetBufferParameteri64v_ptr,
                    )(target as GLenum, pname as GLenum, &mut params);

                    Ok(params)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut params = 0 as GLint64;
                #[cfg(target_os = "ios")]
                ffi::glGetBufferParameteri64v(target as GLenum, pname as GLenum, &mut params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(
                    self.glGetBufferParameteri64v_ptr,
                )(target as GLenum, pname as GLenum, &mut params);

                Ok(params)
            }
        }
    }
//    pub fn gl_tex_image_3d(
//        &mut self,
//        target: TextureTarget,
//        level: i32,
//        internal_format: i32,
//        width: i32,
//        height: i32,
//        depth: GLsizei,
//        border: i32,
//        format: PixelDataFormat,
//        type_: GLenum,
//        opt_data: Option<&[u8]>,
//    ) -> Result<(), Error> {
//        if self.is_debug() {
//            let mut param_info_target = ParamInfo::new("TextureTarget", "target");
//
//            let mut param_info_level = ParamInfo::new("i32", "level");
//
//            let mut param_info_internal_format = ParamInfo::new("i32", "internal_format");
//
//            let mut param_info_width = ParamInfo::new("i32", "width");
//
//            let mut param_info_height = ParamInfo::new("i32", "height");
//
//            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");
//
//            let mut param_info_border = ParamInfo::new("i32", "border");
//
//            let mut param_info_format = ParamInfo::new("PixelDataFormat", "format");
//
//            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");
//
//            let mut param_info_opt_data = ParamInfo::new("Option<&[u8]>", "opt_data");
//            let param_value_opt_data = opt_data.to_vec();
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_tex_image_3d".to_string();
//
//            param_infos.push(&param_info_target);
//            param_values.push(&target);
//
//            param_infos.push(&param_info_level);
//            param_values.push(&level);
//
//            param_infos.push(&param_info_internal_format);
//            param_values.push(&internal_format);
//
//            param_infos.push(&param_info_width);
//            param_values.push(&width);
//
//            param_infos.push(&param_info_height);
//            param_values.push(&height);
//
//            param_infos.push(&param_info_depth);
//            param_values.push(&depth);
//
//            param_infos.push(&param_info_border);
//            param_values.push(&border);
//
//            param_infos.push(&param_info_format);
//            param_values.push(&format);
//
//            param_infos.push(&param_info_type_);
//            param_values.push(&type_);
//
//            param_infos.push(&param_info_opt_data);
//            param_values.push(&param_value_opt_data);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    let pdata = match opt_data {
//                        Some(data) => mem::transmute(data.as_ptr()),
//                        None => ptr::null(),
//                    };
//                    #[cfg(target_os = "ios")]
//                    ffi::glTexImage3D(
//                        target as GLenum,
//                        level as GLint,
//                        internal_format as GLint,
//                        width as GLsizei,
//                        height as GLsizei,
//                        depth,
//                        border as GLint,
//                        format as GLenum,
//                        type_,
//                        pdata,
//                    );
//                    #[cfg(target_os = "android")]
//                    std::mem::transmute::<
//                        _,
//                        extern "system" fn(
//                            GLenum,
//                            GLint,
//                            GLint,
//                            GLsizei,
//                            GLsizei,
//                            GLsizei,
//                            GLint,
//                            GLenum,
//                            GLenum,
//                            *const GLvoid,
//                        ) -> (),
//                    >(self.glTexImage3D_ptr)(
//                        target as GLenum,
//                        level as GLint,
//                        internal_format as GLint,
//                        width as GLsizei,
//                        height as GLsizei,
//                        depth,
//                        border as GLint,
//                        format as GLenum,
//                        type_,
//                        pdata,
//                    );
//                }
//                Ok(())
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                let pdata = match opt_data {
//                    Some(data) => mem::transmute(data.as_ptr()),
//                    None => ptr::null(),
//                };
//                #[cfg(target_os = "ios")]
//                ffi::glTexImage3D(
//                    target as GLenum,
//                    level as GLint,
//                    internal_format as GLint,
//                    width as GLsizei,
//                    height as GLsizei,
//                    depth,
//                    border as GLint,
//                    format as GLenum,
//                    type_,
//                    pdata,
//                );
//                #[cfg(target_os = "android")]
//                std::mem::transmute::<
//                    _,
//                    extern "system" fn(
//                        GLenum,
//                        GLint,
//                        GLint,
//                        GLsizei,
//                        GLsizei,
//                        GLsizei,
//                        GLint,
//                        GLenum,
//                        GLenum,
//                        *const GLvoid,
//                    ) -> (),
//                >(self.glTexImage3D_ptr)(
//                    target as GLenum,
//                    level as GLint,
//                    internal_format as GLint,
//                    width as GLsizei,
//                    height as GLsizei,
//                    depth,
//                    border as GLint,
//                    format as GLenum,
//                    type_,
//                    pdata,
//                );
//            }
//            Ok(())
//        }
//    }
//    pub fn gl_tex_sub_image_3d(
//        &mut self,
//        target: TextureTarget,
//        level: GLint,
//        x_offset: GLint,
//        y_offset: GLint,
//        z_offset: GLint,
//        width: i32,
//        height: i32,
//        depth: GLsizei,
//        format: PixelDataFormat,
//        type_: GLenum,
//        opt_data: Option<&[u8]>,
//    ) -> Result<(), Error> {
//        if self.is_debug() {
//            let mut param_info_target = ParamInfo::new("TextureTarget", "target");
//
//            let mut param_info_level = ParamInfo::new("GLint", "level");
//
//            let mut param_info_x_offset = ParamInfo::new("GLint", "x_offset");
//
//            let mut param_info_y_offset = ParamInfo::new("GLint", "y_offset");
//
//            let mut param_info_z_offset = ParamInfo::new("GLint", "z_offset");
//
//            let mut param_info_width = ParamInfo::new("i32", "width");
//
//            let mut param_info_height = ParamInfo::new("i32", "height");
//
//            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");
//
//            let mut param_info_format = ParamInfo::new("PixelDataFormat", "format");
//
//            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");
//
//            let mut param_info_opt_data = ParamInfo::new("Option<&[u8]>", "opt_data");
//            let param_value_opt_data = opt_data.to_vec();
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_tex_sub_image_3d".to_string();
//
//            param_infos.push(&param_info_target);
//            param_values.push(&target);
//
//            param_infos.push(&param_info_level);
//            param_values.push(&level);
//
//            param_infos.push(&param_info_x_offset);
//            param_values.push(&x_offset);
//
//            param_infos.push(&param_info_y_offset);
//            param_values.push(&y_offset);
//
//            param_infos.push(&param_info_z_offset);
//            param_values.push(&z_offset);
//
//            param_infos.push(&param_info_width);
//            param_values.push(&width);
//
//            param_infos.push(&param_info_height);
//            param_values.push(&height);
//
//            param_infos.push(&param_info_depth);
//            param_values.push(&depth);
//
//            param_infos.push(&param_info_format);
//            param_values.push(&format);
//
//            param_infos.push(&param_info_type_);
//            param_values.push(&type_);
//
//            param_infos.push(&param_info_opt_data);
//            param_values.push(&param_value_opt_data);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    let pdata = match opt_data {
//                        Some(data) => mem::transmute(data.as_ptr()),
//                        None => ptr::null(),
//                    };
//
//                    #[cfg(target_os = "ios")]
//                    ffi::glTexSubImage3D(
//                        target as GLenum,
//                        level,
//                        x_offset,
//                        y_offset,
//                        z_offset,
//                        width as GLsizei,
//                        height as GLsizei,
//                        depth,
//                        format as GLenum,
//                        type_,
//                        pdata,
//                    );
//                    #[cfg(target_os = "android")]
//                    std::mem::transmute::<
//                        _,
//                        extern "system" fn(
//                            GLenum,
//                            GLint,
//                            GLint,
//                            GLint,
//                            GLint,
//                            GLsizei,
//                            GLsizei,
//                            GLsizei,
//                            GLenum,
//                            GLenum,
//                            *const GLvoid,
//                        ) -> (),
//                    >(self.glTexSubImage3D_ptr)(
//                        target as GLenum,
//                        level,
//                        x_offset,
//                        y_offset,
//                        z_offset,
//                        width as GLsizei,
//                        height as GLsizei,
//                        depth,
//                        format as GLenum,
//                        type_,
//                        pdata,
//                    );
//                }
//                Ok(())
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                let pdata = match opt_data {
//                    Some(data) => mem::transmute(data.as_ptr()),
//                    None => ptr::null(),
//                };
//
//                #[cfg(target_os = "ios")]
//                ffi::glTexSubImage3D(
//                    target as GLenum,
//                    level,
//                    x_offset,
//                    y_offset,
//                    z_offset,
//                    width as GLsizei,
//                    height as GLsizei,
//                    depth,
//                    format as GLenum,
//                    type_,
//                    pdata,
//                );
//                #[cfg(target_os = "android")]
//                std::mem::transmute::<
//                    _,
//                    extern "system" fn(
//                        GLenum,
//                        GLint,
//                        GLint,
//                        GLint,
//                        GLint,
//                        GLsizei,
//                        GLsizei,
//                        GLsizei,
//                        GLenum,
//                        GLenum,
//                        *const GLvoid,
//                    ) -> (),
//                >(self.glTexSubImage3D_ptr)(
//                    target as GLenum,
//                    level,
//                    x_offset,
//                    y_offset,
//                    z_offset,
//                    width as GLsizei,
//                    height as GLsizei,
//                    depth,
//                    format as GLenum,
//                    type_,
//                    pdata,
//                );
//            }
//            Ok(())
//        }
//    }
    pub fn gl_copy_tex_sub_image3d(
        &mut self,
        target: TextureTarget,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        x: GLint,
        y: GLint,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_x_offset = ParamInfo::new("GLint", "x_offset");

            let mut param_info_y_offset = ParamInfo::new("GLint", "y_offset");

            let mut param_info_z_offset = ParamInfo::new("GLint", "z_offset");

            let mut param_info_x = ParamInfo::new("GLint", "x");

            let mut param_info_y = ParamInfo::new("GLint", "y");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_copy_tex_sub_image3d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_x_offset);
            param_values.push(&x_offset);

            param_infos.push(&param_info_y_offset);
            param_values.push(&y_offset);

            param_infos.push(&param_info_z_offset);
            param_values.push(&z_offset);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glCopyTexSubImage3D(
                        target as GLenum,
                        level,
                        x_offset,
                        y_offset,
                        z_offset,
                        x,
                        y,
                        width as GLsizei,
                        height as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLsizei,
                            GLsizei,
                        ) -> (),
                    >(self.glCopyTexSubImage3D_ptr)(
                        target as GLenum,
                        level,
                        x_offset,
                        y_offset,
                        z_offset,
                        x,
                        y,
                        width as GLsizei,
                        height as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glCopyTexSubImage3D(
                    target as GLenum,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    x,
                    y,
                    width as GLsizei,
                    height as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLsizei,
                        GLsizei,
                    ) -> (),
                >(self.glCopyTexSubImage3D_ptr)(
                    target as GLenum,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    x,
                    y,
                    width as GLsizei,
                    height as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_compressed_tex_image3d<T>(
        &mut self,
        target: TextureTarget,
        level: GLint,
        internal_format: PixelDataFormat,
        width: i32,
        height: i32,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_internal_format =
                ParamInfo::new("PixelDataFormat", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");

            let mut param_info_border = ParamInfo::new("GLint", "border");

            let mut param_info_imageSize = ParamInfo::new("GLsizei", "imageSize");

            let mut param_info_data = ParamInfo::new("&[T]", "data");
            let param_value_data = data.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_compressed_tex_image3d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            param_infos.push(&param_info_border);
            param_values.push(&border);

            param_infos.push(&param_info_imageSize);
            param_values.push(&imageSize);

            param_infos.push(&param_info_data);
            param_values.push(&param_value_data);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glCompressedTexImage3D(
                        target as GLenum,
                        level,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                        border,
                        imageSize,
                        data.as_ptr() as *const GLvoid,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLint,
                            GLenum,
                            GLsizei,
                            GLsizei,
                            GLsizei,
                            GLint,
                            GLsizei,
                            *const GLvoid,
                        ) -> (),
                    >(self.glCompressedTexImage3D_ptr)(
                        target as GLenum,
                        level,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                        border,
                        imageSize,
                        data.as_ptr() as *const GLvoid,
                    );

                    ;
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glCompressedTexImage3D(
                    target as GLenum,
                    level,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                    border,
                    imageSize,
                    data.as_ptr() as *const GLvoid,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLint,
                        GLenum,
                        GLsizei,
                        GLsizei,
                        GLsizei,
                        GLint,
                        GLsizei,
                        *const GLvoid,
                    ) -> (),
                >(self.glCompressedTexImage3D_ptr)(
                    target as GLenum,
                    level,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                    border,
                    imageSize,
                    data.as_ptr() as *const GLvoid,
                );

                    ;
            }
            Ok(())
        }
    }
    pub fn gl_compressed_tex_sub_image3d<T>(
        &mut self,
        target: TextureTarget,
        level: GLint,
        x_offset: GLint,
        y_offset: GLint,
        z_offset: GLint,
        width: i32,
        height: i32,
        depth: GLsizei,
        format: PixelDataFormat,
        image_size: GLsizei,
        data: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_x_offset = ParamInfo::new("GLint", "x_offset");

            let mut param_info_y_offset = ParamInfo::new("GLint", "y_offset");

            let mut param_info_z_offset = ParamInfo::new("GLint", "z_offset");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");

            let mut param_info_format = ParamInfo::new("PixelDataFormat", "format");

            let mut param_info_image_size = ParamInfo::new("GLsizei", "image_size");

            let mut param_info_data = ParamInfo::new("&[T]", "data");
            let param_value_data = data.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_compressed_tex_sub_image3d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_x_offset);
            param_values.push(&x_offset);

            param_infos.push(&param_info_y_offset);
            param_values.push(&y_offset);

            param_infos.push(&param_info_z_offset);
            param_values.push(&z_offset);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_image_size);
            param_values.push(&image_size);

            param_infos.push(&param_info_data);
            param_values.push(&param_value_data);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glCompressedTexSubImage3D(
                        target as GLenum,
                        level,
                        x_offset,
                        y_offset,
                        z_offset,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                        format as GLenum,
                        image_size,
                        data.as_ptr() as *const GLvoid,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLsizei,
                            GLsizei,
                            GLsizei,
                            GLenum,
                            GLsizei,
                            *const GLvoid,
                        ) -> (),
                    >(self.glCompressedTexSubImage3D_ptr)(
                        target as GLenum,
                        level,
                        x_offset,
                        y_offset,
                        z_offset,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                        format as GLenum,
                        image_size,
                        data.as_ptr() as *const GLvoid,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glCompressedTexSubImage3D(
                    target as GLenum,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                    format as GLenum,
                    image_size,
                    data.as_ptr() as *const GLvoid,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLsizei,
                        GLsizei,
                        GLsizei,
                        GLenum,
                        GLsizei,
                        *const GLvoid,
                    ) -> (),
                >(self.glCompressedTexSubImage3D_ptr)(
                    target as GLenum,
                    level,
                    x_offset,
                    y_offset,
                    z_offset,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                    format as GLenum,
                    image_size,
                    data.as_ptr() as *const GLvoid,
                );
            }
            Ok(())
        }
    }
    pub fn gl_gen_queries(&mut self, size: i32) -> Result<Vec<GLuint>, Error> {
        if self.is_debug() {
            let mut param_info_size = ParamInfo::new("i32", "size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_queries".to_string();

            param_infos.push(&param_info_size);
            param_values.push(&size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGenQueries(size as GLsizei, ids.as_ptr() as *mut GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                        self.glGenQueries_ptr,
                    )(size as GLsizei, ids.as_ptr() as *mut GLuint);

                    Ok(ids)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
                #[cfg(target_os = "ios")]
                ffi::glGenQueries(size as GLsizei, ids.as_ptr() as *mut GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenQueries_ptr,
                )(size as GLsizei, ids.as_ptr() as *mut GLuint);

                Ok(ids)
            }
        }
    }
    pub fn gl_delete_queries(&mut self, ids: &mut [GLuint]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_ids = ParamInfo::new("&mut [GLuint]", "ids");
            let param_value_ids = ids.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_queries".to_string();

            param_infos.push(&param_info_ids);
            param_values.push(&param_value_ids);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let n: GLsizei = ids.len() as i32;
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteQueries(n, ids.as_ptr() as *mut GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                        self.glDeleteQueries_ptr,
                    )(n, ids.as_ptr() as *mut GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let n: GLsizei = ids.len() as i32;
                #[cfg(target_os = "ios")]
                ffi::glDeleteQueries(n, ids.as_ptr() as *mut GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                    self.glDeleteQueries_ptr,
                )(n, ids.as_ptr() as *mut GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_is_query(&mut self, id: u32) -> Result<GLboolean, Error> {
        if self.is_debug() {
            let mut param_info_id = ParamInfo::new("u32", "id");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_query".to_string();

            param_infos.push(&param_info_id);
            param_values.push(&id);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsQuery(id as GLuint);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                        self.glIsQuery_ptr,
                    )(id as GLuint);

                    Ok(result)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsQuery(id as GLuint);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsQuery_ptr,
                )(id as GLuint);

                Ok(result)
            }
        }
    }
    pub fn gl_begin_query(&mut self, target: GLenum, id: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_id = ParamInfo::new("u32", "id");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_begin_query".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_id);
            param_values.push(&id);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBeginQuery(target as GLenum, id as GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                        self.glBeginQuery_ptr,
                    )(target as GLenum, id as GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBeginQuery(target as GLenum, id as GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glBeginQuery_ptr,
                )(target as GLenum, id as GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_end_query(&mut self, target: GLenum) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_end_query".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glEndQuery(target);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum) -> ()>(self.glEndQuery_ptr)(
                        target,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glEndQuery(target);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum) -> ()>(self.glEndQuery_ptr)(
                    target,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_queryiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("&mut [GLint]", "params");
            let param_value_params = params.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_queryiv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&param_value_params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetQueryiv(
                        target as GLenum,
                        pname as GLenum,
                        params.as_ptr() as *mut GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                        self.glGetQueryiv_ptr,
                    )(
                        target as GLenum,
                        pname as GLenum,
                        params.as_ptr() as *mut GLint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetQueryiv(
                    target as GLenum,
                    pname as GLenum,
                    params.as_ptr() as *mut GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                    self.glGetQueryiv_ptr,
                )(
                    target as GLenum,
                    pname as GLenum,
                    params.as_ptr() as *mut GLint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_query_objectuiv(
        &mut self,
        id: u32,
        pname: GLenum,
        params: &mut [GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_id = ParamInfo::new("u32", "id");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("&mut [GLuint]", "params");
            let param_value_params = params.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_query_objectuiv".to_string();

            param_infos.push(&param_info_id);
            param_values.push(&id);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&param_value_params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetQueryObjectuiv(
                        id as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                        self.glGetQueryObjectuiv_ptr,
                    )(
                        id as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLuint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetQueryObjectuiv(
                    id as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLuint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                    self.glGetQueryObjectuiv_ptr,
                )(
                    id as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLuint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix2x3fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix2x3fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix2x3fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix2x3fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix2x3fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix2x3fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix3x2fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix3x2fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix3x2fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix3x2fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix3x2fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix3x2fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix2x4fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix2x4fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix2x4fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix2x4fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix2x4fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix2x4fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix4x2fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix4x2fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix4x2fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix4x2fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix4x2fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix4x2fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix3x4fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix3x4fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix3x4fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix3x4fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix3x4fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix3x4fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform_matrix4x3fv(
        &mut self,
        location: i32,
        count: i32,
        transpose: bool,
        value: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_transpose = ParamInfo::new("bool", "transpose");

            let mut param_info_value = ParamInfo::new("&[GLfloat]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_matrix4x3fv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformMatrix4x3fv(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glUniformMatrix4x3fv_ptr)(
                        location as GLint,
                        count as GLsizei,
                        transpose as GLboolean,
                        value.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformMatrix4x3fv(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glUniformMatrix4x3fv_ptr)(
                    location as GLint,
                    count as GLsizei,
                    transpose as GLboolean,
                    value.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_renderbuffer_storage_multisample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internal_format: GLenum,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_samples = ParamInfo::new("GLsizei", "samples");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_renderbuffer_storage_multisample".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_samples);
            param_values.push(&samples);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glRenderbufferStorageMultisample(
                        target as GLenum,
                        samples,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> (),
                    >(self.glRenderbufferStorageMultisample_ptr)(
                        target as GLenum,
                        samples,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glRenderbufferStorageMultisample(
                    target as GLenum,
                    samples,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> (),
                >(self.glRenderbufferStorageMultisample_ptr)(
                    target as GLenum,
                    samples,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_bind_vertex_array(&mut self, array: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_array = ParamInfo::new("GLuint", "array");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_vertex_array".to_string();

            param_infos.push(&param_info_array);
            param_values.push(&array);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindVertexArray(array);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                        self.glBindVertexArray_ptr,
                    )(array);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindVertexArray(array);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                    self.glBindVertexArray_ptr,
                )(array);
            }
            Ok(())
        }
    }
    pub fn gl_delete_vertex_arrays(&mut self, arrays: &[u32]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_arrays = ParamInfo::new("&[u32]", "arrays");
            let param_value_arrays = arrays.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_vertex_arrays".to_string();

            param_infos.push(&param_info_arrays);
            param_values.push(&param_value_arrays);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    if arrays.len() > 0 {
                        let n = arrays.len() as i32;
                        #[cfg(target_os = "ios")]
                        ffi::glDeleteVertexArrays(n, arrays.as_ptr() as *const GLuint);
                        #[cfg(target_os = "android")]
                        std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                            self.glDeleteVertexArrays_ptr,
                        )(n, arrays.as_ptr() as *const GLuint);
                    }
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                if arrays.len() > 0 {
                    let n = arrays.len() as i32;
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteVertexArrays(n, arrays.as_ptr() as *const GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                        self.glDeleteVertexArrays_ptr,
                    )(n, arrays.as_ptr() as *const GLuint);
                }
            }
            Ok(())
        }
    }
    pub fn gl_gen_vertex_arrays(&mut self, count: GLsizei) -> Result<Vec<GLuint>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_vertex_arrays".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGenVertexArrays(count as GLsizei, vec.as_mut_ptr());
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                        self.glGenVertexArrays_ptr,
                    )(count as GLsizei, vec.as_mut_ptr());

                    vec.set_len(count as usize);
                    Ok(vec)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);
                #[cfg(target_os = "ios")]
                ffi::glGenVertexArrays(count as GLsizei, vec.as_mut_ptr());
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenVertexArrays_ptr,
                )(count as GLsizei, vec.as_mut_ptr());

                vec.set_len(count as usize);
                Ok(vec)
            }
        }
    }
    pub fn gl_is_vertex_array(&mut self, array: GLuint) -> Result<GLboolean, Error> {
        if self.is_debug() {
            let mut param_info_array = ParamInfo::new("GLuint", "array");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_vertex_array".to_string();

            param_infos.push(&param_info_array);
            param_values.push(&array);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsVertexArray(array);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                        self.glIsVertexArray_ptr,
                    )(array);

                    if result == GL_TRUE {
                        Ok(GL_TRUE)
                    } else {
                        Ok(GL_FALSE)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsVertexArray(array);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsVertexArray_ptr,
                )(array);

                if result == GL_TRUE {
                    Ok(GL_TRUE)
                } else {
                    Ok(GL_FALSE)
                }
            }
        }
    }
    pub fn gl_get_integeri_v(&mut self, target: GLenum, index: GLuint) -> Result<GLint, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_integeri_v".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut value: GLint = 0;
                    #[cfg(target_os = "ios")]
                    ffi::glGetIntegeri_v(target as GLenum, index, &mut value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(
                        self.glGetIntegeri_v_ptr,
                    )(target as GLenum, index, &mut value);

                    Ok(value)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut value: GLint = 0;
                #[cfg(target_os = "ios")]
                ffi::glGetIntegeri_v(target as GLenum, index, &mut value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(
                    self.glGetIntegeri_v_ptr,
                )(target as GLenum, index, &mut value);

                Ok(value)
            }
        }
    }
    pub fn gl_end_transform_feedback(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_end_transform_feedback".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glEndTransformFeedback();
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn() -> ()>(
                        self.glEndTransformFeedback_ptr,
                    )();
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glEndTransformFeedback();
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn() -> ()>(
                    self.glEndTransformFeedback_ptr,
                )();
            }
            Ok(())
        }
    }
//    pub fn gl_transform_feedback_varyings(
//        &mut self,
//        program: u32,
//        count: i32,
//        varyings: &Vec<String>,
//        buffer_mode: TransformFeedbackMode,
//    ) -> Result<(), Error> {
//        if self.is_debug() {
//            let mut param_info_program = ParamInfo::new("u32", "program");
//
//            let mut param_info_count = ParamInfo::new("i32", "count");
//
//            let mut param_info_varyings = ParamInfo::new("&Vec<String>", "varyings");
//
//            let mut param_info_buffer_mode = ParamInfo::new("TransformFeedbackMode", "buffer_mode");
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_transform_feedback_varyings".to_string();
//
//            param_infos.push(&param_info_program);
//            param_values.push(&program);
//
//            param_infos.push(&param_info_count);
//            param_values.push(&count);
//
//            param_infos.push(&param_info_varyings);
//            param_values.push(&varyings);
//
//            param_infos.push(&param_info_buffer_mode);
//            param_values.push(&buffer_mode);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    let mut names: Vec<CString> = Vec::with_capacity(count as usize);
//                    let mut index = 0 as usize;
//                    while index < count as usize {
//                        names.push(CString::new(&(varyings[index])[..]).unwrap());
//                        index = index + 1;
//                    }
//                    index = 0;
//                    let ptr = names[index].as_ptr();
//                    let mut names_ptr: Vec<usize> = Vec::with_capacity(count as usize);
//
//                    while index < count as usize {
//                        names_ptr.push(names[index].as_ptr() as usize);
//                        index = index + 1;
//                    }
//                    #[cfg(target_os = "ios")]
//                    ffi::glTransformFeedbackVaryings(
//                        program as GLuint,
//                        count as GLsizei,
//                        names_ptr.as_ptr() as *const *const GLchar,
//                        buffer_mode as GLenum,
//                    );
//                    #[cfg(target_os = "android")]
//                    std::mem::transmute::<
//                        _,
//                        extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> (),
//                    >(self.glTransformFeedbackVaryings_ptr)(
//                        program as GLuint,
//                        count as GLsizei,
//                        names_ptr.as_ptr() as *const *const GLchar,
//                        buffer_mode as GLenum,
//                    );
//                }
//                Ok(())
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                let mut names: Vec<CString> = Vec::with_capacity(count as usize);
//                let mut index = 0 as usize;
//                while index < count as usize {
//                    names.push(CString::new(&(varyings[index])[..]).unwrap());
//                    index = index + 1;
//                }
//                index = 0;
//                let ptr = names[index].as_ptr();
//                let mut names_ptr: Vec<usize> = Vec::with_capacity(count as usize);
//
//                while index < count as usize {
//                    names_ptr.push(names[index].as_ptr() as usize);
//                    index = index + 1;
//                }
//                #[cfg(target_os = "ios")]
//                ffi::glTransformFeedbackVaryings(
//                    program as GLuint,
//                    count as GLsizei,
//                    names_ptr.as_ptr() as *const *const GLchar,
//                    buffer_mode as GLenum,
//                );
//                #[cfg(target_os = "android")]
//                std::mem::transmute::<
//                    _,
//                    extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> (),
//                >(self.glTransformFeedbackVaryings_ptr)(
//                    program as GLuint,
//                    count as GLsizei,
//                    names_ptr.as_ptr() as *const *const GLchar,
//                    buffer_mode as GLenum,
//                );
//            }
//            Ok(())
//        }
//    }
    pub fn gl_get_transform_feedback_varying(
        &mut self,
        program: u32,
        index: u32,
        buffer_size: GLsizei,
    ) -> Result<Active, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_buffer_size = ParamInfo::new("GLsizei", "buffer_size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_transform_feedback_varying".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_buffer_size);
            param_values.push(&buffer_size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length: GLsizei = 0;
                    let mut size: i32 = 0;
                    let mut type_: GLenum = GL_NONE;
                    let mut name = String::with_capacity(256);

                    #[cfg(target_os = "ios")]
                    ffi::glGetTransformFeedbackVarying(
                        program as GLuint,
                        index,
                        buffer_size,
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLuint,
                            GLuint,
                            GLsizei,
                            *mut GLsizei,
                            *mut GLsizei,
                            *mut GLenum,
                            *mut GLchar,
                        ) -> (),
                    >(self.glGetTransformFeedbackVarying_ptr)(
                        program as GLuint,
                        index,
                        buffer_size,
                        &mut length,
                        &mut size,
                        &mut type_,
                        name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                    );

                    if length > 0 {
                        name.as_mut_vec().set_len(length as usize);
                        name.truncate(length as usize);

                        Ok(Active {
                            name,
                            size,
                            type_: DataType::BOOL,
                            length,
                        })
                    } else {
                        Err(Error {})
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length: GLsizei = 0;
                let mut size: i32 = 0;
                let mut type_: GLenum = GL_NONE;
                let mut name = String::with_capacity(256);

                #[cfg(target_os = "ios")]
                ffi::glGetTransformFeedbackVarying(
                    program as GLuint,
                    index,
                    buffer_size,
                    &mut length,
                    &mut size,
                    &mut type_,
                    name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLuint,
                        GLuint,
                        GLsizei,
                        *mut GLsizei,
                        *mut GLsizei,
                        *mut GLenum,
                        *mut GLchar,
                    ) -> (),
                >(self.glGetTransformFeedbackVarying_ptr)(
                    program as GLuint,
                    index,
                    buffer_size,
                    &mut length,
                    &mut size,
                    &mut type_,
                    name.as_mut_vec().as_mut_ptr() as *mut GLchar,
                );

                if length > 0 {
                    name.as_mut_vec().set_len(length as usize);
                    name.truncate(length as usize);

                    Ok(Active {
                        name,
                        size,
                        type_: DataType::BOOL,
                        length,
                    })
                } else {
                    Err(Error {})
                }
            }
        }
    }
    pub fn gl_bind_transform_feedback(
        &mut self,
        target: TransformFeedbackObjectTarget,
        id: u32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TransformFeedbackObjectTarget", "target");

            let mut param_info_id = ParamInfo::new("u32", "id");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_transform_feedback".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_id);
            param_values.push(&id);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindTransformFeedback(target as GLenum, id as GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                        self.glBindTransformFeedback_ptr,
                    )(target as GLenum, id as GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindTransformFeedback(target as GLenum, id as GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glBindTransformFeedback_ptr,
                )(target as GLenum, id as GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_delete_transform_feedbacks(&mut self, ids: &[GLuint]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_ids = ParamInfo::new("&[GLuint]", "ids");
            let param_value_ids = ids.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_transform_feedbacks".to_string();

            param_infos.push(&param_info_ids);
            param_values.push(&param_value_ids);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let n = ids.len() as i32;
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteTransformFeedbacks(n, ids.as_ptr() as *const GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                        self.glDeleteTransformFeedbacks_ptr,
                    )(n, ids.as_ptr() as *const GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let n = ids.len() as i32;
                #[cfg(target_os = "ios")]
                ffi::glDeleteTransformFeedbacks(n, ids.as_ptr() as *const GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                    self.glDeleteTransformFeedbacks_ptr,
                )(n, ids.as_ptr() as *const GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_gen_transform_feedbacks(&mut self, size: i32) -> Result<Vec<GLuint>, Error> {
        if self.is_debug() {
            let mut param_info_size = ParamInfo::new("i32", "size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_transform_feedbacks".to_string();

            param_infos.push(&param_info_size);
            param_values.push(&size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGenTransformFeedbacks(size as GLsizei, ids.as_mut_ptr() as *mut GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                        self.glGenTransformFeedbacks_ptr,
                    )(size as GLsizei, ids.as_mut_ptr() as *mut GLuint);

                    Ok(ids)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut ids: Vec<GLuint> = Vec::with_capacity(size as usize);
                #[cfg(target_os = "ios")]
                ffi::glGenTransformFeedbacks(size as GLsizei, ids.as_mut_ptr() as *mut GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenTransformFeedbacks_ptr,
                )(size as GLsizei, ids.as_mut_ptr() as *mut GLuint);

                Ok(ids)
            }
        }
    }
    pub fn gl_is_transform_feedback(&mut self, id: u32) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_id = ParamInfo::new("u32", "id");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_transform_feedback".to_string();

            param_infos.push(&param_info_id);
            param_values.push(&id);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsTransformFeedback(id as GLuint);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                        self.glIsTransformFeedback_ptr,
                    )(id as GLuint);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsTransformFeedback(id as GLuint);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsTransformFeedback_ptr,
                )(id as GLuint);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
    pub fn gl_pause_transform_feedback(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_pause_transform_feedback".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glPauseTransformFeedback();
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn() -> ()>(
                        self.glPauseTransformFeedback_ptr,
                    )();
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glPauseTransformFeedback();
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn() -> ()>(
                    self.glPauseTransformFeedback_ptr,
                )();
            }
            Ok(())
        }
    }
    pub fn gl_resume_transform_feedback(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_resume_transform_feedback".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glResumeTransformFeedback();
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn() -> ()>(
                        self.glResumeTransformFeedback_ptr,
                    )();
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glResumeTransformFeedback();
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn() -> ()>(
                    self.glResumeTransformFeedback_ptr,
                )();
            }
            Ok(())
        }
    }
    pub fn gl_vertex_attrib_ipointer<T>(
        &mut self,
        index: u32,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_size = ParamInfo::new("GLint", "size");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_stride = ParamInfo::new("GLsizei", "stride");

            let mut param_info_pointer = ParamInfo::new("&[T]", "pointer");
            let param_value_pointer = pointer.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_ipointer".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_stride);
            param_values.push(&stride);

            param_infos.push(&param_info_pointer);
            param_values.push(&param_value_pointer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribIPointer(
                        index,
                        size as GLsizei,
                        type_,
                        stride,
                        pointer.as_ptr() as *const GLvoid,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const GLvoid) -> (),
                    >(self.glVertexAttribIPointer_ptr)(
                        index,
                        size as GLsizei,
                        type_,
                        stride,
                        pointer.as_ptr() as *const GLvoid,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribIPointer(
                    index,
                    size as GLsizei,
                    type_,
                    stride,
                    pointer.as_ptr() as *const GLvoid,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const GLvoid) -> (),
                >(self.glVertexAttribIPointer_ptr)(
                    index,
                    size as GLsizei,
                    type_,
                    stride,
                    pointer.as_ptr() as *const GLvoid,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_vertex_attrib_iiv(&mut self, index: u32, pname: GLenum) -> Result<GLint, Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_vertex_attrib_iiv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut params: GLint = 0;
                    #[cfg(target_os = "ios")]
                    ffi::glGetVertexAttribIiv(index, pname as GLenum, &mut params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                        self.glGetVertexAttribIiv_ptr,
                    )(index, pname as GLenum, &mut params);

                    Ok(params)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut params: GLint = 0;
                #[cfg(target_os = "ios")]
                ffi::glGetVertexAttribIiv(index, pname as GLenum, &mut params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                    self.glGetVertexAttribIiv_ptr,
                )(index, pname as GLenum, &mut params);

                Ok(params)
            }
        }
    }
    pub fn gl_get_vertex_attrib_iuiv(
        &mut self,
        index: u32,
        pname: GLenum,
    ) -> Result<GLuint, Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_vertex_attrib_iuiv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut params: GLuint = 0;
                    #[cfg(target_os = "ios")]
                    ffi::glGetVertexAttribIuiv(index, pname as GLenum, &mut params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                        self.glGetVertexAttribIuiv_ptr,
                    )(index, pname as GLenum, &mut params);

                    Ok(params)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut params: GLuint = 0;
                #[cfg(target_os = "ios")]
                ffi::glGetVertexAttribIuiv(index, pname as GLenum, &mut params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                    self.glGetVertexAttribIuiv_ptr,
                )(index, pname as GLenum, &mut params);

                Ok(params)
            }
        }
    }
    pub fn gl_vertex_attrib_i4i(
        &mut self,
        index: u32,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("GLint", "x");

            let mut param_info_y = ParamInfo::new("GLint", "y");

            let mut param_info_z = ParamInfo::new("GLint", "z");

            let mut param_info_w = ParamInfo::new("GLint", "w");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_i4i".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            param_infos.push(&param_info_w);
            param_values.push(&w);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribI4i(index, x, y, z, w);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> (),
                    >(self.glVertexAttribI4i_ptr)(index, x, y, z, w);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribI4i(index, x, y, z, w);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> (),
                >(self.glVertexAttribI4i_ptr)(index, x, y, z, w);
            }
            Ok(())
        }
    }
    pub fn gl_vertex_attrib_i4ui(
        &mut self,
        index: u32,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_x = ParamInfo::new("GLuint", "x");

            let mut param_info_y = ParamInfo::new("GLuint", "y");

            let mut param_info_z = ParamInfo::new("GLuint", "z");

            let mut param_info_w = ParamInfo::new("GLuint", "w");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_i4ui".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_z);
            param_values.push(&z);

            param_infos.push(&param_info_w);
            param_values.push(&w);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribI4ui(index, x, y, z, w);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> (),
                    >(self.glVertexAttribI4ui_ptr)(index, x, y, z, w);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribI4ui(index, x, y, z, w);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> (),
                >(self.glVertexAttribI4ui_ptr)(index, x, y, z, w);
            }
            Ok(())
        }
    }
    pub fn gl_vertex_attrib_i4iv(&mut self, index: u32, v: &[GLint]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_v = ParamInfo::new("&[GLint]", "v");
            let param_value_v = v.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_i4iv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_v);
            param_values.push(&param_value_v);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribI4iv(index, v.as_ptr() as *const GLint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
                        self.glVertexAttribI4iv_ptr,
                    )(index, v.as_ptr() as *const GLint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribI4iv(index, v.as_ptr() as *const GLint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
                    self.glVertexAttribI4iv_ptr,
                )(index, v.as_ptr() as *const GLint);
            }
            Ok(())
        }
    }
    pub fn gl_vertex_attrib_i4uiv(&mut self, index: u32, v: &[GLint]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_v = ParamInfo::new("&[GLint]", "v");
            let param_value_v = v.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_i4uiv".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_v);
            param_values.push(&param_value_v);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribI4uiv(index, v.as_ptr() as *const GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
                        self.glVertexAttribI4uiv_ptr,
                    )(index, v.as_ptr() as *const GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribI4uiv(index, v.as_ptr() as *const GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
                    self.glVertexAttribI4uiv_ptr,
                )(index, v.as_ptr() as *const GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_get_uniformuiv(&mut self, program: u32, location: i32) -> Result<GLuint, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_uniformuiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut value: GLuint = 0;
                    #[cfg(target_os = "ios")]
                    ffi::glGetUniformuiv(program as GLuint, location as GLint, &mut value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(
                        self.glGetUniformuiv_ptr,
                    )(program as GLuint, location as GLint, &mut value);

                    Ok(value)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut value: GLuint = 0;
                #[cfg(target_os = "ios")]
                ffi::glGetUniformuiv(program as GLuint, location as GLint, &mut value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(
                    self.glGetUniformuiv_ptr,
                )(program as GLuint, location as GLint, &mut value);

                Ok(value)
            }
        }
    }
    pub fn gl_get_frag_data_location(&mut self, program: u32, name: &str) -> Result<GLint, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_name = ParamInfo::new("&str", "name");
            let param_value_name = name.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_frag_data_location".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_name);
            param_values.push(&param_value_name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let c_str = CString::new(name).unwrap();
                    #[cfg(target_os = "ios")]
                    let location = ffi::glGetFragDataLocation(
                        program as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );
                    #[cfg(target_os = "android")]
                    let location = std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, *const GLchar) -> GLint,
                    >(self.glGetFragDataLocation_ptr)(
                        program as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );

                    Ok(location)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let c_str = CString::new(name).unwrap();
                #[cfg(target_os = "ios")]
                let location =
                    ffi::glGetFragDataLocation(program as GLuint, c_str.as_ptr() as *const GLchar);
                #[cfg(target_os = "android")]
                let location =
                    std::mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
                        self.glGetFragDataLocation_ptr,
                    )(program as GLuint, c_str.as_ptr() as *const GLchar);

                Ok(location)
            }
        }
    }
    pub fn gl_uniform1ui(&mut self, location: i32, v0: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_v0 = ParamInfo::new("GLuint", "v0");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1ui".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform1ui(location as GLint, v0);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(
                        self.glUniform1ui_ptr,
                    )(location as GLint, v0);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform1ui(location as GLint, v0);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(
                    self.glUniform1ui_ptr,
                )(location as GLint, v0);
            }
            Ok(())
        }
    }
    pub fn gl_uniform2ui(&mut self, location: i32, v0: u32, v1: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_v0 = ParamInfo::new("u32", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2ui".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform2ui(location as GLint, v0 as GLuint, v1);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(
                        self.glUniform2ui_ptr,
                    )(location as GLint, v0 as GLuint, v1);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform2ui(location as GLint, v0 as GLuint, v1);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(
                    self.glUniform2ui_ptr,
                )(location as GLint, v0 as GLuint, v1);
            }
            Ok(())
        }
    }
    pub fn gl_uniform3ui(
        &mut self,
        location: i32,
        v0: u32,
        v1: GLuint,
        v2: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_v0 = ParamInfo::new("u32", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLuint", "v2");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3ui".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform3ui(location as GLint, v0 as GLuint, v1, v2);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(
                        self.glUniform3ui_ptr,
                    )(location as GLint, v0 as GLuint, v1, v2);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform3ui(location as GLint, v0 as GLuint, v1, v2);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(
                    self.glUniform3ui_ptr,
                )(location as GLint, v0 as GLuint, v1, v2);
            }
            Ok(())
        }
    }
    pub fn gl_uniform4ui(
        &mut self,
        location: i32,
        v0: u32,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_v0 = ParamInfo::new("u32", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLuint", "v2");

            let mut param_info_v3 = ParamInfo::new("GLuint", "v3");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4ui".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            param_infos.push(&param_info_v3);
            param_values.push(&v3);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform4ui(location as GLint, v0 as GLuint, v1, v2, v3);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> (),
                    >(self.glUniform4ui_ptr)(
                        location as GLint, v0 as GLuint, v1, v2, v3
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform4ui(location as GLint, v0 as GLuint, v1, v2, v3);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> (),
                >(self.glUniform4ui_ptr)(
                    location as GLint, v0 as GLuint, v1, v2, v3
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform1uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_value = ParamInfo::new("&[GLuint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform1uiv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform1uiv(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                        self.glUniform1uiv_ptr,
                    )(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform1uiv(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                    self.glUniform1uiv_ptr,
                )(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform2uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_value = ParamInfo::new("&[GLuint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform2uiv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform2uiv(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                        self.glUniform2uiv_ptr,
                    )(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform2uiv(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                    self.glUniform2uiv_ptr,
                )(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform3uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_value = ParamInfo::new("&[GLuint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform3uiv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform3uiv(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                        self.glUniform3uiv_ptr,
                    )(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform3uiv(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                    self.glUniform3uiv_ptr,
                )(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_uniform4uiv(
        &mut self,
        location: i32,
        count: i32,
        value: &[GLuint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_location = ParamInfo::new("i32", "location");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_value = ParamInfo::new("&[GLuint]", "value");
            let param_value_value = value.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform4uiv".to_string();

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&param_value_value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniform4uiv(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                        self.glUniform4uiv_ptr,
                    )(
                        location as GLint,
                        count as GLsizei,
                        value.as_ptr() as *const GLuint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniform4uiv(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
                    self.glUniform4uiv_ptr,
                )(
                    location as GLint,
                    count as GLsizei,
                    value.as_ptr() as *const GLuint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_stringi(&mut self, name: GLenum, index: GLuint) -> Result<String, Error> {
        if self.is_debug() {
            let mut param_info_name = ParamInfo::new("GLenum", "name");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_stringi".to_string();

            param_infos.push(&param_info_name);
            param_values.push(&name);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let c_str = ffi::glGetStringi(name, index);
                    #[cfg(target_os = "android")]
                    let c_str = std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint) -> *const GLubyte,
                    >(self.glGetStringi_ptr)(name, index);

                    if !c_str.is_null() {
                        match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                            Ok(s) => Ok(s.to_string()),
                            Err(_) => Err(Error {}),
                        }
                    } else {
                        Err(Error {})
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let c_str = ffi::glGetStringi(name, index);
                #[cfg(target_os = "android")]
                let c_str = std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint) -> *const GLubyte,
                >(self.glGetStringi_ptr)(name, index);

                if !c_str.is_null() {
                    match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                        Ok(s) => Ok(s.to_string()),
                        Err(_) => Err(Error {}),
                    }
                } else {
                    Err(Error {})
                }
            }
        }
    }
//    pub fn gl_get_uniform_indices(
//        &mut self,
//        program: u32,
//        uniform_count: i32,
//        uniform_names: &Vec<String>,
//    ) -> Result<Vec<GLuint>, Error> {
//        if self.is_debug() {
//            let mut param_info_program = ParamInfo::new("u32", "program");
//
//            let mut param_info_uniform_count = ParamInfo::new("i32", "uniform_count");
//
//            let mut param_info_uniform_names = ParamInfo::new("&Vec<String>", "uniform_names");
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_get_uniform_indices".to_string();
//
//            param_infos.push(&param_info_program);
//            param_values.push(&program);
//
//            param_infos.push(&param_info_uniform_count);
//            param_values.push(&uniform_count);
//
//            param_infos.push(&param_info_uniform_names);
//            param_values.push(&uniform_names);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    let mut names: Vec<CString> = Vec::with_capacity(uniform_count as usize);
//                    let mut index = 0 as usize;
//                    while index < uniform_count as usize {
//                        names.push(CString::new(&(uniform_names[index])[..]).unwrap());
//                        index = index + 1;
//                    }
//                    index = 0;
//                    let ptr = names[index].as_ptr();
//                    let mut names_ptr: Vec<usize> = Vec::with_capacity(uniform_count as usize);
//
//                    while index < uniform_count as usize {
//                        names_ptr.push(names[index].as_ptr() as usize);
//                        index = index + 1;
//                    }
//
//                    let mut uniform_indices: Vec<GLuint> =
//                        Vec::with_capacity(uniform_count as usize);
//
//                    #[cfg(target_os = "ios")]
//                    ffi::glGetUniformIndices(
//                        program as GLuint,
//                        uniform_count as GLsizei,
//                        names_ptr.as_ptr() as *const *const GLchar,
//                        uniform_indices.as_ptr() as *mut GLuint,
//                    );
//                    #[cfg(target_os = "android")]
//                    std::mem::transmute::<
//                        _,
//                        extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint)
//                            -> (),
//                    >(self.glGetUniformIndices_ptr)(
//                        program as GLuint,
//                        uniform_count as GLsizei,
//                        names_ptr.as_ptr() as *const *const GLchar,
//                        uniform_indices.as_ptr() as *mut GLuint,
//                    );
//
//                    Ok(uniform_indices)
//                }
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                let mut names: Vec<CString> = Vec::with_capacity(uniform_count as usize);
//                let mut index = 0 as usize;
//                while index < uniform_count as usize {
//                    names.push(CString::new(&(uniform_names[index])[..]).unwrap());
//                    index = index + 1;
//                }
//                index = 0;
//                let ptr = names[index].as_ptr();
//                let mut names_ptr: Vec<usize> = Vec::with_capacity(uniform_count as usize);
//
//                while index < uniform_count as usize {
//                    names_ptr.push(names[index].as_ptr() as usize);
//                    index = index + 1;
//                }
//
//                let mut uniform_indices: Vec<GLuint> = Vec::with_capacity(uniform_count as usize);
//
//                #[cfg(target_os = "ios")]
//                ffi::glGetUniformIndices(
//                    program as GLuint,
//                    uniform_count as GLsizei,
//                    names_ptr.as_ptr() as *const *const GLchar,
//                    uniform_indices.as_ptr() as *mut GLuint,
//                );
//                #[cfg(target_os = "android")]
//                std::mem::transmute::<
//                    _,
//                    extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> (),
//                >(self.glGetUniformIndices_ptr)(
//                    program as GLuint,
//                    uniform_count as GLsizei,
//                    names_ptr.as_ptr() as *const *const GLchar,
//                    uniform_indices.as_ptr() as *mut GLuint,
//                );
//
//                Ok(uniform_indices)
//            }
//        }
//    }
    pub fn gl_get_active_uniformsiv(
        &mut self,
        program: u32,
        uniform_count: i32,
        uniform_indices: &[GLuint],
        pname: GLenum,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_uniform_count = ParamInfo::new("i32", "uniform_count");

            let mut param_info_uniform_indices = ParamInfo::new("&[GLuint]", "uniform_indices");
            let param_value_uniform_indices = uniform_indices.to_vec();

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("&mut [GLint]", "params");
            let param_value_params = params.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_active_uniformsiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_uniform_count);
            param_values.push(&uniform_count);

            param_infos.push(&param_info_uniform_indices);
            param_values.push(&param_value_uniform_indices);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&param_value_params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetActiveUniformsiv(
                        program as GLuint,
                        uniform_count as GLsizei,
                        uniform_indices.as_ptr() as *const GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint)
                            -> (),
                    >(self.glGetActiveUniformsiv_ptr)(
                        program as GLuint,
                        uniform_count as GLsizei,
                        uniform_indices.as_ptr() as *const GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetActiveUniformsiv(
                    program as GLuint,
                    uniform_count as GLsizei,
                    uniform_indices.as_ptr() as *const GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> (),
                >(self.glGetActiveUniformsiv_ptr)(
                    program as GLuint,
                    uniform_count as GLsizei,
                    uniform_indices.as_ptr() as *const GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_uniform_block_index(
        &mut self,
        program: u32,
        uniform_block_name: &str,
    ) -> Result<GLuint, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_uniform_block_name = ParamInfo::new("&str", "uniform_block_name");
            let param_value_uniform_block_name = uniform_block_name.to_string();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_uniform_block_index".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_uniform_block_name);
            param_values.push(&param_value_uniform_block_name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let c_str = CString::new(uniform_block_name).unwrap();
                    #[cfg(target_os = "ios")]
                    let index = ffi::glGetUniformBlockIndex(
                        program as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );
                    #[cfg(target_os = "android")]
                    let index = std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, *const GLchar) -> GLuint,
                    >(self.glGetUniformBlockIndex_ptr)(
                        program as GLuint,
                        c_str.as_ptr() as *const GLchar,
                    );

                    Ok(index)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let c_str = CString::new(uniform_block_name).unwrap();
                #[cfg(target_os = "ios")]
                let index =
                    ffi::glGetUniformBlockIndex(program as GLuint, c_str.as_ptr() as *const GLchar);
                #[cfg(target_os = "android")]
                let index =
                    std::mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(
                        self.glGetUniformBlockIndex_ptr,
                    )(program as GLuint, c_str.as_ptr() as *const GLchar);

                Ok(index)
            }
        }
    }
    pub fn gl_get_active_uniform_blockiv(
        &mut self,
        program: u32,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Result<GLint, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_uniform_block_index =
                ParamInfo::new("GLuint", "uniform_block_index");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_active_uniform_blockiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_uniform_block_index);
            param_values.push(&uniform_block_index);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut value = 0 as GLint;
                    #[cfg(target_os = "ios")]
                    ffi::glGetActiveUniformBlockiv(
                        program as GLuint,
                        uniform_block_index,
                        pname as GLenum,
                        &mut value,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> (),
                    >(self.glGetActiveUniformBlockiv_ptr)(
                        program as GLuint,
                        uniform_block_index,
                        pname as GLenum,
                        &mut value,
                    );

                    Ok(value)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut value = 0 as GLint;
                #[cfg(target_os = "ios")]
                ffi::glGetActiveUniformBlockiv(
                    program as GLuint,
                    uniform_block_index,
                    pname as GLenum,
                    &mut value,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> (),
                >(self.glGetActiveUniformBlockiv_ptr)(
                    program as GLuint,
                    uniform_block_index,
                    pname as GLenum,
                    &mut value,
                );

                Ok(value)
            }
        }
    }
    pub fn gl_uniform_block_binding(
        &mut self,
        program: u32,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_uniform_block_index =
                ParamInfo::new("GLuint", "uniform_block_index");

            let mut param_info_uniform_block_binding =
                ParamInfo::new("GLuint", "uniform_block_binding");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_uniform_block_binding".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_uniform_block_index);
            param_values.push(&uniform_block_index);

            param_infos.push(&param_info_uniform_block_binding);
            param_values.push(&uniform_block_binding);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUniformBlockBinding(
                        program as GLuint,
                        uniform_block_index,
                        uniform_block_binding,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
                        self.glUniformBlockBinding_ptr,
                    )(
                        program as GLuint,
                        uniform_block_index,
                        uniform_block_binding,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUniformBlockBinding(
                    program as GLuint,
                    uniform_block_index,
                    uniform_block_binding,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
                    self.glUniformBlockBinding_ptr,
                )(
                    program as GLuint,
                    uniform_block_index,
                    uniform_block_binding,
                );
            }
            Ok(())
        }
    }
    pub fn gl_draw_range_elements<T>(
        &mut self,
        mode: BeginMode,
        start: GLuint,
        end: GLuint,
        count: i32,
        type_: GLenum,
        indices: &[T],
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BeginMode", "mode");

            let mut param_info_start = ParamInfo::new("GLuint", "start");

            let mut param_info_end = ParamInfo::new("GLuint", "end");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("&[T]", "indices");
            let param_value_indices = indices.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_range_elements".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_start);
            param_values.push(&start);

            param_infos.push(&param_info_end);
            param_values.push(&end);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&param_value_indices);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawRangeElements(
                        mode as GLenum,
                        start,
                        end,
                        count as GLsizei,
                        type_,
                        indices.as_ptr() as *const GLvoid,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const GLvoid)
                            -> (),
                    >(self.glDrawRangeElements_ptr)(
                        mode as GLenum,
                        start,
                        end,
                        count as GLsizei,
                        type_,
                        indices.as_ptr() as *const GLvoid,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawRangeElements(
                    mode as GLenum,
                    start,
                    end,
                    count as GLsizei,
                    type_,
                    indices.as_ptr() as *const GLvoid,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const GLvoid)
                        -> (),
                >(self.glDrawRangeElements_ptr)(
                    mode as GLenum,
                    start,
                    end,
                    count as GLsizei,
                    type_,
                    indices.as_ptr() as *const GLvoid,
                );
            }
            Ok(())
        }
    }
    pub fn gl_draw_arrays_instanced(
        &mut self,
        mode: BeginMode,
        first: GLint,
        count: i32,
        instance_count: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BeginMode", "mode");

            let mut param_info_first = ParamInfo::new("GLint", "first");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_instance_count = ParamInfo::new("i32", "instance_count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_arrays_instanced".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_first);
            param_values.push(&first);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_instance_count);
            param_values.push(&instance_count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawArraysInstanced(
                        mode as GLenum,
                        first,
                        count as GLsizei,
                        instance_count as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> (),
                    >(self.glDrawArraysInstanced_ptr)(
                        mode as GLenum,
                        first,
                        count as GLsizei,
                        instance_count as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawArraysInstanced(
                    mode as GLenum,
                    first,
                    count as GLsizei,
                    instance_count as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(
                    self.glDrawArraysInstanced_ptr,
                )(
                    mode as GLenum,
                    first,
                    count as GLsizei,
                    instance_count as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_draw_elements_instanced<T>(
        &mut self,
        mode: BeginMode,
        count: i32,
        type_: GLenum,
        indices: &[T],
        instance_count: i32,
    ) -> Result<(), Error>
    where
        T: std::fmt::Debug + Clone,
    {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("BeginMode", "mode");

            let mut param_info_count = ParamInfo::new("i32", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("&[T]", "indices");
            let param_value_indices = indices.to_vec();

            let mut param_info_instance_count = ParamInfo::new("i32", "instance_count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_elements_instanced".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&param_value_indices);

            param_infos.push(&param_info_instance_count);
            param_values.push(&instance_count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawElementsInstanced(
                        mode as GLenum,
                        count as GLsizei,
                        type_,
                        indices.as_ptr() as *const GLvoid,
                        instance_count as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLsizei) -> (),
                    >(self.glDrawElementsInstanced_ptr)(
                        mode as GLenum,
                        count as GLsizei,
                        type_,
                        indices.as_ptr() as *const GLvoid,
                        instance_count as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawElementsInstanced(
                    mode as GLenum,
                    count as GLsizei,
                    type_,
                    indices.as_ptr() as *const GLvoid,
                    instance_count as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLsizei) -> (),
                >(self.glDrawElementsInstanced_ptr)(
                    mode as GLenum,
                    count as GLsizei,
                    type_,
                    indices.as_ptr() as *const GLvoid,
                    instance_count as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_fence_sync(&mut self, condition: GLenum, flags: GLbitfield) -> Result<GLsync, Error> {
        if self.is_debug() {
            let mut param_info_condition = ParamInfo::new("GLenum", "condition");

            let mut param_info_flags = ParamInfo::new("GLbitfield", "flags");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_fence_sync".to_string();

            param_infos.push(&param_info_condition);
            param_values.push(&condition);

            param_infos.push(&param_info_flags);
            param_values.push(&flags);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let sync = ffi::glFenceSync(condition, flags);
                    #[cfg(target_os = "android")]
                    let sync = std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLbitfield) -> GLsync,
                    >(self.glFenceSync_ptr)(condition, flags);

                    Ok(sync)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let sync = ffi::glFenceSync(condition, flags);
                #[cfg(target_os = "android")]
                let sync = std::mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(
                    self.glFenceSync_ptr,
                )(condition, flags);

                Ok(sync)
            }
        }
    }
    pub fn gl_is_sync(&mut self, sync: GLsync) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_sync = ParamInfo::new("GLsync", "sync");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_sync".to_string();

            param_infos.push(&param_info_sync);
            param_values.push(&sync);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsSync(sync);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(
                        self.glIsSync_ptr,
                    )(sync);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsSync(sync);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(
                    self.glIsSync_ptr,
                )(sync);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
    pub fn gl_delete_sync(&mut self, sync: GLsync) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sync = ParamInfo::new("GLsync", "sync");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_sync".to_string();

            param_infos.push(&param_info_sync);
            param_values.push(&sync);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteSync(sync);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsync) -> ()>(
                        self.glDeleteSync_ptr,
                    )(sync);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDeleteSync(sync);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsync) -> ()>(self.glDeleteSync_ptr)(
                    sync,
                );
            }
            Ok(())
        }
    }
    pub fn gl_client_wait_sync(
        &mut self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> Result<GLenum, Error> {
        if self.is_debug() {
            let mut param_info_sync = ParamInfo::new("GLsync", "sync");

            let mut param_info_flags = ParamInfo::new("GLbitfield", "flags");

            let mut param_info_timeout = ParamInfo::new("GLuint64", "timeout");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_client_wait_sync".to_string();

            param_infos.push(&param_info_sync);
            param_values.push(&sync);

            param_infos.push(&param_info_flags);
            param_values.push(&flags);

            param_infos.push(&param_info_timeout);
            param_values.push(&timeout);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glClientWaitSync(sync, flags, timeout);
                    #[cfg(target_os = "android")]
                    let result =
                        std::mem::transmute::<
                            _,
                            extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum,
                        >(self.glClientWaitSync_ptr)(sync, flags, timeout);

                    Ok(result)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glClientWaitSync(sync, flags, timeout);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum,
                >(self.glClientWaitSync_ptr)(sync, flags, timeout);

                Ok(result)
            }
        }
    }
    pub fn gl_wait_sync(
        &mut self,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sync = ParamInfo::new("GLsync", "sync");

            let mut param_info_flags = ParamInfo::new("GLbitfield", "flags");

            let mut param_info_timeout = ParamInfo::new("GLuint64", "timeout");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_wait_sync".to_string();

            param_infos.push(&param_info_sync);
            param_values.push(&sync);

            param_infos.push(&param_info_flags);
            param_values.push(&flags);

            param_infos.push(&param_info_timeout);
            param_values.push(&timeout);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glWaitSync(sync, flags, timeout);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(
                        self.glWaitSync_ptr,
                    )(sync, flags, timeout);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glWaitSync(sync, flags, timeout);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(
                    self.glWaitSync_ptr,
                )(sync, flags, timeout);
            }
            Ok(())
        }
    }
    pub fn gl_get_integer64v(&mut self, pname: GLenum) -> Result<GLint64, Error> {
        if self.is_debug() {
            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_integer64v".to_string();

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut value = 0 as GLint64;
                    #[cfg(target_os = "ios")]
                    ffi::glGetInteger64v(pname as GLenum, &mut value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(
                        self.glGetInteger64v_ptr,
                    )(pname as GLenum, &mut value);

                    Ok(value)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut value = 0 as GLint64;
                #[cfg(target_os = "ios")]
                ffi::glGetInteger64v(pname as GLenum, &mut value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(
                    self.glGetInteger64v_ptr,
                )(pname as GLenum, &mut value);

                Ok(value)
            }
        }
    }
    pub fn gl_get_synciv(
        &mut self,
        sync: GLsync,
        pname: GLenum,
        buffer_size: GLsizei,
        length: GLsizei,
    ) -> Result<Vec<GLint>, Error> {
        if self.is_debug() {
            let mut param_info_sync = ParamInfo::new("GLsync", "sync");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_buffer_size = ParamInfo::new("GLsizei", "buffer_size");

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_synciv".to_string();

            param_infos.push(&param_info_sync);
            param_values.push(&sync);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_buffer_size);
            param_values.push(&buffer_size);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut values: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGetSynciv(
                        sync,
                        pname as GLenum,
                        buffer_size,
                        length as *mut GLsizei,
                        values.as_mut_ptr() as *mut GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> (),
                    >(self.glGetSynciv_ptr)(
                        sync,
                        pname as GLenum,
                        buffer_size,
                        length as *mut GLsizei,
                        values.as_mut_ptr() as *mut GLint,
                    );

                    Ok(values)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut values: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
                #[cfg(target_os = "ios")]
                ffi::glGetSynciv(
                    sync,
                    pname as GLenum,
                    buffer_size,
                    length as *mut GLsizei,
                    values.as_mut_ptr() as *mut GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> (),
                >(self.glGetSynciv_ptr)(
                    sync,
                    pname as GLenum,
                    buffer_size,
                    length as *mut GLsizei,
                    values.as_mut_ptr() as *mut GLint,
                );

                Ok(values)
            }
        }
    }
    pub fn gl_get_integer64i_v(&mut self, target: GLenum, index: GLuint) -> Result<GLint64, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_integer64i_v".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut value = 0 as GLint64;
                    #[cfg(target_os = "ios")]
                    ffi::glGetInteger64i_v(target as GLenum, index, &mut value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(
                        self.glGetInteger64i_v_ptr,
                    )(target as GLenum, index, &mut value);

                    Ok(value)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut value = 0 as GLint64;
                #[cfg(target_os = "ios")]
                ffi::glGetInteger64i_v(target as GLenum, index, &mut value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(
                    self.glGetInteger64i_v_ptr,
                )(target as GLenum, index, &mut value);

                Ok(value)
            }
        }
    }
    pub fn gl_gen_samplers(&mut self, count: GLsizei) -> Result<Vec<GLuint>, Error> {
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_samplers".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut sampler: Vec<GLuint> = Vec::with_capacity(count as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGenSamplers(count as GLsizei, sampler.as_mut_ptr() as *mut GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                        self.glGenSamplers_ptr,
                    )(count as GLsizei, sampler.as_mut_ptr() as *mut GLuint);

                    Ok(sampler)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut sampler: Vec<GLuint> = Vec::with_capacity(count as usize);
                #[cfg(target_os = "ios")]
                ffi::glGenSamplers(count as GLsizei, sampler.as_mut_ptr() as *mut GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenSamplers_ptr,
                )(count as GLsizei, sampler.as_mut_ptr() as *mut GLuint);

                Ok(sampler)
            }
        }
    }
    pub fn gl_delete_samplers(&mut self, samplers: &[GLuint]) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_samplers = ParamInfo::new("&[GLuint]", "samplers");
            let param_value_samplers = samplers.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_samplers".to_string();

            param_infos.push(&param_info_samplers);
            param_values.push(&param_value_samplers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let count = samplers.len() as i32;
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteSamplers(count as GLsizei, samplers.as_ptr() as *const GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                        self.glDeleteSamplers_ptr,
                    )(count as GLsizei, samplers.as_ptr() as *const GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let count = samplers.len() as i32;
                #[cfg(target_os = "ios")]
                ffi::glDeleteSamplers(count as GLsizei, samplers.as_ptr() as *const GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                    self.glDeleteSamplers_ptr,
                )(count as GLsizei, samplers.as_ptr() as *const GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_is_sampler(&mut self, sampler: GLuint) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_sampler".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsSampler(sampler);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                        self.glIsSampler_ptr,
                    )(sampler);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsSampler(sampler);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsSampler_ptr,
                )(sampler);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
    pub fn gl_bind_sampler(&mut self, unit: GLuint, sampler: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_unit = ParamInfo::new("GLuint", "unit");

            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_sampler".to_string();

            param_infos.push(&param_info_unit);
            param_values.push(&unit);

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindSampler(unit, sampler);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                        self.glBindSampler_ptr,
                    )(unit, sampler);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindSampler(unit, sampler);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glBindSampler_ptr,
                )(unit, sampler);
            }
            Ok(())
        }
    }
    pub fn gl_sampler_parameteri(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_param = ParamInfo::new("GLint", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameteri".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameteri(sampler as GLuint, pname as GLenum, param);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
                        self.glSamplerParameteri_ptr,
                    )(sampler as GLuint, pname as GLenum, param);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameteri(sampler as GLuint, pname as GLenum, param);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
                    self.glSamplerParameteri_ptr,
                )(sampler as GLuint, pname as GLenum, param);
            }
            Ok(())
        }
    }
    pub fn gl_sampler_parameteriv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: &[GLint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_param = ParamInfo::new("&[GLint]", "param");
            let param_value_param = param.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameteriv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param_value_param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameteriv(
                        sampler as GLuint,
                        pname as GLenum,
                        param.as_ptr() as *const GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                        self.glSamplerParameteriv_ptr,
                    )(
                        sampler as GLuint,
                        pname as GLenum,
                        param.as_ptr() as *const GLint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameteriv(
                    sampler as GLuint,
                    pname as GLenum,
                    param.as_ptr() as *const GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                    self.glSamplerParameteriv_ptr,
                )(
                    sampler as GLuint,
                    pname as GLenum,
                    param.as_ptr() as *const GLint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_sampler_parameterf(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_param = ParamInfo::new("GLfloat", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameterf".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameterf(sampler as GLuint, pname as GLenum, param);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(
                        self.glSamplerParameterf_ptr,
                    )(sampler as GLuint, pname as GLenum, param);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameterf(sampler as GLuint, pname as GLenum, param);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(
                    self.glSamplerParameterf_ptr,
                )(sampler as GLuint, pname as GLenum, param);
            }
            Ok(())
        }
    }
    pub fn gl_sampler_parameterfv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        param: &[GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_param = ParamInfo::new("&[GLfloat]", "param");
            let param_value_param = param.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameterfv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param_value_param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameterfv(
                        sampler as GLuint,
                        pname as GLenum,
                        param.as_ptr() as *const GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, *const GLfloat) -> (),
                    >(self.glSamplerParameterfv_ptr)(
                        sampler as GLuint,
                        pname as GLenum,
                        param.as_ptr() as *const GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameterfv(
                    sampler as GLuint,
                    pname as GLenum,
                    param.as_ptr() as *const GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(
                    self.glSamplerParameterfv_ptr,
                )(
                    sampler as GLuint,
                    pname as GLenum,
                    param.as_ptr() as *const GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_sampler_parameteriv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        params: &mut [GLint],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_params = ParamInfo::new("&mut [GLint]", "params");
            let param_value_params = params.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_sampler_parameteriv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&param_value_params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetSamplerParameteriv(
                        sampler as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                        self.glGetSamplerParameteriv_ptr,
                    )(
                        sampler as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLint,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetSamplerParameteriv(
                    sampler as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                    self.glGetSamplerParameteriv_ptr,
                )(
                    sampler as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLint,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_sampler_parameterfv(
        &mut self,
        sampler: u32,
        pname: SamplerParameter,
        params: &mut [GLfloat],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("u32", "sampler");

            let mut param_info_pname = ParamInfo::new("SamplerParameter", "pname");

            let mut param_info_params = ParamInfo::new("&mut [GLfloat]", "params");
            let param_value_params = params.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_sampler_parameterfv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&param_value_params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetSamplerParameterfv(
                        sampler as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLfloat,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
                        self.glGetSamplerParameterfv_ptr,
                    )(
                        sampler as GLuint,
                        pname as GLenum,
                        params.as_mut_ptr() as *mut GLfloat,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetSamplerParameterfv(
                    sampler as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLfloat,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
                    self.glGetSamplerParameterfv_ptr,
                )(
                    sampler as GLuint,
                    pname as GLenum,
                    params.as_mut_ptr() as *mut GLfloat,
                );
            }
            Ok(())
        }
    }
    pub fn gl_vertex_attrib_divisor(&mut self, index: u32, divisor: u32) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("u32", "index");

            let mut param_info_divisor = ParamInfo::new("u32", "divisor");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_divisor".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_divisor);
            param_values.push(&divisor);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribDivisor(index, divisor as GLuint);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                        self.glVertexAttribDivisor_ptr,
                    )(index, divisor as GLuint);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribDivisor(index, divisor as GLuint);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glVertexAttribDivisor_ptr,
                )(index, divisor as GLuint);
            }
            Ok(())
        }
    }
    pub fn gl_get_program_binary(
        &mut self,
        program: u32,
        buffer_size: GLsizei,
    ) -> Result<ProgramBinary, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_buffer_size = ParamInfo::new("GLsizei", "buffer_size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_binary".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_buffer_size);
            param_values.push(&buffer_size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut length = 0 as GLsizei;
                    let mut binary_format = GL_NONE as GLenum;
                    let mut binary: Vec<u8> = Vec::with_capacity(buffer_size as usize);

                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramBinary(
                        program as GLuint,
                        buffer_size,
                        &mut length as *mut GLsizei,
                        &mut binary_format as *mut GLenum,
                        binary.as_mut_ptr() as *mut GLvoid,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut GLvoid)
                            -> (),
                    >(self.glGetProgramBinary_ptr)(
                        program as GLuint,
                        buffer_size,
                        &mut length as *mut GLsizei,
                        &mut binary_format as *mut GLenum,
                        binary.as_mut_ptr() as *mut GLvoid,
                    );

                    if length == 0 {
                        Err(Error {})
                    } else {
                        Ok(ProgramBinary {
                            length,
                            binary_format,
                            binary,
                        })
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut length = 0 as GLsizei;
                let mut binary_format = GL_NONE as GLenum;
                let mut binary: Vec<u8> = Vec::with_capacity(buffer_size as usize);

                #[cfg(target_os = "ios")]
                ffi::glGetProgramBinary(
                    program as GLuint,
                    buffer_size,
                    &mut length as *mut GLsizei,
                    &mut binary_format as *mut GLenum,
                    binary.as_mut_ptr() as *mut GLvoid,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut GLvoid)
                        -> (),
                >(self.glGetProgramBinary_ptr)(
                    program as GLuint,
                    buffer_size,
                    &mut length as *mut GLsizei,
                    &mut binary_format as *mut GLenum,
                    binary.as_mut_ptr() as *mut GLvoid,
                );

                if length == 0 {
                    Err(Error {})
                } else {
                    Ok(ProgramBinary {
                        length,
                        binary_format,
                        binary,
                    })
                }
            }
        }
    }
    pub fn gl_program_binary(
        &mut self,
        program: u32,
        binary_format: GLenum,
        binary: &[u8],
        length: GLsizei,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_binary_format = ParamInfo::new("GLenum", "binary_format");

            let mut param_info_binary = ParamInfo::new("&[u8]", "binary");
            let param_value_binary = binary.to_vec();

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_binary".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_binary_format);
            param_values.push(&binary_format);

            param_infos.push(&param_info_binary);
            param_values.push(&param_value_binary);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramBinary(
                        program as GLuint,
                        binary_format,
                        binary.as_ptr() as *const GLvoid,
                        length,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, *const GLvoid, GLsizei) -> (),
                    >(self.glProgramBinary_ptr)(
                        program as GLuint,
                        binary_format,
                        binary.as_ptr() as *const GLvoid,
                        length,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramBinary(
                    program as GLuint,
                    binary_format,
                    binary.as_ptr() as *const GLvoid,
                    length,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, *const GLvoid, GLsizei) -> (),
                >(self.glProgramBinary_ptr)(
                    program as GLuint,
                    binary_format,
                    binary.as_ptr() as *const GLvoid,
                    length,
                );
            }
            Ok(())
        }
    }
    pub fn gl_program_parameteri(
        &mut self,
        program: u32,
        pname: GLenum,
        value: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("u32", "program");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_value = ParamInfo::new("GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_parameteri".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramParameteri(program as GLuint, pname as GLenum, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
                        self.glProgramParameteri_ptr,
                    )(program as GLuint, pname as GLenum, value);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramParameteri(program as GLuint, pname as GLenum, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
                    self.glProgramParameteri_ptr,
                )(program as GLuint, pname as GLenum, value);
            }
            Ok(())
        }
    }
    pub fn gl_invalidate_framebuffer(
        &mut self,
        target: FrameBufferTarget,
        num_attachments: GLsizei,
        attachments: &[AttachmentTarget],
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_num_attachments = ParamInfo::new("GLsizei", "num_attachments");

            let mut param_info_attachments = ParamInfo::new("&[AttachmentTarget]", "attachments");
            let param_value_attachments = attachments.to_vec();

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_invalidate_framebuffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_num_attachments);
            param_values.push(&num_attachments);

            param_infos.push(&param_info_attachments);
            param_values.push(&param_value_attachments);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glInvalidateFramebuffer(
                        target as GLenum,
                        num_attachments,
                        attachments.as_ptr() as *const GLenum,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, *const GLenum) -> (),
                    >(self.glInvalidateFramebuffer_ptr)(
                        target as GLenum,
                        num_attachments,
                        attachments.as_ptr() as *const GLenum,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glInvalidateFramebuffer(
                    target as GLenum,
                    num_attachments,
                    attachments.as_ptr() as *const GLenum,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>(
                    self.glInvalidateFramebuffer_ptr,
                )(
                    target as GLenum,
                    num_attachments,
                    attachments.as_ptr() as *const GLenum,
                );
            }
            Ok(())
        }
    }
    pub fn gl_invalidate_sub_framebuffer(
        &mut self,
        target: FrameBufferTarget,
        num_attachments: GLsizei,
        attachments: &[AttachmentTarget],
        x: GLint,
        y: GLint,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FrameBufferTarget", "target");

            let mut param_info_num_attachments = ParamInfo::new("GLsizei", "num_attachments");

            let mut param_info_attachments = ParamInfo::new("&[AttachmentTarget]", "attachments");
            let param_value_attachments = attachments.to_vec();

            let mut param_info_x = ParamInfo::new("GLint", "x");

            let mut param_info_y = ParamInfo::new("GLint", "y");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_invalidate_sub_framebuffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_num_attachments);
            param_values.push(&num_attachments);

            param_infos.push(&param_info_attachments);
            param_values.push(&param_value_attachments);

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glInvalidateSubFramebuffer(
                        target as GLenum,
                        num_attachments,
                        attachments.as_ptr() as *const GLenum,
                        x,
                        y,
                        width as GLsizei,
                        height as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLsizei,
                            *const GLenum,
                            GLint,
                            GLint,
                            GLsizei,
                            GLsizei,
                        ) -> (),
                    >(self.glInvalidateSubFramebuffer_ptr)(
                        target as GLenum,
                        num_attachments,
                        attachments.as_ptr() as *const GLenum,
                        x,
                        y,
                        width as GLsizei,
                        height as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glInvalidateSubFramebuffer(
                    target as GLenum,
                    num_attachments,
                    attachments.as_ptr() as *const GLenum,
                    x,
                    y,
                    width as GLsizei,
                    height as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLsizei,
                        *const GLenum,
                        GLint,
                        GLint,
                        GLsizei,
                        GLsizei,
                    ) -> (),
                >(self.glInvalidateSubFramebuffer_ptr)(
                    target as GLenum,
                    num_attachments,
                    attachments.as_ptr() as *const GLenum,
                    x,
                    y,
                    width as GLsizei,
                    height as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_blit_framebuffer(
        &mut self,
        src_X0: GLint,
        src_Y0: GLint,
        src_X1: GLint,
        src_Y1: GLint,
        dst_X0: GLint,
        dst_Y0: GLint,
        dst_X1: GLint,
        dst_Y1: GLint,
        mask: BufferMask,
        filter: FilterMode,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_src_X0 = ParamInfo::new("GLint", "src_X0");

            let mut param_info_src_Y0 = ParamInfo::new("GLint", "src_Y0");

            let mut param_info_src_X1 = ParamInfo::new("GLint", "src_X1");

            let mut param_info_src_Y1 = ParamInfo::new("GLint", "src_Y1");

            let mut param_info_dst_X0 = ParamInfo::new("GLint", "dst_X0");

            let mut param_info_dst_Y0 = ParamInfo::new("GLint", "dst_Y0");

            let mut param_info_dst_X1 = ParamInfo::new("GLint", "dst_X1");

            let mut param_info_dst_Y1 = ParamInfo::new("GLint", "dst_Y1");

            let mut param_info_mask = ParamInfo::new("BufferMask", "mask");

            let mut param_info_filter = ParamInfo::new("FilterMode", "filter");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blit_framebuffer".to_string();

            param_infos.push(&param_info_src_X0);
            param_values.push(&src_X0);

            param_infos.push(&param_info_src_Y0);
            param_values.push(&src_Y0);

            param_infos.push(&param_info_src_X1);
            param_values.push(&src_X1);

            param_infos.push(&param_info_src_Y1);
            param_values.push(&src_Y1);

            param_infos.push(&param_info_dst_X0);
            param_values.push(&dst_X0);

            param_infos.push(&param_info_dst_Y0);
            param_values.push(&dst_Y0);

            param_infos.push(&param_info_dst_X1);
            param_values.push(&dst_X1);

            param_infos.push(&param_info_dst_Y1);
            param_values.push(&dst_Y1);

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            param_infos.push(&param_info_filter);
            param_values.push(&filter);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlitFramebuffer(
                        src_X0,
                        src_Y0,
                        src_X1,
                        src_Y1,
                        dst_X0,
                        dst_Y0,
                        dst_X1,
                        dst_Y1,
                        mask as GLenum,
                        filter as GLenum,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLint,
                            GLbitfield,
                            GLenum,
                        ) -> (),
                    >(self.glBlitFramebuffer_ptr)(
                        src_X0,
                        src_Y0,
                        src_X1,
                        src_Y1,
                        dst_X0,
                        dst_Y0,
                        dst_X1,
                        dst_Y1,
                        mask as GLenum,
                        filter as GLenum,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlitFramebuffer(
                    src_X0,
                    src_Y0,
                    src_X1,
                    src_Y1,
                    dst_X0,
                    dst_Y0,
                    dst_X1,
                    dst_Y1,
                    mask as GLenum,
                    filter as GLenum,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLint,
                        GLbitfield,
                        GLenum,
                    ) -> (),
                >(self.glBlitFramebuffer_ptr)(
                    src_X0,
                    src_Y0,
                    src_X1,
                    src_Y1,
                    dst_X0,
                    dst_Y0,
                    dst_X1,
                    dst_Y1,
                    mask as GLenum,
                    filter as GLenum,
                );
            }
            Ok(())
        }
    }
    pub fn gl_framebuffer_texture_layer(
        &mut self,
        target: FramebufferTarget,
        attachment: AttachmentTarget,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("FramebufferTarget", "target");

            let mut param_info_attachment = ParamInfo::new("AttachmentTarget", "attachment");

            let mut param_info_texture = ParamInfo::new("GLuint", "texture");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_layer = ParamInfo::new("GLint", "layer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_framebuffer_texture_layer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_attachment);
            param_values.push(&attachment);

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_layer);
            param_values.push(&layer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glFramebufferTextureLayer(
                        target as GLenum,
                        attachment as GLenum,
                        texture,
                        level,
                        layer,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> (),
                    >(self.glFramebufferTextureLayer_ptr)(
                        target as GLenum,
                        attachment as GLenum,
                        texture,
                        level,
                        layer,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glFramebufferTextureLayer(
                    target as GLenum,
                    attachment as GLenum,
                    texture,
                    level,
                    layer,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> (),
                >(self.glFramebufferTextureLayer_ptr)(
                    target as GLenum,
                    attachment as GLenum,
                    texture,
                    level,
                    layer,
                );
            }
            Ok(())
        }
    }
    pub fn gl_tex_storage2d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internal_format: TextureTarget,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_levels = ParamInfo::new("GLsizei", "levels");

            let mut param_info_internal_format = ParamInfo::new("TextureTarget", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_storage2d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_levels);
            param_values.push(&levels);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexStorage2D(
                        target as GLenum,
                        levels,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> (),
                    >(self.glTexStorage2D_ptr)(
                        target as GLenum,
                        levels,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexStorage2D(
                    target as GLenum,
                    levels,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> (),
                >(self.glTexStorage2D_ptr)(
                    target as GLenum,
                    levels,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                );
            }
            Ok(())
        }
    }
    pub fn gl_tex_storage3d(
        &mut self,
        target: TextureTarget,
        levels: GLsizei,
        internal_format: TextureTarget,
        width: i32,
        height: i32,
        depth: GLsizei,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("TextureTarget", "target");

            let mut param_info_levels = ParamInfo::new("GLsizei", "levels");

            let mut param_info_internal_format = ParamInfo::new("TextureTarget", "internal_format");

            let mut param_info_width = ParamInfo::new("i32", "width");

            let mut param_info_height = ParamInfo::new("i32", "height");

            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_storage3d".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_levels);
            param_values.push(&levels);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexStorage3D(
                        target as GLenum,
                        levels,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei)
                            -> (),
                    >(self.glTexStorage3D_ptr)(
                        target as GLenum,
                        levels,
                        internal_format as GLenum,
                        width as GLsizei,
                        height as GLsizei,
                        depth,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexStorage3D(
                    target as GLenum,
                    levels,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> (),
                >(self.glTexStorage3D_ptr)(
                    target as GLenum,
                    levels,
                    internal_format as GLenum,
                    width as GLsizei,
                    height as GLsizei,
                    depth,
                );
            }
            Ok(())
        }
    }
    pub fn gl_get_internal_formativ(
        &mut self,
        target: GLenum,
        internal_format: TextureTarget,
        pname: GLenum,
        buffer_size: GLsizei,
    ) -> Result<Vec<GLint>, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_internal_format = ParamInfo::new("TextureTarget", "internal_format");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_buffer_size = ParamInfo::new("GLsizei", "buffer_size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_internal_formativ".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_buffer_size);
            param_values.push(&buffer_size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    let mut params: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
                    #[cfg(target_os = "ios")]
                    ffi::glGetInternalformativ(
                        target as GLenum,
                        internal_format as GLenum,
                        pname as GLenum,
                        buffer_size,
                        params.as_mut_ptr() as *mut GLint,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> (),
                    >(self.glGetInternalformativ_ptr)(
                        target as GLenum,
                        internal_format as GLenum,
                        pname as GLenum,
                        buffer_size,
                        params.as_mut_ptr() as *mut GLint,
                    );

                    Ok(params)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                let mut params: Vec<GLint> = Vec::with_capacity(buffer_size as usize);
                #[cfg(target_os = "ios")]
                ffi::glGetInternalformativ(
                    target as GLenum,
                    internal_format as GLenum,
                    pname as GLenum,
                    buffer_size,
                    params.as_mut_ptr() as *mut GLint,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> (),
                >(self.glGetInternalformativ_ptr)(
                    target as GLenum,
                    internal_format as GLenum,
                    pname as GLenum,
                    buffer_size,
                    params.as_mut_ptr() as *mut GLint,
                );

                Ok(params)
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_dispatch_compute(
        &mut self,
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_num_groups_x = ParamInfo::new("GLuint", "num_groups_x");

            let mut param_info_num_groups_y = ParamInfo::new("GLuint", "num_groups_y");

            let mut param_info_num_groups_z = ParamInfo::new("GLuint", "num_groups_z");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_dispatch_compute".to_string();

            param_infos.push(&param_info_num_groups_x);
            param_values.push(&num_groups_x);

            param_infos.push(&param_info_num_groups_y);
            param_values.push(&num_groups_y);

            param_infos.push(&param_info_num_groups_z);
            param_values.push(&num_groups_z);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDispatchCompute(num_groups_x, num_groups_y, num_groups_z);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
                        self.glDispatchCompute_ptr,
                    )(num_groups_x, num_groups_y, num_groups_z);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDispatchCompute(num_groups_x, num_groups_y, num_groups_z);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
                    self.glDispatchCompute_ptr,
                )(num_groups_x, num_groups_y, num_groups_z);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_dispatch_compute_indirect(&mut self, indirect: GLintptr) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_indirect = ParamInfo::new("GLintptr", "indirect");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_dispatch_compute_indirect".to_string();

            param_infos.push(&param_info_indirect);
            param_values.push(&indirect);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDispatchComputeIndirect(indirect);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(
                        self.glDispatchComputeIndirect_ptr,
                    )(indirect);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDispatchComputeIndirect(indirect);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(
                    self.glDispatchComputeIndirect_ptr,
                )(indirect);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_draw_arrays_indirect(
        &mut self,
        mode: GLenum,
        indirect: *const ::std::os::raw::c_void,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_info_indirect = ParamInfo::new("*const", "indirect");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_arrays_indirect".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_indirect);
            param_values.push(&indirect);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawArraysIndirect(mode as GLenum, indirect as *const GLvoid);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, *const GLvoid) -> ()>(
                        self.glDrawArraysIndirect_ptr,
                    )(mode as GLenum, indirect as *const GLvoid);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawArraysIndirect(mode as GLenum, indirect as *const GLvoid);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, *const GLvoid) -> ()>(
                    self.glDrawArraysIndirect_ptr,
                )(mode as GLenum, indirect as *const GLvoid);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_draw_elements_indirect(
        &mut self,
        mode: GLenum,
        type_: GLenum,
        indirect: *const GLvoid,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indirect = ParamInfo::new("*const GLvoid", "indirect");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_elements_indirect".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indirect);
            param_values.push(&indirect);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawElementsIndirect(mode, type_, indirect);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLvoid) -> ()>(
                        self.glDrawElementsIndirect_ptr,
                    )(mode, type_, indirect);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawElementsIndirect(mode, type_, indirect);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLvoid) -> ()>(
                    self.glDrawElementsIndirect_ptr,
                )(mode, type_, indirect);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_framebuffer_parameteri(
        &mut self,
        target: GLenum,
        pname: GLenum,
        param: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_param = ParamInfo::new("GLint", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_framebuffer_parameteri".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glFramebufferParameteri(target, pname, param);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
                        self.glFramebufferParameteri_ptr,
                    )(target, pname, param);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glFramebufferParameteri(target, pname, param);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
                    self.glFramebufferParameteri_ptr,
                )(target, pname, param);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_framebuffer_parameteriv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_framebuffer_parameteriv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetFramebufferParameteriv(target, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                        self.glGetFramebufferParameteriv_ptr,
                    )(target, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetFramebufferParameteriv(target, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                    self.glGetFramebufferParameteriv_ptr,
                )(target, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_interfaceiv(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_program_interface = ParamInfo::new("GLenum", "program_interface");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_interfaceiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_program_interface);
            param_values.push(&program_interface);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramInterfaceiv(program, program_interface, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> (),
                    >(self.glGetProgramInterfaceiv_ptr)(
                        program, program_interface, pname, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetProgramInterfaceiv(program, program_interface, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> (),
                >(self.glGetProgramInterfaceiv_ptr)(
                    program, program_interface, pname, params
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_resource_index(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> Result<u32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_program_interface = ParamInfo::new("GLenum", "program_interface");

            let mut param_info_name = ParamInfo::new("*const GLchar", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_resource_index".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_program_interface);
            param_values.push(&program_interface);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glGetProgramResourceIndex(program, program_interface, name);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
                    >(self.glGetProgramResourceIndex_ptr)(
                        program, program_interface, name
                    );

                    Ok(result as u32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glGetProgramResourceIndex(program, program_interface, name);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint,
                >(self.glGetProgramResourceIndex_ptr)(
                    program, program_interface, name
                );

                Ok(result as u32)
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_resource_name(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_program_interface = ParamInfo::new("GLenum", "program_interface");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_info_buf_size = ParamInfo::new("GLsizei", "buf_size");

            let mut param_info_length = ParamInfo::new("*mut GLsizei", "length");

            let mut param_info_name = ParamInfo::new("*mut GLchar", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_resource_name".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_program_interface);
            param_values.push(&program_interface);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_buf_size);
            param_values.push(&buf_size);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramResourceName(
                        program,
                        program_interface,
                        index,
                        buf_size,
                        length,
                        name,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLuint,
                            GLenum,
                            GLuint,
                            GLsizei,
                            *mut GLsizei,
                            *mut GLchar,
                        ) -> (),
                    >(self.glGetProgramResourceName_ptr)(
                        program,
                        program_interface,
                        index,
                        buf_size,
                        length,
                        name,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetProgramResourceName(
                    program,
                    program_interface,
                    index,
                    buf_size,
                    length,
                    name,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar)
                        -> (),
                >(self.glGetProgramResourceName_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_resourceiv(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        index: GLuint,
        propCount: GLsizei,
        props: *const GLenum,
        buf_size: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_program_interface = ParamInfo::new("GLenum", "program_interface");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_info_propCount = ParamInfo::new("GLsizei", "propCount");

            let mut param_info_props = ParamInfo::new("*const GLenum", "props");

            let mut param_info_buf_size = ParamInfo::new("GLsizei", "buf_size");

            let mut param_info_length = ParamInfo::new("*mut GLsizei", "length");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_resourceiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_program_interface);
            param_values.push(&program_interface);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_propCount);
            param_values.push(&propCount);

            param_infos.push(&param_info_props);
            param_values.push(&props);

            param_infos.push(&param_info_buf_size);
            param_values.push(&buf_size);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramResourceiv(
                        program,
                        program_interface,
                        index,
                        propCount,
                        props,
                        buf_size,
                        length,
                        params,
                    );
                    #[cfg(target_os = "android")]
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
                    >(self.glGetProgramResourceiv_ptr)(
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetProgramResourceiv(
                    program,
                    program_interface,
                    index,
                    propCount,
                    props,
                    buf_size,
                    length,
                    params,
                );
                #[cfg(target_os = "android")]
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
                >(self.glGetProgramResourceiv_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_resource_location(
        &mut self,
        program: GLuint,
        program_interface: GLenum,
        name: *const GLchar,
    ) -> Result<i32, Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_program_interface = ParamInfo::new("GLenum", "program_interface");

            let mut param_info_name = ParamInfo::new("*const GLchar", "name");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_resource_location".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_program_interface);
            param_values.push(&program_interface);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result =
                        ffi::glGetProgramResourceLocation(program, program_interface, name);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
                    >(self.glGetProgramResourceLocation_ptr)(
                        program, program_interface, name
                    );

                    Ok(result as i32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glGetProgramResourceLocation(program, program_interface, name);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint,
                >(self.glGetProgramResourceLocation_ptr)(
                    program, program_interface, name
                );

                Ok(result as i32)
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_use_program_stages(
        &mut self,
        pipeline: GLuint,
        stages: GLbitfield,
        program: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_info_stages = ParamInfo::new("GLbitfield", "stages");

            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_use_program_stages".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            param_infos.push(&param_info_stages);
            param_values.push(&stages);

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glUseProgramStages(pipeline, stages, program);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(
                        self.glUseProgramStages_ptr,
                    )(pipeline, stages, program);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glUseProgramStages(pipeline, stages, program);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(
                    self.glUseProgramStages_ptr,
                )(pipeline, stages, program);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_active_shader_program(
        &mut self,
        pipeline: GLuint,
        program: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_active_shader_program".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            param_infos.push(&param_info_program);
            param_values.push(&program);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glActiveShaderProgram(pipeline, program);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                        self.glActiveShaderProgram_ptr,
                    )(pipeline, program);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glActiveShaderProgram(pipeline, program);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glActiveShaderProgram_ptr,
                )(pipeline, program);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_create_shader_program_v(
        &mut self,
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar,
    ) -> Result<u32, Error> {
        if self.is_debug() {
            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_strings = ParamInfo::new("*const *const GLchar", "strings");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_create_shader_program_v".to_string();

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_strings);
            param_values.push(&strings);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glCreateShaderProgramv(type_, count, strings);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint,
                    >(self.glCreateShaderProgramv_ptr)(
                        type_, count, strings
                    );

                    Ok(result as u32)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glCreateShaderProgramv(type_, count, strings);
                #[cfg(target_os = "android")]
                let result =
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint,
                    >(self.glCreateShaderProgramv_ptr)(type_, count, strings);

                Ok(result as u32)
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_bind_program_pipeline(&mut self, pipeline: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_program_pipeline".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindProgramPipeline(pipeline);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                        self.glBindProgramPipeline_ptr,
                    )(pipeline);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindProgramPipeline(pipeline);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                    self.glBindProgramPipeline_ptr,
                )(pipeline);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_delete_program_pipelines(
        &mut self,
        n: GLsizei,
        pipelines: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_n = ParamInfo::new("GLsizei", "n");

            let mut param_info_pipelines = ParamInfo::new("*const GLuint", "pipelines");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_delete_program_pipelines".to_string();

            param_infos.push(&param_info_n);
            param_values.push(&n);

            param_infos.push(&param_info_pipelines);
            param_values.push(&pipelines);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDeleteProgramPipelines(n, pipelines);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                        self.glDeleteProgramPipelines_ptr,
                    )(n, pipelines);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDeleteProgramPipelines(n, pipelines);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
                    self.glDeleteProgramPipelines_ptr,
                )(n, pipelines);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_gen_program_pipelines(
        &mut self,
        n: GLsizei,
        pipelines: *mut GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_n = ParamInfo::new("GLsizei", "n");

            let mut param_info_pipelines = ParamInfo::new("*mut GLuint", "pipelines");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_gen_program_pipelines".to_string();

            param_infos.push(&param_info_n);
            param_values.push(&n);

            param_infos.push(&param_info_pipelines);
            param_values.push(&pipelines);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGenProgramPipelines(n, pipelines);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                        self.glGenProgramPipelines_ptr,
                    )(n, pipelines);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGenProgramPipelines(n, pipelines);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
                    self.glGenProgramPipelines_ptr,
                )(n, pipelines);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_is_program_pipeline(&mut self, pipeline: GLuint) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_program_pipeline".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsProgramPipeline(pipeline);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                        self.glIsProgramPipeline_ptr,
                    )(pipeline);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsProgramPipeline(pipeline);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
                    self.glIsProgramPipeline_ptr,
                )(pipeline);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_pipelineiv(
        &mut self,
        pipeline: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_pipelineiv".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramPipelineiv(pipeline, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                        self.glGetProgramPipelineiv_ptr,
                    )(pipeline, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetProgramPipelineiv(pipeline, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
                    self.glGetProgramPipelineiv_ptr,
                )(pipeline, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLint", "v0");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1i".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform1i(program, location, v0);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(
                        self.glProgramUniform1i_ptr,
                    )(program, location, v0);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform1i(program, location, v0);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(
                    self.glProgramUniform1i_ptr,
                )(program, location, v0);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLint", "v1");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2i".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2i(program, location, v0, v1);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(
                        self.glProgramUniform2i_ptr,
                    )(program, location, v0, v1);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2i(program, location, v0, v1);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(
                    self.glProgramUniform2i_ptr,
                )(program, location, v0, v1);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLint", "v2");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3i".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3i(program, location, v0, v1, v2);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> (),
                    >(self.glProgramUniform3i_ptr)(
                        program, location, v0, v1, v2
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3i(program, location, v0, v1, v2);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> (),
                >(self.glProgramUniform3i_ptr)(program, location, v0, v1, v2);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4i(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLint", "v2");

            let mut param_info_v3 = ParamInfo::new("GLint", "v3");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4i".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            param_infos.push(&param_info_v3);
            param_values.push(&v3);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4i(program, location, v0, v1, v2, v3);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> (),
                    >(self.glProgramUniform4i_ptr)(
                        program, location, v0, v1, v2, v3
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4i(program, location, v0, v1, v2, v3);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> (),
                >(self.glProgramUniform4i_ptr)(program, location, v0, v1, v2, v3);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLuint", "v0");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1ui".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform1ui(program, location, v0);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(
                        self.glProgramUniform1ui_ptr,
                    )(program, location, v0);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform1ui(program, location, v0);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(
                    self.glProgramUniform1ui_ptr,
                )(program, location, v0);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLuint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2ui".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2ui(program, location, v0, v1);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(
                        self.glProgramUniform2ui_ptr,
                    )(program, location, v0, v1);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2ui(program, location, v0, v1);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(
                    self.glProgramUniform2ui_ptr,
                )(program, location, v0, v1);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLuint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLuint", "v2");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3ui".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3ui(program, location, v0, v1, v2);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> (),
                    >(self.glProgramUniform3ui_ptr)(
                        program, location, v0, v1, v2
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3ui(program, location, v0, v1, v2);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> (),
                >(self.glProgramUniform3ui_ptr)(program, location, v0, v1, v2);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4ui(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLuint", "v0");

            let mut param_info_v1 = ParamInfo::new("GLuint", "v1");

            let mut param_info_v2 = ParamInfo::new("GLuint", "v2");

            let mut param_info_v3 = ParamInfo::new("GLuint", "v3");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4ui".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            param_infos.push(&param_info_v3);
            param_values.push(&v3);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4ui(program, location, v0, v1, v2, v3);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> (),
                    >(self.glProgramUniform4ui_ptr)(
                        program, location, v0, v1, v2, v3
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4ui(program, location, v0, v1, v2, v3);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> (),
                >(self.glProgramUniform4ui_ptr)(program, location, v0, v1, v2, v3);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLfloat", "v0");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1f".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform1f(program, location, v0);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(
                        self.glProgramUniform1f_ptr,
                    )(program, location, v0);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform1f(program, location, v0);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(
                    self.glProgramUniform1f_ptr,
                )(program, location, v0);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLfloat", "v0");

            let mut param_info_v1 = ParamInfo::new("GLfloat", "v1");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2f".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2f(program, location, v0, v1);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> (),
                    >(self.glProgramUniform2f_ptr)(program, location, v0, v1);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2f(program, location, v0, v1);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>(
                    self.glProgramUniform2f_ptr,
                )(program, location, v0, v1);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLfloat", "v0");

            let mut param_info_v1 = ParamInfo::new("GLfloat", "v1");

            let mut param_info_v2 = ParamInfo::new("GLfloat", "v2");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3f".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3f(program, location, v0, v1, v2);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> (),
                    >(self.glProgramUniform3f_ptr)(
                        program, location, v0, v1, v2
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3f(program, location, v0, v1, v2);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> (),
                >(self.glProgramUniform3f_ptr)(program, location, v0, v1, v2);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4f(
        &mut self,
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_v0 = ParamInfo::new("GLfloat", "v0");

            let mut param_info_v1 = ParamInfo::new("GLfloat", "v1");

            let mut param_info_v2 = ParamInfo::new("GLfloat", "v2");

            let mut param_info_v3 = ParamInfo::new("GLfloat", "v3");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4f".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_v0);
            param_values.push(&v0);

            param_infos.push(&param_info_v1);
            param_values.push(&v1);

            param_infos.push(&param_info_v2);
            param_values.push(&v2);

            param_infos.push(&param_info_v3);
            param_values.push(&v3);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4f(program, location, v0, v1, v2, v3);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
                    >(self.glProgramUniform4f_ptr)(
                        program, location, v0, v1, v2, v3
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4f(program, location, v0, v1, v2, v3);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
                >(self.glProgramUniform4f_ptr)(program, location, v0, v1, v2, v3);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1iv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform1iv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                    >(self.glProgramUniform1iv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform1iv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform1iv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2iv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2iv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                    >(self.glProgramUniform2iv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2iv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform2iv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3iv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3iv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                    >(self.glProgramUniform3iv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3iv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform3iv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4iv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4iv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4iv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                    >(self.glProgramUniform4iv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4iv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> (),
                >(self.glProgramUniform4iv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLuint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1uiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform1uiv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                    >(self.glProgramUniform1uiv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform1uiv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform1uiv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLuint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2uiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2uiv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                    >(self.glProgramUniform2uiv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2uiv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform2uiv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLuint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3uiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3uiv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                    >(self.glProgramUniform3uiv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3uiv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform3uiv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLuint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4uiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4uiv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                    >(self.glProgramUniform4uiv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4uiv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> (),
                >(self.glProgramUniform4uiv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform1fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform1fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4fv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                    >(self.glProgramUniform4fv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4fv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform4fv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform2fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform2fv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                    >(self.glProgramUniform2fv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform2fv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform2fv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform3fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform3fv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                    >(self.glProgramUniform3fv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform3fv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform3fv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform4fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniform4fv(program, location, count, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                    >(self.glProgramUniform4fv_ptr)(
                        program, location, count, value
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniform4fv(program, location, count, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> (),
                >(self.glProgramUniform4fv_ptr)(program, location, count, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix2fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix2fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix2fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix2fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix3fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix3fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix3fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix3fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix4fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix4fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix4fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix4fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix2x3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix2x3fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix2x3fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix2x3fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix2x3fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2x3fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix3x2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix3x2fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix3x2fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix3x2fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix3x2fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3x2fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix2x4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix2x4fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix2x4fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix2x4fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix2x4fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix2x4fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix4x2fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix4x2fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix4x2fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix4x2fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix4x2fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4x2fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix3x4fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix3x4fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix3x4fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix3x4fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix3x4fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix3x4fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_program_uniform_matrix4x3fv(
        &mut self,
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_transpose = ParamInfo::new("GLboolean", "transpose");

            let mut param_info_value = ParamInfo::new("*const GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_program_uniform_matrix4x3fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_transpose);
            param_values.push(&transpose);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glProgramUniformMatrix4x3fv(program, location, count, transpose, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                    >(self.glProgramUniformMatrix4x3fv_ptr)(
                        program, location, count, transpose, value,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glProgramUniformMatrix4x3fv(program, location, count, transpose, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
                >(self.glProgramUniformMatrix4x3fv_ptr)(
                    program, location, count, transpose, value
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_validate_program_pipeline(&mut self, pipeline: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_validate_program_pipeline".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glValidateProgramPipeline(pipeline);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                        self.glValidateProgramPipeline_ptr,
                    )(pipeline);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glValidateProgramPipeline(pipeline);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
                    self.glValidateProgramPipeline_ptr,
                )(pipeline);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_program_pipeline_info_log(
        &mut self,
        pipeline: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pipeline = ParamInfo::new("GLuint", "pipeline");

            let mut param_info_buf_size = ParamInfo::new("GLsizei", "buf_size");

            let mut param_info_length = ParamInfo::new("*mut GLsizei", "length");

            let mut param_info_infoLog = ParamInfo::new("*mut GLchar", "infoLog");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_program_pipeline_info_log".to_string();

            param_infos.push(&param_info_pipeline);
            param_values.push(&pipeline);

            param_infos.push(&param_info_buf_size);
            param_values.push(&buf_size);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_infoLog);
            param_values.push(&infoLog);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetProgramPipelineInfoLog(pipeline, buf_size, length, infoLog);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                    >(self.glGetProgramPipelineInfoLog_ptr)(
                        pipeline, buf_size, length, infoLog
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetProgramPipelineInfoLog(pipeline, buf_size, length, infoLog);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetProgramPipelineInfoLog_ptr)(
                    pipeline, buf_size, length, infoLog
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_bind_image_texture(
        &mut self,
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_unit = ParamInfo::new("GLuint", "unit");

            let mut param_info_texture = ParamInfo::new("GLuint", "texture");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_layered = ParamInfo::new("GLboolean", "layered");

            let mut param_info_layer = ParamInfo::new("GLint", "layer");

            let mut param_info_access = ParamInfo::new("GLenum", "access");

            let mut param_info_format = ParamInfo::new("GLenum", "format");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_image_texture".to_string();

            param_infos.push(&param_info_unit);
            param_values.push(&unit);

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_layered);
            param_values.push(&layered);

            param_infos.push(&param_info_layer);
            param_values.push(&layer);

            param_infos.push(&param_info_access);
            param_values.push(&access);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindImageTexture(unit, texture, level, layered, layer, access, format);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum)
                            -> (),
                    >(self.glBindImageTexture_ptr)(
                        unit, texture, level, layered, layer, access, format,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindImageTexture(unit, texture, level, layered, layer, access, format);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum)
                        -> (),
                >(self.glBindImageTexture_ptr)(
                    unit, texture, level, layered, layer, access, format,
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_boolean_iv(
        &mut self,
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_info_data = ParamInfo::new("*mut GLboolean", "data");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_boolean_iv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_data);
            param_values.push(&data);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetBooleani_v(target, index, data);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint, *mut GLboolean) -> (),
                    >(self.glGetBooleani_v_ptr)(target, index, data);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetBooleani_v(target, index, data);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(
                    self.glGetBooleani_v_ptr,
                )(target, index, data);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_memory_barrier(&mut self, barriers: GLbitfield) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_barriers = ParamInfo::new("GLbitfield", "barriers");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_memory_barrier".to_string();

            param_infos.push(&param_info_barriers);
            param_values.push(&barriers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glMemoryBarrier(barriers);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                        self.glMemoryBarrier_ptr,
                    )(barriers);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glMemoryBarrier(barriers);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                    self.glMemoryBarrier_ptr,
                )(barriers);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_memory_barrier_by_region(&mut self, barriers: GLbitfield) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_barriers = ParamInfo::new("GLbitfield", "barriers");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_memory_barrier_by_region".to_string();

            param_infos.push(&param_info_barriers);
            param_values.push(&barriers);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glMemoryBarrierByRegion(barriers);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                        self.glMemoryBarrierByRegion_ptr,
                    )(barriers);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glMemoryBarrierByRegion(barriers);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
                    self.glMemoryBarrierByRegion_ptr,
                )(barriers);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_tex_storage2D_multi_sample(
        &mut self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_samples = ParamInfo::new("GLsizei", "samples");

            let mut param_info_internalformat = ParamInfo::new("GLenum", "internalformat");

            let mut param_info_width = ParamInfo::new("GLsizei", "width");

            let mut param_info_height = ParamInfo::new("GLsizei", "height");

            let mut param_info_fixedsamplelocations =
                ParamInfo::new("GLboolean", "fixedsamplelocations");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_storage2D_multi_sample".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_samples);
            param_values.push(&samples);

            param_infos.push(&param_info_internalformat);
            param_values.push(&internalformat);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_fixedsamplelocations);
            param_values.push(&fixedsamplelocations);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexStorage2DMultisample(
                        target,
                        samples,
                        internalformat,
                        width,
                        height,
                        fixedsamplelocations,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean)
                            -> (),
                    >(self.glTexStorage2DMultisample_ptr)(
                        target,
                        samples,
                        internalformat,
                        width,
                        height,
                        fixedsamplelocations,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexStorage2DMultisample(
                    target,
                    samples,
                    internalformat,
                    width,
                    height,
                    fixedsamplelocations,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> (),
                >(self.glTexStorage2DMultisample_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_multisamplefv(
        &mut self,
        pname: GLenum,
        index: GLuint,
        val: *mut GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_info_val = ParamInfo::new("*mut GLfloat", "val");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_multisamplefv".to_string();

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_val);
            param_values.push(&val);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetMultisamplefv(pname, index, val);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(
                        self.glGetMultisamplefv_ptr,
                    )(pname, index, val);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetMultisamplefv(pname, index, val);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(
                    self.glGetMultisamplefv_ptr,
                )(pname, index, val);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_sample_mask_i(&mut self, maskNumber: GLuint, mask: GLbitfield) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_maskNumber = ParamInfo::new("GLuint", "maskNumber");

            let mut param_info_mask = ParamInfo::new("GLbitfield", "mask");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sample_mask_i".to_string();

            param_infos.push(&param_info_maskNumber);
            param_values.push(&maskNumber);

            param_infos.push(&param_info_mask);
            param_values.push(&mask);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSampleMaski(maskNumber, mask);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(
                        self.glSampleMaski_ptr,
                    )(maskNumber, mask);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSampleMaski(maskNumber, mask);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(
                    self.glSampleMaski_ptr,
                )(maskNumber, mask);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_tex_level_parameter_iv(
        &mut self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_level_parameter_iv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetTexLevelParameteriv(target, level, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> (),
                    >(self.glGetTexLevelParameteriv_ptr)(
                        target, level, pname, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetTexLevelParameteriv(target, level, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(
                    self.glGetTexLevelParameteriv_ptr,
                )(target, level, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_tex_level_parameter_fv(
        &mut self,
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLfloat", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_level_parameter_fv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetTexLevelParameterfv(target, level, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> (),
                    >(self.glGetTexLevelParameterfv_ptr)(
                        target, level, pname, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetTexLevelParameterfv(target, level, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> (),
                >(self.glGetTexLevelParameterfv_ptr)(target, level, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_bind_vertex_buffer(
        &mut self,
        binding_index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_binding_index = ParamInfo::new("GLuint", "binding_index");

            let mut param_info_buffer = ParamInfo::new("GLuint", "buffer");

            let mut param_info_offset = ParamInfo::new("GLintptr", "offset");

            let mut param_info_stride = ParamInfo::new("GLsizei", "stride");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_bind_vertex_buffer".to_string();

            param_infos.push(&param_info_binding_index);
            param_values.push(&binding_index);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            param_infos.push(&param_info_stride);
            param_values.push(&stride);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBindVertexBuffer(binding_index, buffer, offset, stride);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> (),
                    >(self.glBindVertexBuffer_ptr)(
                        binding_index, buffer, offset, stride
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBindVertexBuffer(binding_index, buffer, offset, stride);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>(
                    self.glBindVertexBuffer_ptr,
                )(binding_index, buffer, offset, stride);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_vertex_attrib_format(
        &mut self,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_attribindex = ParamInfo::new("GLuint", "attribindex");

            let mut param_info_size = ParamInfo::new("GLint", "size");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_normalized = ParamInfo::new("GLboolean", "normalized");

            let mut param_info_relativeoffset = ParamInfo::new("GLuint", "relativeoffset");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_format".to_string();

            param_infos.push(&param_info_attribindex);
            param_values.push(&attribindex);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_normalized);
            param_values.push(&normalized);

            param_infos.push(&param_info_relativeoffset);
            param_values.push(&relativeoffset);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribFormat(attribindex, size, type_, normalized, relativeoffset);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> (),
                    >(self.glVertexAttribFormat_ptr)(
                        attribindex,
                        size,
                        type_,
                        normalized,
                        relativeoffset,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribFormat(attribindex, size, type_, normalized, relativeoffset);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> (),
                >(self.glVertexAttribFormat_ptr)(
                    attribindex,
                    size,
                    type_,
                    normalized,
                    relativeoffset,
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_vertex_attrib_I_format(
        &mut self,
        attri_bindex: GLuint,
        size: GLint,
        type_: GLenum,
        relative_offset: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_attri_bindex = ParamInfo::new("GLuint", "attri_bindex");

            let mut param_info_size = ParamInfo::new("GLint", "size");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_relative_offset = ParamInfo::new("GLuint", "relative_offset");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_I_format".to_string();

            param_infos.push(&param_info_attri_bindex);
            param_values.push(&attri_bindex);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_relative_offset);
            param_values.push(&relative_offset);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribIFormat(attri_bindex, size, type_, relative_offset);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(
                        self.glVertexAttribIFormat_ptr,
                    )(attri_bindex, size, type_, relative_offset);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribIFormat(attri_bindex, size, type_, relative_offset);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(
                    self.glVertexAttribIFormat_ptr,
                )(attri_bindex, size, type_, relative_offset);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_vertex_attrib_binding(
        &mut self,
        attri_bindex: GLuint,
        binding_index: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_attri_bindex = ParamInfo::new("GLuint", "attri_bindex");

            let mut param_info_binding_index = ParamInfo::new("GLuint", "binding_index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_attrib_binding".to_string();

            param_infos.push(&param_info_attri_bindex);
            param_values.push(&attri_bindex);

            param_infos.push(&param_info_binding_index);
            param_values.push(&binding_index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexAttribBinding(attri_bindex, binding_index);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                        self.glVertexAttribBinding_ptr,
                    )(attri_bindex, binding_index);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexAttribBinding(attri_bindex, binding_index);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glVertexAttribBinding_ptr,
                )(attri_bindex, binding_index);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_vertex_binding_divisor(
        &mut self,
        binding_index: GLuint,
        divisor: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_binding_index = ParamInfo::new("GLuint", "binding_index");

            let mut param_info_divisor = ParamInfo::new("GLuint", "divisor");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_vertex_binding_divisor".to_string();

            param_infos.push(&param_info_binding_index);
            param_values.push(&binding_index);

            param_infos.push(&param_info_divisor);
            param_values.push(&divisor);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glVertexBindingDivisor(binding_index, divisor);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                        self.glVertexBindingDivisor_ptr,
                    )(binding_index, divisor);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glVertexBindingDivisor(binding_index, divisor);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
                    self.glVertexBindingDivisor_ptr,
                )(binding_index, divisor);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_blend_barrier(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_barrier".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlendBarrier();
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn() -> ()>(self.glBlendBarrier_ptr)();
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlendBarrier();
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn() -> ()>(self.glBlendBarrier_ptr)();
            }

            Ok(())
        }
    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_srcName = ParamInfo::new("GLuint", "srcName");

            let mut param_info_srcTarget = ParamInfo::new("GLenum", "srcTarget");

            let mut param_info_srcLevel = ParamInfo::new("GLint", "srcLevel");

            let mut param_info_srcX = ParamInfo::new("GLint", "srcX");

            let mut param_info_srcY = ParamInfo::new("GLint", "srcY");

            let mut param_info_srcZ = ParamInfo::new("GLint", "srcZ");

            let mut param_info_dst_Name = ParamInfo::new("GLuint", "dst_Name");

            let mut param_info_dst_Target = ParamInfo::new("GLenum", "dst_Target");

            let mut param_info_dst_Level = ParamInfo::new("GLint", "dst_Level");

            let mut param_info_dst_X = ParamInfo::new("GLint", "dst_X");

            let mut param_info_dst_Y = ParamInfo::new("GLint", "dst_Y");

            let mut param_info_dst_Z = ParamInfo::new("GLint", "dst_Z");

            let mut param_info_src_Width = ParamInfo::new("GLsizei", "src_Width");

            let mut param_info_src_Height = ParamInfo::new("GLsizei", "src_Height");

            let mut param_info_src_Depth = ParamInfo::new("GLsizei", "src_Depth");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_copy_image_sub_data".to_string();

            param_infos.push(&param_info_srcName);
            param_values.push(&srcName);

            param_infos.push(&param_info_srcTarget);
            param_values.push(&srcTarget);

            param_infos.push(&param_info_srcLevel);
            param_values.push(&srcLevel);

            param_infos.push(&param_info_srcX);
            param_values.push(&srcX);

            param_infos.push(&param_info_srcY);
            param_values.push(&srcY);

            param_infos.push(&param_info_srcZ);
            param_values.push(&srcZ);

            param_infos.push(&param_info_dst_Name);
            param_values.push(&dst_Name);

            param_infos.push(&param_info_dst_Target);
            param_values.push(&dst_Target);

            param_infos.push(&param_info_dst_Level);
            param_values.push(&dst_Level);

            param_infos.push(&param_info_dst_X);
            param_values.push(&dst_X);

            param_infos.push(&param_info_dst_Y);
            param_values.push(&dst_Y);

            param_infos.push(&param_info_dst_Z);
            param_values.push(&dst_Z);

            param_infos.push(&param_info_src_Width);
            param_values.push(&src_Width);

            param_infos.push(&param_info_src_Height);
            param_values.push(&src_Height);

            param_infos.push(&param_info_src_Depth);
            param_values.push(&src_Depth);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glCopyImageSubData(
                        srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target,
                        dst_Level, dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
                    );
                    #[cfg(target_os = "android")]
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
                    >(self.glCopyImageSubData_ptr)(
                        srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target,
                        dst_Level, dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glCopyImageSubData(
                    srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target,
                    dst_Level, dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
                );
                #[cfg(target_os = "android")]
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
                >(self.glCopyImageSubData_ptr)(
                    srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dst_Name, dst_Target,
                    dst_Level, dst_X, dst_Y, dst_Z, src_Width, src_Height, src_Depth,
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_debug_message_control(
        &mut self,
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_source = ParamInfo::new("GLenum", "source");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_severity = ParamInfo::new("GLenum", "severity");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_ids = ParamInfo::new("*const GLuint", "ids");

            let mut param_info_enabled = ParamInfo::new("GLboolean", "enabled");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_debug_message_control".to_string();

            param_infos.push(&param_info_source);
            param_values.push(&source);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_severity);
            param_values.push(&severity);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_ids);
            param_values.push(&ids);

            param_infos.push(&param_info_enabled);
            param_values.push(&enabled);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDebugMessageControl(source, type_, severity, count, ids, enabled);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLenum,
                            GLenum,
                            GLsizei,
                            *const GLuint,
                            GLboolean,
                        ) -> (),
                    >(self.glDebugMessageControl_ptr)(
                        source, type_, severity, count, ids, enabled
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDebugMessageControl(source, type_, severity, count, ids, enabled);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean)
                        -> (),
                >(self.glDebugMessageControl_ptr)(
                    source, type_, severity, count, ids, enabled
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_debug_message_insert(
        &mut self,
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_source = ParamInfo::new("GLenum", "source");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_id = ParamInfo::new("GLuint", "id");

            let mut param_info_severity = ParamInfo::new("GLenum", "severity");

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_info_buf = ParamInfo::new("*const GLchar", "buf");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_debug_message_insert".to_string();

            param_infos.push(&param_info_source);
            param_values.push(&source);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_id);
            param_values.push(&id);

            param_infos.push(&param_info_severity);
            param_values.push(&severity);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_buf);
            param_values.push(&buf);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDebugMessageInsert(source, type_, id, severity, length, buf);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar)
                            -> (),
                    >(self.glDebugMessageInsert_ptr)(
                        source, type_, id, severity, length, buf
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDebugMessageInsert(source, type_, id, severity, length, buf);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar)
                        -> (),
                >(self.glDebugMessageInsert_ptr)(
                    source, type_, id, severity, length, buf
                );
            }
            Ok(())
        }
    }
//    pub fn gl_debug_message_callback(
//        &mut self,
//        callback: GLDEBUGPROC,
//        userParam: *const GLvoid,
//    ) -> Result<(), Error> {
//        if self.is_debug() {
//            let mut param_info_callback = ParamInfo::new("GLDEBUGPROC", "callback");
//
//            let mut param_info_userParam = ParamInfo::new("*const GLvoid", "userParam");
//
//            let mut param_values: Vec<&Param> = vec![];
//            let mut param_infos: Vec<&ParamInfo> = vec![];
//
//            let mut func_info = FuncInfo::new();
//            func_info.func_name = "gl_debug_message_callback".to_string();
//
//            param_infos.push(&param_info_callback);
//            param_values.push(&callback);
//
//            param_infos.push(&param_info_userParam);
//            param_values.push(&userParam);
//
//            func_info.func_param_infos = param_infos;
//            func_info.func_param_values = param_values;
//            self.pre_process(&func_info)?;
//
//            let res = {
//                unsafe {
//                    #[cfg(target_os = "ios")]
//                    ffi::glDebugMessageCallback(callback, userParam);
//                    #[cfg(target_os = "android")]
//                    std::mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const GLvoid) -> ()>(
//                        self.glDebugMessageCallback_ptr,
//                    )(callback, userParam);
//                }
//                Ok(())
//            };
//
//            let res_desc = format!("{:?}", res);
//
//            self.post_process(&func_info, &res_desc)?;
//
//            res
//        } else {
//            unsafe {
//                #[cfg(target_os = "ios")]
//                ffi::glDebugMessageCallback(callback, userParam);
//                #[cfg(target_os = "android")]
//                std::mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const GLvoid) -> ()>(
//                    self.glDebugMessageCallback_ptr,
//                )(callback, userParam);
//            }
//            Ok(())
//        }
//    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_count = ParamInfo::new("GLuint", "count");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_sources = ParamInfo::new("*mut GLenum", "sources");

            let mut param_info_types = ParamInfo::new("*mut GLenum", "types");

            let mut param_info_ids = ParamInfo::new("*mut GLuint", "ids");

            let mut param_info_severities = ParamInfo::new("*mut GLenum", "severities");

            let mut param_info_lengths = ParamInfo::new("*mut GLsizei", "lengths");

            let mut param_info_message_log = ParamInfo::new("*mut GLchar", "message_log");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_debug_message_Log".to_string();

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_sources);
            param_values.push(&sources);

            param_infos.push(&param_info_types);
            param_values.push(&types);

            param_infos.push(&param_info_ids);
            param_values.push(&ids);

            param_infos.push(&param_info_severities);
            param_values.push(&severities);

            param_infos.push(&param_info_lengths);
            param_values.push(&lengths);

            param_infos.push(&param_info_message_log);
            param_values.push(&message_log);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
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
                    #[cfg(target_os = "android")]
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
                        ) -> GLuint,
                    >(self.glGetDebugMessageLog_ptr)(
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
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
                #[cfg(target_os = "android")]
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
                    ) -> GLuint,
                >(self.glGetDebugMessageLog_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_push_debug_group(
        &mut self,
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_source = ParamInfo::new("GLenum", "source");

            let mut param_info_id = ParamInfo::new("GLuint", "id");

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_info_message = ParamInfo::new("*const GLchar", "message");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_push_debug_group".to_string();

            param_infos.push(&param_info_source);
            param_values.push(&source);

            param_infos.push(&param_info_id);
            param_values.push(&id);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_message);
            param_values.push(&message);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glPushDebugGroup(source, id, length, message);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                    >(self.glPushDebugGroup_ptr)(source, id, length, message);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glPushDebugGroup(source, id, length, message);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                >(self.glPushDebugGroup_ptr)(source, id, length, message);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_pop_debug_group(&mut self) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_pop_debug_group".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glPopDebugGroup();
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn() -> ()>(self.glPopDebugGroup_ptr)(
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glPopDebugGroup();
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn() -> ()>(self.glPopDebugGroup_ptr)();
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_object_label(
        &mut self,
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_identifier = ParamInfo::new("GLenum", "identifier");

            let mut param_info_name = ParamInfo::new("GLuint", "name");

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_info_label = ParamInfo::new("*const GLchar", "label");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_object_label".to_string();

            param_infos.push(&param_info_identifier);
            param_values.push(&identifier);

            param_infos.push(&param_info_name);
            param_values.push(&name);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_label);
            param_values.push(&label);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glObjectLabel(identifier, name, length, label);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                    >(self.glObjectLabel_ptr)(identifier, name, length, label);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glObjectLabel(identifier, name, length, label);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> (),
                >(self.glObjectLabel_ptr)(identifier, name, length, label);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_object_label(
        &mut self,
        ptr: *const GLvoid,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_ptr = ParamInfo::new("*const GLvoid", "ptr");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_length = ParamInfo::new("*mut GLsizei", "length");

            let mut param_info_label = ParamInfo::new("*mut GLchar", "label");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_object_label".to_string();

            param_infos.push(&param_info_ptr);
            param_values.push(&ptr);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_label);
            param_values.push(&label);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(*const GLvoid, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                    >(self.glGetObjectPtrLabel_ptr)(ptr, bufSize, length, label);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(*const GLvoid, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetObjectPtrLabel_ptr)(ptr, bufSize, length, label);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        length: GLsizei,
        label: *const GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_ptr = ParamInfo::new("*const GLvoid", "ptr");

            let mut param_info_length = ParamInfo::new("GLsizei", "length");

            let mut param_info_label = ParamInfo::new("*const GLchar", "label");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_object_ptr_label".to_string();

            param_infos.push(&param_info_ptr);
            param_values.push(&ptr);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_label);
            param_values.push(&label);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glObjectPtrLabel(ptr, length, label);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(*const GLvoid, GLsizei, *const GLchar) -> (),
                    >(self.glObjectPtrLabel_ptr)(ptr, length, label);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glObjectPtrLabel(ptr, length, label);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(*const GLvoid, GLsizei, *const GLchar) -> (),
                >(self.glObjectPtrLabel_ptr)(ptr, length, label);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_object_ptr_label(
        &mut self,
        ptr: *const GLvoid,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_ptr = ParamInfo::new("*const GLvoid", "ptr");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_length = ParamInfo::new("*mut GLsizei", "length");

            let mut param_info_label = ParamInfo::new("*mut GLchar", "label");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_object_ptr_label".to_string();

            param_infos.push(&param_info_ptr);
            param_values.push(&ptr);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_length);
            param_values.push(&length);

            param_infos.push(&param_info_label);
            param_values.push(&label);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(*const GLvoid, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                    >(self.glGetObjectPtrLabel_ptr)(ptr, bufSize, length, label);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetObjectPtrLabel(ptr, bufSize, length, label);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(*const GLvoid, GLsizei, *mut GLsizei, *mut GLchar) -> (),
                >(self.glGetObjectPtrLabel_ptr)(ptr, bufSize, length, label);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_pointer_v(
        &mut self,
        pname: GLenum,
        params: *mut *mut GLvoid,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut *mut GLvoid", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_pointer_v".to_string();

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetPointerv(pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, *mut *mut GLvoid) -> ()>(
                        self.glGetPointerv_ptr,
                    )(pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetPointerv(pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, *mut *mut GLvoid) -> ()>(
                    self.glGetPointerv_ptr,
                )(pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_enable_i(&mut self, target: GLenum, index: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_enable_i".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glEnablei(target, index);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                        self.glEnablei_ptr,
                    )(target, index);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glEnablei(target, index);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glEnablei_ptr,
                )(target, index);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_disable_i(&mut self, target: GLenum, index: GLuint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_disable_i".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDisablei(target, index);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                        self.glDisablei_ptr,
                    )(target, index);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDisablei(target, index);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
                    self.glDisablei_ptr,
                )(target, index);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_blend_equation_i(&mut self, buf: GLuint, mode: GLenum) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buf = ParamInfo::new("GLuint", "buf");

            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_equation_i".to_string();

            param_infos.push(&param_info_buf);
            param_values.push(&buf);

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlendEquationi(buf, mode);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
                        self.glBlendEquationi_ptr,
                    )(buf, mode);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlendEquationi(buf, mode);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
                    self.glBlendEquationi_ptr,
                )(buf, mode);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_blend_equation_separate_i(
        &mut self,
        buf: GLuint,
        mode_RGB: GLenum,
        mode_alpha: GLenum,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buf = ParamInfo::new("GLuint", "buf");

            let mut param_info_mode_RGB = ParamInfo::new("GLenum", "mode_RGB");

            let mut param_info_mode_alpha = ParamInfo::new("GLenum", "mode_alpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_equation_separate_i".to_string();

            param_infos.push(&param_info_buf);
            param_values.push(&buf);

            param_infos.push(&param_info_mode_RGB);
            param_values.push(&mode_RGB);

            param_infos.push(&param_info_mode_alpha);
            param_values.push(&mode_alpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlendEquationSeparatei(buf, mode_RGB, mode_alpha);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                        self.glBlendEquationSeparatei_ptr,
                    )(buf, mode_RGB, mode_alpha);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlendEquationSeparatei(buf, mode_RGB, mode_alpha);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                    self.glBlendEquationSeparatei_ptr,
                )(buf, mode_RGB, mode_alpha);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_blend_func_i(&mut self, buf: GLuint, src: GLenum, dst: GLenum) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buf = ParamInfo::new("GLuint", "buf");

            let mut param_info_src = ParamInfo::new("GLenum", "src");

            let mut param_info_dst = ParamInfo::new("GLenum", "dst");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_func_i".to_string();

            param_infos.push(&param_info_buf);
            param_values.push(&buf);

            param_infos.push(&param_info_src);
            param_values.push(&src);

            param_infos.push(&param_info_dst);
            param_values.push(&dst);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlendFunci(buf, src, dst);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                        self.glBlendFunci_ptr,
                    )(buf, src, dst);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlendFunci(buf, src, dst);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
                    self.glBlendFunci_ptr,
                )(buf, src, dst);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_blend_func_separate_i(
        &mut self,
        buf: GLuint,
        src_rgb: GLenum,
        d_stRG_b: GLenum,
        _srcAlpha: GLenum,
        d_stAlpha: GLenum,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_buf = ParamInfo::new("GLuint", "buf");

            let mut param_info_src_rgb = ParamInfo::new("GLenum", "src_rgb");

            let mut param_info_d_stRG_b = ParamInfo::new("GLenum", "d_stRG_b");

            let mut param_info__srcAlpha = ParamInfo::new("GLenum", "_srcAlpha");

            let mut param_info_d_stAlpha = ParamInfo::new("GLenum", "d_stAlpha");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_blend_func_separate_i".to_string();

            param_infos.push(&param_info_buf);
            param_values.push(&buf);

            param_infos.push(&param_info_src_rgb);
            param_values.push(&src_rgb);

            param_infos.push(&param_info_d_stRG_b);
            param_values.push(&d_stRG_b);

            param_infos.push(&param_info__srcAlpha);
            param_values.push(&_srcAlpha);

            param_infos.push(&param_info_d_stAlpha);
            param_values.push(&d_stAlpha);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glBlendFuncSeparatei(buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> (),
                    >(self.glBlendFuncSeparatei_ptr)(
                        buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glBlendFuncSeparatei(buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> (),
                >(self.glBlendFuncSeparatei_ptr)(
                    buf, src_rgb, d_stRG_b, _srcAlpha, d_stAlpha
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_color_mask_i(
        &mut self,
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_info_r = ParamInfo::new("GLboolean", "r");

            let mut param_info_g = ParamInfo::new("GLboolean", "g");

            let mut param_info_b = ParamInfo::new("GLboolean", "b");

            let mut param_info_a = ParamInfo::new("GLboolean", "a");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_color_mask_i".to_string();

            param_infos.push(&param_info_index);
            param_values.push(&index);

            param_infos.push(&param_info_r);
            param_values.push(&r);

            param_infos.push(&param_info_g);
            param_values.push(&g);

            param_infos.push(&param_info_b);
            param_values.push(&b);

            param_infos.push(&param_info_a);
            param_values.push(&a);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glColorMaski(index, r, g, b, a);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean)
                            -> (),
                    >(self.glColorMaski_ptr)(index, r, g, b, a);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glColorMaski(index, r, g, b, a);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> (),
                >(self.glColorMaski_ptr)(index, r, g, b, a);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_is_enabled_i(&mut self, target: GLenum, index: GLuint) -> Result<bool, Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_index = ParamInfo::new("GLuint", "index");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_is_enabled_i".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_index);
            param_values.push(&index);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glIsEnabledi(target, index);
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLuint) -> GLboolean,
                    >(self.glIsEnabledi_ptr)(target, index);

                    if result == GL_TRUE {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glIsEnabledi(target, index);
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLuint) -> GLboolean,
                >(self.glIsEnabledi_ptr)(target, index);

                if result == GL_TRUE {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_draw_elements_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("*const GLvoid", "indices");

            let mut param_info_base_vertex = ParamInfo::new("GLint", "base_vertex");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_elements_base_vertex".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&indices);

            param_infos.push(&param_info_base_vertex);
            param_values.push(&base_vertex);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawElementsBaseVertex(mode, count, type_, indices, base_vertex);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLint) -> (),
                    >(self.glDrawElementsBaseVertex_ptr)(
                        mode, count, type_, indices, base_vertex
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawElementsBaseVertex(mode, count, type_, indices, base_vertex);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLint) -> (),
                >(self.glDrawElementsBaseVertex_ptr)(
                    mode, count, type_, indices, base_vertex
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_info_start = ParamInfo::new("GLuint", "start");

            let mut param_info_end = ParamInfo::new("GLuint", "end");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("*const GLvoid", "indices");

            let mut param_info_base_vertex = ParamInfo::new("GLint", "base_vertex");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_range_elements_base_vertex".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_start);
            param_values.push(&start);

            param_infos.push(&param_info_end);
            param_values.push(&end);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&indices);

            param_infos.push(&param_info_base_vertex);
            param_values.push(&base_vertex);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawRangeElementsBaseVertex(
                        mode,
                        start,
                        end,
                        count,
                        type_,
                        indices,
                        base_vertex,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(
                            GLenum,
                            GLuint,
                            GLuint,
                            GLsizei,
                            GLenum,
                            *const GLvoid,
                            GLint,
                        ) -> (),
                    >(self.glDrawRangeElementsBaseVertex_ptr)(
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawRangeElementsBaseVertex(
                    mode,
                    start,
                    end,
                    count,
                    type_,
                    indices,
                    base_vertex,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(
                        GLenum,
                        GLuint,
                        GLuint,
                        GLsizei,
                        GLenum,
                        *const GLvoid,
                        GLint,
                    ) -> (),
                >(self.glDrawRangeElementsBaseVertex_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_draw_elements_instanced_base_vertex(
        &mut self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
        instance_count: GLsizei,
        base_vertex: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_mode = ParamInfo::new("GLenum", "mode");

            let mut param_info_count = ParamInfo::new("GLsizei", "count");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_indices = ParamInfo::new("*const GLvoid", "indices");

            let mut param_info_instance_count = ParamInfo::new("GLsizei", "instance_count");

            let mut param_info_base_vertex = ParamInfo::new("GLint", "base_vertex");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_draw_elements_instanced_base_vertex".to_string();

            param_infos.push(&param_info_mode);
            param_values.push(&mode);

            param_infos.push(&param_info_count);
            param_values.push(&count);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_indices);
            param_values.push(&indices);

            param_infos.push(&param_info_instance_count);
            param_values.push(&instance_count);

            param_infos.push(&param_info_base_vertex);
            param_values.push(&base_vertex);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glDrawElementsInstancedBaseVertex(
                        mode,
                        count,
                        type_,
                        indices,
                        instance_count,
                        base_vertex,
                    );
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLsizei, GLint)
                            -> (),
                    >(self.glDrawElementsInstancedBaseVertex_ptr)(
                        mode,
                        count,
                        type_,
                        indices,
                        instance_count,
                        base_vertex,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glDrawElementsInstancedBaseVertex(
                    mode,
                    count,
                    type_,
                    indices,
                    instance_count,
                    base_vertex,
                );
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLsizei, GLenum, *const GLvoid, GLsizei, GLint)
                        -> (),
                >(self.glDrawElementsInstancedBaseVertex_ptr)(
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
    }

    #[cfg(target_os = "android")]
    pub fn gl_frame_buffer_texture(
        &mut self,
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_attachment = ParamInfo::new("GLenum", "attachment");

            let mut param_info_texture = ParamInfo::new("GLuint", "texture");

            let mut param_info_level = ParamInfo::new("GLint", "level");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_frame_buffer_texture".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_attachment);
            param_values.push(&attachment);

            param_infos.push(&param_info_texture);
            param_values.push(&texture);

            param_infos.push(&param_info_level);
            param_values.push(&level);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glFramebufferTexture(target, attachment, texture, level);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(
                        self.glFramebufferTexture_ptr,
                    )(target, attachment, texture, level);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glFramebufferTexture(target, attachment, texture, level);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(
                    self.glFramebufferTexture_ptr,
                )(target, attachment, texture, level);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_minX = ParamInfo::new("GLfloat", "minX");

            let mut param_info_minY = ParamInfo::new("GLfloat", "minY");

            let mut param_info_minZ = ParamInfo::new("GLfloat", "minZ");

            let mut param_info_minW = ParamInfo::new("GLfloat", "minW");

            let mut param_info_maxX = ParamInfo::new("GLfloat", "maxX");

            let mut param_info_maxY = ParamInfo::new("GLfloat", "maxY");

            let mut param_info_maxZ = ParamInfo::new("GLfloat", "maxZ");

            let mut param_info_maxW = ParamInfo::new("GLfloat", "maxW");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_primitive_bounding_box".to_string();

            param_infos.push(&param_info_minX);
            param_values.push(&minX);

            param_infos.push(&param_info_minY);
            param_values.push(&minY);

            param_infos.push(&param_info_minZ);
            param_values.push(&minZ);

            param_infos.push(&param_info_minW);
            param_values.push(&minW);

            param_infos.push(&param_info_maxX);
            param_values.push(&maxX);

            param_infos.push(&param_info_maxY);
            param_values.push(&maxY);

            param_infos.push(&param_info_maxZ);
            param_values.push(&maxZ);

            param_infos.push(&param_info_maxW);
            param_values.push(&maxW);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glPrimitiveBoundingBox(minX, minY, minZ, minW, maxX, maxY, maxZ, maxW);
                    #[cfg(target_os = "android")]
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
                    >(self.glPrimitiveBoundingBox_ptr)(
                        minX, minY, minZ, minW, maxX, maxY, maxZ, maxW,
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glPrimitiveBoundingBox(minX, minY, minZ, minW, maxX, maxY, maxZ, maxW);
                #[cfg(target_os = "android")]
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
                >(self.glPrimitiveBoundingBox_ptr)(
                    minX, minY, minZ, minW, maxX, maxY, maxZ, maxW
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_graphics_reset_status(&mut self) -> Result<GLenum, Error> {
        if self.is_debug() {
            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_graphics_reset_status".to_string();

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    let result = ffi::glGetGraphicsResetStatus();
                    #[cfg(target_os = "android")]
                    let result = std::mem::transmute::<_, extern "system" fn() -> GLenum>(
                        self.glGetGraphicsResetStatus_ptr,
                    )();

                    Ok(result)
                }
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                let result = ffi::glGetGraphicsResetStatus();
                #[cfg(target_os = "android")]
                let result = std::mem::transmute::<_, extern "system" fn() -> GLenum>(
                    self.glGetGraphicsResetStatus_ptr,
                )();

                Ok(result)
            }
        }
    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_x = ParamInfo::new("GLint", "x");

            let mut param_info_y = ParamInfo::new("GLint", "y");

            let mut param_info_width = ParamInfo::new("GLsizei", "width");

            let mut param_info_height = ParamInfo::new("GLsizei", "height");

            let mut param_info_format = ParamInfo::new("GLenum", "format");

            let mut param_info_type_ = ParamInfo::new("GLenum", "type_");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_data = ParamInfo::new("*mut GLvoid", "data");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_read_n_pixels".to_string();

            param_infos.push(&param_info_x);
            param_values.push(&x);

            param_infos.push(&param_info_y);
            param_values.push(&y);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_format);
            param_values.push(&format);

            param_infos.push(&param_info_type_);
            param_values.push(&type_);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_data);
            param_values.push(&data);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glReadnPixels(x, y, width, height, format, type_, bufSize, data);
                    #[cfg(target_os = "android")]
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
                            *mut GLvoid,
                        ) -> (),
                    >(self.glReadnPixels_ptr)(
                        x, y, width, height, format, type_, bufSize, data
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glReadnPixels(x, y, width, height, format, type_, bufSize, data);
                #[cfg(target_os = "android")]
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
                        *mut GLvoid,
                    ) -> (),
                >(self.glReadnPixels_ptr)(
                    x, y, width, height, format, type_, bufSize, data
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_n_uniform_fv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_params = ParamInfo::new("*mut GLfloat", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_n_uniform_fv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetnUniformfv(program, location, bufSize, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> (),
                    >(self.glGetnUniformfv_ptr)(
                        program, location, bufSize, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetnUniformfv(program, location, bufSize, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> (),
                >(self.glGetnUniformfv_ptr)(program, location, bufSize, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_n_uniform_iv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_n_uniform_iv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetnUniformiv(program, location, bufSize, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> (),
                    >(self.glGetnUniformiv_ptr)(
                        program, location, bufSize, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetnUniformiv(program, location, bufSize, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> (),
                >(self.glGetnUniformiv_ptr)(program, location, bufSize, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_n_uniform_uiv(
        &mut self,
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_program = ParamInfo::new("GLuint", "program");

            let mut param_info_location = ParamInfo::new("GLint", "location");

            let mut param_info_bufSize = ParamInfo::new("GLsizei", "bufSize");

            let mut param_info_params = ParamInfo::new("*mut GLuint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_n_uniform_uiv".to_string();

            param_infos.push(&param_info_program);
            param_values.push(&program);

            param_infos.push(&param_info_location);
            param_values.push(&location);

            param_infos.push(&param_info_bufSize);
            param_values.push(&bufSize);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetnUniformuiv(program, location, bufSize, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> (),
                    >(self.glGetnUniformuiv_ptr)(
                        program, location, bufSize, params
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetnUniformuiv(program, location, bufSize, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> (),
                >(self.glGetnUniformuiv_ptr)(program, location, bufSize, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_minsampleshading(&mut self, value: GLfloat) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_value = ParamInfo::new("GLfloat", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_minsampleshading".to_string();

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glMinSampleShading(value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
                        self.glMinSampleShading_ptr,
                    )(value);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glMinSampleShading(value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
                    self.glMinSampleShading_ptr,
                )(value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_patch_parameter_i(&mut self, pname: GLenum, value: GLint) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_value = ParamInfo::new("GLint", "value");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_patch_parameter_i".to_string();

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_value);
            param_values.push(&value);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glPatchParameteri(pname, value);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
                        self.glPatchParameteri_ptr,
                    )(pname, value);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glPatchParameteri(pname, value);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
                    self.glPatchParameteri_ptr,
                )(pname, value);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*const GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameter_iiv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexParameterIiv(target, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
                        self.glTexParameterIiv_ptr,
                    )(target, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexParameterIiv(target, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
                    self.glTexParameterIiv_ptr,
                )(target, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*const GLuint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_parameter_iuiv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexParameterIuiv(target, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
                        self.glTexParameterIuiv_ptr,
                    )(target, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexParameterIuiv(target, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
                    self.glTexParameterIuiv_ptr,
                )(target, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_tex_parameter_iiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_parameter_iiv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetTexParameterIiv(target, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                        self.glGetTexParameterIiv_ptr,
                    )(target, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetTexParameterIiv(target, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
                    self.glGetTexParameterIiv_ptr,
                )(target, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_tex_parameter_iuiv(
        &mut self,
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLuint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_tex_parameter_iuiv".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetTexParameterIuiv(target, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(
                        self.glGetTexParameterIuiv_ptr,
                    )(target, pname, params);
                }

                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetTexParameterIuiv(target, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(
                    self.glGetTexParameterIuiv_ptr,
                )(target, pname, params);
            }

            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_param = ParamInfo::new("*const GLint", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameter_iiv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameterIiv(sampler, pname, param);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                        self.glSamplerParameterIiv_ptr,
                    )(sampler, pname, param);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameterIiv(sampler, pname, param);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                    self.glSamplerParameterIiv_ptr,
                )(sampler, pname, param);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_param = ParamInfo::new("*const GLuint", "param");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_sampler_parameter_iuiv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_param);
            param_values.push(&param);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameterIuiv(sampler, pname, param);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(
                        self.glSamplerParameterIuiv_ptr,
                    )(sampler, pname, param);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameterIuiv(sampler, pname, param);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(
                    self.glSamplerParameterIuiv_ptr,
                )(sampler, pname, param);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_sampler_parameter_iiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_sampler_parameter_iiv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glSamplerParameterIiv(sampler, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                        self.glSamplerParameterIiv_ptr,
                    )(sampler, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glSamplerParameterIiv(sampler, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
                    self.glSamplerParameterIiv_ptr,
                )(sampler, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_get_sampler_parameter_iuiv(
        &mut self,
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_sampler = ParamInfo::new("GLuint", "sampler");

            let mut param_info_pname = ParamInfo::new("GLenum", "pname");

            let mut param_info_params = ParamInfo::new("*mut GLuint", "params");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_get_sampler_parameter_iuiv".to_string();

            param_infos.push(&param_info_sampler);
            param_values.push(&sampler);

            param_infos.push(&param_info_pname);
            param_values.push(&pname);

            param_infos.push(&param_info_params);
            param_values.push(&params);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glGetSamplerParameterIuiv(sampler, pname, params);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                        self.glGetSamplerParameterIuiv_ptr,
                    )(sampler, pname, params);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glGetSamplerParameterIuiv(sampler, pname, params);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
                    self.glGetSamplerParameterIuiv_ptr,
                )(sampler, pname, params);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_tex_buffer(
        &mut self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_buffer = ParamInfo::new("GLuint", "buffer");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_buffer".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexBuffer(target, internal_format, buffer);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
                        self.glTexBuffer_ptr,
                    )(target, internal_format, buffer);
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexBuffer(target, internal_format, buffer);
                #[cfg(target_os = "android")]
                std::mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
                    self.glTexBuffer_ptr,
                )(target, internal_format, buffer);
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
    pub fn gl_tex_buffer_range(
        &mut self,
        target: GLenum,
        internal_format: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    ) -> Result<(), Error> {
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_buffer = ParamInfo::new("GLuint", "buffer");

            let mut param_info_offset = ParamInfo::new("GLintptr", "offset");

            let mut param_info_size = ParamInfo::new("GLsizeiptr", "size");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_buffer_range".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_buffer);
            param_values.push(&buffer);

            param_infos.push(&param_info_offset);
            param_values.push(&offset);

            param_infos.push(&param_info_size);
            param_values.push(&size);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexBufferRange(target, internal_format, buffer, offset, size);
                    #[cfg(target_os = "android")]
                    std::mem::transmute::<
                        _,
                        extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> (),
                    >(self.glTexBufferRange_ptr)(
                        target, internal_format, buffer, offset, size
                    );
                }
                Ok(())
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexBufferRange(target, internal_format, buffer, offset, size);
                #[cfg(target_os = "android")]
                std::mem::transmute::<
                    _,
                    extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> (),
                >(self.glTexBufferRange_ptr)(
                    target, internal_format, buffer, offset, size
                );
            }
            Ok(())
        }
    }

    #[cfg(target_os = "android")]
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
        if self.is_debug() {
            let mut param_info_target = ParamInfo::new("GLenum", "target");

            let mut param_info_samples = ParamInfo::new("GLsizei", "samples");

            let mut param_info_internal_format = ParamInfo::new("GLenum", "internal_format");

            let mut param_info_width = ParamInfo::new("GLsizei", "width");

            let mut param_info_height = ParamInfo::new("GLsizei", "height");

            let mut param_info_depth = ParamInfo::new("GLsizei", "depth");

            let mut param_info_fixed_sample_locations =
                ParamInfo::new("GLboolean", "fixed_sample_locations");

            let mut param_values: Vec<&Param> = vec![];
            let mut param_infos: Vec<&ParamInfo> = vec![];

            let mut func_info = FuncInfo::new();
            func_info.func_name = "gl_tex_storage_3D_multi_sample".to_string();

            param_infos.push(&param_info_target);
            param_values.push(&target);

            param_infos.push(&param_info_samples);
            param_values.push(&samples);

            param_infos.push(&param_info_internal_format);
            param_values.push(&internal_format);

            param_infos.push(&param_info_width);
            param_values.push(&width);

            param_infos.push(&param_info_height);
            param_values.push(&height);

            param_infos.push(&param_info_depth);
            param_values.push(&depth);

            param_infos.push(&param_info_fixed_sample_locations);
            param_values.push(&fixed_sample_locations);

            func_info.func_param_infos = param_infos;
            func_info.func_param_values = param_values;
            self.pre_process(&func_info)?;

            let res = {
                unsafe {
                    #[cfg(target_os = "ios")]
                    ffi::glTexStorage3DMultisample(
                        target,
                        samples,
                        internal_format,
                        width,
                        height,
                        depth,
                        fixed_sample_locations,
                    );
                    #[cfg(target_os = "android")]
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
                    >(self.glTexStorage3DMultisample_ptr)(
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
            };

            let res_desc = format!("{:?}", res);

            self.post_process(&func_info, &res_desc)?;

            res
        } else {
            unsafe {
                #[cfg(target_os = "ios")]
                ffi::glTexStorage3DMultisample(
                    target,
                    samples,
                    internal_format,
                    width,
                    height,
                    depth,
                    fixed_sample_locations,
                );
                #[cfg(target_os = "android")]
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
                >(self.glTexStorage3DMultisample_ptr)(
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
}
