use std::{
    ffi::{CStr, CString},
    path::Path,
};

use gl::types::*;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        // Compile Shaders
        let vertex_shader = compile_shader(gl::VERTEX_SHADER, vertex_source);
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, fragment_source);

        // Link Shaders
        let shader_program = link_shaders(vertex_shader, fragment_shader);

        Self { id: shader_program }
    }

    pub fn from_files<P: AsRef<Path>>(vertex_path: P, fragment_path: P) -> Self {
        use std::fs;

        let vertex_source = fs::read_to_string(&vertex_path).unwrap_or_else(|_| {
            panic!(
                "Could not read vertex shader file: {}",
                vertex_path.as_ref().display()
            )
        });
        let fragment_source = fs::read_to_string(fragment_path).unwrap_or_else(|_| {
            panic!(
                "Could not read fragment shader file: {}",
                vertex_path.as_ref().display()
            )
        });

        Self::new(&vertex_source, &fragment_source)
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub unsafe fn id(&self) -> GLuint {
        self.id
    }
}

macro_rules! impl_set_uniform {
    ($type:ty, $func_name:ident, $gl_func:ident, $cast_type:ty) => {
        impl Shader {
            pub fn $func_name(&mut self, name: &str, value: $type) {
                unsafe {
                    let name = CString::new(name).expect("Could not convert name to CString");
                    let uniform_location = gl::GetUniformLocation(self.id, name.as_ptr());

                    if uniform_location < 0 {
                        panic!("Could not set uniform: {}", name.to_str().unwrap());
                    }
                    
                    gl::$gl_func(uniform_location, value as $cast_type);
                }
            }
        }
    };
}

impl_set_uniform!(bool, set_bool, Uniform1i, i32);
impl_set_uniform!(i32, set_i32, Uniform1i, i32);
impl_set_uniform!(f32, set_f32, Uniform1f, f32);

fn compile_shader(type_: GLenum, source: &str) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(type_);
        gl::ShaderSource(shader, 1, &(source.as_ptr().cast()), &(source.len() as i32));
        gl::CompileShader(shader);

        // Check for shader compile errors
        let mut success: i32 = 0;
        let mut info_log: [i8; 1024] = [0; 1024];
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        // If compilation failed, print the error message
        if success == 0 {
            gl::GetShaderInfoLog(shader, 1024, &mut 0, info_log.as_mut_ptr());
            panic!(
                "ERROR::SHADER::COMPILATION_FAILED\n{}",
                CStr::from_ptr(info_log.as_ptr())
                    .to_str()
                    .expect("Could not convert GL compilation error to string")
            );
        }

        shader
    }
}

fn link_shaders(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for shader linking errors
        let mut success: i32 = 0;
        let mut info_log: [i8; 1024] = [0; 1024];
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        // If linking failed, print the error message
        if success == 0 {
            gl::GetProgramInfoLog(shader_program, 1024, &mut 0, info_log.as_mut_ptr());
            panic!(
                "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                CStr::from_ptr(info_log.as_ptr())
                    .to_str()
                    .expect("Could not convert GL compilation error to string")
            );
        }

        // Delete Now Unneeded Shader Objects
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}
