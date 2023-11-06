#![allow(unused)]

extern crate gl;
extern crate glfw;

use gl::types::*;

use glfw::{fail_on_errors, Window, WindowEvent};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    
    // Hint to GLFW what kind of window we want
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
        
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

    // Main render loop
    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        window.swap_buffers();
    }
}

fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }
}