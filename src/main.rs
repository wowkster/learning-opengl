#![allow(unused)]

extern crate gl;
extern crate glfw;

use std::ffi::{c_void, CStr};
use std::mem::{size_of, size_of_val};
use std::ptr::null;

use gl::types::*;

use glfw::{fail_on_errors, Window, WindowEvent};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};

use c_str_macro::c_str;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [
    [-0.5, -0.5, 0.0], // top right
    [0.5, -0.5, 0.0],  // bottom right
    [0.0, 0.5, 0.0],   // bottom left
];

type Triangle = [u32; 3];
const INDICES: [Triangle; 1] = [[0, 1, 2]];

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/frag.glsl");

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Hint to GLFW what kind of window we want
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // Create a window for rendering
    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window the current GL context
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // Set the viewport and register a callback function for window resize events
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    window.set_framebuffer_size_callback(|width, height| unsafe {
        gl::Viewport(0, 0, width, height)
    });

    // Compile Shaders
    let vertex_shader = compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
    let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

    // Link Shaders
    let mut shader_program = link_shaders(vertex_shader, fragment_shader);

    // Initialize VAO
    let mut vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    // Initialize VBO
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }

    // Initialize EBO
    let mut ebo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&INDICES) as isize,
            INDICES.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }

    // Link Vertex Attributes
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>() as i32,
            null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    // Unbind Buffers
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Uncomment this to draw in wireframe mode
    // unsafe {
    //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // }

    // Main render loop
    while !window.should_close() {
        process_input(&mut window);

        // Draw the background
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Update shader uniforms
        unsafe {
            let time_value = glfw.get_time() as f32;
            let green_value = (f32::sin(time_value) / 2.0) + 0.5;
            let vertex_color_location =
                gl::GetUniformLocation(shader_program, c_str!("ourColor").as_ptr());

            gl::UseProgram(shader_program);
            gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);
        }

        // Draw our triangle
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }

        // Poll for events
        glfw.poll_events();

        // Swap the front and back buffers
        window.swap_buffers();
    }
}

fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }
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
