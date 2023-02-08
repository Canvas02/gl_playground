// Copyright 2023 Canvas02 <Canvas02@protonmail.com>.
// SPDX-License-Identifier: MIT

use std::path::Path;

pub struct Texture<'a> {
    gl: &'a gl::Gl,
    id: u32,
    image_size: (u32, u32),
    color_channels: u8,
    label: Option<&'a str>,
}

impl<'a> Texture<'a> {
    pub fn from_raw(
        gl: &'a gl::Gl,
        image_size: (u32, u32),
        channels: u8,
        data: &[u8],
        label: Option<&'a str>,
    ) -> Result<Self, TextureError> {
        let (width, height) = image_size;
        let mut id = 0;

        if channels as usize * width as usize * height as usize != data.len() {
            return Err(TextureError::WrongSizedData);
        }

        unsafe {
            gl.CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            log::trace!(
                "Created Texture ({}) ({} x {}) with {} channels",
                id,
                width,
                height,
                channels
            );

            if let Some(label) = label {
                gl.ObjectLabel(gl::TEXTURE, id, label.len() as i32, label.as_ptr().cast());
                log::trace!("Adding label to Texture ({}): {}", id, label);
            }

            gl.TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            log::trace!(
                r#"Texture parameters:
                Wrap S: Repeat,
                Wrap T: Repeat,
                Min Filter: Linear,
                Mag Filter: Linear
                "#
            );

            let internal_format = match channels {
                3 => gl::RGB8,
                4 => gl::RGBA8,
                _ => return Err(TextureError::UnsupportedFormat),
            };
            let data_format = match channels {
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => return Err(TextureError::UnsupportedFormat),
            };

            gl.TextureStorage2D(id, 1, internal_format, width as i32, height as i32);
            gl.TextureSubImage2D(
                id,
                0,
                0,
                0,
                width as i32,
                height as i32,
                data_format,
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast(),
            );
        }

        Ok(Self {
            gl,
            id,
            color_channels: channels,
            label,
            image_size,
        })
    }

    pub fn from_file(
        gl: &'a gl::Gl,
        path: impl AsRef<Path>,
        label: Option<&'a str>,
    ) -> Result<Self, TextureError> {
        fn inner<'b>(
            gl: &'b gl::Gl,
            path: &Path,
            label: Option<&'b str>,
        ) -> Result<Texture<'b>, TextureError> {
            let image = image::open(path)?;
            let channels = match image {
                image::DynamicImage::ImageRgb8(..) => 3,
                image::DynamicImage::ImageRgba8(..) => 4,
                _ => return Err(TextureError::UnsupportedFormat),
            };

            Texture::from_raw(
                gl,
                (image.width(), image.height()),
                channels,
                image.as_bytes(),
                label,
            )
        }

        inner(gl, path.as_ref(), label)
    }

    pub fn from_memory(
        gl: &'a gl::Gl,
        data: &[u8],
        label: Option<&'a str>,
    ) -> Result<Self, TextureError> {
        let image = image::load_from_memory(data)?;
        let channels = match image {
            image::DynamicImage::ImageRgb8(..) => 3,
            image::DynamicImage::ImageRgba8(..) => 4,
            _ => return Err(TextureError::UnsupportedFormat),
        };

        Texture::from_raw(
            gl,
            (image.width(), image.height()),
            channels,
            image.as_bytes(),
            label,
        )
    }

    pub unsafe fn bind(&self, slot: u32) {
        self.gl.BindTextureUnit(slot, self.id);
    }

    pub unsafe fn unbind(&self, slot: u32) {
        self.gl.BindTextureUnit(slot, 0);
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn image_size(&self) -> (u32, u32) {
        self.image_size
    }
    pub fn color_channels(&self) -> u8 {
        self.color_channels
    }
    pub fn label(&self) -> Option<&'a str> {
        self.label
    }
}

impl Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("Image format not supported")]
    UnsupportedFormat,
    #[error("The size of the data doesn't match")]
    WrongSizedData,
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}
