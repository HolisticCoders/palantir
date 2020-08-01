use crate::Texture;
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ShaderError {
    CompileError { path: PathBuf, message: String },
    LinkError { path: PathBuf, message: String },
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShaderError::CompileError { path, message } => write!(
                f,
                "Shader {} failed to compile: {}",
                path.as_path().display().to_string(),
                message
            ),
            ShaderError::LinkError { path, message } => write!(
                f,
                "Program {} failed to link shaders: {}",
                path.as_path().display().to_string(),
                message
            ),
        }
    }
}

impl std::error::Error for ShaderError {}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    // pub fn from_res(res: &Resources, name: &str) -> Result<Shader, ShaderError> {
    //     const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
    //         [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

    //     let shader_kind = POSSIBLE_EXT
    //         .iter()
    //         .find(|&&(file_extension, _)| name.ends_with(file_extension))
    //         .map(|&(_, kind)| kind)
    //         .ok_or_else(|| ShaderError::CanNotDetermineShaderTypeForResource {
    //             name: name.into(),
    //         })?;

    //     let source = res
    //         .load_cstring(name)
    //         .map_err(|e| ShaderError::ResourceLoad {
    //             name: name.into(),
    //             inner: e,
    //         })?;

    //     Shader::from_source(gl, &source, shader_kind).map_err(|message| ShaderError::CompileError {
    //         name: name.into(),
    //         message,
    //     })
    // }

    pub fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    id: gl::types::GLuint,
    uniform_location_cache: HashMap<String, i32>,
    texture: Option<Texture>,
}

impl ShaderProgram {
    // pub fn from_res(res: &Resources, name: &str) -> Result<ShaderProgram, ShaderError> {
    //     const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

    //     let resource_names = POSSIBLE_EXT
    //         .iter()
    //         .map(|file_extension| format!("{}{}", name, file_extension))
    //         .collect::<Vec<String>>();

    //     let shaders = resource_names
    //         .iter()
    //         .map(|resource_name| Shader::from_res(res, resource_name))
    //         .collect::<Result<Vec<Shader>, ShaderError>>()?;

    //     ShaderProgram::from_shaders(&shaders[..]).map_err(|message| ShaderError::LinkError {
    //         name: name.into(),
    //         message,
    //     })
    // }

    pub fn from_path(path: PathBuf) -> Result<Self, ShaderError> {
        let mut shader_sources = HashMap::new();
        shader_sources.insert("vertex", String::new());
        shader_sources.insert("fragment", String::new());
        let file = File::open(path.clone()).expect("Couldn't open the file");

        let mut shader_type = None;
        for line in BufReader::new(file).lines() {
            if let Ok(content) = line {
                if content == "#vertex" {
                    shader_type = Some("vertex")
                } else if content == "#fragment" {
                    shader_type = Some("fragment")
                } else {
                    match shader_type {
                        Some(value) => {
                            let current_content = shader_sources.get_mut(&value).unwrap();
                            current_content.push_str(&content[..]);
                            current_content.push_str("\n");
                        }
                        None => (),
                    }
                }
            }
        }
        let vertex_source = shader_sources.get(&"vertex").unwrap();
        let vertex_cstr = CString::new(&vertex_source[..]).unwrap();
        let vertex_shader = Shader::from_source(&vertex_cstr, gl::VERTEX_SHADER).unwrap();

        let fragment_source = shader_sources.get(&"fragment").unwrap();
        let fragment_cstr = CString::new(&fragment_source[..]).unwrap();
        let fragment_shader = Shader::from_source(&fragment_cstr, gl::FRAGMENT_SHADER).unwrap();

        let shaders = [vertex_shader, fragment_shader];
        ShaderProgram::from_shaders(&shaders).map_err(|message| ShaderError::LinkError {
            path: path,
            message,
        })
    }
    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(ShaderProgram {
            id: program_id,
            uniform_location_cache: HashMap::new(),
            texture: None,
        })
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.set_uniform_bool("u_use_texture".to_string(), true);
        self.texture = Some(texture);
    }
    pub fn bind(&self) {
        match &self.texture {
            Some(texture) => texture.bind(),
            None => unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) },
        }
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniform_matrix4(&mut self, name: String, value: &Matrix4<f32>) {
        unsafe {
            let name = self.get_uniform_location(name);
            gl::UniformMatrix4fv(name, 1, gl::FALSE, value.as_ptr())
        }
    }
    pub fn set_uniform_bool(&mut self, name: String, value: bool) {
        unsafe {
            let name = self.get_uniform_location(name);
            gl::Uniform1ui(name, value as u32)
        }
    }
    pub fn set_uniform_float(&mut self, name: String, value: f32) {
        unsafe {
            let name = self.get_uniform_location(name);
            gl::Uniform1f(name, value)
        }
    }
    pub fn set_uniform_vector3(&mut self, name: String, value: &Vector3<f32>) {
        unsafe {
            let name = self.get_uniform_location(name);
            gl::Uniform3f(name, value.x, value.y, value.z)
        }
    }
    fn get_uniform_location(&mut self, name: String) -> i32 {
        let location: i32;

        match self.uniform_location_cache.get(&name) {
            Some(value) => {
                location = *value;
            }
            None => {
                unsafe {
                    location = gl::GetUniformLocation(
                        self.id,
                        CString::new(name.as_str()).unwrap().as_ptr(),
                    );
                }
                self.uniform_location_cache.insert(name, location);
            }
        }
        location
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
