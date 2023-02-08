// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

pub struct Camera {
    position: glam::Vec3,
    view_dir: glam::Vec3,
    look_x: f32,
    look_y: f32,
    up: glam::Vec3,
    view_matrix: glam::Mat4,

    velocity: glam::Vec3,
    speed: f32,
    sensitivity: f32,

    forward: bool,
    backward: bool,
    left: bool,
    right: bool,

    fast: bool,
    slow: bool,

    last_mouse_pos: (f32, f32),
}

impl Camera {
    pub fn new(
        position: glam::Vec3,
        up: glam::Vec3,
        look_x: Option<f32>,
        look_y: Option<f32>,
        speed: f32,
        sensitivity: f32,
    ) -> Self {
        let look_x = look_x.unwrap_or(-90.0);
        let look_y = look_y.unwrap_or(0.0);

        let view_dir = Self::get_view_from_angles(look_x, look_y);
        let view_matrix = Self::generate_view_matrix(position, view_dir, up);

        Self {
            position,
            look_x,
            look_y,
            up,
            view_dir,
            view_matrix,
            sensitivity,
            speed,
            velocity: glam::Vec3::ZERO,

            forward: false,
            backward: false,
            left: false,
            right: false,
            fast: false,
            slow: false,

            last_mouse_pos: (0.0, 0.0),
        }
    }

    // Stolen from https://github.com/BoyBaykiller/IDKEngine/blob/f01828d8992dba25fd39d97d0aea622c07b3a528/IDKEngine/src/Camera.cs
    pub fn proccess_movement(&mut self, dt: f32) {
        // TODO: Handle mouse movment

        let mut accel = glam::Vec3::ZERO;

        if self.forward {
            accel += self.view_dir;
            // log::trace!("Camera: Moving forward");
        } else if self.backward {
            accel -= self.view_dir;
            // log::trace!("Camera: Moving backward");
        } else if self.right {
            accel += self.view_dir.cross(self.up).normalize();
            // log::trace!("Camera: Moving right");
        } else if self.left {
            accel -= self.view_dir.cross(self.up).normalize();
            // log::trace!("Camera: Moving left");
        }

        accel *= 144.0;

        self.velocity *= ((0.95f32).log10() * 144.0 * dt).exp();
        self.position += dt * self.velocity * self.speed + 0.5 * accel * dt * dt;
        self.velocity += if self.fast {
            accel * 5.0
        } else if self.slow {
            accel * 0.25
        } else {
            accel
        } * dt;

        if self.velocity.dot(self.velocity) < 0.01 {
            self.velocity = glam::Vec3::ZERO;
        }

        self.view_matrix = Self::generate_view_matrix(self.position, self.view_dir, self.up);
    }

    pub fn proccess_event(&mut self, event: &glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, _, action, _) => {
                let pressed = *action == glfw::Action::Press || *action == glfw::Action::Repeat;
                match key {
                    glfw::Key::W => self.forward = pressed,
                    glfw::Key::S => self.backward = pressed,
                    glfw::Key::A => self.left = pressed,
                    glfw::Key::D => self.right = pressed,
                    glfw::Key::LeftShift => self.fast = pressed,
                    glfw::Key::LeftControl => self.slow = pressed,
                    _ => {}
                }
            }
            // Doing the movement in the event handler (can't think of anything better)
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let xoffset = *xpos as f32 - self.last_mouse_pos.0;
                let yoffset = *ypos as f32 - self.last_mouse_pos.1;

                self.look_x += xoffset * self.sensitivity;
                self.look_y += yoffset * self.sensitivity;

                self.look_y = self.look_x.clamp(-90.0, 90.0);

                self.view_dir = Camera::get_view_from_angles(self.look_x, self.look_y);
            }
            _ => {}
        }
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        self.view_matrix
    }
}

impl Camera {
    pub fn generate_view_matrix(
        position: glam::Vec3,
        view_dir: glam::Vec3,
        up: glam::Vec3,
    ) -> glam::Mat4 {
        glam::Mat4::look_at_rh(position, position + view_dir, up)
    }

    pub fn get_view_from_angles(look_x: f32, look_y: f32) -> glam::Vec3 {
        let (sin_x, cos_x) = look_x.to_radians().sin_cos();
        let (sin_y, cos_y) = look_y.to_radians().sin_cos();

        glam::vec3(cos_x * cos_y, sin_y, sin_x * cos_y)
    }
}
