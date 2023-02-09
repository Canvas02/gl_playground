// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const fn new(position: [f32; 3], uv: [f32; 2]) -> Self {
        Self { position, uv }
    }
}

pub const CUBE: [Vertex; 36] = [
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]),
    Vertex::new([0.5, -0.5, -0.5], [1.0, 0.0]),
    Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0]),
    //
    Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),
    Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0]),
    Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0]),
    Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
    //
    Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0]),
    //
    Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex::new([0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
    //
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0]),
    Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex::new([-0.5, -0.5, -0.5], [0.0, 1.0]),
    //
    Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
    Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0]),
    Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0]),
];
