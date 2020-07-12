use crate::resources::{self, Resources};
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub enum ShaderError {
    ResourceLoad {
        name: String,
        inner: resources::ResourceError,
    },
    CanNotDetermineShaderTypeForResource {
        name: String,
    },
    CompileError {
        name: String,
        message: String,
    },
    LinkError {
        name: String,
        message: String,
    },
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShaderError::ResourceLoad { name, .. } => {
                write!(f, "Could not load resource {}.", name)
            }
            ShaderError::CanNotDetermineShaderTypeForResource { name } => {
                write!(f, "Could not guess shader type from resource {}.", name)
            }
            ShaderError::CompileError { name, message } => {
                write!(f, "Shader {} failed to compile: {}", name, message)
            }
            ShaderError::LinkError { name, message } => {
                write!(f, "Program {} failed to link shaders: {}", name, message)
            }
        }
    }
}

impl std::error::Error for ShaderError {}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, ShaderError> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| ShaderError::CanNotDetermineShaderTypeForResource {
                name: name.into(),
            })?;

        let source = res
            .load_cstring(name)
            .map_err(|e| ShaderError::ResourceLoad {
                name: name.into(),
                inner: e,
            })?;

        Shader::from_source(gl, &source, shader_kind).map_err(|message| ShaderError::CompileError {
            name: name.into(),
            message,
        })
    }
    pub fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLuint,
    ) -> Result<Shader, String> {
        let id = shader_from_source(&gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    gl: gl::Gl,
    id: gl::types::GLuint,
    uniform_location_cache: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn from_res(
        gl: &gl::Gl,
        res: &Resources,
        name: &str,
    ) -> Result<ShaderProgram, ShaderError> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let resource_names = POSSIBLE_EXT
            .iter()
            .map(|file_extension| format!("{}{}", name, file_extension))
            .collect::<Vec<String>>();

        let shaders = resource_names
            .iter()
            .map(|resource_name| Shader::from_res(gl, res, resource_name))
            .collect::<Result<Vec<Shader>, ShaderError>>()?;

        ShaderProgram::from_shaders(gl, &shaders[..]).map_err(|message| ShaderError::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
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
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(ShaderProgram {
            gl: gl.clone(),
            id: program_id,
            uniform_location_cache: HashMap::new(),
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            self.gl.UseProgram(0);
        }
    }
    pub fn set_uniform_matrix4(&mut self, name: String, value: &Matrix4<f32>) {
        unsafe {
            let name = self.get_uniform_location(name);
            self.gl.UniformMatrix4fv(name, 1, gl::FALSE, value.as_ptr())
        }
    }
    pub fn set_uniform_float(&mut self, name: String, value: f32) {
        unsafe {
            let name = self.get_uniform_location(name);
            self.gl.Uniform1f(name, value)
        }
    }
    pub fn set_uniform_vector3(&mut self, name: String, value: &Vector3<f32>) {
        unsafe {
            let name = self.get_uniform_location(name);
            self.gl.Uniform3f(name, value.x, value.y, value.z)
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
                    // TODO: refacto as_str.unwrap.as_ptr
                    location = self
                        .gl
                        .GetUniformLocation(self.id, CString::new(name.as_str()).unwrap().as_ptr());
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
            self.gl.DeleteProgram(self.id);
        }
    }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLuint,
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl.CreateShader(kind) };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
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
