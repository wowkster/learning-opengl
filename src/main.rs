#![allow(unused)]
#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

use std::ffi::{c_void, CStr};
use std::mem::{size_of, size_of_val};
use std::ptr::null;
use std::sync::RwLock;

use c_str_macro::c_str;
use gl::types::*;
use glfw::{fail_on_errors, SwapInterval, Window, WindowEvent};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use nalgebra_glm as glm;

use shader::Shader;
use texture::{ActiveTextureSlot, Texture2d};

mod shader;
mod texture;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[rustfmt::skip]
const VERTICES: [f32; 180] = [
    // vertices        // texture coords
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
];

#[rustfmt::skip]
const CUBE_POSITIONS: [[f32; 3]; 10] = [
    [  0.0,  0.0,  0.0  ],
    [  2.0,  5.0, -15.0 ],
    [ -1.5, -2.2, -2.5  ],
    [ -3.8, -2.0, -12.3 ],
    [  2.4, -0.4, -3.5  ],
    [ -1.7,  3.0, -7.5  ],
    [  1.3, -2.0, -2.5  ],
    [  1.5,  2.0, -2.5  ],
    [  1.5,  0.2, -1.5  ],
    [ -1.3,  1.0, -1.5  ],
];

const VERTEX_SHADER_SOURCE: &str = include_str!("../shaders/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../shaders/frag.glsl");

lazy_static::lazy_static! {
    static ref CAMERA_POS: RwLock<glm::Vec3> = RwLock::new(glm::vec3(0.0, 0.0, 3.0));
    static ref CAMERA_FRONT: RwLock<glm::Vec3> = RwLock::new(glm::vec3(0.0, 0.0, -1.0));
    static ref CAMERA_UP: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);
}

static mut CAMERA_FOV: f32 = 45.0;

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

    // Link Vertex Attributes
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    // Initialize Textures
    let container_texture = Texture2d::new("assets/container.jpg", texture::TextureFormat::RGB);
    let awesome_face_texture =
        Texture2d::new("assets/awesomeface.png", texture::TextureFormat::RGBA);

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
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Bind GL Objects
        container_texture.bind_to(ActiveTextureSlot::Texture0);
        awesome_face_texture.bind_to(ActiveTextureSlot::Texture1);

        unsafe {
            gl::BindVertexArray(vao);
        }

        // Create our view matrix
        let (pos, front, up) = (
            CAMERA_POS.read().unwrap(),
            CAMERA_FRONT.read().unwrap(),
            &CAMERA_UP,
        );
        let view = glm::look_at(&pos, &(*pos + *front), up);
        drop(pos);
        drop(front);

        // Create our projection matrix
        let proj = glm::perspective(
            SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
            f32::to_radians(unsafe { CAMERA_FOV }),
            0.1,
            100.0,
        );

        // Set Shader Uniforms
        shader_program.use_program();

        shader_program.set_i32("texture1", 0);
        shader_program.set_i32("texture2", 1);

        shader_program.set_mat4_f32("view", view);
        shader_program.set_mat4_f32("projection", proj);

        // Draw our cubes
        for (i, cube) in CUBE_POSITIONS.iter().enumerate() {
            // Create our model matrix
            let mut model = glm::identity::<f32, 4>();
            model = glm::translate(&model, &glm::make_vec3(cube));
            model = glm::rotate(
                &model,
                f32::to_radians(20.0 * i as f32),
                &glm::vec3(1.0, 0.3, 0.5),
            );

            shader_program.set_mat4_f32("model", model);

            // Draw the cube
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
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

    let camera_speed: f32 = 2.5 * unsafe { DELTA_TIME };

    let (mut pos, front, up) = (
        CAMERA_POS.write().unwrap(),
        CAMERA_FRONT.read().unwrap(),
        &CAMERA_UP,
    );

    if window.get_key(Key::W) == Action::Press {
        *pos += camera_speed * *front;
    }
    if window.get_key(Key::S) == Action::Press {
        *pos -= camera_speed * *front;
    }
    if window.get_key(Key::A) == Action::Press {
        *pos -= camera_speed * glm::normalize(&glm::cross(&front, up));
    }
    if window.get_key(Key::D) == Action::Press {
        *pos += camera_speed * glm::normalize(&glm::cross(&front, up));
    }

    if window.get_key(Key::Space) == Action::Press {
        *pos += camera_speed * **up;
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        *pos -= camera_speed * **up;
    }
}

fn mouse_callback(x: f32, y: f32) {
    static mut LAST_X: f32 = SCREEN_WIDTH as f32 / 2.0;
    static mut LAST_Y: f32 = SCREEN_HEIGHT as f32 / 2.0;

    static mut YAW: f32 = -90.0;
    static mut PITCH: f32 = 0.0;

    static mut FIRST_MOUSE: bool = true;

    unsafe {
        if FIRST_MOUSE {
            FIRST_MOUSE = false;
            LAST_X = x;
            LAST_Y = y;
        }

        let mut x_offset = x - LAST_X;
        let mut y_offset = LAST_Y - y;
        LAST_X = x;
        LAST_Y = y;

        const MOUSE_SENSITIVITY: f32 = 0.1;
        x_offset *= MOUSE_SENSITIVITY;
        y_offset *= MOUSE_SENSITIVITY;

        YAW += x_offset;
        PITCH += y_offset;

        if PITCH > 89.0 {
            PITCH = 89.0
        }
        if PITCH < -89.0 {
            PITCH = -89.0
        }

        let direction = glm::vec3(
            f32::cos(f32::to_radians(YAW)) * f32::cos(f32::to_radians(PITCH)),
            f32::sin(f32::to_radians(PITCH)),
            f32::sin(f32::to_radians(YAW)) * f32::cos(f32::to_radians(PITCH)),
        );

        let mut front = CAMERA_FRONT.write().unwrap();
        *front = glm::normalize(&direction);
    }
}

fn scroll_callback(x_offset: f32, y_offset: f32) {
    const SCROLL_SENSITIVITY: f32 = 1.0;

    const FOV_MIN: f32 = 1.0;
    const FOV_MAX: f32 = 90.0;

    unsafe {
        CAMERA_FOV -= y_offset * SCROLL_SENSITIVITY;

        if CAMERA_FOV < FOV_MIN {
            CAMERA_FOV = FOV_MIN;
        }
        if CAMERA_FOV > FOV_MAX {
            CAMERA_FOV = FOV_MAX;
        }
    }
}
