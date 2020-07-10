pub mod render_gl;
pub mod resources;

use nalgebra::Vector3;
use render_gl::Vertex;
use resources::Resources;
use std::path::Path;

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Palantir", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    #[rustfmt::skip]
    let vertices: Vec<Vertex> = vec![
        Vertex {position: Vector3::new(0.5, -0.5, 0.0), color: Vector3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vector3::new(-0.5, -0.5, 0.0),color: Vector3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vector3::new(0.0, 0.5, 0.0),  color: Vector3::new(0.0, 0.0, 1.0)}
    ];

    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);

        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,                                                          // target
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );

        gl.GenVertexArrays(1, &mut vao);

        gl.BindVertexArray(vao);

        Vertex::vertex_attrib_pointer(&gl);

        // Unbind before the main loop
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}
