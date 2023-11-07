use std::{
    ffi::{CStr, CString},
    path::Path,
};

use gl::types::*;
use nalgebra_glm as glm;

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

    fn get_uniform_location(&self, name: &str) -> i32 {
        unsafe {
            let name = CString::new(name).expect("Could not convert name to CString");
            let uniform_location = gl::GetUniformLocation(self.id, name.as_ptr());

            if uniform_location < 0 {
                panic!("Could not set uniform: {}", name.to_str().unwrap());
            }

            uniform_location
        }
    }
}

macro_rules! impl_set_uniform_1 {
    ($type:ty, $func_name:ident, $gl_func:ident, $cast_type:ty) => {
        pub fn $func_name(&mut self, name: &str, value: $type) {
            unsafe {
                gl::$gl_func(self.get_uniform_location(name), value as $cast_type);
            }
        }
    };
}

macro_rules! impl_set_uniform_2 {
    ($type:ty, $func_name:ident, $gl_func:ident, $cast_type:ty) => {
        pub fn $func_name(&mut self, name: &str, v0: $type, v1: $type) {
            unsafe {
                gl::$gl_func(
                    self.get_uniform_location(name),
                    v0 as $cast_type,
                    v1 as $cast_type,
                );
            }
        }
    };
}

macro_rules! impl_set_uniform_3 {
    ($type:ty, $func_name:ident, $gl_func:ident, $cast_type:ty) => {
        pub fn $func_name(&mut self, name: &str, v0: $type, v1: $type, v2: $type) {
            unsafe {
                gl::$gl_func(
                    self.get_uniform_location(name),
                    v0 as $cast_type,
                    v1 as $cast_type,
                    v2 as $cast_type,
                );
            }
        }
    };
}

macro_rules! impl_set_uniform_4 {
    ($type:ty, $func_name:ident, $gl_func:ident, $cast_type:ty) => {
        pub fn $func_name(&mut self, name: &str, v0: $type, v1: $type, v2: $type, v3: $type) {
            unsafe {
                gl::$gl_func(
                    self.get_uniform_location(name),
                    v0 as $cast_type,
                    v1 as $cast_type,
                    v2 as $cast_type,
                    v3 as $cast_type,
                );
            }
        }
    };
}

macro_rules! impl_set_uniform_vector {
    ($type:ty, $func_name:ident, $gl_func:ident) => {
        pub fn $func_name(&mut self, name: &str, value: $type) {
            unsafe {
                gl::$gl_func(self.get_uniform_location(name), 1, value.as_ptr());
            }
        }
    };
}

macro_rules! impl_set_uniform_matrix {
    ($type:ty, $func_name:ident, $gl_func:ident) => {
        pub fn $func_name(&mut self, name: &str, value: $type) {
            unsafe {
                gl::$gl_func(
                    self.get_uniform_location(name),
                    1,
                    gl::FALSE,
                    value.as_ptr(),
                );
            }
        }
    };
}

impl Shader {
    impl_set_uniform_1!(f32, set_f32, Uniform1f, f32);
    impl_set_uniform_2!(f32, set_f32_2, Uniform2f, f32);
    impl_set_uniform_3!(f32, set_f32_3, Uniform3f, f32);
    impl_set_uniform_4!(f32, set_f32_4, Uniform4f, f32);

    impl_set_uniform_1!(i32, set_i32, Uniform1i, i32);
    impl_set_uniform_2!(i32, set_i32_2, Uniform2i, i32);
    impl_set_uniform_3!(i32, set_i32_3, Uniform3i, i32);
    impl_set_uniform_4!(i32, set_i32_4, Uniform4i, i32);

    impl_set_uniform_1!(u32, set_u32, Uniform1ui, u32);
    impl_set_uniform_2!(u32, set_u32_2, Uniform2ui, u32);
    impl_set_uniform_3!(u32, set_u32_3, Uniform3ui, u32);
    impl_set_uniform_4!(u32, set_u32_4, Uniform4ui, u32);

    impl_set_uniform_1!(bool, set_bool, Uniform1i, i32);
    impl_set_uniform_2!(bool, set_bool_2, Uniform2i, i32);
    impl_set_uniform_3!(bool, set_bool_3, Uniform3i, i32);
    impl_set_uniform_4!(bool, set_bool_4, Uniform4i, i32);

    impl_set_uniform_vector!(glm::Vec1, set_vec1, Uniform1fv);
    impl_set_uniform_vector!(glm::Vec2, set_vec2, Uniform2fv);
    impl_set_uniform_vector!(glm::Vec3, set_vec3, Uniform3fv);
    impl_set_uniform_vector!(glm::Vec4, set_vec4, Uniform4fv);

    impl_set_uniform_matrix!(glm::Mat2, set_mat2, UniformMatrix2fv);
    impl_set_uniform_matrix!(glm::Mat3, set_mat3, UniformMatrix3fv);
    impl_set_uniform_matrix!(glm::Mat4, set_mat4, UniformMatrix4fv);

    impl_set_uniform_matrix!(glm::Mat2x3, set_mat2x3, UniformMatrix2x3fv);
    impl_set_uniform_matrix!(glm::Mat3x2, set_mat3x2, UniformMatrix3x2fv);
    impl_set_uniform_matrix!(glm::Mat2x4, set_mat2x4, UniformMatrix2x4fv);
    impl_set_uniform_matrix!(glm::Mat4x2, set_mat4x2, UniformMatrix4x2fv);
    impl_set_uniform_matrix!(glm::Mat3x4, set_mat3x4, UniformMatrix3x4fv);
    impl_set_uniform_matrix!(glm::Mat4x3, set_mat4x3, UniformMatrix4x3fv);
}

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
