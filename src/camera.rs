// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use glam::{Mat4, Vec3};

// Taken from https://learnopengl.com
pub struct Camera {
    position: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,

    yaw: f32,
    pitch: f32,

    speed: f32,
    sensitivity: f32,
    zoom: f32,

    forward: bool,
    backward: bool,
    left_movement: bool,
    right_movement: bool,
    // fast: bool,
    // slow: bool,
    last_mouse_pos: (f32, f32),
    first_click: bool,

    aspect_ratio: f32,

    view_matrix: Mat4,
    proj_matrix: Mat4,
    proj_view_matrix: Mat4,
}

impl Camera {
    pub fn proccess_movement(&mut self, dt: f32) {
        let velocity = dt * self.speed;
        if self.forward {
            self.position += self.front * velocity;
        }
        if self.backward {
            self.position -= self.front * velocity;
        }
        if self.right_movement {
            self.position += self.right * velocity;
        }
        if self.left_movement {
            self.position -= self.right * velocity;
        }
        // Uncomment for true FPS camera
        // self.position.y = 0.0;

        self.update_camera_matrices();
    }

    pub fn proccess_event(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, _, action, _) => {
                let pressed = *action == glfw::Action::Press || *action == glfw::Action::Repeat;
                match key {
                    glfw::Key::W => self.forward = pressed,
                    glfw::Key::S => self.backward = pressed,
                    glfw::Key::A => self.left_movement = pressed,
                    glfw::Key::D => self.right_movement = pressed,
                    // glfw::Key::LeftShift => self.fast = pressed,
                    // glfw::Key::LeftControl => self.slow = pressed,
                    _ => {}
                }
            }
            // Doing the movement in the event handler (no need for delta_time)
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let xpos = *xpos as f32;
                let ypos = *ypos as f32;

                if self.first_click {
                    self.last_mouse_pos.0 = xpos;
                    self.last_mouse_pos.1 = ypos;
                    self.first_click = false;
                }

                let mut xoffset = xpos - self.last_mouse_pos.0;
                let mut yoffset = self.last_mouse_pos.1 - ypos; // Y-coordinates go from bottom to top

                self.last_mouse_pos = (xpos, ypos);

                xoffset *= self.sensitivity;
                yoffset *= self.sensitivity;

                self.yaw += xoffset;
                self.pitch += yoffset;

                self.pitch = self.pitch.clamp(-89.0, 89.0);

                self.update_camera_vectors();
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                const SCROLL_SENSITIVITY: f32 = 4.0;
                let yoffset = *yoffset as f32 * SCROLL_SENSITIVITY;

                self.zoom -= yoffset;
                self.zoom = self.zoom.clamp(1.0, 120.0);

                self.update_camera_matrices();
            }
            _ => {}
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn proj_view_matrix(&self) -> Mat4 {
        self.proj_view_matrix
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
}

impl Camera {
    fn update_camera_vectors(&mut self) {
        self.front = Self::get_view_from_angles(self.yaw, self.pitch).normalize();

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();

        self.update_camera_matrices();
    }

    fn update_camera_matrices(&mut self) {
        self.view_matrix = self.generate_view_matrix();
        self.proj_matrix = self.generate_proj_matrix();

        self.proj_view_matrix = self.proj_matrix * self.view_matrix;
    }

    fn get_view_from_angles(yaw: f32, pitch: f32) -> Vec3 {
        let (sin_yaw, cos_yaw) = yaw.to_radians().sin_cos();
        let (sin_pitch, cos_pitch) = pitch.to_radians().sin_cos();

        glam::vec3(cos_yaw * cos_pitch, sin_pitch, sin_yaw * cos_pitch)
    }

    fn generate_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    fn generate_proj_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.zoom.to_radians(), self.aspect_ratio, 0.1, 100.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut s = Self {
            position: Vec3::ZERO,
            world_up: Vec3::Y,
            yaw: -90.0,
            pitch: 0.0,
            speed: 2.5,
            sensitivity: 0.1,
            zoom: 100.0,

            forward: false,
            backward: false,
            left_movement: false,
            right_movement: false,

            // Changed in update_camera_vectors
            front: Vec3::NEG_Z,
            right: Vec3::ZERO,
            up: Vec3::ZERO,

            last_mouse_pos: (0.0, 0.0),
            first_click: true,

            aspect_ratio: 16.0 / 9.0,

            // Changed in update_camera_matrices
            view_matrix: Mat4::ZERO,
            proj_matrix: Mat4::ZERO,
            proj_view_matrix: Mat4::ZERO,
        };
        s.update_camera_vectors();
        s.update_camera_matrices();

        s
    }
}
