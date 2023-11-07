#![allow(unused)]
#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

use std::ffi::{c_void, CStr};
use std::mem::{size_of, size_of_val};
use std::ptr::null;
use std::sync::RwLock;

use c_str_macro::c_str;
use camera::CameraMovement;
use gl::types::*;
use glfw::{fail_on_errors, SwapInterval, Window, WindowEvent};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use nalgebra_glm as glm;

use shader::Shader;
use texture::{ActiveTextureSlot, Texture2d};

use crate::camera::Camera;

mod camera;
mod shader;
mod texture;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[rustfmt::skip]
const VERTICES: [f32; 108] = [
    // vertices
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,

    -0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,

    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,

     0.5,  0.5,  0.5,
     0.5,  0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5,  0.5,

    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,

    -0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
];

const CUBE_POSITION: [f32; 3] = [0.0, 0.0, 0.0];
const LIGHT_POSITION: [f32; 3] = [1.2, 1.0, 2.0];

lazy_static::lazy_static! {
    static ref CAMERA: RwLock<Camera> = {
        let mut camera = Camera::default();
        camera.position = glm::vec3(0.0, 0.0, 3.0);

        RwLock::new(camera)
    };
}

static mut DELTA_TIME: f32 = 0.0;
static mut LAST_FRAME_TIME: f32 = 0.0;

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
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window the current GL context
    window.make_current();

    // Enable Mouse Input
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // Set the viewport and register a callback function for window resize events
    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    }

    window.set_framebuffer_size_callback(|width, height| unsafe {
        gl::Viewport(0, 0, width, height)
    });

    // Set the mouse input callbacks
    window.set_cursor_pos_callback(|x, y| mouse_callback(x as f32, y as f32));
    window.set_scroll_callback(|x, y| scroll_callback(x as f32, y as f32));

    // Initialize the shader programs
    let mut cube_shader = Shader::new(
        include_str!("../shaders/vert.glsl"),
        include_str!("../shaders/cube.frag.glsl"),
    );
    let mut light_shader = Shader::new(
        include_str!("../shaders/vert.glsl"),
        include_str!("../shaders/light.frag.glsl"),
    );

    // Initialize Cube VAO
    let mut cube_vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::BindVertexArray(cube_vao);
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

    // Link Vertex Attributes
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }

    // Initialize Light VAO
    let mut light_vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }

    // Unbind Buffers
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    // Enable Depth Testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Main render loop
    while !window.should_close() {
        // Poll for events
        glfw.poll_events();

        // Calculate Frame Times
        let current_time = glfw.get_time() as f32;
        unsafe {
            DELTA_TIME = current_time - LAST_FRAME_TIME;
            LAST_FRAME_TIME = current_time;
        }

        // Check window events
        process_input(&mut window);

        // Draw the background
        unsafe {
            gl::ClearColor(25.0 / 255.0, 25.0 / 255.0, 25.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Create our view matrix
        let camera = CAMERA.read().unwrap();
        let view = camera.get_view_matrix();

        // Create our projection matrix
        let proj = glm::perspective(
            SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
            f32::to_radians(camera.fov),
            0.1,
            100.0,
        );

        // Render the cube
        {
            // Create our model matrix
            let mut model = glm::identity::<f32, 4>();
            model = glm::translate(&model, &glm::make_vec3(&CUBE_POSITION));

            // Set Shader Uniforms
            cube_shader.use_program();

            cube_shader.set_mat4("model", model);
            cube_shader.set_mat4("view", view);
            cube_shader.set_mat4("projection", proj);

            cube_shader.set_vec3("objectColor", glm::vec3(1.0, 0.5, 0.31));
            cube_shader.set_vec3("lightColor", glm::vec3(1.0, 1.0, 1.0));

            // Draw the cube
            unsafe {
                gl::BindVertexArray(cube_vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // Render the light
        {
            // Create our model matrix
            let mut model = glm::identity::<f32, 4>();
            model = glm::translate(&model, &glm::make_vec3(&LIGHT_POSITION));
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));

            // Set Shader Uniforms
            light_shader.use_program();

            light_shader.set_mat4("model", model);
            light_shader.set_mat4("view", view);
            light_shader.set_mat4("projection", proj);

            // Draw the cube
            unsafe {
                gl::BindVertexArray(light_vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // Swap the front and back buffers
        window.swap_buffers();
    }
}

fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
        return;
    }

    let delta_time = unsafe { DELTA_TIME };

    let mut camera = CAMERA.write().unwrap();

    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(CameraMovement::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(CameraMovement::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(CameraMovement::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(CameraMovement::Right, delta_time);
    }
    if window.get_key(Key::Space) == Action::Press {
        camera.process_keyboard(CameraMovement::Up, delta_time);
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        camera.process_keyboard(CameraMovement::Down, delta_time);
    }
}

fn mouse_callback(x: f32, y: f32) {
    static mut LAST_X: f32 = SCREEN_WIDTH as f32 / 2.0;
    static mut LAST_Y: f32 = SCREEN_HEIGHT as f32 / 2.0;

    static mut FIRST_MOUSE: bool = true;

    let mut camera = CAMERA.write().unwrap();

    unsafe {
        if FIRST_MOUSE {
            FIRST_MOUSE = false;
            LAST_X = x;
            LAST_Y = y;
        }

        let x_offset = x - LAST_X;
        let y_offset = LAST_Y - y;

        LAST_X = x;
        LAST_Y = y;

        camera.process_mouse_movement(x_offset, y_offset, true);
    }
}

fn scroll_callback(x_offset: f32, y_offset: f32) {
    let mut camera = CAMERA.write().unwrap();

    camera.process_mouse_scroll(y_offset)
}
