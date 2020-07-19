use image::GenericImageView;
use std::os::raw::c_void;
use std::path::PathBuf;

pub struct Texture;

impl Texture {
    pub fn from_path(gl: &gl::Gl, path: PathBuf) -> u32 {
        let texture_image = image::open(path).expect("Failed to load texture.").flipv();
        let data = texture_image.to_bytes();

        unsafe {
            let mut texture = 0;

            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                                     // set the texture wrapping parameters

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // set texture filtering parameters
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                texture_image.width() as i32,
                texture_image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
            texture
        }
    }
}
