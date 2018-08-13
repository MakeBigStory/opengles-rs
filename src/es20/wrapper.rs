
use super::data_struct::*;
use super::ffi::*;
use super::*;
use types::TextureUnit;
use types::BufferTarget;
use types::FrameBufferTarget;
use types::RenderBufferTarget;
use types::TextureBindTarget;
use types::BlendEquationMode;
use types::BlendFactor;
use types::BufferUsage;
use types::FrameBufferStatus;
use types::TextureTarget;
use types::ShaderType;
use types::FaceMode;
use types::FuncType;
use types::FeatureType;
use types::BeginMode;
use types::FrameBufferAttachmentType;
use types::FrontFaceDirection;
use types::StateType;
use types::BufferParamName;
use types::ErrorType;
use types::FrameBufferAttachmentParamType;
use types::ProgramParamType;
use types::RenderBufferParamType;
use types::ShaderParamType;
use types::ShaderPrecisionType;
use types::ConstantType;
use types::TextureParamType;
use types::VertexAttributeParamType;
use types::HintTargetType;
use types::HintBehaviorType;
use types::PackParamType;
use types::PixelFormat;
use types::PixelDataType;
use types::ActionType;
use types::DataType;

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

#[derive(Debug)]
pub struct Error {

}

pub struct Wrapper {
}

trait Interceptor {
    fn intercept(&mut self, fun_name: &str) -> Result<(), Error>;
}

trait Param: std::fmt::Debug {
}

impl Param for i32 {
}

impl Param for u32 {
}

impl Param for f32 {
}

impl Param for bool {
}

impl Param for String {
}

impl Param for TextureUnit {
}

impl Param for BufferTarget {
}

impl Param for FrameBufferTarget {
}

impl Param for RenderBufferTarget {
}

impl Param for TextureBindTarget {
}

impl Param for BlendEquationMode {
}

impl Param for BlendFactor {
}

impl Param for BufferUsage {
}

impl Param for FrameBufferStatus {
}

impl Param for TextureTarget {
}

impl Param for ShaderType {
}

impl Param for FaceMode {
}

impl Param for FuncType {
}

impl Param for FeatureType {
}

impl Param for BeginMode {
}

impl Param for FrameBufferAttachmentType {
}

impl Param for FrontFaceDirection {
}

impl Param for StateType {
}

impl Param for BufferParamName {
}

impl Param for ErrorType {
}

impl Param for FrameBufferAttachmentParamType {
}

impl Param for ProgramParamType {
}

impl Param for RenderBufferParamType {
}

impl Param for ShaderParamType {
}

impl Param for ShaderPrecisionType {
}

impl Param for ConstantType {
}

impl Param for TextureParamType {
}

impl Param for VertexAttributeParamType {
}

impl Param for HintTargetType {
}

impl Param for HintBehaviorType {
}

impl Param for PackParamType {
}

impl Param for PixelFormat {
}

impl Param for PixelDataType {
}

impl Param for ActionType {
}

impl Param for DataType {
}

impl <T> Param for Vec<T> where T: std::fmt::Debug {
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
            param_name: param_name.to_string()
        }
    }
}

#[derive(Debug)]
struct FuncInfo<'a> {
    func_name: String,
    func_param_infos: Vec<&'a ParamInfo>,
    func_param_values: Vec<&'a Param>
}

impl<'a> FuncInfo<'a> {
    fn new() -> Self {
        FuncInfo {
            func_name: "".to_string(),
            func_param_infos: vec![],
            func_param_values: vec![]
        }
    }
}

impl Wrapper {

    fn pre_process(&mut self, func_info: &FuncInfo) -> Result<(), Error> {
        Ok(())
    }

    fn post_process(&mut self, func_info: &FuncInfo, error_desc: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn gl_active_texture(&mut self, texture_unit: TextureUnit) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_active_texture".to_string();

        let mut param_info = ParamInfo::new("texture_unit", "TextureUnit");
        param_infos.push(&param_info);
        param_values.push(&texture_unit);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glActiveTexture(texture_unit as GLenum);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_attach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_attach_shader".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glAttachShader(program as GLuint, shader as GLuint);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_bind_attrib_location(&mut self, program: u32, index: u32, name: &str) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_bind_attrib_location".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("name", "&str");
        param_infos.push(&param_info);
        let param_value = name.to_string();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let c_str = CString::new(name).unwrap();

                ffi::glBindAttribLocation(program as GLuint, index as GLuint,
                                          c_str.as_ptr() as *const c_char);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_bind_buffer(&mut self, target: BufferTarget, buffer: GLuint) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_bind_buffer".to_string();

        let mut param_info = ParamInfo::new("target", "BufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("buffer", "GLuint");
        param_infos.push(&param_info);
        param_values.push(&buffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBindBuffer(target as GLenum, buffer as GLuint);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_bind_framebuffer(&mut self, target: FrameBufferTarget, framebuffer: GLuint) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_bind_framebuffer".to_string();

        let mut param_info = ParamInfo::new("target", "FrameBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("framebuffer", "GLuint");
        param_infos.push(&param_info);
        param_values.push(&framebuffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBindFramebuffer(target as GLenum, framebuffer as GLuint);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_bind_renderbuffer(&mut self, target: RenderBufferTarget, renderbuffer: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_bind_renderbuffer".to_string();

        let mut param_info = ParamInfo::new("target", "RenderBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("renderbuffer", "u32");
        param_infos.push(&param_info);
        param_values.push(&renderbuffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBindRenderbuffer(target as GLenum, renderbuffer as GLuint);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_bind_texture(&mut self, target: TextureBindTarget, texture: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_bind_texture".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("texture", "u32");
        param_infos.push(&param_info);
        param_values.push(&texture);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBindTexture(target as GLenum, texture as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_blend_color(&mut self, red: f32, green: f32, blue: f32,
                          alpha: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_blend_color".to_string();

        let mut param_info = ParamInfo::new("red", "f32");
        param_infos.push(&param_info);
        param_values.push(&red);

        let mut param_info = ParamInfo::new("green", "f32");
        param_infos.push(&param_info);
        param_values.push(&green);

        let mut param_info = ParamInfo::new("blue", "f32");
        param_infos.push(&param_info);
        param_values.push(&blue);

        let mut param_info = ParamInfo::new("alpha", "f32");
        param_infos.push(&param_info);
        param_values.push(&alpha);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBlendColor(red as GLclampf, green as GLclampf,
                                  blue as GLclampf, alpha as GLclampf)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_blend_equation(&mut self, mode: BlendEquationMode) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_blend_equation".to_string();

        let mut param_info = ParamInfo::new("mode", "BlendEquationMode");
        param_infos.push(&param_info);
        param_values.push(&mode);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBlendEquation(mode as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_blend_equation_separate(&mut self, mode_rgb: BlendEquationMode, mode_alpha: BlendEquationMode)
                                      -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_blend_equation_separate".to_string();

        let mut param_info = ParamInfo::new("mode_rgb", "BlendEquationMode");
        param_infos.push(&param_info);
        param_values.push(&mode_rgb);

        let mut param_info = ParamInfo::new("mode_alpha", "BlendEquationMode");
        param_infos.push(&param_info);
        param_values.push(&mode_alpha);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBlendEquationSeparate(mode_rgb as GLenum, mode_alpha as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_blend_func(&mut self, src_factor: BlendFactor, dst_factor: BlendFactor) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_blend_func".to_string();

        let mut param_info = ParamInfo::new("src_factor", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&src_factor);

        let mut param_info = ParamInfo::new("dst_factor", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&dst_factor);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBlendFunc(src_factor as GLenum, dst_factor as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_blend_func_separate(&mut self, src_rgb: BlendFactor, dst_rgb: BlendFactor,
                                  src_alpha: BlendFactor, dst_alpha: BlendFactor) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_blend_func_separate".to_string();

        let mut param_info = ParamInfo::new("src_rgb", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&src_rgb);

        let mut param_info = ParamInfo::new("dst_rgb", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&dst_rgb);

        let mut param_info = ParamInfo::new("src_alpha", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&src_alpha);

        let mut param_info = ParamInfo::new("dst_alpha", "BlendFactor");
        param_infos.push(&param_info);
        param_values.push(&dst_alpha);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glBlendFuncSeparate(src_rgb as GLenum, dst_rgb as GLenum,
                                         src_alpha as GLenum, dst_alpha as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_buffer_data<T>(&mut self, target: BufferTarget,
                             buffer: &[T], usage: BufferUsage) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_buffer_data".to_string();

        let mut param_info = ParamInfo::new("target", "BufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

        let mut param_info = ParamInfo::new("usage", "BufferUsage");
        param_infos.push(&param_info);
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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_buffer_sub_data<T>(&mut self, target: BufferTarget, offset: u32, buffer: &[T])
                                 -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_buffer_sub_data".to_string();

        let mut param_info = ParamInfo::new("target", "BufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("offset", "u32");
        param_infos.push(&param_info);
        param_values.push(&offset);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_check_framebuffer_status(&mut self, target: FrameBufferTarget) -> Result<FrameBufferStatus, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_check_framebuffer_status".to_string();

        let mut param_info = ParamInfo::new("target", "FrameBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let status = ffi::glCheckFramebufferStatus(target as GLenum);

                Ok(FrameBufferStatus::from(status))
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_clear(&mut self, mask: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_clear".to_string();

        let mut param_info = ParamInfo::new("mask", "u32");
        param_infos.push(&param_info);
        param_values.push(&mask);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glClear(mask as GLbitfield)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_clear_color(&mut self, red: f32, green: f32,
                          blue: f32, alpha: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_clear_color".to_string();

        let mut param_info = ParamInfo::new("red", "f32");
        param_infos.push(&param_info);
        param_values.push(&red);

        let mut param_info = ParamInfo::new("green", "f32");
        param_infos.push(&param_info);
        param_values.push(&green);

        let mut param_info = ParamInfo::new("blue", "f32");
        param_infos.push(&param_info);
        param_values.push(&blue);

        let mut param_info = ParamInfo::new("alpha", "f32");
        param_infos.push(&param_info);
        param_values.push(&alpha);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glClearColor(red as GLclampf, green as GLclampf, blue as GLclampf,
                                  alpha as GLclampf)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_clear_depthf(&mut self, depth: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_clear_depthf".to_string();

        let mut param_info = ParamInfo::new("depth", "f32");
        param_infos.push(&param_info);
        param_values.push(&depth);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glClearDepthf(depth as GLclampf)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_clear_stencil(&mut self, stencil: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_clear_stencil".to_string();

        let mut param_info = ParamInfo::new("stencil", "i32");
        param_infos.push(&param_info);
        param_values.push(&stencil);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glClearStencil(stencil as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_color_mask(&mut self, red: bool, green: bool, blue: bool, alpha: bool) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_color_mask".to_string();

        let mut param_info = ParamInfo::new("red", "bool");
        param_infos.push(&param_info);
        param_values.push(&red);

        let mut param_info = ParamInfo::new("green", "bool");
        param_infos.push(&param_info);
        param_values.push(&green);

        let mut param_info = ParamInfo::new("blue", "bool");
        param_infos.push(&param_info);
        param_values.push(&blue);

        let mut param_info = ParamInfo::new("alpha", "bool");
        param_infos.push(&param_info);
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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_compile_shader(&mut self, shader: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_compile_shader".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glCompileShader(shader as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_compressed_tex_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "i32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("internal_format", "GLenum");
        param_infos.push(&param_info);
        param_values.push(&internal_format);

        let mut param_info = ParamInfo::new("width", "u32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "u32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("border", "u32");
        param_infos.push(&param_info);
        param_values.push(&border);

        let mut param_info = ParamInfo::new("image_size", "u32");
        param_infos.push(&param_info);
        param_values.push(&image_size);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_compressed_tex_sub_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "u32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("x_offset", "u32");
        param_infos.push(&param_info);
        param_values.push(&x_offset);

        let mut param_info = ParamInfo::new("y_offset", "u32");
        param_infos.push(&param_info);
        param_values.push(&y_offset);

        let mut param_info = ParamInfo::new("width", "u32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "u32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("format", "GLenum");
        param_infos.push(&param_info);
        param_values.push(&format);

        let mut param_info = ParamInfo::new("image_size", "u32");
        param_infos.push(&param_info);
        param_values.push(&image_size);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_copy_tex_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "u32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("internal_format", "GLenum");
        param_infos.push(&param_info);
        param_values.push(&internal_format);

        let mut param_info = ParamInfo::new("x", "u32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "u32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("width", "u32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "u32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("border", "u32");
        param_infos.push(&param_info);
        param_values.push(&border);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glCopyTexImage2D(target as GLenum,
                                      level as GLint,
                                      internal_format,
                                      x as GLint, y as GLint,
                                      width as GLsizei, height as GLsizei, border as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_copy_tex_sub_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "u32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("x_offset", "u32");
        param_infos.push(&param_info);
        param_values.push(&x_offset);

        let mut param_info = ParamInfo::new("y_offset", "u32");
        param_infos.push(&param_info);
        param_values.push(&y_offset);

        let mut param_info = ParamInfo::new("x", "u32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "u32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("width", "u32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "u32");
        param_infos.push(&param_info);
        param_values.push(&height);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glCopyTexSubImage2D(target as GLenum,
                                         level as GLint,
                                         x_offset as GLint, y_offset as GLint,
                                         x as GLint, y as GLint,
                                         width as GLsizei, height as GLsizei)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_create_program(&mut self) -> Result<u32, Error> {


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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_create_shader(&mut self, type_: ShaderType) -> Result<u32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_create_shader".to_string();

        let mut param_info = ParamInfo::new("type_", "ShaderType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let shader_id = ffi::glCreateShader(type_ as GLenum);

                Ok(shader_id as u32)
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_cull_face(&mut self, mode: FaceMode) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_cull_face".to_string();

        let mut param_info = ParamInfo::new("mode", "FaceMode");
        param_infos.push(&param_info);
        param_values.push(&mode);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glCullFace(mode as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_buffers(&mut self, buffers: &[u32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_buffers".to_string();

        let mut param_info = ParamInfo::new("buffers", "&[u32]");
        param_infos.push(&param_info);
        let param_value = buffers.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_framebuffers(&mut self, framebuffers: &[u32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_framebuffers".to_string();

        let mut param_info = ParamInfo::new("framebuffers", "&[u32]");
        param_infos.push(&param_info);
        let param_value = framebuffers.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_program(&mut self, program: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_program".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteProgram(program as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_renderbuffers(&mut self, renderbuffers: &[u32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_renderbuffers".to_string();

        let mut param_info = ParamInfo::new("renderbuffers", "&[u32]");
        param_infos.push(&param_info);
        let param_value = renderbuffers.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei,
                                           renderbuffers.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_shader(&mut self, shader: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_shader".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteShader(shader as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_delete_textures(&mut self, textures: &[u32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_delete_textures".to_string();

        let mut param_info = ParamInfo::new("textures", "&[u32]");
        param_infos.push(&param_info);
        let param_value = textures.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_depth_func(&mut self, func: FuncType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_depth_func".to_string();

        let mut param_info = ParamInfo::new("func", "FuncType");
        param_infos.push(&param_info);
        param_values.push(&func);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDepthFunc(func as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_depth_mask(&mut self, flag: bool) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_depth_mask".to_string();

        let mut param_info = ParamInfo::new("flag", "bool");
        param_infos.push(&param_info);
        param_values.push(&flag);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDepthMask(flag as GLboolean)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_depth_rangef(&mut self, z_near: f32, z_far: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_depth_rangef".to_string();

        let mut param_info = ParamInfo::new("z_near", "f32");
        param_infos.push(&param_info);
        param_values.push(&z_near);

        let mut param_info = ParamInfo::new("z_far", "f32");
        param_infos.push(&param_info);
        param_values.push(&z_far);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDepthRangef(z_near as GLclampf, z_far as GLclampf)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_detach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_detach_shader".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDetachShader(program as GLuint, shader as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_disable(&mut self, feature: FeatureType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_disable".to_string();

        let mut param_info = ParamInfo::new("feature", "FeatureType");
        param_infos.push(&param_info);
        param_values.push(&feature);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDisable(feature as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_disable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_disable_vertex_attrib_array".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDisableVertexAttribArray(index as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_draw_arrays(&mut self, mode: BeginMode, first: i32, count: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_draw_arrays".to_string();

        let mut param_info = ParamInfo::new("mode", "BeginMode");
        param_infos.push(&param_info);
        param_values.push(&mode);

        let mut param_info = ParamInfo::new("first", "i32");
        param_infos.push(&param_info);
        param_values.push(&first);

        let mut param_info = ParamInfo::new("count", "i32");
        param_infos.push(&param_info);
        param_values.push(&count);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDrawArrays(mode as GLenum, first as GLint, count as GLsizei)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_draw_elements<T>(&mut self, mode: BeginMode, count: i32, type_: GLenum, indices: &[T]) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_draw_elements".to_string();

        let mut param_info = ParamInfo::new("mode", "BeginMode");
        param_infos.push(&param_info);
        param_values.push(&mode);

        let mut param_info = ParamInfo::new("count", "i32");
        param_infos.push(&param_info);
        param_values.push(&count);

        let mut param_info = ParamInfo::new("type_", "GLenum");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("indices", "&[T]");
        param_infos.push(&param_info);
        let param_value = indices.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glDrawElements(mode as GLenum, count as GLsizei,
                                    type_, indices.as_ptr() as *const GLvoid)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_enable(&mut self, feature: FeatureType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_enable".to_string();

        let mut param_info = ParamInfo::new("feature", "FeatureType");
        param_infos.push(&param_info);
        param_values.push(&feature);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glEnable(feature as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_enable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_enable_vertex_attrib_array".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glEnableVertexAttribArray(index as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_finish(&mut self) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_finish".to_string();

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glFinish()
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_flush(&mut self) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_flush".to_string();

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glFlush()
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_framebuffer_renderbuffer(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        renderbuffer_target: RenderBufferTarget,
        renderbuffer: u32,
    ) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_framebuffer_renderbuffer".to_string();

        let mut param_info = ParamInfo::new("target", "FrameBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("attachment", "FrameBufferAttachmentType");
        param_infos.push(&param_info);
        param_values.push(&attachment);

        let mut param_info = ParamInfo::new("renderbuffer_target", "RenderBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&renderbuffer_target);

        let mut param_info = ParamInfo::new("renderbuffer", "u32");
        param_infos.push(&param_info);
        param_values.push(&renderbuffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glFramebufferRenderbuffer(target as GLenum,
                                               attachment as GLenum,
                                               renderbuffer_target as GLenum,
                                               renderbuffer as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_framebuffer_texture_2d(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        texture_target: TextureTarget,
        texture: u32,
        level: i32,
    ) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_framebuffer_texture_2d".to_string();

        let mut param_info = ParamInfo::new("target", "FrameBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("attachment", "FrameBufferAttachmentType");
        param_infos.push(&param_info);
        param_values.push(&attachment);

        let mut param_info = ParamInfo::new("texture_target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&texture_target);

        let mut param_info = ParamInfo::new("texture", "u32");
        param_infos.push(&param_info);
        param_values.push(&texture);

        let mut param_info = ParamInfo::new("level", "i32");
        param_infos.push(&param_info);
        param_values.push(&level);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glFramebufferTexture2D(target as GLenum,
                                            attachment as GLenum,
                                            texture_target as GLenum,
                                            texture as GLuint,
                                            level as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_front_face(&mut self, mode: FrontFaceDirection) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_front_face".to_string();

        let mut param_info = ParamInfo::new("mode", "FrontFaceDirection");
        param_infos.push(&param_info);
        param_values.push(&mode);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glFrontFace(mode as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_gen_buffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_gen_buffers".to_string();

        let mut param_info = ParamInfo::new("count", "u32");
        param_infos.push(&param_info);
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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_generate_mipmap(&mut self, target: TextureBindTarget) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_generate_mipmap".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glGenerateMipmap(target as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_gen_framebuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_gen_framebuffers".to_string();

        let mut param_info = ParamInfo::new("count", "u32");
        param_infos.push(&param_info);
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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_gen_renderbuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_gen_renderbuffers".to_string();

        let mut param_info = ParamInfo::new("count", "u32");
        param_infos.push(&param_info);
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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_gen_textures(&mut self, count: u32) -> Result<Vec<u32>, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_gen_textures".to_string();

        let mut param_info = ParamInfo::new("count", "u32");
        param_infos.push(&param_info);
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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_active_attrib(&mut self, program: u32, index: u32) -> Result<Active, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_active_attrib".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
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
                    Err(Error{})
                }
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_active_uniform(&mut self, program: u32, index: u32) -> Result<Active, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_active_uniform".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
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
                    Err(Error{})
                }
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_attached_shaders(&mut self, program: u32, max_count: i32) -> Result<Vec<u32>, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_attached_shaders".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("max_count", "i32");
        param_infos.push(&param_info);
        param_values.push(&max_count);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let mut count: GLsizei = 0;
                let mut vec: Vec<u32> = Vec::with_capacity(max_count as usize);

                ffi::glGetAttachedShaders(program as GLuint,
                                          max_count as GLsizei, &mut count,
                                          vec.as_mut_ptr());

                vec.set_len(count as usize);
                vec.truncate(count as usize);
                Ok(vec)
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_attrib_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_attrib_location".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("name", "&str");
        param_infos.push(&param_info);
        let param_value = name.to_string();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let c_str = CString::new(name).unwrap();

                let loc = ffi::glGetAttribLocation(program as GLuint, c_str.as_ptr() as *const c_char);

                Ok(loc as i32)
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_booleanv(&mut self, name: StateType) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_booleanv".to_string();

        let mut param_info = ParamInfo::new("name", "StateType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLboolean = 0;

            unsafe {
                ffi::glGetBooleanv(name as GLenum, &mut value);
            }

            Ok(value == GL_TRUE)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_buffer_parameteriv(&mut self, target: BufferTarget, name: BufferParamName) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_buffer_parameteriv".to_string();

        let mut param_info = ParamInfo::new("target", "BufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "BufferParamName");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetBufferParameteriv(target as GLenum, name as GLenum,
                                            &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_error(&mut self) -> ErrorType {
        let mut error = GL_NO_ERROR;

        unsafe {
            error = ffi::glGetError();
        }

        ErrorType::from(error)
    }pub fn gl_get_floatv(&mut self, name: StateType) -> Result<f32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_floatv".to_string();

        let mut param_info = ParamInfo::new("name", "StateType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetFloatv(name as GLenum, &mut value);
            }

            Ok(value as f32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_framebuffer_attachment_parameteriv(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        name: FrameBufferAttachmentParamType,
    ) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_framebuffer_attachment_parameteriv".to_string();

        let mut param_info = ParamInfo::new("target", "FrameBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("attachment", "FrameBufferAttachmentType");
        param_infos.push(&param_info);
        param_values.push(&attachment);

        let mut param_info = ParamInfo::new("name", "FrameBufferAttachmentParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetFramebufferAttachmentParameteriv(target as GLenum,
                                                           attachment as GLenum,
                                                           name as GLenum,
                                                           &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_integerv(&mut self, name: StateType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_integerv".to_string();

        let mut param_info = ParamInfo::new("name", "StateType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetIntegerv(name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_programiv(&mut self, program: u32, name: ProgramParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_programiv".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("name", "ProgramParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetProgramiv(program as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_program_info_log(&mut self, program: u32, max_length: i32) -> Result<String, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_program_info_log".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("max_length", "i32");
        param_infos.push(&param_info);
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
                    log.as_mut_vec().as_mut_ptr() as *mut u8,
                );

                if length > 0 {
                    log.as_mut_vec().set_len(length as usize);
                    log.truncate(length as usize);

                    Ok(log)
                } else {
                    Ok("".to_string())
                }
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_renderbuffer_parameteriv(&mut self, target: RenderBufferTarget,
                                           name: RenderBufferParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_renderbuffer_parameteriv".to_string();

        let mut param_info = ParamInfo::new("target", "RenderBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "RenderBufferParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetRenderbufferParameteriv(target as GLenum,
                                                  name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_shaderiv(&mut self, shader: u32, name: ShaderParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_shaderiv".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        let mut param_info = ParamInfo::new("name", "ShaderParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetShaderiv(shader as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_shader_info_log(&mut self, shader: u32, max_length: i32) -> Result<String, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_shader_info_log".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        let mut param_info = ParamInfo::new("max_length", "i32");
        param_infos.push(&param_info);
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
                    log.as_mut_vec().as_mut_ptr() as *mut u8,
                );

                if length > 0 {
                    log.as_mut_vec().set_len(length as usize);
                    log.truncate(length as usize);

                    Ok(log)
                } else {
                    Ok("".to_string())
                }
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_shader_precision_format(
        &mut self,
        shader_type: ShaderType,
        precision_type: ShaderPrecisionType,
    ) -> Result<ShaderPrecisionFormat, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_shader_precision_format".to_string();

        let mut param_info = ParamInfo::new("shader_type", "ShaderType");
        param_infos.push(&param_info);
        param_values.push(&shader_type);

        let mut param_info = ParamInfo::new("precision_type", "ShaderPrecisionType");
        param_infos.push(&param_info);
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
            })};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_shader_source(&mut self, shader: u32, max_length: i32) -> Result<String, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_shader_source".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        let mut param_info = ParamInfo::new("max_length", "i32");
        param_infos.push(&param_info);
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
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_string(&mut self, name: ConstantType) -> Result<String, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_string".to_string();

        let mut param_info = ParamInfo::new("name", "ConstantType");
        param_infos.push(&param_info);
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
                        Err(_) => Err(Error{}),
                    }
                } else {
                    // TODO: Ok is not proper ?
                    Ok("".to_string())
                }
            }};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_tex_parameterfv(&mut self, target: TextureTarget, name: TextureParamType) -> Result<f32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_tex_parameterfv".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetTexParameterfv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as f32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_tex_parameteriv(&mut self, target: TextureTarget, name: TextureParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_tex_parameteriv".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetTexParameteriv(target as GLenum, name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_uniformfv(&mut self, program: u32, location: i32) -> Result<f32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_uniformfv".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLfloat = 0.0;
            unsafe {
                ffi::glGetUniformfv(program as GLuint, location as GLint, &mut value);
            }

            Ok(value as f32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_uniformiv(&mut self, program: u32, location: i32) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_uniformiv".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetUniformiv(program as GLuint, location as GLint, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_uniform_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_uniform_location".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        let mut param_info = ParamInfo::new("name", "&str");
        param_infos.push(&param_info);
        let param_value = name.to_string();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut loc: GLint = 0;

            unsafe {
                let name_c_str = CString::new(name).unwrap();

                loc = ffi::glGetUniformLocation(program as GLuint, name_c_str.as_ptr() as *const c_char);
            }

            Ok(loc as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_vertex_attribfv(&mut self, index: u32, name: VertexAttributeParamType) -> Result<f32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_vertex_attribfv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("name", "VertexAttributeParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLfloat = 0.0;

            unsafe {
                ffi::glGetVertexAttribfv(index as GLuint, name as GLenum, &mut value);
            }

            Ok(value as f32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_get_vertex_attribiv(&mut self, index: u32, name: VertexAttributeParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_get_vertex_attribiv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("name", "VertexAttributeParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut value: GLint = 0;

            unsafe {
                ffi::glGetVertexAttribiv(index as GLuint, name as GLenum, &mut value);
            }

            Ok(value as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_hint(&mut self, target: HintTargetType, mode: HintBehaviorType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_hint".to_string();

        let mut param_info = ParamInfo::new("target", "HintTargetType");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("mode", "HintBehaviorType");
        param_infos.push(&param_info);
        param_values.push(&mode);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glHint(target as GLenum, mode as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_buffer(&mut self, buffer: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_buffer".to_string();

        let mut param_info = ParamInfo::new("buffer", "u32");
        param_infos.push(&param_info);
        param_values.push(&buffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsBuffer(buffer as GLuint) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_enabled(&mut self, feature: FeatureType) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_enabled".to_string();

        let mut param_info = ParamInfo::new("feature", "FeatureType");
        param_infos.push(&param_info);
        param_values.push(&feature);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsEnabled(feature as GLenum) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_framebuffer(&mut self, framebuffer: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_framebuffer".to_string();

        let mut param_info = ParamInfo::new("framebuffer", "u32");
        param_infos.push(&param_info);
        param_values.push(&framebuffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsFramebuffer(framebuffer as GLuint) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_program(&mut self, program: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_program".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsProgram(program as GLuint) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_renderbuffer(&mut self, renderbuffer: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_renderbuffer".to_string();

        let mut param_info = ParamInfo::new("renderbuffer", "u32");
        param_infos.push(&param_info);
        param_values.push(&renderbuffer);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsRenderbuffer(renderbuffer as u32) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_shader(&mut self, shader: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_shader".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsShader(shader as u32) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_is_texture(&mut self, texture: u32) -> Result<bool, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_is_texture".to_string();

        let mut param_info = ParamInfo::new("texture", "u32");
        param_infos.push(&param_info);
        param_values.push(&texture);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let mut res = false;

            unsafe {
                res = ffi::glIsTexture(texture as u32) == GL_TRUE;
            }

            Ok(res)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_line_width(&mut self, width: f32) -> Result<(), Error>  {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_line_width".to_string();

        let mut param_info = ParamInfo::new("width", "f32");
        param_infos.push(&param_info);
        param_values.push(&width);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glLineWidth(width as GLfloat);
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_link_program(&mut self, program: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_link_program".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glLinkProgram(program as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_pixel_storei(&mut self, name: PackParamType, param: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_pixel_storei".to_string();

        let mut param_info = ParamInfo::new("name", "PackParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        let mut param_info = ParamInfo::new("param", "i32");
        param_infos.push(&param_info);
        param_values.push(&param);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glPixelStorei(name as GLenum, param as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_polygon_offset(&mut self, factor: f32, units: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_polygon_offset".to_string();

        let mut param_info = ParamInfo::new("factor", "f32");
        param_infos.push(&param_info);
        param_values.push(&factor);

        let mut param_info = ParamInfo::new("units", "f32");
        param_infos.push(&param_info);
        param_values.push(&units);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glPolygonOffset(factor as GLfloat, units as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_read_pixels".to_string();

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("format", "PixelFormat");
        param_infos.push(&param_info);
        param_values.push(&format);

        let mut param_info = ParamInfo::new("type_", "PixelDataType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("buffer", "&mut [T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_release_shader_compiler(&mut self) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_release_shader_compiler".to_string();

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glReleaseShaderCompiler()
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_renderbuffer_storage(
        &mut self,
        target: RenderBufferTarget,
        internal_format: PixelFormat,
        width: i32,
        height: i32,
    ) -> Result<(), Error>  {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_renderbuffer_storage".to_string();

        let mut param_info = ParamInfo::new("target", "RenderBufferTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("internal_format", "PixelFormat");
        param_infos.push(&param_info);
        param_values.push(&internal_format);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glRenderbufferStorage(target as GLenum, internal_format as GLenum,
                                           width as GLsizei, height as GLsizei)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_sample_coverage(
        &mut self,
        value: f32,
        invert: bool) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_sample_coverage".to_string();

        let mut param_info = ParamInfo::new("value", "f32");
        param_infos.push(&param_info);
        param_values.push(&value);

        let mut param_info = ParamInfo::new("invert", "bool");
        param_infos.push(&param_info);
        param_values.push(&invert);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glSampleCoverage(value as GLclampf, invert as GLboolean)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_scissor(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_scissor".to_string();

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glScissor(x as GLint, y as GLint,
                               width as GLsizei, height as GLsizei)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_shader_binary<T>(&mut self, shaders: &[u32], data_format: GLenum,
                               data: &[T], length: i32) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_shader_binary".to_string();

        let mut param_info = ParamInfo::new("shaders", "&[u32]");
        param_infos.push(&param_info);
        let param_value = shaders.to_vec();
        param_values.push(&param_value);

        let mut param_info = ParamInfo::new("data_format", "GLenum");
        param_infos.push(&param_info);
        param_values.push(&data_format);

        let mut param_info = ParamInfo::new("data", "&[T]");
        param_infos.push(&param_info);
        let param_value = data.to_vec();
        param_values.push(&param_value);

        let mut param_info = ParamInfo::new("length", "i32");
        param_infos.push(&param_info);
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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_shader_source(&mut self, shader: u32, source: &str) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_shader_source".to_string();

        let mut param_info = ParamInfo::new("shader", "u32");
        param_infos.push(&param_info);
        param_values.push(&shader);

        let mut param_info = ParamInfo::new("source", "&str");
        param_infos.push(&param_info);
        let param_value = source.to_string();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                let length: GLsizei = source.len() as GLsizei;

                ffi::glShaderSource(shader as GLuint, 1,
                                    &(source.as_ptr() as *const GLchar), &length)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_func(&mut self, func: FuncType, ref_: i32, mask: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_func".to_string();

        let mut param_info = ParamInfo::new("func", "FuncType");
        param_infos.push(&param_info);
        param_values.push(&func);

        let mut param_info = ParamInfo::new("ref_", "i32");
        param_infos.push(&param_info);
        param_values.push(&ref_);

        let mut param_info = ParamInfo::new("mask", "u32");
        param_infos.push(&param_info);
        param_values.push(&mask);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilFunc(func as GLenum, ref_ as GLint, mask as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_func_separate(&mut self, face: FaceMode, func: FuncType,
                                    ref_: i32, mask: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_func_separate".to_string();

        let mut param_info = ParamInfo::new("face", "FaceMode");
        param_infos.push(&param_info);
        param_values.push(&face);

        let mut param_info = ParamInfo::new("func", "FuncType");
        param_infos.push(&param_info);
        param_values.push(&func);

        let mut param_info = ParamInfo::new("ref_", "i32");
        param_infos.push(&param_info);
        param_values.push(&ref_);

        let mut param_info = ParamInfo::new("mask", "u32");
        param_infos.push(&param_info);
        param_values.push(&mask);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilFuncSeparate(face as GLenum, func as GLenum,
                                           ref_ as GLint, mask as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_mask(&mut self, mask: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_mask".to_string();

        let mut param_info = ParamInfo::new("mask", "u32");
        param_infos.push(&param_info);
        param_values.push(&mask);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilMask(mask as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_mask_separate(&mut self, face: FaceMode, mask: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_mask_separate".to_string();

        let mut param_info = ParamInfo::new("face", "FaceMode");
        param_infos.push(&param_info);
        param_values.push(&face);

        let mut param_info = ParamInfo::new("mask", "u32");
        param_infos.push(&param_info);
        param_values.push(&mask);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilMaskSeparate(face as GLenum, mask as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_op(&mut self, s_fail: ActionType, dp_fail: ActionType, dp_pass: ActionType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_op".to_string();

        let mut param_info = ParamInfo::new("s_fail", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&s_fail);

        let mut param_info = ParamInfo::new("dp_fail", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&dp_fail);

        let mut param_info = ParamInfo::new("dp_pass", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&dp_pass);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilOp(s_fail as GLenum, dp_fail as GLenum, dp_pass as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_stencil_op_separate(&mut self, face: FaceMode, s_fail: ActionType,
                                  dp_fail: ActionType, dp_pass: ActionType) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_stencil_op_separate".to_string();

        let mut param_info = ParamInfo::new("face", "FaceMode");
        param_infos.push(&param_info);
        param_values.push(&face);

        let mut param_info = ParamInfo::new("s_fail", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&s_fail);

        let mut param_info = ParamInfo::new("dp_fail", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&dp_fail);

        let mut param_info = ParamInfo::new("dp_pass", "ActionType");
        param_infos.push(&param_info);
        param_values.push(&dp_pass);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glStencilOpSeparate(face as GLenum, s_fail as GLenum, dp_fail as GLenum,
                                         dp_pass as GLenum)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "i32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("internal_format", "GLint");
        param_infos.push(&param_info);
        param_values.push(&internal_format);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("border", "i32");
        param_infos.push(&param_info);
        param_values.push(&border);

        let mut param_info = ParamInfo::new("format", "PixelFormat");
        param_infos.push(&param_info);
        param_values.push(&format);

        let mut param_info = ParamInfo::new("type_", "PixelDataType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_tex_parameterf(&mut self, target: TextureBindTarget, name: TextureParamType,
                             value: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_parameterf".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        let mut param_info = ParamInfo::new("value", "f32");
        param_infos.push(&param_info);
        param_values.push(&value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glTexParameterf(target as GLenum, name as GLenum, value as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_tex_parameterfv(&mut self, target: TextureBindTarget,
                              name: TextureParamType) -> Result<f32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_parameterfv".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {


            let res: GLfloat = 0.0;
            unsafe {
                ffi::glTexParameterfv(target as GLenum, name as GLenum, &res)
            }

            Ok(res as f32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_tex_parameteri(&mut self, target: TextureBindTarget, name: TextureParamType,
                             value: GLint) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_parameteri".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        let mut param_info = ParamInfo::new("value", "GLint");
        param_infos.push(&param_info);
        param_values.push(&value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glTexParameteri(target as GLenum, name as GLenum, value)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_tex_parameteriv(&mut self, target: TextureBindTarget, name: TextureParamType) -> Result<i32, Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_parameteriv".to_string();

        let mut param_info = ParamInfo::new("target", "TextureBindTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("name", "TextureParamType");
        param_infos.push(&param_info);
        param_values.push(&name);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            let res: GLint = 0;

            unsafe {
                ffi::glTexParameteriv(target as GLenum, name as GLenum, &res)
            }

            Ok(res as i32)};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_tex_sub_image_2d".to_string();

        let mut param_info = ParamInfo::new("target", "TextureTarget");
        param_infos.push(&param_info);
        param_values.push(&target);

        let mut param_info = ParamInfo::new("level", "i32");
        param_infos.push(&param_info);
        param_values.push(&level);

        let mut param_info = ParamInfo::new("x_offset", "i32");
        param_infos.push(&param_info);
        param_values.push(&x_offset);

        let mut param_info = ParamInfo::new("y_offset", "i32");
        param_infos.push(&param_info);
        param_values.push(&y_offset);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        let mut param_info = ParamInfo::new("format", "PixelFormat");
        param_infos.push(&param_info);
        param_values.push(&format);

        let mut param_info = ParamInfo::new("type_", "PixelDataType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform1f(&mut self, location: i32, x: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform1f".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform1f(location as GLint, x as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform1fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform1fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform1fv(location as GLint, values.len() as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform1i(&mut self, location: i32, x: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform1i".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform1i(location as GLint, x as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform1iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform1iv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[i32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform1iv(location as GLint, values.len() as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform2f(&mut self, location: i32, x: f32, y: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform2f".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform2f(location as GLint, x as GLfloat, y as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform2fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform2fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform2fv(location as GLint, (values.len() / 2) as GLsizei,
                                  values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform2i(&mut self, location: i32, x: i32, y: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform2i".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform2i(location as GLint, x as GLint, y as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform2iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform2iv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[i32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform2iv(location as GLint, (values.len() / 2) as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform3f(&mut self, location: i32, x: f32, y: f32, z: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform3f".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "f32");
        param_infos.push(&param_info);
        param_values.push(&z);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform3f(location as GLint, x as GLfloat, y as GLfloat, z as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform3fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform3fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform3fv(location as GLint, (values.len() / 3) as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform3i(&mut self, location: i32, x: i32, y: i32, z: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform3i".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "i32");
        param_infos.push(&param_info);
        param_values.push(&z);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform3i(location as GLint, x as GLint, y as GLint, z as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform3iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform3iv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[i32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform3iv(location as GLint, (values.len() / 3) as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform4f(&mut self, location: i32, x: f32, y: f32, z: f32,
                        w: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform4f".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "f32");
        param_infos.push(&param_info);
        param_values.push(&z);

        let mut param_info = ParamInfo::new("w", "f32");
        param_infos.push(&param_info);
        param_values.push(&w);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform4f(location as GLint, x as GLfloat,
                                 y as GLfloat, z as GLfloat, w as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform4fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform4fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform4fv(location as GLint, (values.len() / 4) as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform4i(&mut self, location: i32, x: i32, y: i32, z: i32, w: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform4i".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "i32");
        param_infos.push(&param_info);
        param_values.push(&z);

        let mut param_info = ParamInfo::new("w", "i32");
        param_infos.push(&param_info);
        param_values.push(&w);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform4i(location as GLint, x as GLint, y as GLint, z as GLint, w as GLint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform4iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform4iv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("values", "&[i32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniform4iv(location as GLint, (values.len() / 4) as GLsizei, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform_matrix2fv(&mut self, location: i32, transpose: bool, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform_matrix2fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("transpose", "bool");
        param_infos.push(&param_info);
        param_values.push(&transpose);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniformMatrix2fv(
                    location as i32,
                    (values.len() / 2*2) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform_matrix3fv(&mut self, location: i32, transpose: bool, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform_matrix3fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("transpose", "bool");
        param_infos.push(&param_info);
        param_values.push(&transpose);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniformMatrix3fv(
                    location as GLint,
                    (values.len() / 3*3) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_uniform_matrix4fv(&mut self, location: i32, transpose: bool,
                                values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_uniform_matrix4fv".to_string();

        let mut param_info = ParamInfo::new("location", "i32");
        param_infos.push(&param_info);
        param_values.push(&location);

        let mut param_info = ParamInfo::new("transpose", "bool");
        param_infos.push(&param_info);
        param_values.push(&transpose);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUniformMatrix4fv(
                    location as GLint,
                    (values.len() / 4*4) as GLsizei,
                    transpose as GLboolean,
                    values.as_ptr() as *const GLfloat,
                )
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_use_program(&mut self, program: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_use_program".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glUseProgram(program as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_validate_program(&mut self, program: u32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_validate_program".to_string();

        let mut param_info = ParamInfo::new("program", "u32");
        param_infos.push(&param_info);
        param_values.push(&program);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glValidateProgram(program as GLuint)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib1f(&mut self, index: u32, x: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib1f".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib1f(index as GLuint, x as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib1fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib1fv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib1fv(index as GLuint, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib2f(&mut self, index: u32, x: f32, y: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib2f".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib2f(index as GLuint, x as GLfloat, y as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib2fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib2fv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib2fv(index as GLuint, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib3f(&mut self, index: u32, x: f32, y: f32, z: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib3f".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "f32");
        param_infos.push(&param_info);
        param_values.push(&z);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib3f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib3fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib3fv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib3fv(index as GLuint, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib4f(&mut self, index: u32, x: f32, y: f32, z: f32, w: f32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib4f".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("x", "f32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "f32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("z", "f32");
        param_infos.push(&param_info);
        param_values.push(&z);

        let mut param_info = ParamInfo::new("w", "f32");
        param_infos.push(&param_info);
        param_values.push(&w);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib4f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat,
                                      w as GLfloat)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib4fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib4fv".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("values", "&[f32]");
        param_infos.push(&param_info);
        let param_value = values.to_vec();
        param_values.push(&param_value);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glVertexAttrib4fv(index as GLuint, values.as_ptr())
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_vertex_attrib_pointer<T>(
        &mut self,
        index: u32,
        size: i32,
        type_: DataType,
        normalized: bool,
        stride: i32,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib_pointer".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("size", "i32");
        param_infos.push(&param_info);
        param_values.push(&size);

        let mut param_info = ParamInfo::new("type_", "DataType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("normalized", "bool");
        param_infos.push(&param_info);
        param_values.push(&normalized);

        let mut param_info = ParamInfo::new("stride", "i32");
        param_infos.push(&param_info);
        param_values.push(&stride);

        let mut param_info = ParamInfo::new("buffer", "&[T]");
        param_infos.push(&param_info);
        let param_value = buffer.to_vec();
        param_values.push(&param_value);

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

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
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


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_vertex_attrib_pointer_offset".to_string();

        let mut param_info = ParamInfo::new("index", "u32");
        param_infos.push(&param_info);
        param_values.push(&index);

        let mut param_info = ParamInfo::new("size", "i32");
        param_infos.push(&param_info);
        param_values.push(&size);

        let mut param_info = ParamInfo::new("type_", "DataType");
        param_infos.push(&param_info);
        param_values.push(&type_);

        let mut param_info = ParamInfo::new("normalized", "bool");
        param_infos.push(&param_info);
        param_values.push(&normalized);

        let mut param_info = ParamInfo::new("stride", "i32");
        param_infos.push(&param_info);
        param_values.push(&stride);

        let mut param_info = ParamInfo::new("offset", "u32");
        param_infos.push(&param_info);
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
                    offset as *const GLvoid)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }
    pub fn gl_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {


        let mut param_values: Vec<&Param> = vec![];
        let mut param_infos: Vec<&ParamInfo> = vec![];

        let mut func_info = FuncInfo::new();
        func_info.func_name = "gl_viewport".to_string();

        let mut param_info = ParamInfo::new("x", "i32");
        param_infos.push(&param_info);
        param_values.push(&x);

        let mut param_info = ParamInfo::new("y", "i32");
        param_infos.push(&param_info);
        param_values.push(&y);

        let mut param_info = ParamInfo::new("width", "i32");
        param_infos.push(&param_info);
        param_values.push(&width);

        let mut param_info = ParamInfo::new("height", "i32");
        param_infos.push(&param_info);
        param_values.push(&height);

        func_info.func_param_infos = param_infos;
        func_info.func_param_values = param_values;
        self.pre_process(&func_info)?;

        let res = {

            unsafe {
                ffi::glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei)
            }

            Ok(())};

        let res_desc = format!("{:?}", res);

        self.post_process(&func_info, &res_desc)?;

        res
    }

}
