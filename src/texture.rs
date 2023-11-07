use std::path::Path;

use gl::types::*;

pub struct Texture2d {
    id: GLuint,
}

#[allow(clippy::upper_case_acronyms)]
#[repr(u32)]
pub enum TextureFormat {
    RGB = gl::RGB,
    RGBA = gl::RGBA,
}

#[repr(u32)]
pub enum TextureWrap {
    Repeat = gl::REPEAT,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
}

#[repr(u32)]
pub enum TextureFilter {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
pub enum ActiveTextureSlot {
    Texture0 = gl::TEXTURE0,
    Texture1 = gl::TEXTURE1,
    Texture2 = gl::TEXTURE2,
    Texture3 = gl::TEXTURE3,
    Texture4 = gl::TEXTURE4,
    Texture5 = gl::TEXTURE5,
    Texture6 = gl::TEXTURE6,
    Texture7 = gl::TEXTURE7,
    Texture8 = gl::TEXTURE8,
    Texture9 = gl::TEXTURE9,
    Texture10 = gl::TEXTURE10,
    Texture11 = gl::TEXTURE11,
    Texture12 = gl::TEXTURE12,
    Texture13 = gl::TEXTURE13,
    Texture14 = gl::TEXTURE14,
    Texture15 = gl::TEXTURE15,
}

impl Texture2d {
    pub fn new<P: AsRef<Path>>(path: P, format: TextureFormat) -> Self {
        let mut texture: u32 = 0;

        unsafe {
            // Generate the texture object
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // Set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                TextureWrap::Repeat as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                TextureWrap::Repeat as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                TextureFilter::LinearMipmapLinear as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                TextureFilter::Linear as i32,
            );

            // Load and generate the texture data
            let img = image::io::Reader::open(path)
                .expect("Failed to load texture file")
                .decode()
                .expect("Failed to decode texture file");

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                TextureFormat::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format as u32,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr().cast(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self { id: texture }
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn bind_to(&self, active_texture_slot: ActiveTextureSlot) {
        unsafe {
            gl::ActiveTexture(active_texture_slot as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn set_wrap_s(wrap: TextureWrap) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap as i32);
        }
    }

    pub fn set_wrap_t(wrap: TextureWrap) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap as i32);
        }
    }

    pub fn set_min_filter(filter: TextureFilter) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter as i32);
        }
    }

    pub fn set_mag_filter(filter: TextureFilter) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as i32);
        }
    }
}
