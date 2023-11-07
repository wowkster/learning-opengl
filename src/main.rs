#![allow(unused)]
#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

use std::ffi::{c_void, CStr};
use std::mem::{size_of, size_of_val};
use std::ptr::null;

use gl::types::*;

use glfw::{fail_on_errors, Window, WindowEvent};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};

use c_str_macro::c_str;
use shader::Shader;

mod shader;

#[rustfmt::skip]
const VERTICES: [f32; 32] = [
    // positions      // colors        // texture coords
    0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
    0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
   -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
   -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
];

const INDICES: [u32; 6] = [
    0, 1, 3,
    1, 2, 3
];

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

    // Initialize the shader program
    let mut shader_program = Shader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

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
            8 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (6 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(2);
    }

    // Initialize Textures
    let mut container_texture: u32 = 0;
    unsafe {
        // Generate the texture object
        gl::GenTextures(1, &mut container_texture);
        gl::BindTexture(gl::TEXTURE_2D, container_texture);

        // Set the texture wrapping/filtering options (on the currently bound texture object)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load and generate the texture data
        let container_image = image::io::Reader::open("assets/container.jpg")
            .expect("Failed to load texture file")
            .decode()
            .expect("Failed to decode texture file");

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            container_image.width() as i32,
            container_image.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            container_image.as_bytes().as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let mut awesome_face_texture: u32 = 0;
    unsafe {
        // Generate the texture object
        gl::GenTextures(1, &mut awesome_face_texture);
        gl::BindTexture(gl::TEXTURE_2D, awesome_face_texture);

        // Set the texture wrapping/filtering options (on the currently bound texture object)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load and generate the texture data
        let awesome_face_image = image::io::Reader::open("assets/awesomeface.png")
            .expect("Failed to load texture file")
            .decode()
            .expect("Failed to decode texture file")
            .flipv();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            awesome_face_image.width() as i32,
            awesome_face_image.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            awesome_face_image.as_bytes().as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    
    // Unbind Buffers
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
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

        // Draw our triangle
        unsafe {
            shader_program.use_program();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, container_texture);
            
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, awesome_face_texture);
            
            shader_program.set_i32("texture1", 0);
            shader_program.set_i32("texture2", 1);

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
