use super::data_struct::*;
use super::ffi::*;
use super::*;
use types::*;

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
