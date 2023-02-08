// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::ffi::CString;

pub struct Program<'a> {
    gl: &'a gl::Gl,
    id: u32,
    label: Option<&'a str>,
}

impl<'a> Program<'a> {
    pub fn from_source(
        gl: &'a gl::Gl,
        vertex: &str,
        fragment: &str,
        label: Option<&'a str>,
    ) -> Result<Self, ProgramError> {
        log::trace!("Shader: Compiling vertex shader: {}", vertex);
        let vertex = create_shader(
            gl,
            vertex,
            gl::VERTEX_SHADER,
            if let Some(label) = label {
                Some(format!("{} - vertex shader", label))
            } else {
                None
            },
        )?;

        log::trace!("Shader: Compiling fragment shader: {}", fragment);
        let fragment = create_shader(
            gl,
            fragment,
            gl::FRAGMENT_SHADER,
            if let Some(label) = label {
                Some(format!("{} - fragment shader", label))
            } else {
                None
            },
        )?;

        let id = unsafe { gl.CreateProgram() };
        if id == 0 {
            return Err(ProgramError::CreationError);
        }

        unsafe {
            gl.AttachShader(id, vertex);
            gl.AttachShader(id, fragment);

            gl.LinkProgram(id);

            gl.DetachShader(id, vertex);
            gl.DetachShader(id, fragment);

            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);

            let mut status = 0;
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as i32) {
                let mut info_log_len = 0;
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
                let mut buffer: Vec<u8> = Vec::with_capacity(info_log_len as usize);
                gl.GetProgramInfoLog(
                    id,
                    info_log_len,
                    &mut info_log_len,
                    buffer.as_mut_ptr().cast(),
                );
                buffer.set_len(info_log_len as usize);

                let info_log = String::from_utf8(buffer).expect("Failed to read info_log");
                // log::debug!("Failed to link shader program: info_log: {}", info_log);
                return Err(ProgramError::ShaderCompilationError(info_log));
            }

            if let Some(label) = label {
                gl.ObjectLabel(gl::PROGRAM, id, label.len() as i32, label.as_ptr().cast());
            }
        }

        Ok(Self { gl, id, label })
    }
}

impl<'a> Program<'a> {
    pub unsafe fn get_unifrom(&self, name: &str) -> Option<i32> {
        let c_string = CString::new(name).expect("Failed to make CString");

        let uniform = self.gl.GetUniformLocation(self.id, c_string.as_ptr());
        if uniform == -1 {
            return None;
        }

        return Some(uniform);
    }

    pub unsafe fn bind(&self) {
        self.gl.UseProgram(self.id);
    }

    pub unsafe fn unbind(&self) {
        self.gl.UseProgram(0);
    }

    pub fn label(&self) -> Option<&'a str> {
        self.label
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Program<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("Shader/Program creation failed")]
    CreationError,
    #[error("Shader compilation: {0}")]
    ShaderCompilationError(String),
    #[error("Program linkage: {0}")]
    ProgramLinkageError(String),
}

fn create_shader<'a>(
    gl: &'a gl::Gl,
    source: &str,
    shader_type: u32,
    label: Option<String>,
) -> Result<u32, ProgramError> {
    let id = unsafe { gl.CreateShader(shader_type) };
    if id == 0 {
        return Err(ProgramError::CreationError);
    } else {
        log::trace!("Create Shader object: {}", id);
    }

    unsafe {
        if let Some(label) = label {
            gl.ObjectLabel(gl::SHADER, id, label.len() as i32, label.as_ptr().cast());
            log::trace!("Adding label to Shader ({}): {}", id, label);
        }

        log::trace!("Adding Shader ({}) source", id);
        let c_source = CString::new(source).expect("Failed to make CString");
        gl.ShaderSource(id, 1, &c_source.as_ptr(), &(source.len() as i32));
        log::trace!("Added Shader ({}) source", id);
        gl.CompileShader(id);
        log::trace!("Compiling Shader ({})", id);

        let mut status: i32 = 0;
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as i32) {
            log::trace!("Shader ({}) compilation failed", id);

            log::trace!("Getting info_log length for Shader ({})", id);
            let mut info_log_len = 0;
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
            log::trace!("Info_log length for Shader ({}) is {}", id, info_log_len);

            log::trace!("Getting info_log for Shader ({})", id);
            let mut buffer: Vec<u8> = Vec::with_capacity(info_log_len as usize);
            gl.GetShaderInfoLog(
                id,
                info_log_len,
                &mut info_log_len,
                buffer.as_mut_ptr() as *mut _,
            );
            buffer.set_len(info_log_len as usize);

            let info_log = String::from_utf8(buffer).expect("Failed to read info_log");
            // log::debug!("Failed to compile shader: info_log: {}", info_log);

            return Err(ProgramError::ShaderCompilationError(info_log));
        }
    }

    Ok(id)
}
