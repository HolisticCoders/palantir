pub mod render_gl;
pub mod resources;

use nalgebra::{Matrix4, Perspective3, Vector3};
use render_gl::{Camera, Mesh, Renderer, Vertex};
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
        .window("Palantir", 900, 900)
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
        gl.Viewport(0, 0, 900, 900);
        gl.Enable(gl::DEPTH_TEST);
    }

    let mut shader_program =
        render_gl::Program::from_res(&gl, &resources, "shaders/default").unwrap();

    #[rustfmt::skip]
    let vertices: Vec<Vertex> = vec![
        Vertex { position: Vector3::new(0.5, 0.5, 0.5), color: Vector3::new(1.0, 0.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, 0.5), color: Vector3::new(0.0, 1.0, 0.0) },
        Vertex { position: Vector3::new(-0.5, 0.5, -0.5), color: Vector3::new(0.0, 0.0, 1.0) },
        Vertex { position: Vector3::new(0.5, 0.5, -0.5), color: Vector3::new(1.0, 1.0, 0.0) },

        Vertex { position: Vector3::new(0.5, -0.5, 0.5), color: Vector3::new(0.0, 1.0, 1.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, 0.5), color: Vector3::new(1.0, 0.0, 1.0) },
        Vertex { position: Vector3::new(-0.5, -0.5, -0.5), color: Vector3::new(1.0, 1.0, 1.0) },
        Vertex { position: Vector3::new(0.5, -0.5, -0.5), color: Vector3::new(0.0, 0.0, 0.0) },
    ];
    #[rustfmt::skip]
    let indices = vec![
        0, 1, 2, // top
        0, 2, 3,
        4, 5, 6, // bot
        4, 6, 7,
        0, 3, 7, // right
        0, 7, 4,
        2, 1, 5, // left
        2, 5, 6,
        3, 2, 6, // front
        3, 6, 7,
        1, 0, 4, // back
        1, 4, 5,
    ];
    let mesh = Mesh::new(&gl, vertices, indices);
    let model_matrix = Matrix4::<f32>::identity();

    let mut camera = Camera::new();

    let projection_matrix = Perspective3::new(1.0, 90.0, 0.1, 100.0);

    'main: loop {
        let mouse_state = sdl2::mouse::MouseState::new(&event_pump);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    const ORBIT_SENSITIVITY: f32 = 0.025;
                    const ZOOM_SENSITIVITY: f32 = 0.01;

                    let y_angle = xrel as f32 * ORBIT_SENSITIVITY;
                    let x_angle = yrel as f32 * ORBIT_SENSITIVITY;
                    let zoom_amount = xrel as f32 * ZOOM_SENSITIVITY;

                    if mouse_state.left() {
                        camera.rotate(Vector3::y_axis(), y_angle);
                        camera.rotate(Vector3::x_axis(), x_angle);
                    } else if mouse_state.right() {
                        camera.zoom(zoom_amount);
                    }
                }
                _ => {}
            }
        }

        shader_program.set_uniform_matrix4(String::from("model"), &model_matrix);
        shader_program.set_uniform_matrix4(String::from("view"), &camera.view_matrix());
        shader_program
            .set_uniform_matrix4(String::from("projection"), projection_matrix.as_matrix());

        Renderer::clear(&gl, 0.3, 0.3, 0.5);
        Renderer::draw(&gl, &mesh, &shader_program, gl::TRIANGLES);

        window.gl_swap_window();
    }
}
