// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

#[cfg(debug_assertions)]
use std::ptr::null;
use std::{
    ffi::CStr,
    mem::{size_of, size_of_val},
    sync::mpsc::Receiver,
};

use gl_playground::{camera::Camera, program::Program, texture::Texture};
use glfw::Context;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const fn new(position: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position,
            color,
            uv,
        }
    }
}

const TRIANGLE: [Vertex; 4] = [
    Vertex::new([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
    Vertex::new([0.5, -0.5, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
    Vertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
    Vertex::new([-0.5, 0.5, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0]),
];

const TRIANGLE_IN: [u32; 6] = [0, 1, 2, 0, 3, 2];

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

fn main() {
    // Always log trace and up in debug
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Trace)
                .try_init()
                .expect("Log: Failed to initialize env_logger");

            log::info!("Program: Running debug build")
        } else {
            env_logger::try_init().expect("Log: Failed to initialize env_logger");
        }
    }

    // Create a glfw context
    let mut glfw_context = glfw::init(glfw::LOG_ERRORS).expect("Failed to init glfw");
    log::debug!("GLFW: Created context");

    // Create a glfw window
    glfw_context.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw_context.window_hint(glfw::WindowHint::Resizable(false));
    glfw_context.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    #[cfg(debug_assertions)]
    glfw_context.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let (mut window, event_receiver) = glfw_context
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "Gl Playground",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window");
    log::debug!("GLFW: Created window");

    window.set_all_polling(true);
    window.make_current();
    log::debug!("GLFW Window: Made window current");

    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    log::debug!("GL: Loaded functions?");

    unsafe {
        #[cfg(debug_assertions)]
        gl.DebugMessageCallback(Some(gl_debug_callback), null());

        gl.Viewport(0, 0, SCR_WIDTH as i32, SCR_HEIGHT as i32);
    }

    log::debug!("GL: Vendor: {}", unsafe {
        CStr::from_ptr(gl.GetString(gl::VENDOR) as *const _)
            .to_str()
            .unwrap()
    });

    log::debug!("GL: Version: {}", unsafe {
        CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_str()
            .unwrap()
    });

    let program = Program::from_source(
        &gl,
        include_str!("shaders/basic.vert"),
        include_str!("shaders/basic.frag"),
        Some("Basic Shader"),
    )
    .expect("Failed to create shader program");
    log::debug!("GL: Built program successfully");

    let texture = Texture::from_memory(
        &gl,
        include_bytes!("../assets/brick.webp"),
        Some("Brick wall"),
    )
    .expect("Failed to load texture");

    unsafe {
        gl.ClearColor(0.2, 0.2, 0.2, 1.0);
        gl.Enable(gl::DEPTH_TEST);

        let mut vao = 0;
        gl.CreateVertexArrays(1, &mut vao);

        let mut buffer = 0;
        gl.CreateBuffers(1, &mut buffer);
        gl.NamedBufferStorage(
            buffer,
            size_of_val(&TRIANGLE) as isize,
            TRIANGLE.as_ptr().cast(),
            gl::DYNAMIC_STORAGE_BIT,
        );
        gl.ObjectLabel(gl::BUFFER, buffer, -1, ("Triangle\0").as_ptr().cast());

        let mut index_buffer = 0;
        gl.CreateBuffers(1, &mut index_buffer);
        gl.NamedBufferStorage(
            index_buffer,
            size_of_val(&TRIANGLE_IN) as isize,
            TRIANGLE_IN.as_ptr().cast(),
            gl::DYNAMIC_STORAGE_BIT,
        );
        gl.ObjectLabel(
            gl::BUFFER,
            index_buffer,
            -1,
            ("Triangle - index\0").as_ptr().cast(),
        );

        gl.VertexArrayVertexBuffer(vao, 0, buffer, 0, size_of::<Vertex>() as i32);
        gl.VertexArrayElementBuffer(vao, index_buffer);

        gl.EnableVertexArrayAttrib(vao, 0);
        gl.EnableVertexArrayAttrib(vao, 1);
        gl.EnableVertexArrayAttrib(vao, 2);

        gl.VertexArrayAttribFormat(
            vao,
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            bytemuck::offset_of!(Vertex, position) as u32,
        );
        gl.VertexArrayAttribFormat(
            vao,
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            bytemuck::offset_of!(Vertex, color) as u32,
        );
        gl.VertexArrayAttribFormat(
            vao,
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            bytemuck::offset_of!(Vertex, uv) as u32,
        );

        gl.VertexArrayAttribBinding(vao, 0, 0);
        gl.VertexArrayAttribBinding(vao, 1, 0);
        gl.VertexArrayAttribBinding(vao, 2, 0);

        let mut camera = Camera::new(
            glam::vec3(0.0, 0.0, 1.0),
            glam::Vec3::Y,
            None,
            None,
            0.25,
            0.01,
        );

        let proj = glam::Mat4::perspective_rh_gl(
            100.0f32.to_radians(),
            SCR_WIDTH as f32 / SCR_HEIGHT as f32,
            0.1,
            100.0,
        );

        let proj_view_loc = program.get_unifrom("uProjView").unwrap();

        let mut current_time = glfw_context.get_time();
        let mut last_time = 0.0f64;
        let mut delta_time;

        // Camera Fix
        // window.set_cursor_pos_polling(false);

        log::debug!("GLFW Window: Starting game loop");
        while !window.should_close() {
            delta_time = current_time - last_time;
            last_time = current_time;
            current_time = glfw_context.get_time();

            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.BindVertexArray(vao);
            program.bind();
            texture.bind(0);

            let proj_view = proj * camera.view_matrix();
            gl.UniformMatrix4fv(proj_view_loc, 1, gl::FALSE, &proj_view.to_cols_array()[0]);

            glfw_context.poll_events();
            handle_events(&gl, &mut window, &event_receiver, &mut camera);

            camera.proccess_movement(delta_time as f32);

            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl.DrawElements(
                gl::TRIANGLES,
                TRIANGLE_IN.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            window.swap_buffers();
        }
        log::debug!("GLFW Window: Ended game loop");

        gl.DeleteBuffers(1, &buffer);
        gl.DeleteBuffers(1, &index_buffer);
        gl.DeleteVertexArrays(1, &vao);
    }

    log::info!("Program: End");
}

fn handle_events(
    gl: &gl::Gl,
    window: &mut glfw::Window,
    receiver: &Receiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(&receiver) {
        camera.proccess_event(&event);
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::FramebufferSize(w, h) => {
                unsafe {
                    gl.Viewport(0, 0, w, h);
                }
                // log::trace!("GLFW Window: Resized framebuffer to {}x{}", w, h);
            }
            _ => {}
        }
    }
}

#[cfg(debug_assertions)]
extern "system" fn gl_debug_callback(
    source: u32,
    type_: u32,
    id: u32,
    severity: u32,
    _: i32, // length
    message: *const i8,
    _: *mut std::ffi::c_void, // user pointer
) {
    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => "!UNKNOWN",
    };

    let type_ = match type_ {
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOR",
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PUSH_GROUP => "PUSH_GROUP",
        gl::DEBUG_TYPE_POP_GROUP => "POP_GROUP",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
        _ => "!UNKNOWN",
    };

    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "!UNKNOWN",
    };

    if let Ok(message) = unsafe { CStr::from_ptr(message as *const _) }.to_str() {
        let message = format!(
            "OpenGL: [source: {}][type: {}][severity: {}][id: {}] {}",
            source, type_, severity_str, id, message
        );

        match severity {
            gl::DEBUG_SEVERITY_HIGH => log::error!("{}", message),
            gl::DEBUG_SEVERITY_MEDIUM => log::warn!("{}", message),
            gl::DEBUG_SEVERITY_LOW => log::warn!("{}", message),
            gl::DEBUG_SEVERITY_NOTIFICATION => log::info!("{}", message),
            _ => log::debug!("{}", message),
        }
    } else {
        log::error!("OpenGL: Failed to convert message from pointer to str");
    }
}