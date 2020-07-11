pub mod render_gl;
pub mod resources;

use nalgebra::Vector3;
use render_gl::{Mesh, Renderer, Vertex};
use resources::Resources;
use std::path::Path;

fn main() {
    let resources = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

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
    }

    let shader_program = render_gl::Program::from_res(&gl, &resources, "shaders/triangle").unwrap();

    #[rustfmt::skip]
    let vertices: Vec<Vertex> = vec![
        Vertex {position: Vector3::new(0.5, -0.5, 0.0), color: Vector3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vector3::new(-0.5, -0.5, 0.0),color: Vector3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vector3::new(0.0, 0.5, 0.0),  color: Vector3::new(0.0, 0.0, 1.0)}
    ];
    let indices = vec![0, 1, 2];
    let mesh = Mesh::new(&gl, vertices, indices);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        Renderer::draw(
            &gl,
            &mesh,
            &shader_program,
            gl::TRIANGLES,
        );
        Renderer::clear(&gl, 0.3, 0.3, 0.5);

        window.gl_swap_window();
    }
}
