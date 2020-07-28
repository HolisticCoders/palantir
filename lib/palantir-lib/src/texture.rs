use image::GenericImageView;
use std::os::raw::c_void;
use std::path::PathBuf;

pub struct Texture {
    id: u32,
    height: i32,
    width: i32,
}

impl Texture {
    pub fn new(path: PathBuf) -> Self {
        let texture_image = image::open(path).expect("Failed to load texture.").flipv();
        let data = texture_image.to_bytes();

        let mut texture = Texture {
            id: 0,
            width: 0,
            height: 0,
        };
        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(gl::TEXTURE_2D, texture.id); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            texture.width = texture_image.width() as i32;
            texture.height = texture_image.height() as i32;

            // store in GPU Memory
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                texture.width,
                texture.height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
